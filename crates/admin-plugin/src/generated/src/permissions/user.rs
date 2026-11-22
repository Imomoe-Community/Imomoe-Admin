// Generated from permissions.pplugin (group: user)

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

/// Check players immunity or groups priority
///
/// # Arguments
/// * `actorID` - (uint64): Player performing the action
/// * `targetID` - (uint64): Player receiving the action
///
/// # Returns - (int32):
/// Allow, Disallow, ActorUserNotFound, or TargetUserNotFound
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn CanAffectUser(actorID: u64, targetID: u64) -> Status {
    unsafe {
        __permissions_CanAffectUser.expect("CanAffectUser function was not found")(
            actorID, targetID,
        )
    }
}
pub type _CanAffectUser = unsafe extern "C" fn(u64, u64) -> Status;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __permissions_CanAffectUser: Option<_CanAffectUser> = None;

/// Check if a user has a specific permission.
///
/// # Arguments
/// * `targetID` - (uint64): Player ID.
/// * `perm` - (string): Permission line.
///
/// # Returns - (int32):
/// Allow, Disallow, PermNotFound, TargetUserNotFound
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn HasPermission(targetID: u64, perm: &Str) -> Status {
    unsafe {
        __permissions_HasPermission.expect("HasPermission function was not found")(targetID, perm)
    }
}
pub type _HasPermission = unsafe extern "C" fn(u64, &Str) -> Status;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __permissions_HasPermission: Option<_HasPermission> = None;

/// Dispatches a request to load user data.
///
/// # Arguments
/// * `pluginID` - (int64): Identifier of the calling plugin.
/// * `targetID` - (uint64): PlayerID of the user to be loaded.
/// * `username` - (string): The user's current username. Intended for synchronizing the username with external storage (e.g. updating an existing record or setting it during initial user creation).
/// * `offline` - (bool): Indicates whether the user's data was loaded without user presence on server.
/// * `dontBroadcast` - (bool): If set to `true`, suppresses dispatching of the OnUserLoaded event.
///
/// # Returns - (int32):
/// Success, DBNotReady
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn LoadUser(
    pluginID: i64,
    targetID: u64,
    username: &Str,
    offline: bool,
    dontBroadcast: bool,
) -> Status {
    unsafe {
        __permissions_LoadUser.expect("LoadUser function was not found")(
            pluginID,
            targetID,
            username,
            offline,
            dontBroadcast,
        )
    }
}
pub type _LoadUser = unsafe extern "C" fn(i64, u64, &Str, bool, bool) -> Status;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __permissions_LoadUser: Option<_LoadUser> = None;
