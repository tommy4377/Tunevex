# TommyTweaker

TommyTweaker is a powerful, modern system optimization utility for Windows 10/11, built with a high-performance **Rust & Tauri 2** backend and a sleek, responsive **SvelteKit** frontend.

It provides a wide array of system-level tweaks spanning:
- **Network & Adapters**: Optimize RSS profiles, power management, Jumbo frames, flow control, LSO, and DNS settings.
- **Security & Privacy**: Manage Windows Defender, real-time protection, telemetry, and exploit protections.
- **Performance**: Tune CPU behavior, memory scheduling, storage caches, and startup items.
- **System Monitoring**: Live tracking of CPU and GPU stats.

## Features

- **Safe Tweaking**: Includes a registry backup manager that automatically saves your configuration before making any changes.
- **Instant Rollback**: Every tweak provides a reliable, transparent `undo` operation to restore factory defaults.
- **Fast Execution**: Uses native Windows APIs (`winreg`, `sc.exe`, `schtasks.exe`) wherever possible to eliminate subprocess overhead.
- **Fully Asynchronous**: Does not block the UI thread during heavy operations like compression or process scans.

## Tech Stack

- **Backend**: Rust (1.78+) with Tauri v2, Tokio, and `winreg`.
- **Frontend**: Svelte 5 + TypeScript + Vite, using single-page application (SPA) mode (`adapter-static`).

## Running the Application

Ensure you have Rust, Node.js, and the Tauri CLI installed.

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

## Contributing

- Commits follow the format: `feat/fix/docs/refactor(module): description`.
- Rust code must pass `cargo clippy` and `cargo check`.
- Tauri commands strictly avoid panics by mapping errors cleanly via `Result<T, String>` and `tokio::task::spawn_blocking`.