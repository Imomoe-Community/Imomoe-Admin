use crate::{
    engine, s2sdk as sdk,
    state::{State, now},
};
use imomoe_admin_core::{
    punishment::{Duration, Kind},
    target::Actor,
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

mod sdk_cases;

const ACCOUNT: u64 = 76_561_198_000_000_000;
static SLOT_ACCOUNT: AtomicU64 = AtomicU64::new(ACCOUNT);
static PLAYER_BINDINGS: Mutex<()> = Mutex::new(());

unsafe extern "C" fn max_clients() -> i32 {
    1
}
unsafe extern "C" fn connected(slot: i32) -> bool {
    slot == 0
}
unsafe extern "C" fn not_in_game(_: i32) -> bool {
    false
}
unsafe extern "C" fn steam_id(_: i32) -> u64 {
    SLOT_ACCOUNT.load(Ordering::Relaxed)
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../configs/imomoe_admin")
}

fn copy_config(config: &Path) {
    fs::create_dir_all(config.join("translations")).unwrap();
    for file in [
        "admin.jsonc",
        "announcements.jsonc",
        "translations/en.jsonc",
        "translations/zh-CN.jsonc",
    ] {
        fs::copy(root().join(file), config.join(file)).unwrap();
    }
}

#[test]
fn ban_rechecks_still_loading_authenticated_account_and_refuses_stale_slot() {
    let _bindings = PLAYER_BINDINGS.lock().unwrap();
    unsafe {
        sdk::__s2sdk_GetMaxClients = Some(max_clients);
        sdk::__s2sdk_IsClientConnected = Some(connected);
        sdk::__s2sdk_IsClientInGame = Some(not_in_game);
        sdk::__s2sdk_IsClientAuthorized = Some(connected);
        sdk::__s2sdk_IsFakeClient = Some(not_in_game);
        sdk::__s2sdk_GetClientSteamID64 = Some(steam_id);
    }

    let tmp = tempfile::tempdir().unwrap();
    let mut state = State::new(root(), tmp.path().into()).unwrap();
    state
        .apply(
            Actor::Console,
            ACCOUNT,
            &[Kind::Ban],
            Duration::parse("10m").unwrap(),
            "test reason",
        )
        .unwrap();
    let expected_expiry = state.store.active(ACCOUNT, now()).unwrap()[0]
        .expires_at
        .unwrap();
    assert!(!engine::online(0));
    assert_eq!(engine::connected_slot(ACCOUNT), Some(0));
    SLOT_ACCOUNT.store(ACCOUNT + 1, Ordering::Relaxed);
    assert!(state.ban_for_connected(0, ACCOUNT).unwrap().is_none());

    SLOT_ACCOUNT.store(ACCOUNT, Ordering::Relaxed);
    let rule = state.ban_for_connected(0, ACCOUNT).unwrap().unwrap();
    assert_eq!(rule.reason, "test reason");
    assert_eq!(rule.expires_at, Some(expected_expiry));
}

#[test]
fn mute_only_changes_the_sender_and_restores_owned_voice_overrides() {
    use sdk::ListenOverride::{self, Default, Hear, Mute};

    static LISTEN: Mutex<[[ListenOverride; 3]; 3]> = Mutex::new([
        [Default; 3],
        [Hear, Default, Default],
        [Mute, Default, Default],
    ]);

    unsafe extern "C" fn three_clients() -> i32 {
        3
    }
    unsafe extern "C" fn present(slot: i32) -> bool {
        (0..3).contains(&slot)
    }
    unsafe extern "C" fn account_for_slot(slot: i32) -> u64 {
        ACCOUNT + slot as u64
    }
    unsafe extern "C" fn get(receiver: i32, sender: i32) -> ListenOverride {
        LISTEN.lock().unwrap()[receiver as usize][sender as usize]
    }
    unsafe extern "C" fn set(receiver: i32, sender: i32, mode: ListenOverride) {
        LISTEN.lock().unwrap()[receiver as usize][sender as usize] = mode;
    }

    let _bindings = PLAYER_BINDINGS.lock().unwrap();
    unsafe {
        sdk::__s2sdk_GetMaxClients = Some(three_clients);
        sdk::__s2sdk_IsClientConnected = Some(present);
        sdk::__s2sdk_IsClientInGame = Some(present);
        sdk::__s2sdk_IsClientAuthorized = Some(present);
        sdk::__s2sdk_IsClientSourceTV = Some(not_in_game);
        sdk::__s2sdk_IsFakeClient = Some(not_in_game);
        sdk::__s2sdk_GetClientSteamID64 = Some(account_for_slot);
        sdk::__s2sdk_GetClientListening = Some(get);
        sdk::__s2sdk_SetClientListening = Some(set);
    }

    let tmp = tempfile::tempdir().unwrap();
    let mut state = State::new(root(), tmp.path().into()).unwrap();
    let apply = |state: &mut State| {
        state
            .apply(
                Actor::Console,
                ACCOUNT,
                &[Kind::Mute],
                Duration::parse("10m").unwrap(),
                "test mute",
            )
            .unwrap();
    };
    apply(&mut state);
    assert!(state.has(0, Kind::Mute).unwrap());
    assert_eq!(sdk::GetClientListening(1, 0), Mute);
    assert_eq!(sdk::GetClientListening(2, 0), Mute);
    assert_eq!(sdk::GetClientListening(0, 1), Default);

    assert_eq!(
        state
            .revoke(Actor::Console, ACCOUNT, &[Kind::Mute])
            .unwrap(),
        1
    );
    assert!(!state.has(0, Kind::Mute).unwrap());
    assert_eq!(sdk::GetClientListening(1, 0), Hear);
    assert_eq!(sdk::GetClientListening(2, 0), Mute);

    apply(&mut state);
    sdk::SetClientListening(1, 0, Default);
    state.clear_map();
    assert_eq!(sdk::GetClientListening(1, 0), Default);
    assert_eq!(sdk::GetClientListening(2, 0), Mute);
}

#[test]
fn shipped_jsonc_config_and_both_locales_load() {
    let config = crate::config::Config::load(&root()).unwrap();
    assert_eq!(config.default_comms.expiration(0), Some(Some(600)));
    assert_eq!(config.phrase("en", "denied"), "Permission denied.");
    assert_eq!(config.phrase("zh-CN", "denied"), "没有权限。");
    assert_eq!(config.phrase("fr", "denied"), "Permission denied.");
}

#[test]
fn rejects_schema_invalid_locale_without_replacing_active_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("config");
    copy_config(&config);
    let mut state = State::new(config.clone(), tmp.path().join("state")).unwrap();
    let path = config.join("translations/zh-CN.jsonc");
    fs::write(&path, "{}").unwrap();
    assert!(
        state
            .reload()
            .unwrap_err()
            .to_string()
            .contains("missing or empty translation denied")
    );
    assert_eq!(state.config.phrase("zh-CN", "denied"), "没有权限。");

    let good = fs::read_to_string(root().join("translations/zh-CN.jsonc")).unwrap();
    fs::write(&path, good.replacen("没有权限。", "", 1)).unwrap();
    assert!(state.reload().is_err());
    assert_eq!(state.config.phrase("zh-CN", "denied"), "没有权限。");

    fs::write(&path, good.replacen("没有权限。", "重载后的文本。", 1)).unwrap();
    state.reload().unwrap();
    assert_eq!(state.config.phrase("zh-CN", "denied"), "重载后的文本。");
}

