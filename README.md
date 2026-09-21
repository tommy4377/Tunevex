# Tunevex

Tunevex is a Windows 10/11 system tuning and optimization utility built with a Rust/Tauri 2 backend and a SvelteKit frontend.

It combines a large tweak catalog with explicit state detection, rollback-aware toggles, guarded power-user controls, startup analysis, system monitoring, profile import/export, and an optional Gemini-powered advisor.

## Highlights

- **Tweak catalog** — CPU, GPU, gaming, network, storage, input, privacy, security, startup, display, interface and system controls.
- **Rollback-aware toggles** — stateful tweaks expose a check and an explicit undo path.
- **Guarded advanced controls** — high-risk settings are marked as dangerous and require an ID-scoped acknowledgement.
- **Portable profiles** — export enabled tweaks or an editable template, then validate and preview imports before applying them.
- **Startup analyzer** — inspect common Windows autostart locations and protect critical system entries.
- **AI Advisor** — optional Gemini integration with credentials stored in Windows Credential Manager; model output is validated against Tunevex's live tweak catalog.
- **System tools** — restore points, monitoring, storage compaction, DNS tools and maintenance actions.
- **Native-first backend** — Windows APIs and direct registry/service operations are preferred over shelling out when practical.

## Safety model

Tunevex changes machine-level Windows settings and therefore requests Administrator privileges.

Before applying groups of tweaks:

1. Create a System Restore point.
2. Review every **Careful** or **Dangerous** item individually.
3. Treat one-shot removal/maintenance actions as potentially irreversible.
4. Measure performance before and after; a tweak that helps one machine can hurt another.

Dangerous controls are excluded from bulk-safe operations and AI application recommendations. They require an explicit acknowledgement such as `APPLY sec_disable_firewall`.

## Requirements

- Windows 10 or Windows 11, 64-bit
- Microsoft Edge WebView2 Runtime
- Administrator privileges for machine-level tweaks

## Development

Prerequisites: Node.js, Rust stable and the Tauri prerequisites for Windows.

```bash
npm ci
npm run check
npm run tauri dev
```

To build the portable executable without generating an installer:

```bash
npm run tauri build -- --no-bundle
```

Output:

```text
src-tauri/target/release/tunevex.exe
```

Debug builds can bypass the elevation request for UI-only development with `TUNEVEX_SKIP_ELEVATION=1`. The legacy `TOMMYTWEAKER_SKIP_ELEVATION` variable remains accepted for compatibility. The bypass is ignored by release builds.

## Tweak profiles

New exports use the versioned `tunevex-profile` JSON format. Tunevex also accepts legacy `tommytweaker-profile` files created before the rename.

Profiles reference only stable catalog IDs; executable commands and scripts are never imported from profile files. Imports are validated against the installed catalog and previewed before anything is applied.

## Compatibility with TommyTweaker

Tunevex is the continuation of TommyTweaker. Existing local data directories and selected internal rollback identifiers are intentionally recognized so that renaming the application does not strand state, backups or saved credentials.

New user-facing branding and newly created profile files use the Tunevex name.

## Repository

- CI validates the Svelte frontend and Rust backend on Windows.
- Release tags matching `v*.*.*` build and publish a single portable `Tunevex.exe`.
- Dependabot tracks npm, Cargo and GitHub Actions dependencies.
- See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md) and [SUPPORT.md](SUPPORT.md).

## License

Tunevex is released under the [MIT License](LICENSE).

Copyright © 2026 Tommaso Verardi.
