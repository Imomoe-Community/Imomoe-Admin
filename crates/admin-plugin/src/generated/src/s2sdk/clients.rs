// Generated from s2sdk.pplugin (group: clients)

#[allow(unused_imports)]
use super::aliases::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use plugify::{Arr, Mat4x4, Str, Var, Vec2, Vec3, Vec4, scope};
#[allow(unused_imports)]
use std::sync::RwLock;

/// Returns the client's SteamID64 - a unique 64-bit identifier of a Steam account.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (uint64):
/// uint64_t The client's SteamID64.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientSteamID64(playerSlot: i32) -> u64 {
    unsafe {
        __s2sdk_GetClientSteamID64.expect("GetClientSteamID64 function was not found")(playerSlot)
    }
}
pub type _GetClientSteamID64 = unsafe extern "C" fn(i32) -> u64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientSteamID64: Option<_GetClientSteamID64> = None;

/// Retrieves a client's language as an ISO code. Unlike GetClientLanguage, which returns the raw cl_language value ("english", "schinese"), this returns the code translation files are keyed by ("en", "zh-CN"). Empty until the client answers the cl_language query, and always empty for bots. Reliably available starting from OnClientAuthenticated and OnClientPutInServer.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (string):
/// The client's ISO language code.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientLanguageCode(playerSlot: i32) -> Str {
    unsafe {
        __s2sdk_GetClientLanguageCode.expect("GetClientLanguageCode function was not found")(
            playerSlot,
        )
    }
}
pub type _GetClientLanguageCode = unsafe extern "C" fn(i32) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientLanguageCode: Option<_GetClientLanguageCode> = None;

/// Returns the client's name.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (string):
/// The client's name.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientName(playerSlot: i32) -> Str {
    unsafe { __s2sdk_GetClientName.expect("GetClientName function was not found")(playerSlot) }
}
pub type _GetClientName = unsafe extern "C" fn(i32) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientName: Option<_GetClientName> = None;

/// Checks if a certain player has been authenticated.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the player is authenticated, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsClientAuthorized(playerSlot: i32) -> bool {
    unsafe {
        __s2sdk_IsClientAuthorized.expect("IsClientAuthorized function was not found")(playerSlot)
    }
}
pub type _IsClientAuthorized = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsClientAuthorized: Option<_IsClientAuthorized> = None;

/// Checks if a certain player is connected.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the player is connected, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsClientConnected(playerSlot: i32) -> bool {
    unsafe {
        __s2sdk_IsClientConnected.expect("IsClientConnected function was not found")(playerSlot)
    }
}
pub type _IsClientConnected = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsClientConnected: Option<_IsClientConnected> = None;

/// Checks if a certain player has entered the game.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the player is in the game, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsClientInGame(playerSlot: i32) -> bool {
    unsafe { __s2sdk_IsClientInGame.expect("IsClientInGame function was not found")(playerSlot) }
}
pub type _IsClientInGame = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsClientInGame: Option<_IsClientInGame> = None;

/// Checks if a certain player is the SourceTV bot.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the client is the SourceTV bot, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsClientSourceTV(playerSlot: i32) -> bool {
    unsafe {
        __s2sdk_IsClientSourceTV.expect("IsClientSourceTV function was not found")(playerSlot)
    }
}
pub type _IsClientSourceTV = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsClientSourceTV: Option<_IsClientSourceTV> = None;

/// Gets the listen override between a receiver and a sender.
///
/// # Arguments
/// * `receiverSlot` - (int32): The slot of the player who listens.
/// * `senderSlot` - (int32): The slot of the player who speaks.
///
/// # Returns - (uint8):
/// The listen override for this pair.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientListening(receiverSlot: i32, senderSlot: i32) -> ListenOverride {
    unsafe {
        __s2sdk_GetClientListening.expect("GetClientListening function was not found")(
            receiverSlot,
            senderSlot,
        )
    }
}
pub type _GetClientListening = unsafe extern "C" fn(i32, i32) -> ListenOverride;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientListening: Option<_GetClientListening> = None;

