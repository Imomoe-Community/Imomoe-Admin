# Contributing Guide / 贡献指南

感谢对 **Imomoe Admin** 项目的关注与贡献！在提交代码或 Pull Request 之前，请阅读以下开发与协同规范。

Thank you for contributing to **Imomoe Admin**! Please review these development and collaboration guidelines before submitting code or a Pull Request.

---

## 1. 开发环境与工具链 / Environment & Toolchain

本项目基于 Rust 2024 Edition，依赖 [Plugify](https://github.com/untrustedmodders/plugify)、[S2SDK](https://github.com/untrustedmodders/plugify-plugin-s2sdk) 与 `permissions`。

Built with Rust 2024 Edition against [Plugify](https://github.com/untrustedmodders/plugify), [S2SDK](https://github.com/untrustedmodders/plugify-plugin-s2sdk), and `permissions`:

- **Rust 工具链 / Toolchain**：CI 固定使用 `1.100.0`（见 [.github/workflows/ci.yml](.github/workflows/ci.yml)），需包含 `rustfmt` 与 `clippy`：
  ```bash
  rustup toolchain install 1.100.0 --profile minimal --component rustfmt --component clippy
  ```
- **PowerShell**：`7.5+`（用于运行配置 Schema 校验与绑定生成脚本 / required for schema validation and binding generation）。
- **操作系统支持 / Supported OS**：Windows x86_64 MSVC、Linux x86_64 GNU（发布环境建议基于 Steam Runtime Sniper）。

---

## 2. 质量门禁与本地验证 / Quality Gates

所有提交必须通过以下五项检查 / Every submission must pass all five gates:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
pwsh -File ./scripts/validate-config.ps1
cargo build --release --workspace --locked
```

---

## 3. 核心设计与编码规范 / Architecture & Safety Contracts

1. **FFI 边界安全 / FFI Boundary Safety**：
   - 暴露给引擎或 Plugify 的外部 C ABI 入口（`start`、`end`、命令与事件回调及 9 个导出方法）必须使用 `boundary` / `catch_unwind` 拦截 panic，所有引擎调用只能在游戏主线程执行。
   - External C ABI entry points must catch Rust panics with `boundary` / `catch_unwind`, and engine calls must run exclusively on the game thread.
2. **权限、免疫与生命周期 / Permissions, Immunity & Lifecycle**：
   - 权限与免疫判定直接复用 `permissions`，不在本插件内另存用户或分组数据；模糊昵称命中多名在线玩家时必须拒绝执行。
   - Reuse `permissions` without duplicating user/group storage; reject ambiguous nickname matches.
   - `freeze` 与 `noclip` 绑定当前 pawn handle 与连接生命周期，在死亡、断线、换图与卸载时安全回收并注销命令与监听器。
   - Bind `freeze` and `noclip` to the current pawn handle and connection, reclaiming state and hooks on death, disconnect, map end, and unload.
3. **配置与绑定维护 / Config & Bindings**：
   - 配置与双语翻译须符合 `configs/imomoe_admin/schemas/`，热重载仅在全部文件校验通过后替换状态。
   - Configs and translations must conform to `configs/imomoe_admin/schemas/`; reloads swap state atomically only when all files validate.
   - 使用 `./scripts/generate-bindings.ps1 -Generator <plugify-gen> -S2SdkManifest <s2sdk.pplugin> -PermissionsManifest <permissions.pplugin>` 依据服务端清单更新绑定。

---

## 4. 提交与 PR 流程 / Pull Request Workflow

1. 基于 `main` 分支创建特性分支（如 `feat/my-feature` 或 `fix/my-bug`），保持提交聚焦单一主题。
   Branch from `main` and keep commits focused on a single concern.
2. 切勿提交私有配置（`*.local.jsonc` / `*.private.jsonc`）、数据库/日志文件、真实 SteamID 或 RCON 密码。
   Never commit private overrides, `.db`/`.log` state, real SteamIDs, or RCON passwords.
3. 本地五项门禁全部通过后发起 Pull Request，并说明改动动机、影响范围与验证结果。
   Open a PR after all five local checks pass, summarizing motivation, scope, and verification.