#[test]
fn jsonc_comments_and_trailing_commas_are_supported_without_json5() {
    use crate::config::Config;

    let tmp = tempfile::tempdir().unwrap();
    copy_config(tmp.path());
    for file in [
        "admin.jsonc",
        "announcements.jsonc",
        "translations/en.jsonc",
        "translations/zh-CN.jsonc",
    ] {
        let path = tmp.path().join(file);
        let good = fs::read_to_string(&path).unwrap();
        let commented = format!(
            "// example\n{},\n}}\n",
            good.trim_end().strip_suffix('}').unwrap().trim_end()
        );
        fs::write(&path, commented).unwrap();
        assert!(Config::load(tmp.path()).is_ok(), "{file}");
        let bad = good.replacen(',', "", 1);
        assert_ne!(bad, good, "{file}");
        fs::write(&path, bad).unwrap();
        assert!(
            Config::load(tmp.path()).is_err(),
            "{file} accepted a missing comma"
        );
        fs::write(&path, good).unwrap();
    }
}

fn assert_config_cases(file: &str, key: &str, cases: &[(Option<serde_json::Value>, &str)]) {
    use crate::config::Config;

    let tmp = tempfile::tempdir().unwrap();
    copy_config(tmp.path());
    let path = tmp.path().join(file);
    let good: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for (value, error) in cases {
        let mut invalid = good.clone();
        if let Some(value) = value {
            invalid[key] = value.clone();
        } else {
            invalid.as_object_mut().unwrap().remove(key);
        }
        fs::write(&path, invalid.to_string()).unwrap();
        let message = Config::load(tmp.path())
            .err()
            .unwrap_or_else(|| panic!("{file}: accepted {key} = {value:?}"))
            .to_string();
        assert!(message.contains(error), "{file}: {message}");
    }
}

