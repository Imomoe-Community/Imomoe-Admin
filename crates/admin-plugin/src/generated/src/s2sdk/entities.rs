// Generated from s2sdk.pplugin (group: entities)

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

/// Converts an entity pointer into an entity handle.
///
/// # Arguments
/// * `entity` - (ptr64): A pointer to the entity to convert.
///
/// # Returns - (int32):
/// The entity handle as an integer, or INVALID_EHANDLE_INDEX if the entity is nullptr.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn EntPointerToEntHandle(entity: usize) -> i32 {
    unsafe {
        __s2sdk_EntPointerToEntHandle.expect("EntPointerToEntHandle function was not found")(entity)
    }
}
pub type _EntPointerToEntHandle = unsafe extern "C" fn(usize) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_EntPointerToEntHandle: Option<_EntPointerToEntHandle> = None;

/// Checks if the provided entity handle is valid.
///
/// # Arguments
/// * `entityHandle` - (int32): The entity handle to check.
///
/// # Returns - (bool):
/// True if the entity handle is valid, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn IsValidEntHandle(entityHandle: i32) -> bool {
    unsafe {
        __s2sdk_IsValidEntHandle.expect("IsValidEntHandle function was not found")(entityHandle)
    }
}
pub type _IsValidEntHandle = unsafe extern "C" fn(i32) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_IsValidEntHandle: Option<_IsValidEntHandle> = None;

/// Retrieves the movement type of an entity.
///
/// # Arguments
/// * `entityHandle` - (int32): The handle of the entity whose movement type is to be retrieved.
///
/// # Returns - (int32):
/// The movement type of the entity, or 0 if the entity is invalid.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn GetEntityMoveType(entityHandle: i32) -> MoveType {
    unsafe {
        __s2sdk_GetEntityMoveType.expect("GetEntityMoveType function was not found")(entityHandle)
    }
}
pub type _GetEntityMoveType = unsafe extern "C" fn(i32) -> MoveType;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_GetEntityMoveType: Option<_GetEntityMoveType> = None;

/// Sets the movement type of an entity.
///
/// # Arguments
/// * `entityHandle` - (int32): The handle of the entity whose movement type is to be set.
/// * `moveType` - (int32): The new movement type to set for the entity.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn SetEntityMoveType(entityHandle: i32, moveType: MoveType) {
    unsafe {
        __s2sdk_SetEntityMoveType.expect("SetEntityMoveType function was not found")(
            entityHandle,
            moveType,
        )
    }
}
pub type _SetEntityMoveType = unsafe extern "C" fn(i32, MoveType);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_SetEntityMoveType: Option<_SetEntityMoveType> = None;
