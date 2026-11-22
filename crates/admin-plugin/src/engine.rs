use crate::{imomoe_native_base as native, permissions as perm, s2sdk as sdk};
use imomoe_admin_core::target::{Actor, Player, Team, steam_id};
use plugify::Str;

pub fn online(slot: i32) -> bool {
    (0..sdk::GetMaxClients()).contains(&slot)
        && sdk::IsClientInGame(slot)
        && !sdk::IsClientSourceTV(slot)
}

pub fn account(slot: i32) -> Option<u64> {
    if !online(slot) {
        return None;
    }
    connected_account(slot)
}

pub fn connected_account(slot: i32) -> Option<u64> {
    if !(0..sdk::GetMaxClients()).contains(&slot)
        || !sdk::IsClientConnected(slot)
        || sdk::IsFakeClient(slot)
        || !sdk::IsClientAuthorized(slot)
    {
        return None;
    }
    let id = sdk::GetClientSteamID64(slot);
    steam_id(&id.to_string())
}

pub fn actor_id(actor: Actor) -> Option<u64> {
    match actor {
        Actor::Console => None,
        Actor::Player(slot) => account(slot),
    }
}

pub fn allowed(actor: Actor, node: &str) -> bool {
    actor == Actor::Console
        || actor_id(actor)
            .is_some_and(|id| perm::HasPermission(id, &Str::from_str(node)) == perm::Status::Allow)
}

pub fn can_affect(actor: Actor, target: Option<u64>, bot: bool) -> bool {
    actor == Actor::Console
        || actor_id(actor).is_some_and(|id| {
            bot || target
                .is_some_and(|target| perm::CanAffectUser(id, target) == perm::Status::Allow)
        })
}

/// An unloaded account is not equivalent to an account with no immunity.
pub fn offline_access(actor: Actor, id: u64) -> Result<bool, &'static str> {
    if actor == Actor::Console {
        return Ok(true);
    }
    let Some(admin) = actor_id(actor) else {
        return Ok(false);
    };
    match perm::CanAffectUser(admin, id) {
        perm::Status::Allow => Ok(true),
        perm::Status::TargetUserNotFound => {
            let plugin = plugify::PLUGIN.get().ok_or("permissions_unavailable")?;
            let status = perm::LoadUser(plugin.id as i64, id, &Str::new(), true, false);
            if !matches!(
                status,
                perm::Status::Success | perm::Status::UserAlreadyExist
            ) {
                return Err("permissions_unavailable");
            }
            match perm::CanAffectUser(admin, id) {
                perm::Status::Allow => Ok(true),
                perm::Status::TargetUserNotFound => Err("permissions_loading"),
                _ => Ok(false),
            }
        }
        _ => Ok(false),
    }
}

pub fn players() -> Vec<Player> {
    (0..sdk::GetMaxClients())
        .filter(|&slot| online(slot))
        .map(|slot| Player {
            slot,
            user_id: native::GetClientUserID(slot),
            steam_id: account(slot),
            name: clean(sdk::GetClientName(slot).as_str()),
            team: match sdk::GetClientTeam(slot) {
                sdk::CSTeam::None => Team::Unassigned,
                sdk::CSTeam::Spectator => Team::Spectator,
                sdk::CSTeam::T => Team::Terrorist,
                sdk::CSTeam::CT => Team::CounterTerrorist,
            },
            alive: sdk::IsClientAlive(slot),
            bot: sdk::IsFakeClient(slot),
        })
        .collect()
}

pub fn connected_slot(id: u64) -> Option<i32> {
    (0..sdk::GetMaxClients()).find(|&slot| connected_account(slot) == Some(id))
}

pub fn clean(message: &str) -> String {
    message
        .chars()
        .filter(|c| !c.is_control())
        .take(256)
        .collect()
}

pub fn pawn(slot: i32) -> Option<i32> {
    let address = sdk::GetClientPawn(slot);
    if address == 0 {
        return None;
    }
    let handle = sdk::EntPointerToEntHandle(address);
    sdk::IsValidEntHandle(handle).then_some(handle)
}

pub fn kick(slot: i32, message: &str) {
    sdk::KickClient(
        slot,
        sdk::NetworkDisconnectionReason::Kicked,
        &Str::from_str(message),
    );
}

pub fn phrase_language(actor: Actor) -> Str {
    match actor {
        Actor::Player(slot) if online(slot) => sdk::GetClientLanguageCode(slot),
        _ => Str::from_str("en"),
    }
}