#[test]
fn admin_config_rejects_unknown_missing_and_out_of_contract_values() {
    use serde_json::json;

    assert_config_cases(
        "admin.jsonc",
        "unexpected",
        &[(Some(json!(true)), "unknown field")],
    );
    assert_config_cases("admin.jsonc", "hide_on_join", &[(None, "missing field")]);
    assert_config_cases(
        "admin.jsonc",
        "$schema",
        &[(Some(json!(null)), "expected a string")],
    );
    assert_config_cases(
        "admin.jsonc",
        "storage",
        &[
            (
                Some(json!({"driver":"file","path":"ignored.db"})),
                "unknown field",
            ),
            (Some(json!({"driver":"sqlite"})), "missing field"),
            (
                Some(json!({"driver":"sqlite","path":"../outside.db"})),
                "relative file path",
            ),
        ],
    );
    assert_config_cases(
        "admin.jsonc",
        "default_ban",
        &[
            (Some(json!("+1m")), "invalid default_ban"),
            (Some(json!("01m")), "invalid default_ban"),
            (Some(json!("18446744073709551615w")), "invalid default_ban"),
        ],
    );
    assert_config_cases(
        "admin.jsonc",
        "allow_self",
        &[(Some(json!(["slap", "slap"])), "duplicate array item")],
    );
    assert_config_cases(
        "admin.jsonc",
        "cvar_allowlist",
        &[
            (
                Some(json!(["mp_freezetime", "mp_freezetime"])),
                "duplicate array item",
            ),
            (Some(json!(["MP_FREEZETIME"])), "lowercase ConVar names"),
        ],
    );
}

#[test]
fn announcements_reject_unknown_missing_and_disabled_active_schedule() {
    use serde_json::json;

    assert_config_cases(
        "announcements.jsonc",
        "unexpected",
        &[(Some(json!(0)), "unknown field")],
    );
    assert_config_cases("announcements.jsonc", "rules", &[(None, "missing field")]);
    assert_config_cases(
        "announcements.jsonc",
        "interval_seconds",
        &[(Some(json!(-1)), "expected u64")],
    );
    let tmp = tempfile::tempdir().unwrap();
    copy_config(tmp.path());
    let file = tmp.path().join("announcements.jsonc");
    fs::write(
        &file,
        r#"{"interval_seconds":0,"messages":["Hello"],"rules":[],"help":[]}"#,
    )
    .unwrap();
    assert!(
        crate::config::Config::load(tmp.path())
            .err()
            .unwrap()
            .to_string()
            .contains("interval must be positive")
    );
    fs::write(
        &file,
        r#"{"interval_seconds":0,"messages":[],"rules":[],"help":[]}"#,
    )
    .unwrap();
    assert!(crate::config::Config::load(tmp.path()).is_ok());
}

