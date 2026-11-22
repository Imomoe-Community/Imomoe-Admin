use crate::{
    config::{Config, Storage},
    engine, s2sdk as sdk, storage,
};
use imomoe_admin_core::{
    command::COMMANDS,
    punishment::{Duration, Kind, Mutation, Punishment, PunishmentStore},
    target::{Actor, Player},
};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const OP: &str = "imomoe.admin.chat";

#[derive(Clone, Copy)]
struct Movement {
    handle: i32,
    prior: sdk::MoveType,
    written: sdk::MoveType,
}

pub struct State {
    pub config: Config,
    config_root: PathBuf,
    data_root: PathBuf,
    pub store: Box<dyn PunishmentStore>,
    hidden: HashMap<i32, (u64, bool)>,
    reports: HashMap<u64, Instant>,
    movement: HashMap<i32, Movement>,
    voice: HashMap<(i32, i32), (u64, u64, sdk::ListenOverride)>,
    announcement_index: usize,
    announcement_time: Instant,
    voice_time: Instant,
}

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

impl State {
    pub fn new(config_root: PathBuf, data_root: PathBuf) -> Result<Self> {
        let config = Config::load(&config_root)?;
        let store = match &config.admin.storage {
            Storage::Sqlite { path } => storage::sqlite(&data_root.join(path))?,
            Storage::File {} => storage::files(&data_root.join("data"))?,
        };
        let instant = Instant::now();
        Ok(Self {
            config,
            config_root,
            data_root,
            store,
            hidden: HashMap::new(),
            reports: HashMap::new(),
            movement: HashMap::new(),
            voice: HashMap::new(),
            announcement_index: 0,
            announcement_time: instant,
            voice_time: instant,
        })
    }

    pub fn reload(&mut self) -> Result<()> {
        let config = Config::load(&self.config_root)?;
        // A configuration reload must not silently swap the database that enforces bans.
        if config.admin.storage != self.config.admin.storage {
            return Err("changing storage requires restarting the plugin".into());
        }
        self.config = config;
        self.announcement_index = 0;
        self.announcement_time = Instant::now();
        Ok(())
    }

    pub fn reply(&self, actor: Actor, key: &str, detail: &str) {
        let lang = engine::phrase_language(actor);
        let phrase = self.config.phrase(lang.as_str(), key);
        if detail.is_empty() {
            engine::send(actor, phrase);
        } else {
            engine::send(actor, &format!("{phrase} {detail}"));
        }
    }

    pub fn is_hidden(&self, slot: i32) -> bool {
        engine::account(slot).is_some_and(|id| {
            self.hidden
                .get(&slot)
                .filter(|&&(account, _)| account == id)
                .map_or(self.config.admin.hide_on_join, |&(_, hidden)| hidden)
        }) && is_op(Actor::Player(slot))
    }

    pub fn toggle_hidden(&mut self, slot: i32) -> bool {
        let Some(id) = engine::account(slot) else {
            return false;
        };
        let hidden = !self.is_hidden(slot);
        self.hidden.insert(slot, (id, hidden));
        hidden
    }

    pub fn notify(&self, label: &str, text: &str) {
        for slot in 0..sdk::GetMaxClients() {
            let actor = Actor::Player(slot);
            if engine::allowed(actor, OP) {
                self.reply(actor, label, text);
            }
        }
    }

    pub fn report(&mut self, player: &Player, message: &str, received_at: Instant) -> Result<bool> {
        let id = player
            .steam_id
            .ok_or("report requires an authenticated Steam player")?;
        let cooldown = self.config.admin.report_cooldown_seconds;
        self.reports
            .retain(|_, t| received_at.duration_since(*t).as_secs() < cooldown);
        if self.reports.contains_key(&id) {
            return Ok(false);
        }
        let path = self.data_root.join("data/reports.log");
        append(
            &path,
            &serde_json::json!({ "time": now(), "steam_id": id, "name": player.name, "message": message }),
        )?;
        self.reports.insert(id, received_at);
        self.notify("report_prefix", &format!("{}: {message}", player.name));
        Ok(true)
    }

    pub fn audit(&self, actor: Actor, command: &str, target: &str) {
        if !self.config.admin.audit_log {
            return;
        }
        let entry = serde_json::json!({ "time": now(), "actor": engine::actor_id(actor), "command": command, "target": target });
        if let Err(error) = append(&self.data_root.join("data/audit.log"), &entry) {
            plugify::error!("imomoe_admin audit: {}", error);
        }
    }

