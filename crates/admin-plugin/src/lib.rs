#[cfg(test)]
mod acceptance_tests;
mod config;
mod dispatch;
mod engine;
#[allow(dead_code, non_camel_case_types)]
#[path = "generated/src/permissions/mod.rs"]
mod permissions;
#[allow(dead_code, non_camel_case_types)]
#[path = "generated/src/s2sdk/mod.rs"]
mod s2sdk;
mod state;
mod storage;

use imomoe_admin_core::{
    command::{COMMANDS, lookup, private_input, words},
    punishment::Kind,
    target::Actor,
};
use plugify::{Arr, Str, register_plugin};
use s2sdk::{ConCommandContext, HookMode, NetworkDisconnectionReason, ResultType};
use state::{Result, State};
use std::{
    cell::RefCell,
    panic::{AssertUnwindSafe, catch_unwind},
    path::PathBuf,
};

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
static mut __imomoe_chat_prefix_GetChatPrefix: Option<unsafe extern "C" fn(&mut Str) -> bool> =
    None;

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static PENDING: RefCell<Vec<Lifecycle>> = const { RefCell::new(Vec::new()) };
    static HOOKS: RefCell<Hooks> = const { RefCell::new(Hooks::new()) };
}

enum Lifecycle {
    Authenticated(i32, u64),
    Disconnect(i32),
    Death(i32),
    Deleted(i32),
    MapEnd,
}

struct Hooks {
    commands: Vec<String>,
    chat: Vec<&'static str>,
    death: bool,
    lifecycle: bool,
}

impl Hooks {
    const fn new() -> Self {
        Self {
            commands: Vec::new(),
            chat: Vec::new(),
            death: false,
            lifecycle: false,
        }
    }

    fn install(&mut self) -> Result<()> {
        for cmd in COMMANDS {
            let name = format!("imomoe_{}", cmd.name);
            if !s2sdk::AddConsoleCommand(
                &Str::from_str(&name),
                &Str::from_str(cmd.usage),
                s2sdk::ConVarFlag::ClientCanExecute,
                on_console,
                HookMode::Pre,
            ) {
                return Err(format!("could not register {name}").into());
            }
            self.commands.push(name);
        }
        for name in ["", "say", "say_team"] {
            if !s2sdk::AddCommandListener(&Str::from_str(name), chat_callback(name), HookMode::Pre)
            {
                return Err(format!("could not register {name} listener").into());
            }
            self.chat.push(name);
        }
        if s2sdk::HookEvent(&Str::from_str("player_death"), on_death, HookMode::Pre)
            != s2sdk::EventHookError::Okay
        {
            return Err("could not register player_death listener".into());
        }
        self.death = true;

        s2sdk::OnClientAuthenticated_Register(on_authenticated);
        s2sdk::OnClientPutInServer_Register(on_put_in_server);
        s2sdk::OnClientDisconnect_Register(on_disconnect);
        s2sdk::OnEntityDeleted_Register(on_deleted);
        s2sdk::OnMapEnd_Register(on_map_end);
        s2sdk::OnGameFrame_Register(on_frame);
        self.lifecycle = true;
        Ok(())
    }

    fn remove(&mut self) {
        if self.lifecycle {
            s2sdk::OnGameFrame_Unregister(on_frame);
            s2sdk::OnMapEnd_Unregister(on_map_end);
            s2sdk::OnEntityDeleted_Unregister(on_deleted);
            s2sdk::OnClientDisconnect_Unregister(on_disconnect);
            s2sdk::OnClientPutInServer_Unregister(on_put_in_server);
            s2sdk::OnClientAuthenticated_Unregister(on_authenticated);
            self.lifecycle = false;
        }
        if self.death {
            let result =
                s2sdk::UnhookEvent(&Str::from_str("player_death"), on_death, HookMode::Pre);
            if result != s2sdk::EventHookError::Okay {
                plugify::error!("imomoe_admin: player_death unhook returned {:?}", result);
            }
            self.death = false;
        }
        for name in self.chat.drain(..) {
            if !s2sdk::RemoveCommandListener(
                &Str::from_str(name),
                chat_callback(name),
                HookMode::Pre,
            ) {
                plugify::error!("imomoe_admin: could not remove {} listener", name);
            }
        }
        for name in self.commands.drain(..) {
            if !s2sdk::RemoveCommand(&Str::from_str(&name), on_console) {
                plugify::error!("imomoe_admin: could not remove {}", name);
            }
        }
    }
}

fn start() -> Result<()> {
    boundary("start", Err("imomoe_admin start panicked".into()), || {
        engine::require_bindings()?;
        let root = plugify::CONFIGS_DIR
            .get()
            .ok_or("Plugify configs directory unavailable")?;
        let data = plugify::DATA_DIR
            .get()
            .ok_or("Plugify data directory unavailable")?;
        let dir = PathBuf::from(root).join("imomoe_admin");
        let state = State::new(dir, PathBuf::from(data).join("imomoe_admin"))?;
        STATE.set(Some(state));
        if let Err(error) = HOOKS.with_borrow_mut(Hooks::install) {
            HOOKS.with_borrow_mut(Hooks::remove);
            STATE.set(None);
            return Err(error);
        }
        with_state(|state| {
            for slot in 0..s2sdk::GetMaxClients() {
                if let Some(id) = engine::connected_account(slot)
                    && let Err(error) = state.enforce_ban(slot, id)
                {
                    plugify::error!("imomoe_admin hot-load ban check: {}", error);
                }
            }
        });
        Ok(())
    })
}

