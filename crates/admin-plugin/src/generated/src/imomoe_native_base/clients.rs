// Generated from imomoe_native_base.pplugin (group: clients)

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

/// Returns the current engine userid, or -1 for a disconnected or invalid slot.
///
/// # Arguments
/// * `playerSlot` - (int32)
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetClientUserID(playerSlot: i32) -> i32 {
    unsafe {
        __imomoe_native_base_GetClientUserID.expect("GetClientUserID function was not found")(
            playerSlot,
        )
    }
}
pub type _GetClientUserID = unsafe extern "C" fn(i32) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __imomoe_native_base_GetClientUserID: Option<_GetClientUserID> = None;
