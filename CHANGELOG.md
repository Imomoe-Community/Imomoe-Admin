# Changelog / 更新日志

All notable changes to this project will be documented in this file.
本项目的所有重要变更均记录在此文件中。

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-11-22

### 简体中文

- **管理命令与目标解析**：实现 35 条控制台与聊天管理命令，复用 `permissions` 权限与免疫体系，支持 SteamID64、slot/userid、唯一昵称与集合目标解析，覆盖踢出、在线/离线封禁、通信处罚、救场与对局控制。
- **执勤与社区工具**：支持管理内聊（`!a` / `say_team @`）、带防刷冷却的举报（`!report` / `say @`）及落盘日志、匿名执勤（`!hide`）、规则、帮助与定时公告（可选联动 `imomoe_chat_prefix`）。
- **配置热重载与持久化**：提供带 JSON Schema 校验的 JSONC 热重载（`imomoe_reload` / `!iareload`）、双语文案（`zh-CN` / `en`）、ConVar 白名单，以及 SQLite 历史记录与原子替换 JSON 文件降级存储。
- **Plugify 跨插件 ABI**：在 [imomoe_admin.pplugin](imomoe_admin.pplugin) 导出 9 个方法，供伴随插件查询状态、判定免疫、执行处罚、通知在线管理及协调 `noclip` 所有权。

### English

- **Moderation Commands & Target Resolver**: Added 35 console and chat moderation commands reusing `permissions` rights and immunity, supporting SteamID64, slot/userid, unique substring names, and group selectors for kicks, online/offline bans, comms restrictions, rescue actions, and match control.
- **Admin Duty & Community Tools**: Added private admin chat (`!a` / `say_team @`), cooldown-protected reports (`!report` / `say @`) with `data/reports.log` persistence, anonymous duty (`!hide`), rules, help, and scheduled announcements (with optional `imomoe_chat_prefix` integration).
- **Schema-Validated Reload & Storage**: Added atomic JSONC configuration hot-reload (`imomoe_reload` / `!iareload`), bilingual translations (`zh-CN` / `en`), ConVar allowlisting, SQLite history storage, and atomic JSON file fallback.
- **Plugify Cross-Plugin ABI**: Exported 9 methods in [imomoe_admin.pplugin](imomoe_admin.pplugin) for companion plugins to query state, check immunity, enforce bans/mutes, notify admins, and coordinate `noclip` ownership.
