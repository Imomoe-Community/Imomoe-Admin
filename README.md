<div align="center">

# Imomoe Admin (`imomoe_admin`)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Rust: 1.100.0](https://img.shields.io/badge/Rust-1.100.0%20(2024)-orange.svg)](.github/workflows/ci.yml)
[![CS2: Compatible](https://img.shields.io/badge/CS2-Dedicated%20Server-black.svg)](https://www.counter-strike.net/)
[![Platform: Plugify](https://img.shields.io/badge/Plugify-v3.1.0%2B-purple.svg)](https://github.com/untrustedmodders/plugify)
[![CI](https://github.com/Imomoe-Community/Imomoe-Admin/actions/workflows/ci.yml/badge.svg)](https://github.com/Imomoe-Community/Imomoe-Admin/actions/workflows/ci.yml)

**面向 Counter-Strike 2 专用服务端的独立 Plugify 管理与社区插件**<br>
<sub><sup>An independent Plugify moderation and community plugin for Counter-Strike 2 dedicated servers.</sup></sub>

**双语 / Bilingual** · [English](README.en.md)

</div>

---

### 项目简介 / Overview

**imomoe_admin** 是面向 Counter-Strike 2 (CS2) 专用服务端的独立 Rust [Plugify](https://github.com/untrustedmodders/plugify) 管理插件。玩家可用 `!admins` 查看在线管理，用 `!report <内容>` 或 `say @<内容>` 私下举报；管理权限、分组继承与免疫体系直接复用 `permissions`。
<sub><sup>**imomoe_admin** is a standalone Rust [Plugify](https://github.com/untrustedmodders/plugify) moderation plugin for CS2 dedicated servers. Players can find online admins with `!admins` and send private reports with `!report <message>` or `say @<message>`, while permissions, inheritance, and immunity are handled by `permissions`.</sup></sub>

---

### 环境要求 / Prerequisites

| 组件 / <sub><sup>Component</sup></sub> | 版本与说明 / <sub><sup>Requirement & Notes</sup></sub> |
| :--- | :--- |
| **Plugify 框架**<br><sub><sup>Plugify Host</sup></sub> | `v3.1.0+`，需加载 Rust / Go 语言模块、PolyHook、Configs 与 Translations。<br><sub><sup>`v3.1.0+` with Rust and Go language modules, PolyHook, Configs, and Translations.</sup></sub> |
| **S2SDK** | `>=3.4.3`；原版 [S2SDK v3.4.3](https://github.com/untrustedmodders/plugify-plugin-s2sdk/blob/v3.4.3/plugify-plugin-s2sdk.pplugin.in) 已含 `GetClientListening`、`SetClientListening` 与 `ServerCommand`，一期另需补充 `GetClientUserID`（见 [userid 补丁](integration/s2sdk-userid.patch)，后续迁入共享 C++ 底座）与 `ForceRespawnClient`。<br><sub><sup>`>=3.4.3`; vanilla v3.4.3 supplies `GetClientListening`, `SetClientListening`, and `ServerCommand`, while Phase 1 also requires `GetClientUserID` (see [userid patch](integration/s2sdk-userid.patch)) and `ForceRespawnClient`.</sup></sub> |
| **Permissions** | `permissions >=1.3.1` 与 `permissions_connector_s2`。<br><sub><sup>`permissions >=1.3.1` and `permissions_connector_s2`.</sup></sub> |
| **ServerFixes（可选）**<br><sub><sup>ServerFixes (Optional)</sup></sub> | 若启用 `sf_noclip_reset 1`，需包含 `Imomoe-ServerFixes-CS2 main@67232f8`（已合入 [noclip 所有权补丁](integration/serverfixes-noclip-ownership.patch)）。<br><sub><sup>When `sf_noclip_reset 1` is enabled, use a build containing `Imomoe-ServerFixes-CS2 main@67232f8` ([noclip ownership patch](integration/serverfixes-noclip-ownership.patch)).</sup></sub> |
| **Chat Prefix（可选）**<br><sub><sup>Chat Prefix (Optional)</sup></sub> | [Imomoe Chat Prefix](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix)（`imomoe_chat_prefix >=0.1.0`）；未安装时规则、帮助与定时公告以原文发送。<br><sub><sup>`imomoe_chat_prefix >=0.1.0`; when absent, rules, help, and scheduled announcements are sent without a prefix.</sup></sub> |

> [!IMPORTANT]
> - **Windows 权限依赖（Go TLS 修复） / Windows Permissions TLS Fix**：`permissions-connector-s2`（1.0.0）与 `permissions-storage`（1.0.2）若以标准 Go 1.27.4 编译同载，会因争用 `ArbitraryUserPointer` 导致 Go TLS 崩溃。部署 Windows 服务端前，请确认两组件已使用包含 [Go 上游修复 `35b807d`](https://github.com/golang/go/commit/35b807d) 的 Go 1.28+ 重编。<br><sub><sup>Recompile `permissions-connector-s2` (1.0.0) and `permissions-storage` (1.0.2) with Go 1.28+ containing upstream fix [`35b807d`](https://github.com/golang/go/commit/35b807d) to avoid the `ArbitraryUserPointer` TLS collision on Windows.</sup></sub>
> - **离线免疫查询 / Offline Immunity Profile Patch**：`permissions-storage` 1.0.2 需合入 [离线资料补丁](integration/permissions-storage-offline-profile.patch)，防止查询离线玩家免疫度时覆盖数据库既有昵称与最后到访时间。<br><sub><sup>Apply the [offline profile patch](integration/permissions-storage-offline-profile.patch) to `permissions-storage` 1.0.2 so offline immunity lookups do not overwrite stored names or `lastvisit_at`.</sup></sub>

> [!NOTE]
> `permissions` 1.3.1 先检查玩家临时节点，再检查永久节点。临时通配（如 `imomoe.admin.*`）会覆盖永久单项拒绝；临时提权时建议仅授予实际需要的节点。<br><sub><sup>`permissions` 1.3.1 checks temporary permissions before permanent ones; temporary wildcards override permanent denies.</sup></sub>

---

### 安装与部署 / Installation

1. 在项目根目录构建发布产物 / Build release artifacts:

   ```bash
   cargo build --release --workspace --locked
   ```

2. 将 [imomoe_admin.pplugin](imomoe_admin.pplugin) 与动态库放入服务端 `game/csgo/addons/plugify/envs/imomoe_admin/` / Deploy manifest and binary:
   - **Windows**：将 `target/release/imomoe_admin.dll` 放入 `bin/imomoe_admin.dll`。
   - **Linux**：将 `target/release/libimomoe_admin.so` 放入 `bin/libimomoe_admin.so`。

3. 将 `configs/imomoe_admin/` 复制到服务端 `game/csgo/addons/plugify/configs/imomoe_admin/`（含 `admin.jsonc`、`announcements.jsonc`、`translations/` 与 `schemas/`）。<br><sub><sup>Copy `configs/imomoe_admin/` to `game/csgo/addons/plugify/configs/imomoe_admin/`.</sup></sub>

4. 将 `imomoe_admin`、`s2sdk` 与 `imomoe_server_fixes` 分别置于独立插件目录（`s2sdk.*.jsonc` 与其清单同级），重启服务端使插件生效。<br><sub><sup>Keep `imomoe_admin`, `s2sdk`, and `imomoe_server_fixes` in separate plugin directories and restart the server.</sup></sub>

---

### 常用操作与目标选择 / Usage & Target Resolution

全部 35 条控制台命令、聊天别名与默认权限节点见 [一期命令矩阵](docs/SPEC.md#24-一期命令矩阵控制台统一-imomoe_-前缀) / See the [Phase 1 Command Matrix](docs/SPEC.md#24-一期命令矩阵控制台统一-imomoe_-前缀) for all 35 commands and permission nodes:

- **封禁与解封 / Bans & Unbans**：`imomoe_ban "player name" 2h <原因>` 封禁在线目标；`imomoe_addban <SteamID64> 7d <原因>` 录入离线封禁（省略时长使用默认值，`0` 表示永久）。对离线账号操作前会加载并比对 `permissions` 免疫度；客户端断连时显示 CS2 原生提示，管理员可用 `imomoe_banlist` 查询原因与到期时间。<br><sub><sup>`imomoe_ban` bans an online player and `imomoe_addban` records an offline SteamID64 ban (`0` = permanent). Offline targets also check `permissions` immunity; admins query reasons and expiry with `imomoe_banlist`.</sup></sub>
- **通信管控与目标定位 / Comms & Selectors**：`!silence #slot:4 10m` 同时禁麦与禁言（需 `imomoe.admin.mute` 与 `imomoe.admin.gag`）。可用 `#userid:12` 精确匹配引擎 userid；裸 `#12` 同时匹配 slot 与 userid，若命中多个候选则拒绝执行。<br><sub><sup>`!silence #slot:4 10m` applies both mute and gag. Use `#userid:12` for an exact engine userid; bare `#12` rejects execution if both a slot and a userid match.</sup></sub>
- **私聊与举报 / Admin Chat & Reports**：`!a <消息>` 或 `say_team @<消息>` 仅发送给拥有 `imomoe.admin.chat` 的在线管理；玩家通过 `say @<内容>` 或 `!report <内容>` 举报，消息不会留在公开聊天。<br><sub><sup>`!a` or `say_team @` reaches online admins with `imomoe.admin.chat`; player `say @` and `!report` never leak into public chat.</sup></sub>
- **救场与调队 / Rescue & Team Control**：`!tp <目标> <目的地>`、`!bring <目标>` 与 `!goto <目的地>` 要求两端均存活。`!team` 支持 `t`、`ct`、`spec`；`!swap` 在 T / CT 间切换。<br><sub><sup>Teleport commands require both endpoints to be alive. `!team` accepts `t`, `ct`, or `spec`; `!swap` toggles between T and CT.</sup></sub>

---

### 配置、存储与热重载 / Configuration & Storage

- **配置文件 / Config Files**：[admin.jsonc](configs/imomoe_admin/admin.jsonc) 配置存储驱动、默认时长、自指策略、ConVar 白名单、举报限流与审计开关；[announcements.jsonc](configs/imomoe_admin/announcements.jsonc) 配置规则、帮助与定时公告；按客户端语言选用 [zh-CN.jsonc](configs/imomoe_admin/translations/zh-CN.jsonc) 或 [en.jsonc](configs/imomoe_admin/translations/en.jsonc)。仓库仅保留通用默认值，本地覆盖副本可使用 `.gitignore` 排除的 `*.local.jsonc` / `*.private.jsonc`。<br><sub><sup>Repo keeps generic defaults only; place server overrides in deployment configs or ignored `*.local.jsonc` / `*.private.jsonc`.</sup></sub>
- **安全热重载 / Validated Reload**：`imomoe_reload` / `!iareload` 仅在全部 JSONC 配置与翻译通过 Schema 校验时替换当前配置；更换存储驱动或数据库路径需重启插件。<br><sub><sup>`imomoe_reload` / `!iareload` swaps config atomically only after all JSONC and translation schemas pass; changing storage driver or DB path requires a plugin restart.</sup></sub>
- **持久化目录 / Instance Storage**：所有状态均写入 Plugify 数据目录下的 `imomoe_admin/`：默认 SQLite 使用 `data/imomoe_admin.db`（保留历史），文件驱动使用 `data/bans.json` 与 `data/comms.json`（仅有效处罚）；举报固定写入 `data/reports.log`，可选审计写入 `data/audit.log`。<br><sub><sup>Stored under `<plugify-data>/imomoe_admin/`: SQLite (`data/imomoe_admin.db`, keeps history) or file driver (`data/bans.json`, `data/comms.json`, active only), plus `data/reports.log` and optional `data/audit.log`.</sup></sub>

---

### Plugify API

调用方只能在游戏主线程调用 [imomoe_admin.pplugin](imomoe_admin.pplugin) 声明的 9 个导出方法 / Callers must invoke the 9 exported methods in [imomoe_admin.pplugin](imomoe_admin.pplugin) on the game thread only:

| 方法 / <sub><sup>Method</sup></sub> | 说明 / <sub><sup>Description</sup></sub> |
| :--- | :--- |
| `IsClientMuted(slot)` | 查询指定槽位当前是否处于语音禁麦状态。<br><sub><sup>Returns whether the slot currently has an active voice mute.</sup></sub> |
| `IsClientGagged(slot)` | 查询指定槽位当前是否处于文字禁言状态。<br><sub><sup>Returns whether the slot currently has an active text gag.</sup></sub> |
| `IsAdminHidden(slot)` | 查询指定管理员槽位是否处于匿名执勤状态。<br><sub><sup>Returns whether the admin slot is currently in anonymous duty mode.</sup></sub> |
| `CanTarget(actorSlot, targetSlot)` | 查询原生免疫比较结果；调用方仍需自行校验权限。<br><sub><sup>Checks native immunity between actor and target; callers must still verify permissions.</sup></sub> |
| `KickClient(slot, reason)` | 受信插件踢出执行接口；调用前须由调用方完成权限与免疫校验。<br><sub><sup>Trusted kick primitive; caller must enforce permissions and immunity beforehand.</sup></sub> |
| `ApplyBan(actorSlot, targetSlot, minutes, reason)` | 按管理节点、免疫与自指规则执行封禁；`actorSlot = -1` 代表服务端，`minutes = 0` 表示永久。<br><sub><sup>Applies a ban enforcing permissions, immunity, and self-target rules (`-1` = server, `0` = permanent).</sup></sub> |
| `ApplyMute(actorSlot, targetSlot, minutes, reason)` | 按管理节点、免疫与自指规则执行禁麦；`actorSlot = -1` 代表服务端，`minutes = 0` 表示永久。<br><sub><sup>Applies a mute enforcing permissions, immunity, and self-target rules (`-1` = server, `0` = permanent).</sup></sub> |
| `NotifyAdmins(message)` | 向具备 `imomoe.admin.chat` 的在线管理员发送通知消息。<br><sub><sup>Broadcasts a notification to online admins holding `imomoe.admin.chat`.</sup></sub> |
| `OwnsClientNoclip(slot, pawnHandle)` | 与 ServerFixes 协同，按当前完整 pawn handle 判定是否由 Admin 持有 `noclip`。<br><sub><sup>Returns `true` only when Admin currently owns `noclip` on that exact pawn handle.</sup></sub> |

---

### 构建与开发 / Building & Development

本仓库使用 Rust 2024 Edition，CI 工具链固定为 `1.100.0`（见 [.github/workflows/ci.yml](.github/workflows/ci.yml)）。提交改动前运行标准门禁与离线配置校验（PowerShell `7.5+`）：
<sub><sup>Built on Rust 2024 Edition with toolchain `1.100.0` in CI. Run all gates and offline schema validation (PowerShell `7.5+`) before submitting changes:</sup></sub>

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
pwsh -File ./scripts/validate-config.ps1
cargo build --release --workspace --locked
```

使用服务端实际加载的清单重新生成强类型绑定 / Regenerate typed bindings against live server manifests:

```powershell
./scripts/generate-bindings.ps1 -Generator <plugify-gen> -S2SdkManifest <s2sdk.pplugin> -PermissionsManifest <permissions.pplugin>
```

- [需求与架构规格 / Spec](docs/SPEC.md) · [一期验收清单 / Acceptance](docs/ACCEPTANCE.md) · [双人实测清单 / Live Checklist](docs/ACCEPTANCE.md#最后集中验收真实客户端)
- [贡献指南 / Contributing](CONTRIBUTING.md) · [安全通告 / Security](SECURITY.md) · [版本记录 / Changelog](CHANGELOG.md)

---

### 开源协议与作者 / License & Author

- **作者 / Author**：Mai Ooizumi ([@Mai-Ooizumi](https://github.com/Mai-Ooizumi))
- **开源协议 / License**：GNU Affero General Public License v3.0 or later ([AGPL-3.0-or-later](LICENSE))
