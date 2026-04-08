# TommyTweaker — Bug Fix Checklist

## STATUS KEY
- `[ ]` = Not started
- `[~]` = Partial fix (see commit notes)
- `[x]` = Fully resolved — `cargo check` passed, committed

***

## ⚠️ Architectural Issue — Root Cause of ~7 bugs

- [ ] **ARCH-00** — Replace all `std::process::Command::output()` inside `async fn` with `tokio::task::spawn_blocking` or `tokio::process::Command`; prefer native Win32 API (windows-rs) for registry, service, and file operations

***

## 🔴 Severity 4 — Broken Functionality

- [ ] **BUG-01** — `registry/backup.rs`: `RegistryBackup::new()` ignores path, `save()` never called in `apply_tweak` → all registry reverts permanently broken
- [ ] **BUG-02** — `commands.rs`: `undo_tweak` is synchronous (not async), blocks Tokio runtime during every revert
- [ ] **BUG-03** — `commands.rs`: `FileOperation` variant is a silent no-op but marks tweak as applied in UI
- [ ] **BUG-04** — `storage/commands.rs`: `compress_folder` / `decompress_folder` run blocking Win32 FFI loop in `async fn` without `spawn_blocking`
- [ ] **BUG-05** — `debloat/apps.rs`: global `$ErrorActionPreference = "SilentlyContinue"` swallows removal errors, reports failures as `"✓ Removed"`
- [ ] **BUG-06** — `network/adapter.rs`: WMI query `$_.NetConnectionID -eq $_.Name` always false → `$pnpDeviceId` always `$null` → `PnPCapabilities` key never written
- [ ] **BUG-07** — `security/defender.rs`: Disable real-time protection silently does nothing when Tamper Protection is active (Windows default), no user warning
- [ ] **BUG-08** — `privacy/apps.rs`: `priv_nvidia_telemetry` uses `Remove-Item -Recurse -Force` (irreversible), classified `WarningLevel::Safe`

***

## 🟠 Severity 3 — Incorrect Behavior

- [ ] **BUG-09** — `system/restore.rs`: `create_restore_point` uses blocking `Command::output()` in `async fn` (can block for minutes)
- [ ] **BUG-10** — `system/maintenance.rs`: `flush_dns_cache` uses blocking `Command::output()` in `async fn`
- [ ] **BUG-11** — `system/maintenance.rs`: `clear_temp_files` uses blocking `fs::*` I/O in `async fn` without `spawn_blocking`
- [ ] **BUG-12** — `commands.rs`: `scan_startup` calls synchronous `scan_all_startup_items()` directly from `async fn`
- [ ] **BUG-13** — `network/dns_benchmark.rs`: `run_benchmark()` is fully synchronous, called from `async fn` without `spawn_blocking`
- [ ] **BUG-14** — `commands.rs`: `undo_tweak` emits no `tweak-progress` events — no UI feedback during revert
- [ ] **BUG-15** — `commands.rs`: `undo_tweak` performs double revert — `backup_restore` + `revert_operations` both run, hardcoded value wins over original
- [ ] **BUG-16** — `commands.rs`: `check_category` `spawn_blocking` not `.await`-ed — fire-and-forget, always returns `Ok(())` immediately
- [ ] **BUG-17** — `commands.rs`: `RegistryValue::MultiString` check always returns `false` (hardcoded stub)
- [ ] **BUG-18** — `commands.rs`: `ScheduledTaskDisable` discards `path` field — no `-TaskPath` in generated PowerShell, ambiguous for nested tasks
- [ ] **BUG-19** — `security/uac.rs`: `sec_uac_lower` check verifies `ConsentPromptBehaviorAdmin = 5` which is the Windows stock default → always shows as ON
- [ ] **BUG-20** — `security/services.rs`: `sec_disable_remote_services` disables 5 services but check only verifies `RemoteRegistry`
- [ ] **BUG-21** — `security/updates.rs`: `sec_disable_windows_update` disables 7 services but check only verifies `wuauserv`
- [ ] **BUG-22** — `system/monitoring.rs`: `SystemMonitor` spawns infinite GPU polling thread with no cancellation channel → orphaned threads/processes
- [ ] **BUG-23** — `activation/mod.rs`: `activation_remove_office` uses PowerShell `return` (exit code 0) instead of `exit 1` on missing file → Rust reads success
- [ ] **BUG-24** — `input/mouse.rs`: `input_disable_mousetrails` apply and revert both set `MouseTrails = "0"` — toggle is permanently stuck ON
- [ ] **BUG-25** — `system/monitoring.rs`: `state.lock().unwrap()` in `spawn_blocking` → Mutex poisoned if `sysinfo` panics, all dashboard calls fail permanently

***

## 🟡 Severity 2 — Minor Issues

- [ ] **BUG-26** — Multiple modules (`exploit.rs` ×4, `authentication.rs` ×5, `hardening.rs` ×3, `policies.rs` ×4, `timer.rs` ×1): `revert_operations` use `RegistryDelete` instead of restoring original value
- [ ] **BUG-27** — `security/`: `sec_disable_remote_assistance` revert depends on backup system broken by BUG-01
- [ ] **BUG-28** — `network/adapter.rs`: `net_disable_checksum_offload` check wildcard `*ChecksumOffload*` broader than apply (`*IPv4` only) → false positives
- [ ] **BUG-29** — `network/adapter.rs`: `net_configure_rss` revert hardcodes `NUMAStatic` profile regardless of user's original value
- [ ] **BUG-30** — `network/tcp.rs`: `net_tcp_autotuning` revert always sets `normal`, ignores original value
- [ ] **BUG-31** — `network/msi.rs`: `net_msi_nic_high` revert removes MSI values but leaves `MSISupported` key orphaned
- [ ] **BUG-32** — `cpu/memory.rs`: `cpu_large_system_cache` sets `LargeSystemCache = 0` which is already the Windows default — inert tweak
- [ ] **BUG-33** — `cpu/power.rs`: `cpu_ultimate_performance` GUID parsing may capture trailing characters from `powercfg` output
- [ ] **BUG-34** — `activation/mod.rs`: `activation_hwid` and `activation_ohook` share the same cache path `aio.cmd` — corrupt download from one poisons the other
- [ ] **BUG-35** — `activation/mod.rs`: MAS script downloaded with no SHA256 integrity check
- [ ] **BUG-36** — `privacy/telemetry.rs`: `priv_disable_telemetry` revert hardcodes `AllowTelemetry = 3` regardless of user's original setting
- [ ] **BUG-37** — `gaming/mod.rs`: `gaming_disable_gamebar` only disables HKCU overlay flags, Xbox background services (`bcastdvr.exe`, `GameBarFTServer.exe`) remain active
- [ ] **BUG-38** — `startup/mod.rs`: `toggle_item("EXP:...")` returns hardcoded `Err("not supported yet")` — Explorer items shown in UI but un-toggleable