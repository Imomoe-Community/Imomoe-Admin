// Generated from s2sdk.pplugin

#[allow(unused_imports)]
use super::aliases::*;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use plugify::{Arr, Mat4x4, Str, Var, Vec2, Vec3, Vec4};

/// Handles the execution of a command triggered by a caller. This function processes the command, interprets its context, and handles any provided arguments.
#[allow(unused)]
pub type ConCommandCallback = unsafe extern "C" fn(i32, ConCommandContext, &Arr<Str>) -> ResultType;

/// Handles events triggered by the game event system. This function processes the event data, determines the necessary action, and optionally prevents event broadcasting.
#[allow(unused)]
pub type EventCallback = unsafe extern "C" fn(&Str, usize, bool) -> ResultType;

/// Called when a client is entering the game.
#[allow(unused)]
pub type OnClientPutInServerCallback = unsafe extern "C" fn(i32);

/// Called when a client leaves the current map, for players and bots, before the player_disconnect event. Paired with OnClientPutInServer: a player staying through a map change gets it too, with the reason NETWORK_DISCONNECT_SHUTDOWN. It can also come without OnClientPutInServer if the client leaves before entering the game. Use OnClientSessionEnd for the end of the connection. The controller and pawn are unavailable once the callbacks return.
#[allow(unused)]
pub type OnClientDisconnectCallback = unsafe extern "C" fn(i32, NetworkDisconnectionReason);

/// Called when a client receives an auth ID.
#[allow(unused)]
pub type OnClientAuthenticatedCallback = unsafe extern "C" fn(i32, u64);

/// Called when when an entity is destroyed.
#[allow(unused)]
pub type OnEntityDeletedCallback = unsafe extern "C" fn(i32);

/// Called on every map end.
#[allow(unused)]
pub type OnMapEndCallback = unsafe extern "C" fn();

/// Called before every server frame. Note that you should avoid doing expensive computations or declaring large local arrays.
#[allow(unused)]
pub type OnGameFrameCallback = unsafe extern "C" fn(bool, bool, bool);
