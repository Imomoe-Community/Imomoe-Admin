use super::{ACCOUNT, PLAYER_BINDINGS, copy_config, not_in_game};
use crate::{self as admin, dispatch, engine, permissions as perm, s2sdk as sdk, state::State};
use imomoe_admin_core::{
    command::{lookup, words},
    punishment::{Duration, Kind},
    target::Actor,
};
use plugify::{Str, Vec3, string};
use std::{
    fs,
    sync::{Mutex, MutexGuard, Once},
    time::{Duration as Elapsed, Instant},
};

use sdk::{CSTeam, ListenOverride, MoveType, NetworkDisconnectionReason as Reason};

const OP: &str = "Lead Admin";
const DEPUTY: &str = "Deputy Admin";
const PLAYER: &str = "Player With Spaces";
const PAWN: i32 = 0x10042;
const CLIENTS: i32 = 4;

struct World {
    messages: Vec<(i32, String)>,
    kicks: Vec<(i32, Reason, String)>,
    voice: [[ListenOverride; 4]; 4],
    cvar_reads: Vec<u64>,
    cvar_writes: Vec<(u64, String)>,
    freezetime: String,
    movement: MoveType,
    teams: [CSTeam; 4],
    alive: [bool; 4],
    origin: [Vec3; 4],
    respawned: Vec<i32>,
    commands: Vec<String>,
}

static WORLD: Mutex<Option<World>> = Mutex::new(None);

fn world<T>(run: impl FnOnce(&mut World) -> T) -> T {
    run(WORLD.lock().unwrap().as_mut().unwrap())
}

// The Plugify string imports use a C-layout (data, size, capacity) token.
// In tests the token owns a Rust boxed byte slice rather than a foreign allocation.
unsafe extern "C" fn construct(data: *const u8, len: usize) -> Str {
    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let ptr = Box::into_raw(Box::<[u8]>::from(bytes));
    unsafe { std::mem::transmute::<[usize; 3], Str>([ptr.cast::<u8>() as usize, len, len]) }
}

unsafe fn raw(s: *const Str) -> [usize; 3] {
    unsafe { s.cast::<[usize; 3]>().read() }
}

unsafe extern "C" fn data(s: *const Str) -> *mut u8 {
    unsafe { raw(s)[0] as *mut u8 }
}

unsafe extern "C" fn len(s: *const Str) -> usize {
    unsafe { raw(s)[1] }
}

unsafe extern "C" fn destroy(s: *mut Str) {
    let [ptr, len, _] = unsafe { raw(s) };
    unsafe {
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            ptr as *mut u8,
            len,
        )))
    };
}

