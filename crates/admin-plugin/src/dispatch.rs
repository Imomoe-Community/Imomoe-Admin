use crate::{
    engine, s2sdk as sdk,
    state::{self, Result, State},
};
use imomoe_admin_core::{
    command::Command,
    punishment::{Duration, Kind},
    target::{self, Actor, Player, Policy, TargetError, Team},
};
use plugify::{Str, Vec3};

pub fn run(state: &mut State, actor: Actor, cmd: &Command, args: &[String]) {
    if !cmd
        .permissions
        .iter()
        .all(|node| engine::allowed(actor, node))
    {
        state.reply(actor, "denied", "");
        return;
    }
    let args = selector_args(cmd, args);
    if let Err(error) = dispatch(state, actor, cmd, &args) {
        plugify::error!("imomoe_admin {}: {}", cmd.name, error);
        state.reply(actor, "failed", "");
    }
}

/// Valve command tokenization can split unquoted #userid:N and #slot:N at the
/// colon. Repair only target arguments; reasons and private chat stay as sent.
fn selector_args<'a>(cmd: &Command, args: &'a [String]) -> std::borrow::Cow<'a, [String]> {
    fn split(args: &[String], at: usize) -> Option<String> {
        let [selector, colon, id, ..] = args.get(at..)? else {
            return None;
        };
        if (selector.eq_ignore_ascii_case("#userid") || selector.eq_ignore_ascii_case("#slot"))
            && colon == ":"
            && !id.is_empty()
            && id.bytes().all(|b| b.is_ascii_digit())
        {
            Some(format!("{selector}:{id}"))
        } else {
            None
        }
    }

    let mut out = std::borrow::Cow::Borrowed(args);
    if (cmd.usage.starts_with("<target>") || cmd.name == "goto")
        && let Some(selector) = split(&out, 0)
    {
        out.to_mut().splice(0..3, [selector]);
    }
    if cmd.name == "tp"
        && let Some(selector) = split(&out, 1)
    {
        out.to_mut().splice(1..4, [selector]);
    }
    out
}

