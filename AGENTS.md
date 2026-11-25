# Imomoe-Admin

Standalone CS2 Plugify server-administration plugin (`imomoe_admin`). Integrates with `plugify-plugin-permissions`, `s2sdk`, `Imomoe-NativeBase-CS2`, and optional `imomoe_chat_prefix`. Keep map voting/rotation in `Imomoe-MapManager` and general voting in `Imomoe-VoteManager`. Read [SPEC](docs/SPEC.md) and [ACCEPTANCE](docs/ACCEPTANCE.md) before changing behavior or ABI contracts.

## Workspace & hygiene

- Standalone clones default to `<repo>/.work/`. When `IMOMOE_WORK_ROOT` is set, place shared Cargo/build caches, task worktrees (`$IMOMOE_WORK_ROOT/worktrees/Imomoe-Admin-<task>`), and disposable session probes under it instead of creating repo-local `.work/` or committing ad-hoc scripts (`smoke*.py`, `drive-*.ps1`, `*_minimal.*`).
- Never commit private server configs, credentials, or personal admin lists.

## Engineering & verification

- Inspect sibling call paths and callers before patching; fix root causes rather than adding symptom workarounds. If a cleaner long-term solution crosses provider or ecosystem boundaries, propose options with a recommendation before working around it.
- Apply one-in, one-out closeout: delete superseded tests, fixtures, and probes replaced by your change in the same diff.
- Verify Rust changes with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `scripts/validate-config.ps1`, and `scripts/package.py` / `scripts/provider_abi.py` when manifests, bindings, or packaging inputs change.