pub fn send(actor: Actor, message: &str) {
    match actor {
        Actor::Console => sdk::PrintToServer(&Str::from_str(message)),
        Actor::Player(slot) if online(slot) => sdk::PrintToChat(slot, &Str::from_str(message)),
        Actor::Player(_) => {}
    }
}

pub fn community_send(actor: Actor, text: &str) {
    let prefix = if matches!(actor, Actor::Player(_)) {
        unsafe { std::ptr::addr_of!(crate::__imomoe_chat_prefix_GetChatPrefix).read() }.and_then(
            |get| {
                let mut prefix = Str::new();
                if !unsafe { get(&mut prefix) } {
                    return None;
                }
                let prefix = prefix.as_str();
                (prefix.len() <= 96
                    && prefix.starts_with(' ')
                    && prefix.ends_with("\x01 ")
                    && prefix
                        .chars()
                        .all(|ch| !ch.is_control() || ('\x01'..='\x10').contains(&ch)))
                .then(|| prefix[1..].replace('\x0a', "{bluegrey}"))
            },
        )
    } else {
        None
    };
    match (actor, prefix) {
        (Actor::Player(slot), Some(prefix)) if online(slot) => {
            sdk::PrintToChatColored(slot, &Str::from_str(&format!("{prefix}{text}")));
        }
        _ => send(actor, text),
    }
}

pub fn require_bindings() -> Result<(), Box<dyn std::error::Error>> {
    macro_rules! require {
        ($module:ident: $($symbol:ident),+ $(,)?) => {
            $(if unsafe { std::ptr::addr_of!($module::$symbol).read() }.is_none() {
                return Err(format!("missing required binding {}", stringify!($symbol)).into());
            })+
        }
    }
    require!(sdk:
        __s2sdk_AddConsoleCommand, __s2sdk_RemoveCommand, __s2sdk_AddCommandListener,
        __s2sdk_RemoveCommandListener, __s2sdk_GetMaxClients, __s2sdk_IsClientInGame,
        __s2sdk_IsClientConnected, __s2sdk_IsClientAlive, __s2sdk_IsFakeClient,
        __s2sdk_IsClientSourceTV, __s2sdk_GetClientName, __s2sdk_GetClientTeam,
        __s2sdk_GetClientSteamID64, __s2sdk_GetClientLanguageCode, __s2sdk_GetClientPawn,
        __s2sdk_EntPointerToEntHandle, __s2sdk_PrintToChat, __s2sdk_PrintToChatColored, __s2sdk_PrintToServer,
        __s2sdk_KickClient, __s2sdk_GetClientListening, __s2sdk_SetClientListening,
        __s2sdk_OnClientAuthenticated_Register, __s2sdk_OnClientAuthenticated_Unregister,
        __s2sdk_OnClientDisconnect_Register, __s2sdk_OnClientDisconnect_Unregister, __s2sdk_OnClientPutInServer_Register, __s2sdk_OnClientPutInServer_Unregister,
        __s2sdk_OnMapEnd_Register, __s2sdk_OnMapEnd_Unregister,
        __s2sdk_OnGameFrame_Register, __s2sdk_OnGameFrame_Unregister,
        __s2sdk_OnEntityDeleted_Register, __s2sdk_OnEntityDeleted_Unregister,
        __s2sdk_ForcePlayerSuicide, __s2sdk_ForceRespawnClient, __s2sdk_ChangeClientTeam,
        __s2sdk_SwitchClientTeam, __s2sdk_GetClientHealth, __s2sdk_SetClientHealth,
        __s2sdk_ApplyAbsVelocityImpulseToClient, __s2sdk_GetClientAbsOrigin,
        __s2sdk_SetClientAbsOrigin, __s2sdk_GetEntityMoveType, __s2sdk_SetEntityMoveType,
        __s2sdk_IsValidEntHandle, __s2sdk_FindConVar, __s2sdk_IsConVarFlagSet,
        __s2sdk_GetConVarValue, __s2sdk_SetConVarValue, __s2sdk_ServerCommand,
        __s2sdk_HookEvent, __s2sdk_UnhookEvent, __s2sdk_GetEventPlayerSlot, __s2sdk_IsClientAuthorized
    );
    require!(native: __imomoe_native_base_GetClientUserID);
    require!(perm: __permissions_HasPermission, __permissions_CanAffectUser, __permissions_LoadUser);
    Ok(())
}