fn dispatch(state: &mut State, actor: Actor, cmd: &Command, args: &[String]) -> Result<()> {
    let arg = |index: usize| args.get(index).map(String::as_str);
    let player_slot = match actor {
        Actor::Player(slot) => Some(slot),
        Actor::Console => None,
    };
    let usage = |state: &State| {
        state.reply(
            actor,
            "usage",
            &format!("imomoe_{} {}", cmd.name, cmd.usage),
        )
    };
    let exact_count = match cmd.name {
        "comms" | "who" | "hide" | "pause" | "unpause" | "reload" | "admins" | "rules" | "help" => {
            Some(0)
        }
        "unban" | "unmute" | "ungag" | "unsilence" | "slay" | "swap" | "respawn" | "bring"
        | "goto" | "freeze" | "unfreeze" | "noclip" => Some(1),
        "team" | "tp" => Some(2),
        _ => None,
    };
    if exact_count.is_some_and(|count| args.len() != count) {
        usage(state);
        return Ok(());
    }
    match cmd.name {
        "adminchat" | "report" => {
            let message = engine::clean(&args.join(" "));
            if message.trim().is_empty() {
                usage(state);
            } else if cmd.name == "adminchat" {
                let who = player_slot.map_or_else(
                    || "Console".into(),
                    |slot| engine::clean(sdk::GetClientName(slot).as_str()),
                );
                state.notify("admin_prefix", &format!("{who}: {message}"));
                if actor == Actor::Console {
                    state.reply(actor, "sent", "");
                }
            } else if let Some(player) = engine::players()
                .into_iter()
                .find(|p| Some(p.slot) == player_slot)
            {
                let sent = state.report(&player, &message, std::time::Instant::now())?;
                state.reply(actor, if sent { "sent" } else { "report_cooldown" }, "");
            } else {
                state.reply(actor, "player_only", "");
            }
            return Ok(());
        }
        "admins" | "who" => {
            let privileged = cmd.name == "who";
            let mut count = 0;
            for p in engine::players() {
                let target = Actor::Player(p.slot);
                if !state::is_op(target) || (!privileged && state.is_hidden(p.slot)) {
                    continue;
                }
                count += 1;
                if privileged {
                    let mut nodes: Vec<_> = imomoe_admin_core::command::COMMANDS
                        .iter()
                        .flat_map(|cmd| cmd.permissions.iter().copied())
                        .filter(|node| engine::allowed(target, node))
                        .collect();
                    nodes.sort_unstable();
                    nodes.dedup();
                    let badge = if state.is_hidden(p.slot) {
                        " [hidden]"
                    } else {
                        ""
                    };
                    engine::send(actor, &format!("{}{badge}: {}", p.name, nodes.join(", ")));
                } else {
                    engine::send(actor, &p.name);
                }
            }
            if count == 0 {
                state.reply(actor, "no_admins", "");
            }
            return Ok(());
        }
        "hide" => {
            if let Some(slot) = player_slot {
                let hidden = state.toggle_hidden(slot);
                state.reply(actor, if hidden { "hide_on" } else { "hide_off" }, "");
            } else {
                state.reply(actor, "player_only", "");
            }
            return Ok(());
        }
        "rules" | "help" => {
            let messages = if cmd.name == "rules" {
                &state.config.announcements.rules
            } else {
                &state.config.announcements.help
            };
            for line in messages {
                engine::community_send(actor, line);
            }
            return Ok(());
        }
        "reload" => {
            state.reload()?;
            state.reply(actor, "reloaded", "");
            state.audit(actor, cmd.name, "");
            return Ok(());
        }
        "comms" => {
            let Some(id) = engine::actor_id(actor) else {
                state.reply(actor, "player_only", "");
                return Ok(());
            };
            let time = state::now();
            let mut has = false;
            for row in state
                .store
                .active(id, time)?
                .into_iter()
                .filter(|p| p.kind != Kind::Ban)
            {
                has = true;
                let remaining = row.expires_at.map_or_else(
                    || {
                        state
                            .config
                            .phrase(engine::phrase_language(actor).as_str(), "permanent")
                            .to_owned()
                    },
                    |end| format!("{}s", end - time),
                );
                state.reply(
                    actor,
                    row.kind.key(),
                    &format!("{remaining} · {}", row.reason),
                );
            }
            if !has {
                state.reply(actor, "no_comms", "");
            }
            return Ok(());
        }
        "banlist" => {
            let page = match arg(0) {
                None => Some(1_usize),
                Some(value) => value
                    .parse::<u32>()
                    .ok()
                    .filter(|page| *page > 0)
                    .map(|page| page as usize),
            };
            let Some(offset) = page
                .and_then(|p| (p - 1).checked_mul(8))
                .filter(|_| args.len() <= 1)
            else {
                usage(state);
                return Ok(());
            };
            let rows = state.store.list_bans(state::now(), offset, 8)?;
            state.reply(
                actor,
                if rows.is_empty() {
                    "no_bans"
                } else {
                    "ban_page"
                },
                &page.unwrap().to_string(),
            );
            for row in rows {
                let expiry = row.expires_at.map_or_else(
                    || {
                        state
                            .config
                            .phrase(engine::phrase_language(actor).as_str(), "permanent")
                            .to_owned()
                    },
                    |end| end.to_string(),
                );
                engine::send(
                    actor,
                    &format!("{} · {expiry} · {}", row.steam_id, row.reason),
                );
            }
            return Ok(());
        }
        "pause" | "unpause" | "restart" => {
            let instruction = match cmd.name {
                "pause" => "mp_pause_match".to_owned(),
                "unpause" => "mp_unpause_match".to_owned(),
                _ => {
                    let delay = arg(0).map_or(Some(1), |arg| {
                        arg.parse::<u8>()
                            .ok()
                            .filter(|delay| (1..=60).contains(delay))
                    });
                    let Some(delay) = delay.filter(|_| args.len() <= 1) else {
                        usage(state);
                        return Ok(());
                    };
                    format!("mp_restartgame {delay}")
                }
            };
            sdk::ServerCommand(&Str::from_str(&instruction));
            state.audit(actor, cmd.name, "");
            state.reply(actor, "queued", "");
            return Ok(());
        }
        "cvar" => {
            if args.is_empty() || args.len() > 2 {
                usage(state);
                return Ok(());
            }
            let name = args[0].to_ascii_lowercase();
            if !state.config.admin.cvar_allowlist.contains(&name) || sensitive_cvar(&name) {
                state.reply(actor, "cvar_denied", "");
                return Ok(());
            }
            let handle = sdk::FindConVar(&Str::from_str(&name));
            if handle == 0
                || [
                    sdk::ConVarFlag::Protected,
                    sdk::ConVarFlag::Unlogged,
                    sdk::ConVarFlag::Cheat,
                ]
                .into_iter()
                .any(|flag| sdk::IsConVarFlagSet(handle, flag as i64))
            {
                state.reply(actor, "cvar_denied", "");
                return Ok(());
            }
            if let Some(value) = arg(1) {
                if value.chars().any(char::is_control) {
                    state.reply(actor, "cvar_denied", "");
                    return Ok(());
                }
                sdk::SetConVarValue(handle, &Str::from_str(value), true, false);
                state.audit(actor, cmd.name, &name);
            }
            let value = sdk::GetConVarValue(handle);
            engine::send(
                actor,
                &format!("{name} = {}", engine::clean(value.as_str())),
            );
            return Ok(());
        }
        "addban" | "unban" => {
            let Some(id) = arg(0).and_then(target::steam_id) else {
                usage(state);
                return Ok(());
            };
            if engine::actor_id(actor) == Some(id)
                && !state.config.admin.allow_self.contains(cmd.name)
            {
                state.reply(actor, "blocked", &id.to_string());
                return Ok(());
            }
            match engine::offline_access(actor, id) {
                Ok(true) => {}
                Ok(false) => {
                    state.reply(actor, "blocked", &id.to_string());
                    return Ok(());
                }
                Err(phrase) => {
                    state.reply(actor, phrase, "");
                    return Ok(());
                }
            }
            if cmd.name == "unban" {
                let changed = state.revoke(actor, id, &[Kind::Ban])?;
                if changed == 0 {
                    state.reply(actor, "no_change", "");
                    return Ok(());
                }
            } else {
                let Some((duration, reason)) = punishment_args(state, actor, Kind::Ban, &args[1..])
                else {
                    return Ok(());
                };
                let rows = state.apply(actor, id, &[Kind::Ban], duration, &reason)?;
                if let Some(slot) = engine::connected_slot(id) {
                    state.disconnect_banned(slot, &rows[0]);
                }
            }
            success(state, actor, cmd.name, &id.to_string());
            return Ok(());
        }
        _ => {}
    }

    let Some(query) = arg(0) else {
        usage(state);
        return Ok(());
    };
    let players = engine::players();
    let targets = select(
        state,
        actor,
        cmd,
        query,
        &players,
        cmd.multi,
        cmd.name == "goto",
    );
    let damage = if cmd.name == "slap" {
        match arg(1).map_or(Some(0), |arg| {
            arg.parse::<i32>().ok().filter(|d| (0..=1000).contains(d))
        }) {
            Some(damage) if args.len() <= 2 => damage,
            _ => {
                usage(state);
                return Ok(());
            }
        }
    } else {
        0
    };
    let team = if cmd.name == "team" {
        match arg(1).map(str::to_ascii_lowercase).as_deref() {
            Some("t" | "2") => Some(sdk::CSTeam::T),
            Some("ct" | "3") => Some(sdk::CSTeam::CT),
            Some("spec" | "1") => Some(sdk::CSTeam::Spectator),
            _ => {
                usage(state);
                return Ok(());
            }
        }
    } else {
        None
    };
    let destination = match cmd.name {
        "tp" => select(state, actor, cmd, &args[1], &players, false, true)
            .first()
            .map(|p| p.slot),
        "bring" => player_slot,
        _ => None,
    };
    if cmd.name == "goto" && player_slot.is_none() || cmd.name == "bring" && player_slot.is_none() {
        state.reply(actor, "player_only", "");
        return Ok(());
    }
    if matches!(cmd.name, "tp" | "bring") && destination.is_none() {
        return Ok(());
    }

    let kinds: &[Kind] = match cmd.name {
        "ban" => &[Kind::Ban],
        "mute" | "unmute" => &[Kind::Mute],
        "gag" | "ungag" => &[Kind::Gag],
        "silence" | "unsilence" => &[Kind::Mute, Kind::Gag],
        _ => &[],
    };
    let rule = if !kinds.is_empty() && !cmd.name.starts_with("un") {
        let Some(rule) = punishment_args(state, actor, kinds[0], &args[1..]) else {
            return Ok(());
        };
        Some(rule)
    } else {
        None
    };
    for p in targets {
        if !engine::online(p.slot)
            || p.steam_id
                .is_some_and(|id| engine::account(p.slot) != Some(id))
        {
            continue;
        }
        let changed = if !kinds.is_empty() {
            let Some(id) = p.steam_id else {
                state.reply(actor, "needs_steam", &p.name);
                continue;
            };
            if let Some((duration, reason)) = &rule {
                let rows = state.apply(actor, id, kinds, *duration, reason)?;
                if cmd.name == "ban" {
                    state.disconnect_banned(p.slot, &rows[0]);
                }
                true
            } else {
                state.revoke(actor, id, kinds)? != 0
            }
        } else {
            match cmd.name {
                "kick" => {
                    let reason = engine::clean(&args[1..].join(" "));
                    state.cleanup_slot(p.slot);
                    engine::kick(
                        p.slot,
                        if reason.is_empty() {
                            "Kicked by an administrator."
                        } else {
                            &reason
                        },
                    );
                    true
                }
                "slay" if p.alive => {
                    state.restore_movement(p.slot);
                    sdk::ForcePlayerSuicide(p.slot, false, true);
                    true
                }
                "slap" if p.alive => {
                    if damage != 0 && damage >= sdk::GetClientHealth(p.slot) {
                        state.restore_movement(p.slot);
                        sdk::ForcePlayerSuicide(p.slot, false, true);
                    } else {
                        if damage != 0 {
                            sdk::SetClientHealth(p.slot, sdk::GetClientHealth(p.slot) - damage);
                        }
                        let push = if p.slot % 2 == 0 { 180.0 } else { -180.0 };
                        sdk::ApplyAbsVelocityImpulseToClient(
                            p.slot,
                            &Vec3::new(push, -push, 280.0),
                        );
                    }
                    true
                }
                "team" => {
                    state.restore_movement(p.slot);
                    sdk::ChangeClientTeam(p.slot, team.expect("parsed team"));
                    true
                }
                "swap" => match p.team {
                    Team::Terrorist | Team::CounterTerrorist => {
                        state.restore_movement(p.slot);
                        sdk::SwitchClientTeam(
                            p.slot,
                            if p.team == Team::Terrorist {
                                sdk::CSTeam::CT
                            } else {
                                sdk::CSTeam::T
                            },
                        );
                        true
                    }
                    _ => false,
                },
                "respawn"
                    if !p.alive && matches!(p.team, Team::Terrorist | Team::CounterTerrorist) =>
                {
                    state.restore_movement(p.slot);
                    sdk::ForceRespawnClient(p.slot);
                    true
                }
                "tp" | "bring" => teleport(p.slot, destination.expect("checked destination")),
                "goto" => teleport(player_slot.expect("checked player"), p.slot),
                "freeze" if p.alive => state.set_movement(p.slot, sdk::MoveType::None),
                "unfreeze" => state.unfreeze(p.slot),
                "noclip" if p.alive => state.toggle_noclip(p.slot),
                _ => false,
            }
        };
        if changed {
            success(state, actor, cmd.name, &p.name);
        } else {
            state.reply(actor, "no_change", &p.name);
        }
    }
    Ok(())
}

