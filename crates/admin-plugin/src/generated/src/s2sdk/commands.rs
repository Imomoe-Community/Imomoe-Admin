// Generated from s2sdk.pplugin (group: commands)

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

/// Creates a console command or hooks an already existing one.
///
/// # Arguments
/// * `name` - (string): The name of the console command.
/// * `description` - (string): A brief description of what the command does.
/// * `flags` - (int64): Command flags that define the behavior of the command.
/// * `callback` - (function): A callback function that is invoked when the command is executed.
/// * `type_` - (uint8): Whether the hook was in post mode (after processing) or pre mode (before processing).
///
/// # Returns - (bool):
/// true if the command was successfully created; otherwise, false.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn AddConsoleCommand(
    name: &Str,
    description: &Str,
    flags: ConVarFlag,
    callback: ConCommandCallback,
    type_: HookMode,
) -> bool {
    unsafe {
        __s2sdk_AddConsoleCommand.expect("AddConsoleCommand function was not found")(
            name,
            description,
            flags,
            callback,
            type_,
        )
    }
}
pub type _AddConsoleCommand =
    unsafe extern "C" fn(&Str, &Str, ConVarFlag, ConCommandCallback, HookMode) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_AddConsoleCommand: Option<_AddConsoleCommand> = None;

/// Removes a console command from the system.
///
/// # Arguments
/// * `name` - (string): The name of the command to be removed.
/// * `callback` - (function): The callback function associated with the command to be removed.
///
/// # Returns - (bool):
/// true if the command was successfully removed; otherwise, false.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn RemoveCommand(name: &Str, callback: ConCommandCallback) -> bool {
    unsafe { __s2sdk_RemoveCommand.expect("RemoveCommand function was not found")(name, callback) }
}
pub type _RemoveCommand = unsafe extern "C" fn(&Str, ConCommandCallback) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_RemoveCommand: Option<_RemoveCommand> = None;

/// Adds a callback that will fire when a command is sent to the server.
///
/// # Arguments
/// * `name` - (string): The name of the command.
/// * `callback` - (function): The callback function that will be invoked when the command is executed.
/// * `type_` - (uint8): Whether the hook was in post mode (after processing) or pre mode (before processing).
///
/// # Returns - (bool):
/// Returns true if the callback was successfully added, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn AddCommandListener(name: &Str, callback: ConCommandCallback, type_: HookMode) -> bool {
    unsafe {
        __s2sdk_AddCommandListener.expect("AddCommandListener function was not found")(
            name, callback, type_,
        )
    }
}
pub type _AddCommandListener = unsafe extern "C" fn(&Str, ConCommandCallback, HookMode) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_AddCommandListener: Option<_AddCommandListener> = None;

/// Removes a callback that fires when a command is sent to the server.
///
/// # Arguments
/// * `name` - (string): The name of the command.
/// * `callback` - (function): The callback function to be removed.
/// * `type_` - (uint8): Whether the hook was in post mode (after processing) or pre mode (before processing).
///
/// # Returns - (bool):
/// Returns true if the callback was successfully removed, false otherwise.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn RemoveCommandListener(name: &Str, callback: ConCommandCallback, type_: HookMode) -> bool {
    unsafe {
        __s2sdk_RemoveCommandListener.expect("RemoveCommandListener function was not found")(
            name, callback, type_,
        )
    }
}
pub type _RemoveCommandListener = unsafe extern "C" fn(&Str, ConCommandCallback, HookMode) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_RemoveCommandListener: Option<_RemoveCommandListener> = None;

/// Executes a server command as if it were run on the server console or through RCON.
///
/// # Arguments
/// * `command` - (string): The command to execute on the server.
#[track_caller]
#[allow(dead_code, non_snake_case)]
pub fn ServerCommand(command: &Str) {
    unsafe { __s2sdk_ServerCommand.expect("ServerCommand function was not found")(command) }
}
pub type _ServerCommand = unsafe extern "C" fn(&Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __s2sdk_ServerCommand: Option<_ServerCommand> = None;
