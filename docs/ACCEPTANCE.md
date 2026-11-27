# 验收指南 / Acceptance Guide

[简体中文](#简体中文) | [English](#english)

---

## 简体中文

### 1. 构建门禁与运行基线

Windows 与 Linux 双端执行标准门禁与离线配置校验（PowerShell `7.5+`）：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
pwsh -File ./scripts/validate-config.ps1
cargo build --release --workspace --locked
```

在隔离测试服验证时，先确认以下配套依赖满足基线要求：

- **S2SDK / NativeBase**：配对安装 `s2sdk-fork-3.5.2-86efb99-<platform>.zip`（SourceSDK `e4e7260`，官方 `v3.5.2` 导出 ABI，无自定义导出）和 `imomoe-native-base-0.4.0-<platform>.zip`（提供 `GetClientUserID` 与 `ForceRespawnClient`）。用 `scripts/package.py --dependency-dir` 核对实际 ZIP 的 ABI 和 SDK 来源。
- **ServerFixes（可选）**：启用 `sf_noclip_reset 1` 时，选用包含 `Imomoe-ServerFixes-CS2 main@67232f8` 的构建，该提交已合入 noclip 所有权协同。
- **Permissions**：`permissions >=1.3.1`、`permissions-connector-s2 >=1.0.1`、`permissions-storage >=1.0.3`。Linux 使用官方包；Windows DLL 如缺 Go TLS `35b807d`，按 [权限组件复建与安装步骤](../README.md#权限组件复建--rebuild-permissions)无补丁重编。验收时在隔离 SQLite 中用有昵称和既定 lastvisit_at 的合成离线资料，检查 Admin 免疫放行与拒绝结果，并核对两个字段均不变。

---

### 2. 隔离服工程验证方向

在隔离服上使用 RCON 与 BOT 验证以下正常路径与安全边界：

1. **SDK 与跨插件 ABI**：确认 `GetClientUserID` 与 `status` 一致、`GetClientListening` / `SetClientListening` 读写恢复正常、`ServerCommand` 可执行对局命令、`KickClient` 正常踢出 BOT；用独立消费插件核对 [`imomoe_admin.pplugin`](../imomoe_admin.pplugin) 全部 9 个导出接口及权限/免疫/边界返回值。
2. **存储、分页与到期**：分别验证 SQLite（`data/imomoe_admin.db`）与文件驱动（`data/bans.json` / `data/comms.json`）的离线封禁、重启保留与解封；验证 8 条/页分页，以及短期封禁到期后自动移出有效列表且 SQLite 保留历史记录。
3. **配置热重载与 ConVar 白名单**：确认缺词条、空词条或非法 JSONC 在 `imomoe_reload` / `!iareload` 时明确报错且保留当前生效配置，修复后可重载；确认 `sv_cheats` 与带 `Cheat` 标志的 ConVar 即使写入 `cvar_allowlist` 仍被拒绝读写，仅安全白名单项允许访问。
4. **目标选择器与权限边界**：聊天优先用 `!admin` 数字菜单或无引号的 `#userid:N` 选人；核对两条路径都命中预期目标。聊天中的引号和尖括号选人不纳入 Phase 1 验收；控制台仍验证用双引号包住含空格昵称能命中预期目标。聊天 `!slap` 仍接受能唯一确定的无引号完整昵称；末尾数字既可能是昵称的一部分也可能是伤害值时，提示改用 `#userid:N`。模糊多候选昵称与高危指令群选（如 `@bots` / `@all`）被拒绝；确认离线免疫查询不覆盖账号已有昵称与最后到访时间，临时通配节点优先于永久拒绝并在到期后自动失效。
5. **救场、对局与 ServerFixes 协同**：验证 `imomoe_slap`、`imomoe_tp`、`imomoe_slay` / `imomoe_respawn`、服务端换队（`imomoe_team` / `imomoe_swap`）与对局控制（`imomoe_restart` / `imomoe_pause` / `imomoe_unpause`）；验证 `sf_noclip_reset 1` 在关闭 `sv_cheats` 时保留 Admin 持有 `noclip` 并清除作弊 `noclip`，且 `freeze` / `noclip` 在死亡重生、换图、断线及卸载回调后均回收至 `Walk`、不串到新 pawn；验证 `say @`、`say_team @` 与 `!` / `/` 命令不泄露到公开聊天。

---

### 最后集中验收真实客户端

确认 `imomoe_admin` 为 `Running` 且测试服已登录 Steam 后，使用两名获准的真实 Steam 账号同时进服，记录各端实际收到的文字、音频方向与处罚状态：

- [ ] **权限与目标**：低免疫不能处罚高免疫，反向可执行；临时权限到期失效。确认聊天用 `!admin` 数字菜单和 `#userid:N` 都能选中预期玩家。对照自指、slot/userid 冲突、重连前后的 userid 和同名前缀歧义；验证群选跳过受保护者、高危命令拒绝群选。
- [ ] **封禁、踢出与重连**：验证原生踢出/封禁断连提示、Steam 身份验证后拒连及解封后正常重连（封禁原因与到期时间通过 `imomoe_banlist` 核对）。
- [ ] **私信与隐身**：将其中一名账号切换为普通玩家，核对 `say @...`、`say_team @...`、`!report`、`!a` 的收件、非收件与公开频道；检查防刷冷却、`data/reports.log` 落盘，以及 `!hide` 对 `!admins` 与公开通告署名的影响。
- [ ] **通信处罚**：两人互听，对照禁麦、禁言、`silence`、到期与解除；验证 `!comms` 自查，解禁后确认已有第三方语音覆盖正常恢复。
- [ ] **客户端交互与 API**：验证管理命令能直接将目标切到指定队伍，不要求玩家自行选队；检查 `!bring`、`!goto`、公告、规则与中英文自适应；用独立消费插件核对真实账号的权限、处罚及管理通知收件。
- [ ] **恢复隔离服**：撤销临时权限与处罚，退出并恢复临时文件；确保日志中不保留任何 Steam 凭据或 RCON 密码。

---

## English

### 1. Build Gates & Runtime Baseline

Run the standard Cargo gates and offline schema validation (PowerShell `7.5+`) on Windows and Linux:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
pwsh -File ./scripts/validate-config.ps1
cargo build --release --workspace --locked
```

Confirm the isolated test server meets these runtime prerequisites:

- **S2SDK / NativeBase**: Install `s2sdk-fork-3.5.2-86efb99-<platform>.zip` (SourceSDK `e4e7260`, official `v3.5.2` export ABI, no custom exports) with `imomoe-native-base-0.4.0-<platform>.zip` (supplies `GetClientUserID` and `ForceRespawnClient`). Use `scripts/package.py --dependency-dir` to check the ZIPs' ABI and SDK provenance.
- **ServerFixes (Optional)**: When `sf_noclip_reset 1` is enabled, use a build containing `Imomoe-ServerFixes-CS2 main@67232f8`, which includes noclip ownership support.
- **Permissions**: `permissions >=1.3.1`, `permissions-connector-s2 >=1.0.1`, and `permissions-storage >=1.0.3`. Use the official packages on Linux. If the Windows DLLs lack Go TLS fix `35b807d`, follow the [unpatched rebuild steps](../README.md#权限组件复建--rebuild-permissions). In an isolated SQLite database, use synthetic offline profiles with known names and last visit timestamps; verify Admin's immunity allow/deny results and that both fields are unchanged.

---

### 2. Isolated Server Verification

Verify the following happy paths and safety boundaries on an isolated server via RCON and bots:

1. **SDK & Cross-Plugin ABI**: Verify `GetClientUserID` matches `status`, voice listening overrides read back and restore cleanly, `ServerCommand` runs match controls, and `KickClient` disconnects bots; verify all 9 exports in [`imomoe_admin.pplugin`](../imomoe_admin.pplugin) and their permission/immunity boundaries from a standalone consumer plugin.
2. **Storage, Pagination & Expiry**: Verify offline ban persistence across restart and unban on both SQLite (`data/imomoe_admin.db`) and file drivers (`data/bans.json` / `data/comms.json`); verify 8-entry pagination and automatic expiry leaving SQLite history intact.
3. **Hot Reload & ConVar Allowlist**: Verify invalid JSONC or incomplete/empty translations fail `imomoe_reload` / `!iareload` without replacing the active config, and succeed once fixed; verify `sv_cheats` and `Cheat`-flagged ConVars are rejected even when listed in `cvar_allowlist`.
4. **Target Selectors & Permissions**: In chat, prefer the `!admin` number menu or an unquoted `#userid:N`; verify both select the intended player. Quoted and angle-bracketed nicknames in chat are outside Phase 1 acceptance. In the console, still verify that double quotes around a nickname containing spaces select the intended player. Chat `!slap` still accepts an unquoted full nickname when the argument boundary is unambiguous; if a trailing number could be part of the name or the damage value, verify it asks for `#userid:N`. Verify that ambiguous substrings and high-risk group selectors (`@bots`, `@all`) are refused; verify offline immunity lookups preserve stored names/`lastvisit_at`, and temporary wildcards override permanent denies until expiry.
5. **Rescue, Match Control & ServerFixes Synergy**: Verify `imomoe_slap`, `imomoe_tp`, `imomoe_slay` / `imomoe_respawn`, server-side team switches (`imomoe_team` / `imomoe_swap`), and match commands (`imomoe_restart` / `imomoe_pause` / `imomoe_unpause`); verify `sf_noclip_reset 1` preserves Admin-owned `noclip` while resetting cheat `noclip`, `freeze` / `noclip` reclaim to `Walk` on death/respawn, map change, disconnect, and unload without leaking onto new pawns, and `say @` / `say_team @` / `!` / `/` commands never leak into public chat.

---

### Final Multi-Client Acceptance Checklist

With `imomoe_admin` in `Running` state, Steam-logged server, and two authorized Steam clients connected, record observed chat text, audio direction, and punishment state on each client:

- [ ] **Permissions & Targets**: Lower immunity cannot target higher immunity; the reverse succeeds; temporary permissions expire cleanly. Verify that the `!admin` number menu and `#userid:N` in chat both select the intended player. Check self-targeting, slot/userid collision, userid before and after reconnect, and ambiguous name prefixes; verify group selectors skip protected players and high-risk commands reject group selection.
- [ ] **Kick, Ban & Reconnect**: Verify native CS2 kick/ban disconnect prompts, post-Steam-auth ban rejection, and reconnect after unban (inspect reason/expiry via `imomoe_banlist`).
- [ ] **Private Chat & Hide**: Switch one client to non-admin; verify `say @...`, `say_team @...`, `!report`, `!a` routing and non-leakage to public chat, report cooldown and `data/reports.log`, plus `!hide` effects on `!admins` and public attribution.
- [ ] **Comms & Bidirectional Audio**: Test mute, gag, `silence`, expiry, unmute/ungag, and `!comms` self-query with live voice checks and third-party voice-override restoration.
- [ ] **Client Interaction & API**: Verify that the admin command moves the target to the requested team without requiring the player to select a team; verify `!bring`, `!goto`, announcements, rules, localization, and real-account API enforcement/notifications from a consumer plugin.
- [ ] **Server Cleanup**: Revoke temporary rights/punishments, exit cleanly, restore temporary files, and keep logs free of Steam credentials or RCON passwords.
