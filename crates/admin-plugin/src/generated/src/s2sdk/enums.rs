// Generated from s2sdk.pplugin

#[allow(unused_imports)]
use plugify::vector_enum_traits;

/// Override of whether a receiver hears a sender.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenOverride {
    /// Voice flags decide.
    Default = 0,
    /// The receiver doesn't hear the sender.
    Mute = 1,
    /// The receiver hears the sender.
    Hear = 2,
}
vector_enum_traits!(ListenOverride, u8);

/// Enum representing the possible teams in Counter-Strike.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CSTeam {
    /// No team.
    None = 0,
    /// Spectator team.
    Spectator = 1,
    /// Terrorist team.
    T = 2,
    /// Counter-Terrorist team.
    CT = 3,
}
vector_enum_traits!(CSTeam, i32);

/// Enum representing reasons for network disconnection.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDisconnectionReason {
    /// Invalid.
    Invalid = 0,
    /// Shutdown.
    Shutdown = 1,
    /// Disconnect by user.
    DisconnectByUser = 2,
    /// Disconnect by server.
    DisconnectByServer = 3,
    /// Lost.
    Lost = 4,
    /// Overflow.
    Overflow = 5,
    /// Steam banned.
    SteamBanned = 6,
    /// Steam inuse.
    SteamInuse = 7,
    /// Steam ticket.
    SteamTicket = 8,
    /// Steam logon.
    SteamLogon = 9,
    /// Steam authcancelled.
    SteamAuthcancelled = 10,
    /// Steam authalreadyused.
    SteamAuthalreadyused = 11,
    /// Steam authinvalid.
    SteamAuthinvalid = 12,
    /// Steam vacbanstate.
    SteamVacbanstate = 13,
    /// Steam logged in elsewhere.
    SteamLoggedInElsewhere = 14,
    /// Steam vac check timedout.
    SteamVacCheckTimedout = 15,
    /// Steam dropped.
    SteamDropped = 16,
    /// Steam ownership.
    SteamOwnership = 17,
    /// Serverinfo overflow.
    ServerinfoOverflow = 18,
    /// Tickmsg overflow.
    TickmsgOverflow = 19,
    /// Stringtablemsg overflow.
    StringtablemsgOverflow = 20,
    /// Deltaentmsg overflow.
    DeltaentmsgOverflow = 21,
    /// Tempentmsg overflow.
    TempentmsgOverflow = 22,
    /// Soundsmsg overflow.
    SoundsmsgOverflow = 23,
    /// Snapshotoverflow.
    Snapshotoverflow = 24,
    /// Snapshoterror.
    Snapshoterror = 25,
    /// Reliableoverflow.
    Reliableoverflow = 26,
    /// Baddeltatick.
    Baddeltatick = 27,
    /// Nomoresplits.
    Nomoresplits = 28,
    /// Timedout.
    Timedout = 29,
    /// Disconnected.
    Disconnected = 30,
    /// Leavingsplit.
    Leavingsplit = 31,
    /// Differentclasstables.
    Differentclasstables = 32,
    /// Badrelaypassword.
    Badrelaypassword = 33,
    /// Badspectatorpassword.
    Badspectatorpassword = 34,
    /// Hltvrestricted.
    Hltvrestricted = 35,
    /// Nospectators.
    Nospectators = 36,
    /// Hltvunavailable.
    Hltvunavailable = 37,
    /// Hltvstop.
    Hltvstop = 38,
    /// Kicked.
    Kicked = 39,
    /// Banadded.
    Banadded = 40,
    /// Kickbanadded.
    Kickbanadded = 41,
    /// Hltvdirect.
    Hltvdirect = 42,
    /// Pureserver clientextra.
    PureserverClientextra = 43,
    /// Pureserver mismatch.
    PureserverMismatch = 44,
    /// Usercmd.
    Usercmd = 45,
    /// Rejected by game.
    RejectedByGame = 46,
    /// Message parse error.
    MessageParseError = 47,
    /// Invalid message error.
    InvalidMessageError = 48,
    /// Bad server password.
    BadServerPassword = 49,
    /// Direct connect reservation.
    DirectConnectReservation = 50,
    /// Connection failure.
    ConnectionFailure = 51,
    /// No peer group handlers.
    NoPeerGroupHandlers = 52,
    /// Reconnection.
    Reconnection = 53,
    /// Loopshutdown.
    Loopshutdown = 54,
    /// Loopdeactivate.
    Loopdeactivate = 55,
    /// Host endgame.
    HostEndgame = 56,
    /// Loop levelload activate.
    LoopLevelloadActivate = 57,
    /// Create server failed.
    CreateServerFailed = 58,
    /// Exiting.
    Exiting = 59,
    /// Request hoststate idle.
    RequestHoststateIdle = 60,
    /// Request hoststate hltvrelay.
    RequestHoststateHltvrelay = 61,
    /// Client consistency fail.
    ClientConsistencyFail = 62,
    /// Client unable to crc map.
    ClientUnableToCrcMap = 63,
    /// Client no map.
    ClientNoMap = 64,
    /// Client different map.
    ClientDifferentMap = 65,
    /// Server requires steam.
    ServerRequiresSteam = 66,
    /// Steam deny misc.
    SteamDenyMisc = 67,
    /// Steam deny bad anti cheat.
    SteamDenyBadAntiCheat = 68,
    /// Server shutdown.
    ServerShutdown = 69,
    /// Replay incompatible.
    ReplayIncompatible = 71,
    /// Connect request timedout.
    ConnectRequestTimedout = 72,
    /// Server incompatible.
    ServerIncompatible = 73,
    /// Localproblem manyrelays.
    LocalproblemManyrelays = 74,
    /// Localproblem hostedserverprimaryrelay.
    LocalproblemHostedserverprimaryrelay = 75,
    /// Localproblem networkconfig.
    LocalproblemNetworkconfig = 76,
    /// Localproblem other.
    LocalproblemOther = 77,
    /// Remote timeout.
    RemoteTimeout = 79,
    /// Remote timeout connecting.
    RemoteTimeoutConnecting = 80,
    /// Remote other.
    RemoteOther = 81,
    /// Remote badcrypt.
    RemoteBadcrypt = 82,
    /// Remote certnottrusted.
    RemoteCertnottrusted = 83,
    /// Unusual.
    Unusual = 84,
    /// Internal error.
    InternalError = 85,
    /// Reject badchallenge.
    RejectBadchallenge = 128,
    /// Reject nolobby.
    RejectNolobby = 129,
    /// Reject background map.
    RejectBackgroundMap = 130,
    /// Reject single player.
    RejectSinglePlayer = 131,
    /// Reject hidden game.
    RejectHiddenGame = 132,
    /// Reject lanrestrict.
    RejectLanrestrict = 133,
    /// Reject badpassword.
    RejectBadpassword = 134,
    /// Reject serverfull.
    RejectServerfull = 135,
    /// Reject invalidreservation.
    RejectInvalidreservation = 136,
    /// Reject failedchannel.
    RejectFailedchannel = 137,
    /// Reject connect from lobby.
    RejectConnectFromLobby = 138,
    /// Reject reserved for lobby.
    RejectReservedForLobby = 139,
    /// Reject invalidkeylength.
    RejectInvalidkeylength = 140,
    /// Reject oldprotocol.
    RejectOldprotocol = 141,
    /// Reject newprotocol.
    RejectNewprotocol = 142,
    /// Reject invalidconnection.
    RejectInvalidconnection = 143,
    /// Reject invalidcertlen.
    RejectInvalidcertlen = 144,
    /// Reject invalidsteamcertlen.
    RejectInvalidsteamcertlen = 145,
    /// Reject steam.
    RejectSteam = 146,
    /// Reject serverauthdisabled.
    RejectServerauthdisabled = 147,
    /// Reject servercdkeyauthinvalid.
    RejectServercdkeyauthinvalid = 148,
    /// Reject banned.
    RejectBanned = 149,
    /// Kicked teamkilling.
    KickedTeamkilling = 150,
    /// Kicked tk start.
    KickedTkStart = 151,
    /// Kicked untrustedaccount.
    KickedUntrustedaccount = 152,
    /// Kicked convictedaccount.
    KickedConvictedaccount = 153,
    /// Kicked competitivecooldown.
    KickedCompetitivecooldown = 154,
    /// Kicked teamhurting.
    KickedTeamhurting = 155,
    /// Kicked hostagekilling.
    KickedHostagekilling = 156,
    /// Kicked votedoff.
    KickedVotedoff = 157,
    /// Kicked idle.
    KickedIdle = 158,
    /// Kicked suicide.
    KickedSuicide = 159,
    /// Kicked nosteamlogin.
    KickedNosteamlogin = 160,
    /// Kicked nosteamticket.
    KickedNosteamticket = 161,
    /// Kicked inputautomation.
    KickedInputautomation = 162,
    /// Kicked vacnetabnormalbehavior.
    KickedVacnetabnormalbehavior = 163,
    /// Kicked insecureclient.
    KickedInsecureclient = 164,
}
vector_enum_traits!(NetworkDisconnectionReason, i32);

