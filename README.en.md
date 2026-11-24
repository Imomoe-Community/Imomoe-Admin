<div align="center">

# Imomoe Admin (`imomoe_admin`)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Rust: 1.100.0](https://img.shields.io/badge/Rust-1.100.0%20(2024)-orange.svg)](.github/workflows/ci.yml)
[![CS2: Compatible](https://img.shields.io/badge/CS2-Dedicated%20Server-black.svg)](https://www.counter-strike.net/)
[![Platform: Plugify](https://img.shields.io/badge/Plugify-v3.1.0%2B-purple.svg)](https://github.com/untrustedmodders/plugify)
[![CI](https://github.com/Imomoe-Community/Imomoe-Admin/actions/workflows/ci.yml/badge.svg)](https://github.com/Imomoe-Community/Imomoe-Admin/actions/workflows/ci.yml)

**An independent Plugify moderation and community plugin for Counter-Strike 2 dedicated servers.**

[双语 / Bilingual](README.md) · **English**

</div>

---

### Overview

**imomoe_admin** is a standalone Rust [Plugify](https://github.com/untrustedmodders/plugify) moderation plugin for Counter-Strike 2 dedicated servers. Players can list online admins via `!admins` and send private reports with `!report <message>` or `say @<message>`, while admin permissions, group inheritance, and immunity reuse `permissions`.

---

### Prerequisites

| Component | Requirement & Notes |
| :--- | :--- |
| **Plugify Host** | `v3.1.0+` with Plugify Rust & Go language modules, PolyHook, Configs, and Translations. |
| **S2SDK** | Pinned fork `3.5.1@54903d0` (SourceSDK `e4e7260`), with the official `v3.5.1` export ABI and no custom exports. |
| **NativeBase** | `0.4.0` supplies `GetClientUserID` and `ForceRespawnClient`. Admin can be installed without FunRounds. |
| **Permissions** | `permissions >=1.3.1` and `permissions_connector_s2`. |
| **ServerFixes (Optional)** | When `sf_noclip_reset 1` is enabled, use a build containing `Imomoe-ServerFixes-CS2 main@67232f8`, which includes noclip ownership support. |
| **Chat Prefix (Optional)** | [Imomoe Chat Prefix](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix) (`imomoe_chat_prefix >=0.1.0`). When absent, rules, help, and scheduled announcements are sent unprefixed. |

> [!IMPORTANT]
> - **Permission hints**: Apply the [connector 1.0.0 patch](integration/permissions-connector-hint-locale.patch) to fall back to English for missing center hints and skip empty hints.
> - **Windows co-loading**: Both stock DLLs lack [Go TLS fix `35b807d`](https://github.com/golang/go/commit/35b807d). Use the [permissions rebuild and install steps](README.md#权限组件复建--rebuild-permissions) to rebuild both libraries, including the platform-specific toolchain on Windows.
> - **Offline Immunity Profile Patch**: Apply [`integration/permissions-storage-offline-profile.patch`](integration/permissions-storage-offline-profile.patch) to `permissions-storage` 1.0.2 so offline immunity lookups do not overwrite stored player names or `lastvisit_at`.

> [!NOTE]
> `permissions` 1.3.1 evaluates temporary nodes before permanent ones. A temporary wildcard such as `imomoe.admin.*` overrides permanent per-node denials; grant only the specific nodes needed.

---

### Installation

1. Obtain the official [S2-Launcher 2.5.8](https://github.com/untrustedmodders/plugify-source2-launcher/releases/tag/v2.5.8) / [Plugify](https://github.com/untrustedmodders/plugify), [Rust module 1.3.0](https://github.com/untrustedmodders/plugify-module-rust/releases/tag/v1.3.0), [C++ module 2.4.5](https://github.com/untrustedmodders/plugify-module-cpp/releases/tag/v2.4.5), [PolyHook 2.5.9](https://github.com/untrustedmodders/plugify-plugin-polyhook/releases/tag/v2.5.9), [Configs 2.1.2](https://github.com/untrustedmodders/plugify-plugin-configs/releases/tag/v2.1.2). Install the Admin permissions dependencies separately: [Go module](https://github.com/untrustedmodders/plugify-module-golang/releases), [Permissions](https://github.com/untrustedmodders/plugify-plugin-permissions/releases), [Translations](https://github.com/untrustedmodders/plugify-plugin-translations/releases), [permissions-connector-s2](https://github.com/fr0nch/permissions-connector-s2/releases), [permissions-storage](https://github.com/fr0nch/permissions-storage/releases). Keep the official package manifests and resources and replace the two libraries as described above.
2. Stop the server. Install the matching-platform `s2sdk-fork-3.5.1-54903d0-<platform>.zip` and `imomoe-native-base-0.4.0-<platform>.zip`, then copy the Admin ZIP's `game/` tree into the server root. The Admin ZIP does not supply the shared plugins or permissions components.
3. If FunRounds already uses this exact pair, install only Admin and any missing permissions components. Before switching pairs, make sure existing consumers support the new pair and move any superseded same-name SDK or old `funrounds_native` out of the plugin scan path. Avoid running `mamba update --all` over the pinned SDK.
4. Keep existing Admin configuration, storage and announcements. On rollback, stop the server and remove only Admin if the other consumer still uses the shared dependencies. To return to the old provider, restore the entire previous consumer/SDK/provider/configuration set.

To build and package only Admin (Windows example):

```text
cargo build --release --workspace --locked
python scripts/package.py --platform windows-x64 --binary target/release/imomoe_admin.dll --archive dist/admin-windows-x64.zip
```

### Usage & Target Resolution

All 35 console commands, chat aliases, and default permission nodes are documented in [docs/SPEC.md](docs/SPEC.md#english):

- **Text Menu**: Open with `!admin` in chat or `imomoe_admin` in the game console. Type the shown digits in chat to choose an action and player; `0` goes back or exits. No addon required; actions keep their command permissions, immunity and defaults.
- **Bans & Unbans**: `imomoe_ban "player name" 2h <reason>` bans an online target; `imomoe_addban <SteamID64> 7d <reason>` records an offline ban (`0` = permanent; omitted uses configured default). Offline actions also load and compare `permissions` immunity. Clients receive CS2 native disconnect prompts while admins inspect reasons and expiry via `imomoe_banlist`.
- **Comms & Selectors**: `!silence #slot:4 10m` applies both voice mute and text gag (requires both `imomoe.admin.mute` and `imomoe.admin.gag`). Use `#userid:12` to target an exact engine userid; bare `#12` matches both slot and userid and refuses execution if ambiguous.
- **Admin Chat & Reports**: `!a <message>` or `say_team @<message>` routes exclusively to online admins holding `imomoe.admin.chat`. Player `say @<message>` and `!report <message>` forward to online admins and log to `data/reports.log` without leaking into public chat.
- **Rescue & Team Control**: `!tp <target> <destination>`, `!bring <target>`, and `!goto <destination>` require both players to be alive. `!team` accepts `t`, `ct`, or `spec`; `!swap` toggles between T and CT.

---

### Configuration & Storage

- **Config Files**: [admin.jsonc](configs/imomoe_admin/admin.jsonc) configures storage driver, default durations, self-targeting rules, ConVar allowlist, report cooldown, and audit logging; [announcements.jsonc](configs/imomoe_admin/announcements.jsonc) configures rules, help, and scheduled announcements; client language selects [zh-CN.jsonc](configs/imomoe_admin/translations/zh-CN.jsonc) or [en.jsonc](configs/imomoe_admin/translations/en.jsonc). Keep server-specific overrides in deployment configs or `.gitignore`-excluded `*.local.jsonc` / `*.private.jsonc`.
- **Validated Reload**: `imomoe_reload` / `!iareload` replaces active settings only after all JSONC configs and translations pass schema validation. Changing storage driver or database path requires restarting the plugin.
- **Instance Storage**: State lives under `<plugify-data>/imomoe_admin/`: SQLite defaults to `data/imomoe_admin.db` (retains history), file driver writes `data/bans.json` and `data/comms.json` (active restrictions only), reports always append to `data/reports.log`, and optional audit records append to `data/audit.log`.

---

### Plugify API

Callers must invoke the 9 exported methods in [imomoe_admin.pplugin](imomoe_admin.pplugin) on the game thread only:

| Method | Description |
| :--- | :--- |
| `IsClientMuted(slot)` | Returns whether the slot currently has an active voice mute. |
| `IsClientGagged(slot)` | Returns whether the slot currently has an active text gag. |
| `IsAdminHidden(slot)` | Returns whether the admin slot is currently in anonymous duty mode. |
| `CanTarget(actorSlot, targetSlot)` | Checks native immunity between actor and target; callers must still verify permissions. |
| `KickClient(slot, reason)` | Trusted kick primitive; callers must verify permissions and immunity beforehand. |
| `ApplyBan(actorSlot, targetSlot, minutes, reason)` | Applies a ban enforcing admin nodes, immunity, and self-target policy (`actorSlot = -1` for server, `0` = permanent). |
| `ApplyMute(actorSlot, targetSlot, minutes, reason)` | Applies a mute enforcing admin nodes, immunity, and self-target policy (`actorSlot = -1` for server, `0` = permanent). |
| `NotifyAdmins(message)` | Sends a notification to online admins holding `imomoe.admin.chat`. |
| `OwnsClientNoclip(slot, pawnHandle)` | Used by ServerFixes to verify whether Admin currently owns `noclip` on that exact full pawn handle. |

---

### Building & Development

Uses Rust 2024 Edition with toolchain `1.100.0` in CI ([.github/workflows/ci.yml](.github/workflows/ci.yml)). Run all Cargo gates and PowerShell `7.5+` schema validation before submitting changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
pwsh -File ./scripts/validate-config.ps1
cargo build --release --workspace --locked
```

Regenerate typed bindings against live server manifests:

```powershell
./scripts/generate-bindings.ps1 -Generator <plugify-gen> -S2SdkManifest <s2sdk.pplugin> -NativeBaseManifest <imomoe_native_base.pplugin> -PermissionsManifest <permissions.pplugin>
```

- [Architecture & Spec](docs/SPEC.md#english) · [Acceptance & Live Checklist](docs/ACCEPTANCE.md#english)
- [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)

---

### License & Author

- **Author**: Mai Ooizumi ([@Mai-Ooizumi](https://github.com/Mai-Ooizumi))
- **License**: GNU Affero General Public License v3.0 or later ([AGPL-3.0-or-later](LICENSE))