fn teleport(from: i32, to: i32) -> bool {
    if from == to
        || !sdk::IsClientAlive(from)
        || !sdk::IsClientAlive(to)
        || engine::pawn(from).is_none()
        || engine::pawn(to).is_none()
    {
        return false;
    }
    let mut pos = sdk::GetClientAbsOrigin(to);
    pos.z += 80.0;
    sdk::SetClientAbsOrigin(from, &pos);
    true
}

fn select<'a>(
    state: &State,
    actor: Actor,
    cmd: &Command,
    query: &str,
    players: &'a [Player],
    multi: bool,
    allow_self: bool,
) -> Vec<&'a Player> {
    let policy = Policy {
        allow_many: multi,
        allow_self: allow_self || state.config.admin.allow_self.contains(cmd.name),
    };
    match target::select(query, actor, players, policy, |p| {
        engine::can_affect(actor, p.steam_id, p.bot)
    }) {
        Ok(selected) => {
            if !selected.blocked.is_empty() {
                let blocked = selected
                    .blocked
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                state.reply(actor, "blocked", &blocked);
            }
            selected.allowed
        }
        Err(error) => {
            match error {
                TargetError::NotFound => state.reply(actor, "not_found", ""),
                TargetError::UnknownSelector => state.reply(actor, "unknown_selector", ""),
                TargetError::GroupForbidden => state.reply(actor, "group_forbidden", ""),
                TargetError::Ambiguous(names) => state.reply(actor, "ambiguous", &names.join(", ")),
            }
            Vec::new()
        }
    }
}