fn end() -> Result<()> {
    boundary("end", Err("imomoe_admin end panicked".into()), || {
        HOOKS.with_borrow_mut(Hooks::remove);
        if let Some(mut state) = STATE.take() {
            state.clear_map();
        }
        PENDING.take();
        Ok(())
    })
}

fn boundary<T>(event: &str, default: T, run: impl FnOnce() -> T) -> T {
    match catch_unwind(AssertUnwindSafe(run)) {
        Ok(value) => value,
        Err(_) => {
            plugify::error!("imomoe_admin: panic in {}", event);
            default
        }
    }
}

fn with_state<T>(run: impl FnOnce(&mut State) -> T) -> Option<T> {
    STATE.with(|state| {
        let mut state = state.try_borrow_mut().ok()?;
        let state = state.as_mut()?;
        let out = run(state);
        // Engine calls such as disconnect and suicide may invoke SDK listeners synchronously.
        loop {
            let pending = PENDING.take();
            if pending.is_empty() {
                break;
            }
            for event in pending {
                match event {
                    Lifecycle::Authenticated(slot, id) => {
                        if let Err(error) = state.enforce_ban(slot, id) {
                            plugify::error!("imomoe_admin authorization check: {}", error);
                        }
                    }
                    Lifecycle::Disconnect(slot) => state.cleanup_slot(slot),
                    Lifecycle::Death(slot) => {
                        state.restore_movement(slot);
                    }
                    Lifecycle::Deleted(handle) => state.deleted(handle),
                    Lifecycle::MapEnd => state.clear_map(),
                }
            }
        }
        Some(out)
    })
}

fn lifecycle(event: Lifecycle) {
    PENDING.with_borrow_mut(|events| events.push(event));
    with_state(|_| {});
}

fn actor(slot: i32) -> Option<Actor> {
    match slot {
        -1 => Some(Actor::Console),
        slot if engine::online(slot) => Some(Actor::Player(slot)),
        _ => None,
    }
}

unsafe extern "C" fn on_console(
    slot: i32,
    _context: ConCommandContext,
    args: &Arr<Str>,
) -> ResultType {
    boundary("command", ResultType::Stop, || {
        let Some(actor) = actor(slot) else {
            return ResultType::Stop;
        };
        let mut args = args.iter().map(|arg| arg.as_str());
        if let Some(cmd) = args
            .next()
            .and_then(|name| name.strip_prefix("imomoe_"))
            .and_then(|name| lookup(name, false))
        {
            let args: Vec<_> = args.map(str::to_owned).collect();
            with_state(|state| dispatch::run(state, actor, cmd, &args));
        }
        ResultType::Stop
    })
}

fn chat_callback(name: &str) -> s2sdk::ConCommandCallback {
    if name.is_empty() {
        on_chat_trigger
    } else {
        on_say
    }
}

unsafe extern "C" fn on_chat_trigger(
    slot: i32,
    context: ConCommandContext,
    args: &Arr<Str>,
) -> ResultType {
    if context != ConCommandContext::Chat {
        return ResultType::Continue;
    }
    boundary("chat trigger", ResultType::Stop, || {
        let mut args = args.iter().map(|arg| arg.as_str());
        let Some(cmd) = args.next().and_then(|name| lookup(name, true)) else {
            return ResultType::Continue;
        };
        if let Some(actor) = actor(slot) {
            let args: Vec<_> = args.map(str::to_owned).collect();
            with_state(|state| dispatch::run(state, actor, cmd, &args));
        }
        ResultType::Stop
    })
}

unsafe extern "C" fn on_say(slot: i32, _ctx: ConCommandContext, args: &Arr<Str>) -> ResultType {
    boundary("chat", ResultType::Stop, || {
        let mut args = args.iter().map(|arg| arg.as_str());
        let Some(name) = args.next() else {
            return ResultType::Continue;
        };
        let text = args.collect::<Vec<_>>().join(" ");
        let unquoted = text.trim();
        if slot == -1 && private_input(name == "say_team", unquoted).is_none() {
            return ResultType::Continue;
        }
        let Some(actor) = actor(slot) else {
            return ResultType::Stop;
        };
        with_state(|state| {
            if let Some((cmd, message)) = private_input(name == "say_team", unquoted) {
                if let Some(cmd) = lookup(cmd, false) {
                    dispatch::run(state, actor, cmd, &[message.to_owned()]);
                }
                return ResultType::Stop;
            }
            if let Some(input) = unquoted.strip_prefix(['!', '/']) {
                let name = input.split_whitespace().next().unwrap_or_default();
                if let Some(cmd) = lookup(name, true) {
                    match words(input) {
                        Some(args) => dispatch::run(state, actor, cmd, &args[1..]),
                        None => {
                            state.reply(actor, "usage", &format!("!{} {}", cmd.chat, cmd.usage))
                        }
                    }
                    return ResultType::Stop;
                }
            }
            match state.has(slot, Kind::Gag) {
                Ok(false) => ResultType::Continue,
                Ok(true) => {
                    state.reply(actor, "gagged", "");
                    ResultType::Stop
                }
                Err(error) => {
                    plugify::error!("imomoe_admin gag check: {}", error);
                    state.reply(actor, "failed", "");
                    ResultType::Stop
                }
            }
        })
        .unwrap_or(ResultType::Stop)
    })
}