/// Enum representing various flags for ConVars and ConCommands.
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConVarFlag {
    /// The default, no flags at all.
    None = 0,
    /// Linked to a ConCommand.
    LinkedConcommand = 1,
    /// Hidden in released products. Automatically removed if ALLOW_DEVELOPMENT_CVARS is defined.
    DevelopmentOnly = 2,
    /// Defined by the game DLL.
    GameDll = 4,
    /// Defined by the client DLL.
    ClientDll = 8,
    /// Hidden. Doesn't appear in find or auto-complete. Like DEVELOPMENTONLY but cannot be compiled out.
    Hidden = 16,
    /// Server cvar; data is not sent since it's sensitive (e.g., passwords).
    Protected = 32,
    /// This cvar cannot be changed by clients connected to a multiplayer server.
    SpOnly = 64,
    /// Saved to vars.rc.
    Archive = 128,
    /// Notifies players when changed.
    Notify = 256,
    /// Changes the client's info string.
    UserInfo = 512,
    /// Hides the cvar from lookups.
    Missing0 = 1024,
    /// If this is a server cvar, changes are not logged to the file or console.
    Unlogged = 2048,
    /// Hides the cvar from lookups.
    Missing1 = 4096,
    /// Server-enforced setting on clients.
    Replicated = 8192,
    /// Only usable in singleplayer/debug or multiplayer with sv_cheats.
    Cheat = 16384,
    /// Causes auto-generated varnameN for splitscreen slots.
    PerUser = 32768,
    /// Records this cvar when starting a demo file.
    Demo = 65536,
    /// Excluded from demo files.
    DontRecord = 131072,
    /// Reserved for future use.
    Missing2 = 262144,
    /// Cvars tagged with this are available to customers.
    Release = 524288,
    /// Marks the cvar as a menu bar item.
    MenuBarItem = 1048576,
    /// Reserved for future use.
    Missing3 = 2097152,
    /// Cannot be changed by a client connected to a server.
    NotConnected = 4194304,
    /// Enables fuzzy matching for vconsole.
    VconsoleFuzzyMatching = 8388608,
    /// The server can execute this command on clients.
    ServerCanExecute = 16777216,
    /// Allows clients to execute this command.
    ClientCanExecute = 33554432,
    /// The server cannot query this cvar's value.
    ServerCannotQuery = 67108864,
    /// Sets focus in the vconsole.
    VconsoleSetFocus = 134217728,
    /// IVEngineClient::ClientCmd can execute this command.
    ClientCmdCanExecute = 268435456,
    /// Executes the cvar every tick.
    ExecutePerTick = 536870912,
}
vector_enum_traits!(ConVarFlag, i64);

