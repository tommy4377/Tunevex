# TommyTweaker

TommyTweaker is a powerful, modern system optimization utility for Windows 10/11, built with a high-performance **Rust & Tauri 2** backend and a sleek, responsive **SvelteKit** frontend.

It provides a wide array of system-level tweaks spanning:
- **Network & Adapters**: Optimize RSS profiles, power management, Jumbo frames, flow control, LSO, and DNS settings.
- **Security & Privacy**: Apply defensive hardening and reduce optional telemetry without disabling core Windows protections.
- **Performance**: Tune supported CPU, power, storage, gaming, and startup settings.
- **System Monitoring**: Live tracking of CPU and GPU stats.

## Features

- **Rollback-aware toggles**: Registry values are captured before a live toggle changes them, and every toggle has an explicit state check and undo path.
- **Honest one-shot actions**: App removals and maintenance operations are labelled as actions when an automatic rollback cannot be guaranteed.
- **Portable tweak profiles**: Export enabled tweaks or an editable JSON catalog, validate imported profiles against the installed catalog, and review every operation before applying it.
- **AI Advisor**: Optional Gemini 3.5 Flash integration stores its key in Windows Credential Manager, validates model output against the live catalog, and surfaces explicit Apply/Undo cards instead of silently changing the PC.
- **Fast Execution**: Uses native Windows APIs (`winreg`, `sc.exe`, `schtasks.exe`) wherever possible to eliminate subprocess overhead.
- **Fully Asynchronous**: Does not block the UI thread during heavy operations like compression or process scans.

## Tech Stack

- **Backend**: Rust (1.78+) with Tauri v2, Tokio, and `winreg`.
- **Frontend**: Svelte 5 + TypeScript + Vite, using single-page application (SPA) mode (`adapter-static`).

## Running the Application

Ensure you have Rust, Node.js, and the Tauri CLI installed.

The desktop executable requests Administrator access because it changes machine-level Windows settings. Create a System Restore point before applying a group of tweaks, and review every `Careful` or `Dangerous` item individually. Advanced power-user controls remain available, are excluded from bulk-safe actions and AI application recommendations, and require typing an ID-scoped acknowledgement such as `APPLY sec_disable_firewall` before the backend will execute them.

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev
# or
npm run tauri dev

# Build the release executable
npm run tauri build
```

Debug builds include a localhost-only Tauri MCP bridge for UI and IPC automation. To run a restricted debug session for UI, AI, and per-user rollback tests, set `TOMMYTWEAKER_SKIP_ELEVATION=1` before `npm run tauri dev`; the bypass is ignored for release builds. Machine-level tweak testing still requires an elevated build.

## Tweak profiles

The **All Tweaks** page can export either the currently enabled stateful tweaks or a complete editable template. Profiles use a versioned JSON format and identify controls only by their stable catalog IDs; executable commands and scripts are never imported from the file.

Toggle entries accept `enable`, `disable`, or `skip`. One-shot actions accept `run` or `skip`. Complete templates deliberately export actions as `skip`, so importing a generated template cannot run every cleanup or removal action by accident. The import preview lets you change operations and selection before anything is applied, and dangerous operations still require the same ID-scoped acknowledgement as the normal UI.

```json
{
  "format": "tommytweaker-profile",
  "version": 1,
  "name": "My profile",
  "generated_at_unix": 0,
  "scope": "custom",
  "tweaks": [
    { "id": "example_toggle", "operation": "enable" },
    { "id": "example_action", "operation": "run" }
  ]
}
```

## Contributing

- Commits follow the format: `feat/fix/docs/refactor(module): description`.
- Rust code must pass `cargo clippy` and `cargo check`.
- Tauri commands strictly avoid panics by mapping errors cleanly via `Result<T, String>` and `tokio::task::spawn_blocking`.