unsafe extern "C" fn on_authenticated(slot: i32, steam_id: u64) {
    boundary("authenticate", (), || {
        lifecycle(Lifecycle::Authenticated(slot, steam_id))
    });
}

unsafe extern "C" fn on_put_in_server(slot: i32) {
    boundary("put in server", (), || {
        if let Some(id) = engine::connected_account(slot) {
            lifecycle(Lifecycle::Authenticated(slot, id));
        }
    })
}

unsafe extern "C" fn on_disconnect(slot: i32, _reason: NetworkDisconnectionReason) {
    boundary("disconnect", (), || lifecycle(Lifecycle::Disconnect(slot)));
}

unsafe extern "C" fn on_death(_name: &Str, event: usize, _broadcast: bool) -> ResultType {
    boundary("death", ResultType::Continue, || {
        let slot = s2sdk::GetEventPlayerSlot(event, &Str::from_str("userid"));
        if slot >= 0 {
            lifecycle(Lifecycle::Death(slot));
        }
        ResultType::Continue
    })
}

unsafe extern "C" fn on_deleted(handle: i32) {
    boundary("entity deletion", (), || {
        lifecycle(Lifecycle::Deleted(handle))
    });
}

unsafe extern "C" fn on_map_end() {
    boundary("map end", (), || lifecycle(Lifecycle::MapEnd));
}

unsafe extern "C" fn on_frame(simulating: bool, _: bool, _: bool) {
    boundary("frame", (), || {
        if simulating {
            with_state(State::frame);
        }
    })
}

fn query(slot: i32, kind: Kind) -> bool {
    boundary("query", false, || {
        with_state(|state| match state.has(slot, kind) {
            Ok(value) => value,
            Err(error) => {
                plugify::error!("imomoe_admin query {}: {}", kind.key(), error);
                false
            }
        })
        .unwrap_or(false)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn IsClientMuted(slot: i32) -> bool {
    query(slot, Kind::Mute)
}

#[unsafe(no_mangle)]
pub extern "C" fn IsClientGagged(slot: i32) -> bool {
    query(slot, Kind::Gag)
}

#[unsafe(no_mangle)]
pub extern "C" fn IsAdminHidden(slot: i32) -> bool {
    boundary("hidden status", false, || {
        with_state(|state| state.is_hidden(slot)).unwrap_or(false)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn CanTarget(actor_slot: i32, target_slot: i32) -> bool {
    boundary("target immunity", false, || {
        with_state(|_| {
            actor(actor_slot).is_some_and(|actor| {
                engine::online(target_slot)
                    && engine::can_affect(
                        actor,
                        engine::account(target_slot),
                        s2sdk::IsFakeClient(target_slot),
                    )
            })
        })
        .unwrap_or(false)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn KickClient(slot: i32, reason: &Str) {
    boundary("kick", (), || {
        with_state(|state| {
            if engine::online(slot) {
                let name = engine::clean(s2sdk::GetClientName(slot).as_str());
                state.cleanup_slot(slot);
                engine::kick(slot, &engine::clean(reason.as_str()));
                state.audit(Actor::Console, "api/kick", &name);
            }
        });
    })
}

fn apply(actor_slot: i32, target: i32, minutes: i32, reason: &Str, kind: Kind) -> bool {
    boundary("apply punishment", false, || {
        with_state(|state| {
            actor(actor_slot).is_some_and(|actor| {
                dispatch::imported_apply(state, actor, target, minutes, reason.as_str(), kind)
            })
        })
        .unwrap_or(false)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn ApplyBan(actor_slot: i32, target_slot: i32, minutes: i32, reason: &Str) -> bool {
    apply(actor_slot, target_slot, minutes, reason, Kind::Ban)
}

#[unsafe(no_mangle)]
pub extern "C" fn ApplyMute(actor_slot: i32, target_slot: i32, minutes: i32, reason: &Str) -> bool {
    apply(actor_slot, target_slot, minutes, reason, Kind::Mute)
}

#[unsafe(no_mangle)]
pub extern "C" fn NotifyAdmins(message: &Str) {
    boundary("notify admins", (), || {
        with_state(|state| state.notify("admin_prefix", &engine::clean(message.as_str())));
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn OwnsClientNoclip(slot: i32, pawn_handle: i32) -> bool {
    boundary("noclip ownership", false, || {
        with_state(|state| state.owns_noclip(slot, pawn_handle)).unwrap_or(false)
    })
}

register_plugin!(start: start, end: end);