/// Sets the listen override between a receiver and a sender until the receiver disconnects.
///
/// # Arguments
/// * `receiverSlot` - (int32): The slot of the player who listens.
/// * `senderSlot` - (int32): The slot of the player who speaks.
/// * `listen` - (uint8): The listen override to set.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn SetClientListening(receiverSlot: i32, senderSlot: i32, listen: ListenOverride) {
    unsafe {
        __s2sdk_SetClientListening.expect("SetClientListening function was not found")(
            receiverSlot,
            senderSlot,
            listen,
        )
    }
}
pub type _SetClientListening = unsafe extern "C" fn(i32, i32, ListenOverride);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_SetClientListening: Option<_SetClientListening> = None;

/// Checks if the client is alive or dead.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the client is alive, false if dead.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsClientAlive(playerSlot: i32) -> bool {
    unsafe { __s2sdk_IsClientAlive.expect("IsClientAlive function was not found")(playerSlot) }
}
pub type _IsClientAlive = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsClientAlive: Option<_IsClientAlive> = None;

/// Checks if a certain player is a fake client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (bool):
/// true if the client is a fake client, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsFakeClient(playerSlot: i32) -> bool {
    unsafe { __s2sdk_IsFakeClient.expect("IsFakeClient function was not found")(playerSlot) }
}
pub type _IsFakeClient = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsFakeClient: Option<_IsFakeClient> = None;

/// Retrieves the health of an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot whose health is to be retrieved.
///
/// # Returns - (int32):
/// The health of the client, or 0 if the client is invalid.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientHealth(playerSlot: i32) -> i32 {
    unsafe { __s2sdk_GetClientHealth.expect("GetClientHealth function was not found")(playerSlot) }
}
pub type _GetClientHealth = unsafe extern "C" fn(i32) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientHealth: Option<_GetClientHealth> = None;

/// Sets the health of an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot whose health is to be set.
/// * `health` - (int32): The new health value to set for the client.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn SetClientHealth(playerSlot: i32, health: i32) {
    unsafe {
        __s2sdk_SetClientHealth.expect("SetClientHealth function was not found")(playerSlot, health)
    }
}
pub type _SetClientHealth = unsafe extern "C" fn(i32, i32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_SetClientHealth: Option<_SetClientHealth> = None;

/// Retrieves the team number of an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot whose team number is to be retrieved.
///
/// # Returns - (int32):
/// The team number of the client, or 0 if the client is invalid.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientTeam(playerSlot: i32) -> CSTeam {
    unsafe { __s2sdk_GetClientTeam.expect("GetClientTeam function was not found")(playerSlot) }
}
pub type _GetClientTeam = unsafe extern "C" fn(i32) -> CSTeam;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientTeam: Option<_GetClientTeam> = None;

/// Retrieves the absolute origin of an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot whose absolute origin is to be retrieved.
///
/// # Returns - (vec3):
/// A vector where the absolute origin will be stored.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientAbsOrigin(playerSlot: i32) -> Vec3 {
    unsafe {
        __s2sdk_GetClientAbsOrigin.expect("GetClientAbsOrigin function was not found")(playerSlot)
    }
}
pub type _GetClientAbsOrigin = unsafe extern "C" fn(i32) -> Vec3;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientAbsOrigin: Option<_GetClientAbsOrigin> = None;

/// Sets the absolute origin of an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot whose absolute origin is to be set.
/// * `origin` - (vec3): The new absolute origin to set for the client.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn SetClientAbsOrigin(playerSlot: i32, origin: &Vec3) {
    unsafe {
        __s2sdk_SetClientAbsOrigin.expect("SetClientAbsOrigin function was not found")(
            playerSlot, origin,
        )
    }
}
pub type _SetClientAbsOrigin = unsafe extern "C" fn(i32, &Vec3);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_SetClientAbsOrigin: Option<_SetClientAbsOrigin> = None;