    pub fn apply(
        &mut self,
        actor: Actor,
        id: u64,
        kinds: &[Kind],
        duration: Duration,
        reason: &str,
    ) -> Result<Vec<Punishment>> {
        let time = now();
        let expires_at = duration.expiration(time).ok_or("duration overflow")?;
        let rows: Vec<_> = kinds
            .iter()
            .map(|&kind| Punishment {
                steam_id: id,
                kind,
                actor: engine::actor_id(actor),
                reason: reason.to_owned(),
                created_at: time,
                expires_at,
            })
            .collect();
        self.store.mutate(Mutation::Apply(&rows), time)?;
        if kinds.contains(&Kind::Mute) {
            self.sync_voice()?;
        }
        Ok(rows)
    }

    pub fn revoke(&mut self, actor: Actor, id: u64, kinds: &[Kind]) -> Result<usize> {
        let changed = self.store.mutate(
            Mutation::Revoke {
                steam_id: id,
                kinds,
                actor: engine::actor_id(actor),
            },
            now(),
        )?;
        if kinds.contains(&Kind::Mute) {
            self.sync_voice()?;
        }
        Ok(changed)
    }

    pub fn has(&self, slot: i32, kind: Kind) -> Result<bool> {
        let Some(id) = engine::account(slot) else {
            return Ok(false);
        };
        Ok(self
            .store
            .active(id, now())?
            .iter()
            .any(|row| row.kind == kind))
    }

    pub fn enforce_ban(&mut self, slot: i32, id: u64) -> Result<()> {
        let ban = match self.ban_for_connected(slot, id) {
            Ok(ban) => ban,
            Err(error) => {
                let lang = sdk::GetClientLanguageCode(slot);
                self.cleanup_slot(slot);
                engine::kick(
                    slot,
                    self.config.phrase(lang.as_str(), "admission_unavailable"),
                );
                return Err(error);
            }
        };
        if let Some(row) = ban {
            self.disconnect_banned(slot, &row);
        }
        Ok(())
    }

    pub fn ban_for_connected(&self, slot: i32, id: u64) -> Result<Option<Punishment>> {
        if engine::connected_account(slot) != Some(id) {
            return Ok(None);
        }
        Ok(self
            .store
            .active(id, now())?
            .into_iter()
            .find(|row| row.kind == Kind::Ban))
    }

    pub fn disconnect_banned(&mut self, slot: i32, row: &Punishment) {
        let lang = sdk::GetClientLanguageCode(slot);
        let phrase = |key| self.config.phrase(lang.as_str(), key);
        let expiry = row.expires_at.map_or_else(
            || phrase("permanent").to_owned(),
            |time| format!("{} {}", phrase("expires_unix"), time),
        );
        let message = format!("{} {} ({expiry})", phrase("banned"), row.reason);
        self.cleanup_slot(slot);
        sdk::KickClient(
            slot,
            sdk::NetworkDisconnectionReason::RejectBanned,
            &plugify::Str::from_str(&message),
        );
    }

    pub fn set_movement(&mut self, slot: i32, mode: sdk::MoveType) -> bool {
        let Some(handle) = engine::pawn(slot) else {
            return false;
        };
        if let Some(owned) = self.movement.get(&slot) {
            if owned.handle == handle
                && owned.written == mode
                && sdk::GetEntityMoveType(handle) == mode
            {
                return false;
            }
            self.restore_movement(slot);
        }
        let prior = sdk::GetEntityMoveType(handle);
        if prior == mode {
            return false;
        }
        sdk::SetEntityMoveType(handle, mode);
        if sdk::GetEntityMoveType(handle) != mode {
            return false;
        }
        self.movement.insert(
            slot,
            Movement {
                handle,
                prior,
                written: mode,
            },
        );
        true
    }

    pub fn restore_movement(&mut self, slot: i32) -> bool {
        let Some(owned) = self.movement.remove(&slot) else {
            return false;
        };
        if sdk::IsValidEntHandle(owned.handle)
            && sdk::GetEntityMoveType(owned.handle) == owned.written
        {
            sdk::SetEntityMoveType(owned.handle, owned.prior);
            return true;
        }
        false
    }

    pub fn owns_noclip(&self, slot: i32, handle: i32) -> bool {
        self.movement.get(&slot).is_some_and(|owned| {
            owned.handle == handle
                && owned.written == sdk::MoveType::Noclip
                && sdk::IsValidEntHandle(handle)
                && sdk::GetEntityMoveType(handle) == owned.written
        })
    }

    pub fn toggle_noclip(&mut self, slot: i32) -> bool {
        if engine::pawn(slot).is_some_and(|handle| self.owns_noclip(slot, handle)) {
            self.restore_movement(slot)
        } else {
            self.set_movement(slot, sdk::MoveType::Noclip)
        }
    }