fn punishment_args(
    state: &State,
    actor: Actor,
    kind: Kind,
    args: &[String],
) -> Option<(Duration, String)> {
    let default = if kind == Kind::Ban {
        state.config.default_ban
    } else {
        state.config.default_comms
    };
    let (duration, remaining) = match args.split_first() {
        Some((first, rest)) => match Duration::parse(first) {
            Some(duration) => (duration, rest),
            None if first.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+') => {
                state.reply(actor, "bad_duration", "");
                return None;
            }
            None => (default, args),
        },
        None => (default, args),
    };
    let reason = engine::clean(&remaining.join(" "));
    let lang = engine::phrase_language(actor);
    let reason = if reason.is_empty() {
        state
            .config
            .phrase(lang.as_str(), "default_reason")
            .to_owned()
    } else {
        reason
    };
    Some((duration, reason))
}

fn success(state: &State, actor: Actor, command: &str, target: &str) {
    state.reply(actor, "done", &format!("{command} · {target}"));
    state.audit(actor, command, target);
    if matches!(
        command,
        "kick"
            | "ban"
            | "addban"
            | "unban"
            | "mute"
            | "unmute"
            | "gag"
            | "ungag"
            | "silence"
            | "unsilence"
    ) {
        let operator = match actor {
            Actor::Console => Some("Console".to_owned()),
            Actor::Player(slot) if !state.is_hidden(slot) => {
                Some(engine::clean(sdk::GetClientName(slot).as_str()))
            }
            Actor::Player(_) => None,
        };
        let detail = format!("{command} · {target}");
        let detail = match operator {
            Some(name) => format!("{name}: {detail}"),
            None => detail,
        };
        for slot in 0..sdk::GetMaxClients() {
            if engine::account(slot).is_some() && actor != Actor::Player(slot) {
                state.reply(Actor::Player(slot), "moderation", &detail);
            }
        }
    }
}

