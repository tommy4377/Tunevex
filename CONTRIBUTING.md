# Contributing to Tunevex

Tunevex changes Windows system state, so correctness and rollback safety matter more than adding as many tweaks as possible.

## Before opening a pull request

Run:

```bash
npm ci
npm run check
npm run build
cd src-tauri
cargo fmt --check
cargo check
cargo test
```

For every stateful tweak:

- define how its current state is detected;
- define an explicit apply path;
- define an explicit rollback path;
- document whether a restart is required;
- assign an appropriate risk level;
- avoid duplicate ownership of the same registry value or Windows setting.

One-shot operations that cannot be reliably reversed must remain actions rather than pretend to be toggles.

Do not commit generated build output, logs, credentials, API keys, screenshots used only for debugging, AI-agent memory/prompt files, temporary reports, copied third-party source trees, or local editor/agent state.

Use focused commit messages such as `feat(network): ...`, `fix(tweaks): ...`, `refactor(ui): ...`, `test: ...` and `docs: ...`.