    pub fn unfreeze(&mut self, slot: i32) -> bool {
        self.movement
            .get(&slot)
            .is_some_and(|owned| owned.written == sdk::MoveType::None)
            && self.restore_movement(slot)
    }

    pub fn deleted(&mut self, handle: i32) {
        self.movement.retain(|_, owned| owned.handle != handle);
    }

    pub fn cleanup_slot(&mut self, slot: i32) {
        self.restore_movement(slot);
        self.hidden.remove(&slot);
        let pairs: Vec<_> = self
            .voice
            .keys()
            .copied()
            .filter(|&(receiver, sender)| receiver == slot || sender == slot)
            .collect();
        for pair in pairs {
            self.restore_voice(pair);
        }
    }

    fn restore_voice(&mut self, key: (i32, i32)) {
        let Some((receiver, sender, before)) = self.voice.remove(&key) else {
            return;
        };
        let same = |slot, id| sdk::IsClientConnected(slot) && sdk::GetClientSteamID64(slot) == id;
        if same(key.0, receiver)
            && same(key.1, sender)
            && sdk::GetClientListening(key.0, key.1) == sdk::ListenOverride::Mute
        {
            sdk::SetClientListening(key.0, key.1, before);
        }
    }

    fn sync_voice(&mut self) -> Result<()> {
        let humans: HashMap<_, _> = (0..sdk::GetMaxClients())
            .filter_map(|slot| engine::account(slot).map(|id| (slot, id)))
            .collect();
        let mut muted = HashSet::new();
        let time = now();
        for (&slot, &id) in &humans {
            if self
                .store
                .active(id, time)?
                .iter()
                .any(|row| row.kind == Kind::Mute)
            {
                muted.insert(slot);
            }
        }
        let stale: Vec<_> = self
            .voice
            .iter()
            .filter_map(|(&key, &(receiver, sender, _))| {
                (humans.get(&key.0) != Some(&receiver)
                    || humans.get(&key.1) != Some(&sender)
                    || !muted.contains(&key.1))
                .then_some(key)
            })
            .collect();
        for pair in stale {
            self.restore_voice(pair);
        }
        for sender in muted {
            for (&receiver, &receiver_id) in &humans {
                if receiver == sender {
                    continue;
                }
                let key = (receiver, sender);
                let prior = sdk::GetClientListening(receiver, sender);
                if prior != sdk::ListenOverride::Mute {
                    sdk::SetClientListening(receiver, sender, sdk::ListenOverride::Mute);
                    self.voice
                        .insert(key, (receiver_id, humans[&sender], prior));
                }
            }
        }
        Ok(())
    }

    pub fn frame(&mut self) {
        if self.voice_time.elapsed().as_millis() >= 250 {
            self.voice_time = Instant::now();
            if let Err(error) = self.sync_voice() {
                plugify::error!("imomoe_admin voice refresh: {}", error);
            }
            // A replacement pawn, even in the same slot and connection, never inherits ownership.
            let stale: Vec<_> = self
                .movement
                .iter()
                .filter_map(|(&slot, owned)| {
                    (!engine::online(slot)
                        || !sdk::IsClientAlive(slot)
                        || engine::pawn(slot) != Some(owned.handle))
                    .then_some(slot)
                })
                .collect();
            for slot in stale {
                self.restore_movement(slot);
            }
        }
        let announcements = &self.config.announcements;
        if !announcements.messages.is_empty()
            && self.announcement_time.elapsed().as_secs() >= announcements.interval_seconds
        {
            let message = &announcements.messages[self.announcement_index];
            for slot in 0..sdk::GetMaxClients() {
                if engine::account(slot).is_some() {
                    engine::community_send(Actor::Player(slot), message);
                }
            }
            self.announcement_index = (self.announcement_index + 1) % announcements.messages.len();
            self.announcement_time = Instant::now();
        }
    }

    pub fn clear_map(&mut self) {
        for slot in self.movement.keys().copied().collect::<Vec<_>>() {
            self.restore_movement(slot);
        }
        for pair in self.voice.keys().copied().collect::<Vec<_>>() {
            self.restore_voice(pair);
        }
        self.hidden.clear();
    }
}

pub fn is_op(actor: Actor) -> bool {
    COMMANDS
        .iter()
        .flat_map(|cmd| cmd.permissions)
        .any(|node| engine::allowed(actor, node))
}

fn append(path: &std::path::Path, entry: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut line = serde_json::to_vec(entry)?;
    line.push(b'\n');
    file.write_all(&line)?;
    Ok(())
}