/// Enum representing the possible results of an operation.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultType {
    /// The action continues to be processed without interruption.
    Continue = 0,
    /// Indicates that the action has altered the state or behavior during execution.
    Changed = 1,
    /// The action has been successfully handled, and no further action is required.
    Handled = 2,
    /// The action processing is halted, and no further steps will be executed.
    Stop = 3,
}
vector_enum_traits!(ResultType, i32);

/// The command execution context.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConCommandContext {
    /// The command execute from the client's console.
    Console = 0,
    /// The command execute from the client's chat.
    Chat = 1,
}
vector_enum_traits!(ConCommandContext, i32);

/// Enum representing the type of callback.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookMode {
    /// Callback will be executed before the original function
    Pre = 0,
    /// Callback will be executed after the original function
    Post = 1,
}
vector_enum_traits!(HookMode, u8);

/// Enum representing various movement types for entities.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    /// Never moves.
    None = 0,
    /// Previously isometric movement type.
    Isometric = 1,
    /// Player only - moving on the ground.
    Walk = 2,
    /// No gravity, but still collides with stuff.
    Fly = 3,
    /// Flies through the air and is affected by gravity.
    Flygravity = 4,
    /// Uses VPHYSICS for simulation.
    Vphysics = 5,
    /// No clip to world, push and crush.
    Push = 6,
    /// No gravity, no collisions, still has velocity/avelocity.
    Noclip = 7,
    /// Used by players only when going onto a ladder.
    Ladder = 8,
    /// Observer movement, depends on player's observer mode.
    Observer = 9,
    /// Allows the entity to describe its own physics.
    Custom = 10,
}
vector_enum_traits!(MoveType, i32);

/// Enum representing the type of callback.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventHookError {
    /// Indicates that the event hook was successfully created.
    Okay = 0,
    /// Indicates that the event name provided is invalid or does not exist.
    InvalidEvent = 1,
    /// Indicates that the event system is not currently active or initialized.
    NotActive = 2,
    /// Indicates that the callback function provided is invalid or not compatible with the event system.
    InvalidCallback = 3,
}
vector_enum_traits!(EventHookError, i32);

/// Ownership type for RAII wrappers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    Borrowed,
    Owned,
}
