# TommyTweaker — Project Rules & Guidelines

## Technology Stack
- **Backend:** Rust 1.78+ · Tauri 2 · Tokio · winreg · windows-rs
- **Frontend:** Svelte · TypeScript · Vite · HTML/CSS
- **Shell:** PowerShell 7+ (`pwsh`) on Windows 10/11

## Frontend & Backend Connection (CRITICAL)
- Agents are fully authorized and expected to inspect/modify the `src/` folder (Svelte, JS, TS, CSS, HTML), `vite.config.ts`, and `tauri.conf.json`.
- A known critical bug causes an empty `index.html` and failure to connect the frontend to the backend. To fix frontend-backend connection issues, always verify:
  1. `tauri.conf.json`: Check `build.frontendDist` or `build.distDir` matches Vite's output folder.
  2. `tauri.conf.json`: Check `build.beforeBuildCommand` (e.g., `npm run build`).
  3. `vite.config.ts`: Verify the build outDir and clearScreen settings.
  4. Ensure `index.html` properly loads the Svelte main entry point.

## Rust Architecture Rules (Enforced — No Exceptions)
- All `#[command]` Tauri handlers are `async fn`. Blocking I/O MUST use `tokio::task::spawn_blocking`.
- Never use `.unwrap()`. Use `?` or `match` with `eprintln!`.
- Prefer Win32 native API over PowerShell spawns. Convert PowerShell to Native Windows API wherever possible. Use web search to find best practices.
- Error propagation: use `?`. In `spawn_blocking` closures: `map_err(|e| e.to_string())`.

## Memory & Filesystem MCP
- Query memory before every fix session. Persist architectural decisions.
- Never read entire files. Use `search_files` first, then `read_file` with `start_line`/`end_line` (±30 lines).

## Git Rules
- One commit per fix: `fix(module): description [FIX-X]`
- Stage with `git add -p`. Update BUGFIX_CHECKLIST.md in the same commit.

## Agent Team & Task Routing
| Agent | Model | Role |
|-------|-------|------|
| orchestrator | google/antigravity-gemini-3.1-pro | Plans and coordinates — never writes code |
| researcher | google/antigravity-gemini-3.1-pro | Reads codebase, maps call chains, writes STATUS.md |
| coder | google/antigravity-claude-sonnet-4-6 | Applies all Rust/JSON/Svelte/JS fixes |
| reviewer | big-pickle | cargo check + clippy + grep + build verification |
| bash-agent | minimax-m2.5-free | git, grep, mechanical file ops |
| escalation-expert | google/antigravity-claude-opus-4-6-thinking | Blocked fixes only — surgical |

| Task | Agent |
|------|-------|
| Read file, map call chain, query memory | researcher |
| Write Rust / edit JSON | coder |
| cargo check / clippy / grep | reviewer |
| git add, git commit, file listing | bash-agent |
| Same fix failed twice | escalation-expert |
| Plan, delegate, sequence | orchestrator |