#[test]
fn both_locales_reject_missing_empty_unknown_and_non_string_entries() {
    use serde_json::json;

    for file in ["translations/en.jsonc", "translations/zh-CN.jsonc"] {
        assert_config_cases(
            file,
            "denied",
            &[
                (None, "missing or empty translation denied"),
                (Some(json!("")), "missing or empty translation denied"),
                (Some(json!(1)), file),
            ],
        );
        assert_config_cases(
            file,
            "unexpected",
            &[(Some(json!("Text")), "unknown translation unexpected")],
        );
    }
}

#[test]
fn noclip_toggles_the_current_pawn_and_releases_only_its_own_change() {
    use sdk::MoveType;
    use std::sync::{Mutex, atomic::AtomicI32};

    const SLOT: i32 = 2;
    const FIRST: i32 = 0x10042;
    const SECOND: i32 = 0x20042;
    static PAWN: AtomicI32 = AtomicI32::new(FIRST);
    static MODES: Mutex<[MoveType; 2]> = Mutex::new([MoveType::Walk, MoveType::Ladder]);

    unsafe extern "C" fn pawn(slot: i32) -> usize {
        if slot == SLOT {
            PAWN.load(Ordering::Relaxed) as usize
        } else {
            0
        }
    }
    unsafe extern "C" fn handle(pointer: usize) -> i32 {
        pointer as i32
    }
    unsafe extern "C" fn valid(handle: i32) -> bool {
        matches!(handle, FIRST | SECOND)
    }
    unsafe extern "C" fn get(handle: i32) -> MoveType {
        MODES.lock().unwrap()[usize::from(handle == SECOND)]
    }
    unsafe extern "C" fn set(handle: i32, mode: MoveType) {
        MODES.lock().unwrap()[usize::from(handle == SECOND)] = mode;
    }

    let _bindings = PLAYER_BINDINGS.lock().unwrap();
    unsafe {
        sdk::__s2sdk_GetClientPawn = Some(pawn);
        sdk::__s2sdk_EntPointerToEntHandle = Some(handle);
        sdk::__s2sdk_IsValidEntHandle = Some(valid);
        sdk::__s2sdk_GetEntityMoveType = Some(get);
        sdk::__s2sdk_SetEntityMoveType = Some(set);
    }

    let tmp = tempfile::tempdir().unwrap();
    let mut state = State::new(root(), tmp.path().into()).unwrap();
    assert!(state.toggle_noclip(SLOT));
    assert!(state.owns_noclip(SLOT, FIRST));
    PAWN.store(SECOND, Ordering::Relaxed);
    assert!(state.toggle_noclip(SLOT));
    assert!(state.owns_noclip(SLOT, SECOND));
    assert!(!state.owns_noclip(SLOT, FIRST));
    assert_eq!(*MODES.lock().unwrap(), [MoveType::Walk, MoveType::Noclip]);
    assert!(state.toggle_noclip(SLOT));
    assert_eq!(*MODES.lock().unwrap(), [MoveType::Walk, MoveType::Ladder]);

    // Disconnect must restore a still-valid frozen pawn before the slot is reused.
    assert!(state.set_movement(SLOT, MoveType::None));
    assert_eq!(MODES.lock().unwrap()[1], MoveType::None);
    state.cleanup_slot(SLOT);
    assert_eq!(MODES.lock().unwrap()[1], MoveType::Ladder);

    // Plugin unload uses clear_map while the server may still own a live pawn.
    assert!(state.toggle_noclip(SLOT));
    state.clear_map();
    assert_eq!(MODES.lock().unwrap()[1], MoveType::Ladder);
    assert!(!state.owns_noclip(SLOT, SECOND));

    assert!(state.toggle_noclip(SLOT));
    MODES.lock().unwrap()[1] = MoveType::Walk; // Engine or another owner changed it.
    state.clear_map();
    assert_eq!(*MODES.lock().unwrap(), [MoveType::Walk, MoveType::Walk]);
    assert!(!state.owns_noclip(SLOT, SECOND));
}
