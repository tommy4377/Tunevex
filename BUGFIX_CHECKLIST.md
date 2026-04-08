# TommyTweaker — Bugfix Checklist

Legend: [x] = fixed & committed · [~] = partial · [ ] = pending

## Priority 0 — Critical Build/Architecture
- [x] FIX-FRONTEND `tauri.conf.json` / `vite.config.ts` / `index.html` — Fix empty index.html and ensure Svelte frontend correctly connects to Tauri backend during build

## Priority 1 — User-visible bugs
- [x] FIX-A `defender.rs` — TamperProtection not checked before apply operations
- [x] FIX-B `adapter.rs` — net_disable_power_mgmt check/apply use different metrics

## Priority 2 — Architectural issues
- [ ] FIX-C `tommy-tweaker-memory.json` — BUG-05 missing memory entry
- [ ] FIX-D `commands.rs` — ServiceDisable/ServiceSetMode spawn powershell.exe (~300ms overhead)
- [ ] FIX-E `commands.rs` — ScheduledTaskDisable spawns powershell.exe
- [ ] FIX-F `commands.rs` — get_tweaks uses .unwrap() on mutex (panic risk)
- [ ] FIX-G `commands.rs` — kill_tweak_process uses .unwrap() on mutex (panic risk)

## Priority 3 — Minor behavioral issues
- [ ] FIX-H `adapter.rs` — net_configure_rss revert applies same value as apply
- [ ] FIX-I `operations.rs` — RegistryDelete returns error if key absent on revert
- [ ] FIX-J `vendor.rs` — gpu_amd_ulps check uses PowerShell for pure registry read

## Documentation
- [ ] README.md — write from scratch after FIX-J