/// Apply an absolute velocity impulse to an client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to apply impulse to
/// * `vecImpulse` - (vec3): Velocity impulse vector to apply
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn ApplyAbsVelocityImpulseToClient(playerSlot: i32, vecImpulse: &Vec3) {
    unsafe {
        __s2sdk_ApplyAbsVelocityImpulseToClient
            .expect("ApplyAbsVelocityImpulseToClient function was not found")(
            playerSlot, vecImpulse,
        )
    }
}
pub type _ApplyAbsVelocityImpulseToClient = unsafe extern "C" fn(i32, &Vec3);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_ApplyAbsVelocityImpulseToClient: Option<_ApplyAbsVelocityImpulseToClient> =
    None;

/// Retrieves the pawn entity pointer associated with a client.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
///
/// # Returns - (ptr64):
/// A pointer to the client's pawn entity, or nullptr if the client or controller is invalid.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientPawn(playerSlot: i32) -> usize {
    unsafe { __s2sdk_GetClientPawn.expect("GetClientPawn function was not found")(playerSlot) }
}
pub type _GetClientPawn = unsafe extern "C" fn(i32) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetClientPawn: Option<_GetClientPawn> = None;

/// Switches the player's team.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
/// * `team` - (int32): The team index to switch the client to.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn SwitchClientTeam(playerSlot: i32, team: CSTeam) {
    unsafe {
        __s2sdk_SwitchClientTeam.expect("SwitchClientTeam function was not found")(playerSlot, team)
    }
}
pub type _SwitchClientTeam = unsafe extern "C" fn(i32, CSTeam);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_SwitchClientTeam: Option<_SwitchClientTeam> = None;

/// Changes the player's team.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
/// * `team` - (int32): The team index to change the client to.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn ChangeClientTeam(playerSlot: i32, team: CSTeam) {
    unsafe {
        __s2sdk_ChangeClientTeam.expect("ChangeClientTeam function was not found")(playerSlot, team)
    }
}
pub type _ChangeClientTeam = unsafe extern "C" fn(i32, CSTeam);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_ChangeClientTeam: Option<_ChangeClientTeam> = None;

/// Forces a full engine respawn for a player, including when the current pawn is alive. Supported in CS2.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to respawn.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn ForceRespawnClient(playerSlot: i32) {
    unsafe {
        __s2sdk_ForceRespawnClient.expect("ForceRespawnClient function was not found")(playerSlot)
    }
}
pub type _ForceRespawnClient = unsafe extern "C" fn(i32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_ForceRespawnClient: Option<_ForceRespawnClient> = None;

/// Forces a player to commit suicide.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot.
/// * `explode` - (bool): If true, the client will explode upon death.
/// * `force` - (bool): If true, the suicide will be forced.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn ForcePlayerSuicide(playerSlot: i32, explode: bool, force: bool) {
    unsafe {
        __s2sdk_ForcePlayerSuicide.expect("ForcePlayerSuicide function was not found")(
            playerSlot, explode, force,
        )
    }
}
pub type _ForcePlayerSuicide = unsafe extern "C" fn(i32, bool, bool);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_ForcePlayerSuicide: Option<_ForcePlayerSuicide> = None;

/// Disconnects a client from the server as soon as the next frame starts.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to be kicked.
/// * `reason` - (int32): The network-level reason code describing why the client is being disconnected.
/// * `message` - (string): The optional internal diagnostic message. If empty, no message is passed to the engine.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn KickClient(playerSlot: i32, reason: NetworkDisconnectionReason, message: &Str) {
    unsafe {
        __s2sdk_KickClient.expect("KickClient function was not found")(playerSlot, reason, message)
    }
}
pub type _KickClient = unsafe extern "C" fn(i32, NetworkDisconnectionReason, &Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_KickClient: Option<_KickClient> = None;
