<div align="center">

# Tunevex

**A native-first Windows tuning and optimization toolkit with reversible controls.**

[![CI](https://github.com/tommy4377/Tunevex/actions/workflows/ci.yml/badge.svg)](https://github.com/tommy4377/Tunevex/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/tommy4377/Tunevex?display_name=tag&sort=semver)](https://github.com/tommy4377/Tunevex/releases/latest)
[![License](https://img.shields.io/github/license/tommy4377/Tunevex)](LICENSE)
![Windows](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0078D4?logo=windows11&logoColor=white)

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri%202-24C8DB?logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte%205-FF3E00?logo=svelte&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

</div>

Tunevex is a Windows 10/11 system tuning utility built around a Rust/Tauri backend and a SvelteKit frontend. It combines a large tweak catalog with live state detection, explicit rollback paths, guarded advanced controls, startup analysis, system tools, portable profiles and an optional Gemini-powered advisor.

## What Tunevex does

- **System tuning** — CPU, GPU, gaming, network, storage, input, display, interface and Windows behavior.
- **State-aware toggles** — Tunevex checks the current system state instead of assuming a tweak is enabled or disabled.
- **Rollback support** — reversible tweaks expose an explicit undo path.
- **Safety levels** — higher-risk actions are separated from normal bulk operations and require deliberate acknowledgement.
- **Startup analysis** — inspect common autostart locations while protecting critical Windows entries.
- **Portable profiles** — export, validate, preview and import tweak selections by stable catalog ID.
- **System tools** — restore points, monitoring, DNS and maintenance utilities.
- **AI Advisor** — optional Gemini integration; credentials are stored through Windows Credential Manager and recommendations are validated against the live Tunevex catalog.
- **Native-first backend** — direct Windows APIs and registry/service operations are preferred where practical.

## Download

Get the latest portable executable from [GitHub Releases](https://github.com/tommy4377/Tunevex/releases/latest).

```text
Tunevex.exe
```

Requirements:

- Windows 10 or Windows 11, 64-bit
- Microsoft Edge WebView2 Runtime
- Administrator privileges for machine-level changes

## Safety

Tunevex can change low-level Windows settings. Before applying broad groups of tweaks:

1. Create a System Restore point.
2. Review every **Careful** or **Dangerous** item individually.
3. Treat one-shot removal and maintenance actions as potentially irreversible.
4. Measure performance before and after; a tweak can help one machine and hurt another.

Dangerous controls are excluded from safe bulk application and AI-generated apply lists. They require an explicit ID-scoped acknowledgement.

## Build from source

Prerequisites: Node.js, Rust stable and the Tauri 2 Windows prerequisites.

```bash
npm ci
npm run check
npm run build

cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --locked --manifest-path src-tauri/Cargo.toml
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo build --release --locked --manifest-path src-tauri/Cargo.toml
```

Portable executable:

```text
src-tauri/target/release/tunevex.exe
```

For development:

```bash
npm run dev
cargo run --manifest-path src-tauri/Cargo.toml
```

## Profiles and compatibility

New exports use the versioned `tunevex-profile` format. Legacy `tommytweaker-profile` files and selected historical local-state identifiers remain recognized so existing users are not stranded by the rename from TommyTweaker. Since v1.0.1, existing TommyTweaker application data, backups and logs are automatically migrated into the canonical Tunevex data directory on first launch.

Imported profiles contain stable catalog IDs rather than executable commands or scripts. Tunevex validates and previews them before applying changes.

## Repository health

- Windows CI validates frontend and Rust changes.
- Tags matching `v*.*.*` publish a single portable `Tunevex.exe`.
- Dependabot tracks npm, Cargo and GitHub Actions dependencies.
- Community and security guidance lives in [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), [SUPPORT.md](SUPPORT.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Maintainer

Maintained by [@tommy4377](https://github.com/tommy4377).

## License

Released under the [MIT License](LICENSE).
