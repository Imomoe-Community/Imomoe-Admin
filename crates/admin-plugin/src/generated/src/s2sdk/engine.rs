// Generated from s2sdk.pplugin (group: engine)

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

/// Returns the maximum number of clients that can connect to the server.
///
/// # Returns - (int32):
/// The maximum client count, or -1 if global variables are not initialized.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetMaxClients() -> i32 {
    unsafe { __s2sdk_GetMaxClients.expect("GetMaxClients function was not found")() }
}
pub type _GetMaxClients = unsafe extern "C" fn() -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetMaxClients: Option<_GetMaxClients> = None;