unsafe extern "C" fn max_clients() -> i32 {
    CLIENTS
}
unsafe extern "C" fn present(slot: i32) -> bool {
    (0..CLIENTS).contains(&slot)
}
unsafe extern "C" fn user_id(slot: i32) -> i32 {
    slot + 10
}
unsafe extern "C" fn account(slot: i32) -> u64 {
    ACCOUNT + slot as u64
}
unsafe extern "C" fn name(slot: i32) -> Str {
    Str::from_str([OP, DEPUTY, PLAYER, "Training Bot"][slot as usize])
}
unsafe extern "C" fn team(slot: i32) -> CSTeam {
    world(|w| w.teams[slot as usize])
}
unsafe extern "C" fn set_team(slot: i32, team: CSTeam) {
    world(|w| w.teams[slot as usize] = team);
}
unsafe extern "C" fn alive(slot: i32) -> bool {
    world(|w| w.alive[slot as usize])
}
unsafe extern "C" fn fake(slot: i32) -> bool {
    slot == 3
}
unsafe extern "C" fn language(_: i32) -> Str {
    Str::from_str("en")
}
unsafe extern "C" fn chat(slot: i32, message: &Str) {
    world(|w| w.messages.push((slot, message.as_str().into())));
}
unsafe extern "C" fn console(message: &Str) {
    unsafe { chat(-1, message) };
}
unsafe extern "C" fn kick(slot: i32, reason: Reason, message: &Str) {
    world(|w| w.kicks.push((slot, reason, message.as_str().into())));
}
unsafe extern "C" fn listen(receiver: i32, sender: i32) -> ListenOverride {
    world(|w| w.voice[receiver as usize][sender as usize])
}
unsafe extern "C" fn set_listen(receiver: i32, sender: i32, value: ListenOverride) {
    world(|w| w.voice[receiver as usize][sender as usize] = value);
}
unsafe extern "C" fn permission(id: u64, node: &Str) -> perm::Status {
    let node = node.as_str();
    match id - ACCOUNT {
        0 => perm::Status::Allow,
        1 if matches!(
            node,
            "imomoe.admin.ban" | "imomoe.admin.mute" | "imomoe.admin.chat"
        ) =>
        {
            perm::Status::Allow
        }
        _ => perm::Status::Disallow,
    }
}
unsafe extern "C" fn immunity(actor: u64, target: u64) -> perm::Status {
    if actor <= target {
        perm::Status::Allow
    } else {
        perm::Status::Disallow
    }
}
unsafe extern "C" fn find_cvar(name: &Str) -> u64 {
    match name.as_str() {
        "mp_freezetime" => 1,
        "example_cheat" => 2,
        "mp_roundtime" => 3,
        _ => 0,
    }
}
unsafe extern "C" fn cvar_flag(handle: u64, flag: i64) -> bool {
    handle == 2 && flag == sdk::ConVarFlag::Cheat as i64
}
unsafe extern "C" fn get_cvar(handle: u64) -> Str {
    world(|w| {
        w.cvar_reads.push(handle);
        Str::from_str(&w.freezetime)
    })
}
unsafe extern "C" fn set_cvar(handle: u64, value: &Str, _: bool, _: bool) {
    world(|w| {
        w.cvar_writes.push((handle, value.as_str().into()));
        w.freezetime = value.as_str().into();
    });
}
unsafe extern "C" fn pawn(slot: i32) -> usize {
    if (0..CLIENTS).contains(&slot) {
        (PAWN + slot) as usize
    } else {
        0
    }
}
unsafe extern "C" fn handle(pointer: usize) -> i32 {
    pointer as i32
}
unsafe extern "C" fn valid(handle: i32) -> bool {
    (PAWN..PAWN + CLIENTS).contains(&handle)
}
unsafe extern "C" fn movement(_: i32) -> MoveType {
    world(|w| w.movement)
}
unsafe extern "C" fn set_movement(_: i32, mode: MoveType) {
    world(|w| w.movement = mode);
}
unsafe extern "C" fn origin(slot: i32) -> Vec3 {
    world(|w| w.origin[slot as usize])
}
unsafe extern "C" fn set_origin(slot: i32, pos: &Vec3) {
    world(|w| w.origin[slot as usize] = *pos);
}
unsafe extern "C" fn respawn(slot: i32) {
    world(|w| w.respawned.push(slot));
}
unsafe extern "C" fn server_command(command: &Str) {
    world(|w| w.commands.push(command.as_str().into()));
}

struct Session {
    dir: tempfile::TempDir,
    _bindings: MutexGuard<'static, ()>,
}