fn sensitive_cvar(name: &str) -> bool {
    name == "sv_cheats"
        || name.contains("password")
        || name.contains("token")
        || name.contains("apikey")
        || name.contains("authkey")
}

pub fn imported_apply(
    state: &mut State,
    actor: Actor,
    slot: i32,
    minutes: i32,
    reason: &str,
    kind: Kind,
) -> bool {
    let Some(id) = engine::account(slot) else {
        return false;
    };
    let Some(duration) = Duration::from_minutes(minutes) else {
        return false;
    };
    let name = kind.key();
    let node = if kind == Kind::Ban {
        "imomoe.admin.ban"
    } else {
        "imomoe.admin.mute"
    };
    if !engine::allowed(actor, node)
        || !engine::can_affect(actor, Some(id), false)
        || actor == Actor::Player(slot) && !state.config.admin.allow_self.contains(name)
    {
        return false;
    }
    match state.apply(actor, id, &[kind], duration, &engine::clean(reason)) {
        Ok(rows) => {
            let target = engine::clean(sdk::GetClientName(slot).as_str());
            if kind == Kind::Ban {
                state.disconnect_banned(slot, &rows[0]);
            }
            success(state, actor, name, &target);
            true
        }
        Err(error) => {
            plugify::error!("imomoe_admin API {}: {}", name, error);
            false
        }
    }
}

#[cfg(test)]
mod selector_arg_tests {
    use super::selector_args;
    use imomoe_admin_core::command::lookup;

    #[test]
    fn restores_split_engine_selectors_only_in_target_positions() {
        let args = ["#userid", ":", "12", "#slot", ":", "4"].map(str::to_owned);
        assert_eq!(
            &*selector_args(lookup("tp", false).unwrap(), &args),
            ["#userid:12", "#slot:4"]
        );
        assert_eq!(
            &*selector_args(lookup("kick", false).unwrap(), &args),
            ["#userid:12", "#slot", ":", "4"]
        );
        assert_eq!(
            &*selector_args(lookup("report", false).unwrap(), &args),
            args
        );
        let args = ["#userid:12", "reason"].map(str::to_owned);
        assert_eq!(&*selector_args(lookup("kick", false).unwrap(), &args), args);
    }
}
