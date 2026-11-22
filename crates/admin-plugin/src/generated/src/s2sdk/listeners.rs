// Generated from s2sdk.pplugin (group: listeners)

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

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientPutInServer_Register(callback: OnClientPutInServerCallback) {
    unsafe {
        __s2sdk_OnClientPutInServer_Register
            .expect("OnClientPutInServer_Register function was not found")(callback)
    }
}
pub type _OnClientPutInServer_Register = unsafe extern "C" fn(OnClientPutInServerCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientPutInServer_Register: Option<_OnClientPutInServer_Register> = None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientPutInServer_Unregister(callback: OnClientPutInServerCallback) {
    unsafe {
        __s2sdk_OnClientPutInServer_Unregister
            .expect("OnClientPutInServer_Unregister function was not found")(callback)
    }
}
pub type _OnClientPutInServer_Unregister = unsafe extern "C" fn(OnClientPutInServerCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientPutInServer_Unregister: Option<_OnClientPutInServer_Unregister> =
    None;

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientDisconnect_Register(callback: OnClientDisconnectCallback) {
    unsafe {
        __s2sdk_OnClientDisconnect_Register
            .expect("OnClientDisconnect_Register function was not found")(callback)
    }
}
pub type _OnClientDisconnect_Register = unsafe extern "C" fn(OnClientDisconnectCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientDisconnect_Register: Option<_OnClientDisconnect_Register> = None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientDisconnect_Unregister(callback: OnClientDisconnectCallback) {
    unsafe {
        __s2sdk_OnClientDisconnect_Unregister
            .expect("OnClientDisconnect_Unregister function was not found")(callback)
    }
}
pub type _OnClientDisconnect_Unregister = unsafe extern "C" fn(OnClientDisconnectCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientDisconnect_Unregister: Option<_OnClientDisconnect_Unregister> = None;

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientAuthenticated_Register(callback: OnClientAuthenticatedCallback) {
    unsafe {
        __s2sdk_OnClientAuthenticated_Register
            .expect("OnClientAuthenticated_Register function was not found")(callback)
    }
}
pub type _OnClientAuthenticated_Register = unsafe extern "C" fn(OnClientAuthenticatedCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientAuthenticated_Register: Option<_OnClientAuthenticated_Register> =
    None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnClientAuthenticated_Unregister(callback: OnClientAuthenticatedCallback) {
    unsafe {
        __s2sdk_OnClientAuthenticated_Unregister
            .expect("OnClientAuthenticated_Unregister function was not found")(callback)
    }
}
pub type _OnClientAuthenticated_Unregister = unsafe extern "C" fn(OnClientAuthenticatedCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnClientAuthenticated_Unregister: Option<_OnClientAuthenticated_Unregister> =
    None;

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnEntityDeleted_Register(callback: OnEntityDeletedCallback) {
    unsafe {
        __s2sdk_OnEntityDeleted_Register.expect("OnEntityDeleted_Register function was not found")(
            callback,
        )
    }
}
pub type _OnEntityDeleted_Register = unsafe extern "C" fn(OnEntityDeletedCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnEntityDeleted_Register: Option<_OnEntityDeleted_Register> = None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnEntityDeleted_Unregister(callback: OnEntityDeletedCallback) {
    unsafe {
        __s2sdk_OnEntityDeleted_Unregister
            .expect("OnEntityDeleted_Unregister function was not found")(callback)
    }
}
pub type _OnEntityDeleted_Unregister = unsafe extern "C" fn(OnEntityDeletedCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnEntityDeleted_Unregister: Option<_OnEntityDeleted_Unregister> = None;

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnMapEnd_Register(callback: OnMapEndCallback) {
    unsafe {
        __s2sdk_OnMapEnd_Register.expect("OnMapEnd_Register function was not found")(callback)
    }
}
pub type _OnMapEnd_Register = unsafe extern "C" fn(OnMapEndCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnMapEnd_Register: Option<_OnMapEnd_Register> = None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnMapEnd_Unregister(callback: OnMapEndCallback) {
    unsafe {
        __s2sdk_OnMapEnd_Unregister.expect("OnMapEnd_Unregister function was not found")(callback)
    }
}
pub type _OnMapEnd_Unregister = unsafe extern "C" fn(OnMapEndCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnMapEnd_Unregister: Option<_OnMapEnd_Unregister> = None;

/// Register callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnGameFrame_Register(callback: OnGameFrameCallback) {
    unsafe {
        __s2sdk_OnGameFrame_Register.expect("OnGameFrame_Register function was not found")(callback)
    }
}
pub type _OnGameFrame_Register = unsafe extern "C" fn(OnGameFrameCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnGameFrame_Register: Option<_OnGameFrame_Register> = None;

/// Unregister callback to event.
///
/// # Arguments
/// * `callback` - (function): Function callback.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn OnGameFrame_Unregister(callback: OnGameFrameCallback) {
    unsafe {
        __s2sdk_OnGameFrame_Unregister.expect("OnGameFrame_Unregister function was not found")(
            callback,
        )
    }
}
pub type _OnGameFrame_Unregister = unsafe extern "C" fn(OnGameFrameCallback);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_OnGameFrame_Unregister: Option<_OnGameFrame_Unregister> = None;