impl Session {
    fn start() -> Self {
        let bindings = PLAYER_BINDINGS.lock().unwrap();
        static STRINGS: Once = Once::new();
        STRINGS.call_once(|| {
            string::init_construct_string(construct as *const () as usize);
            string::init_destroy_string(destroy as *const () as usize);
            string::init_get_string_data(data as *const () as usize);
            string::init_get_string_length(len as *const () as usize);
        });
        *WORLD.lock().unwrap() = Some(World {
            messages: Vec::new(),
            kicks: Vec::new(),
            voice: [[ListenOverride::Default; 4]; 4],
            cvar_reads: Vec::new(),
            cvar_writes: Vec::new(),
            freezetime: "5".into(),
            movement: MoveType::Walk,
            teams: [CSTeam::CT, CSTeam::T, CSTeam::T, CSTeam::CT],
            alive: [true, true, true, true],
            origin: [Vec3::new(0.0, 0.0, 0.0); 4],
            respawned: Vec::new(),
            commands: Vec::new(),
        });
        unsafe {
            sdk::__s2sdk_GetMaxClients = Some(max_clients);
            sdk::__s2sdk_IsClientConnected = Some(present);
            sdk::__s2sdk_IsClientInGame = Some(present);
            sdk::__s2sdk_IsClientAuthorized = Some(present);
            sdk::__s2sdk_IsClientSourceTV = Some(not_in_game);
            sdk::__s2sdk_IsFakeClient = Some(fake);
            sdk::__s2sdk_GetClientSteamID64 = Some(account);
            admin::imomoe_native_base::__imomoe_native_base_GetClientUserID = Some(user_id);
            sdk::__s2sdk_GetClientName = Some(name);
            sdk::__s2sdk_GetClientTeam = Some(team);
            sdk::__s2sdk_IsClientAlive = Some(alive);
            sdk::__s2sdk_GetClientLanguageCode = Some(language);
            sdk::__s2sdk_PrintToChat = Some(chat);
            sdk::__s2sdk_PrintToServer = Some(console);
            sdk::__s2sdk_KickClient = Some(kick);
            sdk::__s2sdk_GetClientListening = Some(listen);
            sdk::__s2sdk_SetClientListening = Some(set_listen);
            sdk::__s2sdk_FindConVar = Some(find_cvar);
            sdk::__s2sdk_IsConVarFlagSet = Some(cvar_flag);
            sdk::__s2sdk_GetConVarValue = Some(get_cvar);
            sdk::__s2sdk_SetConVarValue = Some(set_cvar);
            sdk::__s2sdk_GetClientPawn = Some(pawn);
            sdk::__s2sdk_EntPointerToEntHandle = Some(handle);
            sdk::__s2sdk_IsValidEntHandle = Some(valid);
            sdk::__s2sdk_GetEntityMoveType = Some(movement);
            sdk::__s2sdk_SetEntityMoveType = Some(set_movement);
            sdk::__s2sdk_GetClientAbsOrigin = Some(origin);
            sdk::__s2sdk_SetClientAbsOrigin = Some(set_origin);
            sdk::__s2sdk_ForceRespawnClient = Some(respawn);
            sdk::__s2sdk_ChangeClientTeam = Some(set_team);
            sdk::__s2sdk_SwitchClientTeam = Some(set_team);
            sdk::__s2sdk_ServerCommand = Some(server_command);
            perm::__permissions_HasPermission = Some(permission);
            perm::__permissions_CanAffectUser = Some(immunity);
        }
        let dir = tempfile::tempdir().unwrap();
        let config = dir.path().join("config");
        copy_config(&config);
        admin::STATE.set(Some(State::new(config, dir.path().into()).unwrap()));
        Self {
            dir,
            _bindings: bindings,
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        admin::STATE.set(None);
        *WORLD.lock().unwrap() = None;
    }
}

fn run(actor: Actor, command: &str) {
    let (console, text) = command
        .strip_prefix("imomoe_")
        .map_or((false, &command[1..]), |cmd| (true, cmd));
    let args = words(text).unwrap();
    let cmd = lookup(&args[0], !console).unwrap();
    admin::with_state(|state| dispatch::run(state, actor, cmd, &args[1..])).unwrap();
}

fn messages() -> Vec<(i32, String)> {
    world(|w| std::mem::take(&mut w.messages))
}

#[test]
fn high_risk_commands_reject_groups_and_ambiguity_before_any_penalty() {
    let _session = Session::start();
    for cmd in ["imomoe_ban", "imomoe_kick", "imomoe_noclip"] {
        for selector in ["@all", "@t", "@ct", "@bots"] {
            run(Actor::Console, &format!("{cmd} {selector}"));
            assert_eq!(
                messages(),
                [(-1, "This command requires a single target.".into())]
            );
        }
    }
    run(Actor::Console, "imomoe_kick Admin");
    let output = messages();
    assert!(output[0].1.contains("Lead Admin (#userid:10)"));
    assert!(output[0].1.contains("Deputy Admin (#userid:11)"));
    world(|w| {
        assert!(w.kicks.is_empty());
        assert_eq!(w.movement, MoveType::Walk);
    });
    assert!(
        admin::with_state(|s| s.store.list_bans(crate::state::now(), 0, 8).unwrap())
            .unwrap()
            .is_empty()
    );
    run(
        Actor::Console,
        r#"imomoe_kick "Player With Spaces" quoted reason"#,
    );
    assert_eq!(
        world(|w| std::mem::take(&mut w.kicks)),
        [(2, Reason::Kicked, "quoted reason".into())]
    );
}

#[test]
fn command_permission_immunity_and_self_apply_before_group_penalties() {
    let _session = Session::start();
    run(Actor::Player(2), "!mute #slot:0 0");
    assert_eq!(messages(), [(2, "Permission denied.".into())]);
    run(Actor::Player(1), "!silence #slot:2 0"); // Mute but no gag permission.
    assert_eq!(messages(), [(1, "Permission denied.".into())]);
    assert!(!admin::IsClientMuted(2));
    assert!(!admin::IsClientGagged(2));
    run(Actor::Player(1), "!mute @humans 0");
    assert!(!admin::IsClientMuted(0)); // Higher immunity.
    assert!(!admin::IsClientMuted(1)); // Self.
    assert!(admin::IsClientMuted(2));
    let output = messages();
    assert!(output.iter().any(|(slot, text)| *slot == 1
        && text.starts_with("Not allowed to target:")
        && text.contains(OP)
        && text.contains(DEPUTY)));
}

#[test]
fn cvar_reads_writes_and_restore_are_limited_to_safe_allowlisted_handles() {
    let _session = Session::start();
    admin::with_state(|state| {
        state.config.admin.cvar_allowlist = ["mp_freezetime".into(), "example_cheat".into()].into();
    });
    for name in ["mp_roundtime", "example_cheat"] {
        run(Actor::Console, &format!("imomoe_cvar {name}"));
        run(Actor::Console, &format!("imomoe_cvar {name} 99"));
    }
    assert_eq!(
        messages(),
        vec![
            (
                -1,
                "That ConVar is not available through this command.".into()
            );
            4
        ]
    );
    world(|w| {
        assert!(w.cvar_reads.is_empty());
        assert!(w.cvar_writes.is_empty());
    });
    run(Actor::Console, "imomoe_cvar MP_FREEZETIME");
    run(Actor::Console, "imomoe_cvar mp_freezetime 11");
    run(Actor::Console, "imomoe_cvar mp_freezetime 5");
    assert_eq!(
        messages(),
        [
            (-1, "mp_freezetime = 5".into()),
            (-1, "mp_freezetime = 11".into()),
            (-1, "mp_freezetime = 5".into()),
        ]
    );
    world(|w| {
        assert_eq!(w.cvar_reads, [1, 1, 1]);
        assert_eq!(w.cvar_writes, [(1, "11".into()), (1, "5".into())]);
    });
}

#[test]
fn hidden_admin_leaves_public_list_and_attribution_but_stays_on_duty() {
    let _session = Session::start();
    run(Actor::Player(2), "!admins");
    assert_eq!(messages(), [(2, OP.into()), (2, DEPUTY.into())]);
    run(Actor::Player(0), "!kick #userid:12");
    assert!(messages().contains(&(1, format!("[Admin] {OP}: kick · {PLAYER}"))));
    run(Actor::Player(0), "!hide");
    assert!(admin::IsAdminHidden(0));
    messages();
    run(Actor::Player(2), "!admins");
    assert_eq!(messages(), [(2, DEPUTY.into())]);
    run(Actor::Player(0), "!who");
    assert!(
        messages()
            .iter()
            .any(|(_, text)| text.starts_with("Lead Admin [hidden]:"))
    );
    run(Actor::Player(0), "!kick #userid:12");
    assert!(messages().contains(&(1, format!("[Admin] kick · {PLAYER}"))));
    run(Actor::Player(0), "!hide");
    assert!(!admin::IsAdminHidden(0));
}

#[test]
fn report_cooldown_is_per_account_and_only_accepted_reports_are_saved_and_forwarded() {
    let session = Session::start();
    run(Actor::Player(0), "!hide");
    messages();
    admin::with_state(|state| {
        let players = engine::players();
        let start = Instant::now();
        let mut reconnected = players[2].clone();
        reconnected.slot = 5;
        assert!(state.report(&players[2], "first", start).unwrap());
        assert!(
            !state
                .report(&reconnected, "spam", start + Elapsed::from_secs(59))
                .unwrap()
        );
        assert!(
            state
                .report(&players[1], "another", start + Elapsed::from_secs(59))
                .unwrap()
        );
        assert!(
            state
                .report(&reconnected, "after", start + Elapsed::from_secs(60))
                .unwrap()
        );
        assert!(state.report(&players[3], "bot", start).is_err());
    })
    .unwrap();
    let output = messages();
    assert_eq!(output.len(), 6);
    assert!(output.iter().all(|(slot, text)| [0, 1].contains(slot)
        && text.starts_with("[Report]")
        && !text.contains("spam")));
    let logs = fs::read_to_string(session.dir.path().join("data/reports.log")).unwrap();
    let logged: Vec<serde_json::Value> = logs
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(
        logged
            .iter()
            .map(|entry| entry["message"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["first", "another", "after"]
    );
    assert_eq!(logged[0]["steam_id"].as_u64(), Some(ACCOUNT + 2));
}

#[test]
fn state_queries_and_exports_return_defaults_until_loaded() {
    let _session = Session::start();
    admin::STATE.set(None);
    let reason = Str::from_str("unloaded");
    assert!(!admin::IsClientMuted(0));
    assert!(!admin::IsClientGagged(0));
    assert!(!admin::IsAdminHidden(0));
    assert!(!admin::CanTarget(-1, 0));
    assert!(!admin::ApplyBan(-1, 0, 5, &reason));
    assert!(!admin::ApplyMute(-1, 0, 5, &reason));
    assert!(!admin::OwnsClientNoclip(0, PAWN));
    admin::KickClient(0, &reason);
    admin::NotifyAdmins(&reason);
    assert!(messages().is_empty());
    assert!(world(|w| w.kicks.is_empty()));
}

#[test]
fn exported_target_and_punishment_methods_enforce_actor_target_and_duration_boundaries() {
    let _session = Session::start();
    let reason = Str::from_str("test penalty");
    assert!(!admin::CanTarget(-2, 0));
    assert!(!admin::CanTarget(-1, CLIENTS));
    assert!(!admin::CanTarget(1, 0));
    assert!(admin::CanTarget(1, 2));
    assert!(admin::CanTarget(-1, 0));
    assert!(admin::CanTarget(-1, 3)); // Console can target a bot, but cannot persist a Steam punishment for it.
    for apply in [
        admin::ApplyBan as extern "C" fn(i32, i32, i32, &Str) -> bool,
        admin::ApplyMute,
    ] {
        for (actor, target, duration) in [
            (-2, 2, 1),
            (2, 0, 1),
            (1, 0, 1),
            (0, 0, 1),
            (-1, 2, -1),
            (-1, 3, 1),
        ] {
            assert!(
                !apply(actor, target, duration, &reason),
                "{actor}, {target}, {duration}"
            );
        }
    }
    assert!(world(|w| w.kicks.is_empty()));
    assert!(!admin::IsClientMuted(2));
    assert!(!admin::IsClientGagged(2));
    assert!(admin::ApplyMute(1, 2, 0, &reason));
    assert!(admin::IsClientMuted(2));
    assert!(!admin::IsClientMuted(CLIENTS));
    assert!(!admin::IsClientGagged(2));
    assert_eq!(world(|w| w.voice[0][2]), ListenOverride::Mute);
    assert!(admin::ApplyBan(-1, 2, 2, &reason));
    let kicked = world(|w| std::mem::take(&mut w.kicks));
    assert_eq!(kicked.len(), 1);
    assert_eq!((kicked[0].0, kicked[0].1), (2, Reason::RejectBanned));
    assert!(kicked[0].2.contains("test penalty"));
    admin::with_state(|state| {
        state
            .apply(
                Actor::Console,
                ACCOUNT + 2,
                &[Kind::Gag],
                Duration::parse("0").unwrap(),
                "test gag",
            )
            .unwrap();
        let rows = state
            .store
            .active(ACCOUNT + 2, crate::state::now())
            .unwrap();
        let ban = rows.iter().find(|row| row.kind == Kind::Ban).unwrap();
        let mute = rows.iter().find(|row| row.kind == Kind::Mute).unwrap();
        assert_eq!(ban.expires_at.unwrap() - ban.created_at, 120);
        assert_eq!(mute.expires_at, None);
        assert_eq!(mute.actor, Some(ACCOUNT + 1));
    })
    .unwrap();
    assert!(admin::IsClientGagged(2));
    assert!(!admin::IsClientGagged(CLIENTS));
}

#[test]
fn exported_kick_notification_and_noclip_route_only_to_expected_recipients() {
    let _session = Session::start();
    assert!(!admin::OwnsClientNoclip(0, PAWN));
    run(Actor::Player(0), "!noclip @me");
    assert!(admin::OwnsClientNoclip(0, PAWN));
    assert!(!admin::OwnsClientNoclip(0, PAWN + 1));
    assert!(!admin::OwnsClientNoclip(1, PAWN));
    let reason = Str::from_str("test\nkick");
    admin::KickClient(CLIENTS, &reason);
    assert!(world(|w| w.kicks.is_empty()));
    admin::KickClient(0, &reason);
    assert_eq!(
        world(|w| std::mem::take(&mut w.kicks)),
        [(0, Reason::Kicked, "testkick".into())]
    );
    assert!(!admin::OwnsClientNoclip(0, PAWN));
    assert_eq!(world(|w| w.movement), MoveType::Walk);
    messages();
    admin::NotifyAdmins(&Str::from_str("hello\nadmins"));
    assert_eq!(
        messages(),
        [
            (0, "[Admin chat] helloadmins".into()),
            (1, "[Admin chat] helloadmins".into())
        ]
    );
}

#[test]
fn rescue_and_match_commands_check_player_state_and_deliver_fixed_engine_actions() {
    let _session = Session::start();
    world(|w| {
        w.alive[1] = false;
        w.origin[2] = Vec3::new(10.0, 20.0, 30.0);
    });
    run(Actor::Console, "imomoe_tp #slot:0 #slot:1");
    assert_eq!(world(|w| w.origin[0].z), 0.0);
    run(Actor::Console, "imomoe_tp #slot:0 #slot:2");
    assert_eq!(
        world(|w| (w.origin[0].x, w.origin[0].y, w.origin[0].z)),
        (10.0, 20.0, 110.0)
    );
    run(Actor::Console, "imomoe_respawn #slot:2");
    run(Actor::Console, "imomoe_respawn #slot:1");
    assert_eq!(world(|w| std::mem::take(&mut w.respawned)), [1]);
    run(Actor::Console, "imomoe_team #slot:2 spec");
    run(Actor::Console, "imomoe_swap #slot:2");
    assert_eq!(world(|w| w.teams[2]), CSTeam::Spectator);
    run(Actor::Console, "imomoe_swap #slot:0");
    assert_eq!(world(|w| w.teams[0]), CSTeam::T);
    run(Actor::Player(2), "!pause");
    run(Actor::Console, "imomoe_restart 61");
    assert!(world(|w| w.commands.is_empty()));
    for command in ["imomoe_restart 2", "imomoe_pause", "imomoe_unpause"] {
        run(Actor::Console, command);
    }
    assert_eq!(
        world(|w| std::mem::take(&mut w.commands)),
        ["mp_restartgame 2", "mp_pause_match", "mp_unpause_match"]
    );
}
