// Generated from s2sdk.pplugin (group: console)

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

/// Sends a message to the server console.
///
/// # Arguments
/// * `msg` - (string): The message to be sent to the server console.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn PrintToServer(msg: &Str) {
    unsafe { __s2sdk_PrintToServer.expect("PrintToServer function was not found")(msg) }
}
pub type _PrintToServer = unsafe extern "C" fn(&Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_PrintToServer: Option<_PrintToServer> = None;

/// Prints a message to a specific client in the chat area.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to whom the message will be sent.
/// * `message` - (string): The message to be printed in the chat area.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn PrintToChat(playerSlot: i32, message: &Str) {
    unsafe { __s2sdk_PrintToChat.expect("PrintToChat function was not found")(playerSlot, message) }
}
pub type _PrintToChat = unsafe extern "C" fn(i32, &Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_PrintToChat: Option<_PrintToChat> = None;

/// Prints a html message to a specific client in the center of the screen.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to whom the message will be sent.
/// * `message` - (string): The HTML-formatted message to be printed.
/// * `duration` - (int32): The duration of the message in seconds.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn PrintCentreHtml(playerSlot: i32, message: &Str, duration: i32) {
    unsafe {
        __s2sdk_PrintCentreHtml.expect("PrintCentreHtml function was not found")(
            playerSlot, message, duration,
        )
    }
}
pub type _PrintCentreHtml = unsafe extern "C" fn(i32, &Str, i32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_PrintCentreHtml: Option<_PrintCentreHtml> = None;

/// Prints a colored message to a specific client in the chat area.
///
/// # Arguments
/// * `playerSlot` - (int32): The index of the player's slot to whom the message will be sent.
/// * `message` - (string): The message to be printed in the chat area with color.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn PrintToChatColored(playerSlot: i32, message: &Str) {
    unsafe {
        __s2sdk_PrintToChatColored.expect("PrintToChatColored function was not found")(
            playerSlot, message,
        )
    }
}
pub type _PrintToChatColored = unsafe extern "C" fn(i32, &Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_PrintToChatColored: Option<_PrintToChatColored> = None;
