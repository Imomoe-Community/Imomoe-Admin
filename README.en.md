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
| **S2SDK** | `>=3.4.3`; [vanilla v3.4.3](https://github.com/untrustedmodders/plugify-plugin-s2sdk/blob/v3.4.3/plugify-plugin-s2sdk.pplugin.in) already provides `GetClientListening`, `SetClientListening`, and `ServerCommand`. Phase 1 also requires `GetClientUserID` (see [userid patch](integration/s2sdk-userid.patch), later moving to the shared C++ base) and `ForceRespawnClient`. |
| **Permissions** | `permissions >=1.3.1` and `permissions_connector_s2`. |
| **ServerFixes (Optional)** | When `sf_noclip_reset 1` is enabled, use a build containing `Imomoe-ServerFixes-CS2 main@67232f8` ([noclip ownership patch](integration/serverfixes-noclip-ownership.patch)). |
| **Chat Prefix (Optional)** | [Imomoe Chat Prefix](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix) (`imomoe_chat_prefix >=0.1.0`). When absent, rules, help, and scheduled announcements are sent unprefixed. |

> [!IMPORTANT]
> - **Windows Permissions TLS Fix**: When `permissions-connector-s2` (1.0.0) and `permissions-storage` (1.0.2) built with stock Go 1.27.4 are loaded together on Windows, they contend for `ArbitraryUserPointer` and crash during `LoadGroups`. Rebuild both components with Go 1.28+ containing [Go upstream fix `35b807d`](https://github.com/golang/go/commit/35b807d).
> - **Offline Immunity Profile Patch**: Apply [`integration/permissions-storage-offline-profile.patch`](integration/permissions-storage-offline-profile.patch) to `permissions-storage` 1.0.2 so offline immunity lookups do not overwrite stored player names or `lastvisit_at`.

> [!NOTE]
> `permissions` 1.3.1 evaluates temporary nodes before permanent ones. A temporary wildcard such as `imomoe.admin.*` overrides permanent per-node denials; grant only the specific nodes needed.

---

### Installation

1. Build release artifacts:

   ```bash
   cargo build --release --workspace --locked
   ```

2. Place [imomoe_admin.pplugin](imomoe_admin.pplugin) and the compiled binary under `game/csgo/addons/plugify/envs/imomoe_admin/`:
   - **Windows**: `target/release/imomoe_admin.dll` → `bin/imomoe_admin.dll`
   - **Linux**: `target/release/libimomoe_admin.so` → `bin/libimomoe_admin.so`

3. Copy `configs/imomoe_admin/` to `game/csgo/addons/plugify/configs/imomoe_admin/` (including `admin.jsonc`, `announcements.jsonc`, `translations/`, and `schemas/`).

4. Keep `imomoe_admin`, `s2sdk`, and `imomoe_server_fixes` in separate plugin directories (`s2sdk.*.jsonc` alongside its manifest) and restart the server.

---

### Usage & Target Resolution

All 35 console commands, chat aliases, and default permission nodes are documented in [docs/SPEC.md](docs/SPEC.md#english):

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
./scripts/generate-bindings.ps1 -Generator <plugify-gen> -S2SdkManifest <s2sdk.pplugin> -PermissionsManifest <permissions.pplugin>
```

- [Architecture & Spec](docs/SPEC.md#english) · [Acceptance & Live Checklist](docs/ACCEPTANCE.md#english)
- [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)

---

### License & Author

- **Author**: Mai Ooizumi ([@Mai-Ooizumi](https://github.com/Mai-Ooizumi))
- **License**: GNU Affero General Public License v3.0 or later ([AGPL-3.0-or-later](LICENSE))
