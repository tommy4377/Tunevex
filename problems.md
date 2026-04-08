Ora ho il quadro definitivo: **38 bug + 1 problema architetturale principale**, tutti confermati dal codice. Il `Remove-Item` in `display/gpu.rs` era un falso positivo (è `Remove-ItemProperty`, reversibile). Ecco il documento completo in inglese con il prompt per l'agente AI. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/attachments/108516402/d0ffd704-074b-406b-9e38-d94821196e80/TommyTweaker.md)

***

# 🐛 TOMMYTWEAKER — COMPLETE BUG REPORT (English)

## ⚠️ ARCHITECTURAL ISSUE #0 — Synchronous PowerShell Throughout the Entire Codebase

**Root cause of ~7 bugs below.** The codebase uses `std::process::Command::new("powershell").output()` (blocking call) in **20+ places**, including inside `async fn` Tauri command handlers. Tauri runs handlers on a Tokio async executor — blocking a thread freezes the entire UI and all other IPC commands for the duration of the PowerShell script (seconds to minutes). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/attachments/108516402/d0ffd704-074b-406b-9e38-d94821196e80/TommyTweaker.md)

**Fix strategy:**
- Registry, file, service operations → replace with **native Win32 API** via the `windows` crate (already used for `SHEmptyRecycleBinW`, `DeviceIoControl`, `DwmSetWindowAttribute`)
- Unavoidable PowerShell scripts → wrap in `tokio::task::spawn_blocking` (as done in `apply_tweak`) or use `tokio::process::Command` (async-native)

***

## 🔴 SEVERITY 4 — Broken Functionality

**[BUG-01] `registry/backup.rs` — Registry backup is NEVER saved to disk**
`RegistryBackup::new(_path)` ignores the path and returns an empty default. In `apply_tweak`, the struct is filled in RAM then silently dropped without calling `save()`. In `undo_tweak`, `load()` reads a file that was never written → returns empty HashMap. All registry revert operations are permanently broken.

**[BUG-02] `commands.rs` — `undo_tweak` is synchronous, blocks the Tokio runtime**
Declared as `pub fn` (not async), runs blocking PowerShell inside the Tauri command handler thread. `apply_tweak` correctly uses `spawn_blocking`; `undo_tweak` does not. The entire UI freezes during any undo operation.

**[BUG-03] `commands.rs` — `FileOperation` variant is a silent no-op with false state**
The match arm only calls `eprintln!` (invisible to user) and does not return `Err`. The tweak is still marked as "applied" and shows ON in the UI.

**[BUG-04] `storage/commands.rs` — `compress_folder`/`decompress_folder` block the async runtime**
Both are `async fn` but run a synchronous `walkdir` loop + blocking Win32 `DeviceIoControl` FFI on thousands of files without `spawn_blocking`. Can freeze the Tokio worker for tens of minutes on large directories.

**[BUG-05] `debloat/apps.rs` — App removal script always reports success**
Script has global `$ErrorActionPreference = "SilentlyContinue"` but uses `-ErrorAction Stop` inside try/catch. On protected system apps, the error is swallowed silently, `$removed++` is incremented, and the UI shows `"✓ Removed"` for apps never actually uninstalled.

**[BUG-06] `network/adapter.rs` — WMI lookup always returns `$null`**
Both apply and revert use `Where-Object { $_.NetConnectionID -eq $_.Name }` — comparing two different properties on the same WMI object (connection name vs instance name), always false. `$pnpDeviceId` is always `$null`. The `PnPCapabilities` registry key (preventing adapter power-down) is never written. The tweak's main advertised function silently does nothing.

**[BUG-07] `security/defender.rs` — `sec_disable_realtime` is silently ineffective when Tamper Protection is enabled**
Registry policies under `SOFTWARE\Policies\Microsoft\Windows Defender` are completely ignored by Windows 10/11 when Tamper Protection is active (the factory default). The check emits `"TamperProtectionEnabled"` but Rust only expects `"True"/"False"` → no warning to the user. The tweak appears applied but Defender is still fully active.

**[BUG-08] `privacy/apps.rs` — `priv_nvidia_telemetry` permanently deletes files, classified `WarningLevel::Safe`**
Apply script runs `Remove-Item -Recurse -Force` on NVIDIA ProgramData directories. The revert comment explicitly states `"files cannot be restored"`. `warning_level: WarningLevel::Safe`.

***

## 🟠 SEVERITY 3 — Incorrect Behavior

**[BUG-09] `system/restore.rs` — `create_restore_point` blocks for minutes**
`Command::new("powershell").output()` synchronous inside `async fn`. `Checkpoint-Computer` can take 1–5 minutes. Freezes the entire Tauri backend without `spawn_blocking`.

**[BUG-10] `system/maintenance.rs` — `flush_dns_cache` blocks the async runtime**
`pub async fn flush_dns_cache()` calls `Command::new("ipconfig").args(&["/flushdns"]).output()` synchronously without `spawn_blocking`.

**[BUG-11] `system/maintenance.rs` — `clear_temp_files` uses blocking I/O in `async fn`**
`fs::read_dir`, `fs::remove_dir_all`, `fs::remove_file` — all blocking — executed inside `async fn` without `spawn_blocking`. Can block the worker for several seconds on large temp folders.

**[BUG-12] `commands.rs` — `scan_startup` calls synchronous code from `async fn`**
`pub async fn scan_startup()` directly calls `scan_all_startup_items()` which scans dozens of registry keys, Task Scheduler, and services synchronously. No `spawn_blocking`.

**[BUG-13] `network/dns_benchmark.rs` — `run_benchmark()` is synchronous inside `async fn`**
`run_benchmark()` performs 3 DNS queries × 8+ providers using a blocking `Resolver`. Called directly from `async fn benchmark_dns()` without `spawn_blocking`.

**[BUG-14] `commands.rs` — `undo_tweak` emits no progress events to the UI**
`apply_tweak` emits `tweak-progress { status: "applying" }` and `{ status: "success" }`. `undo_tweak` emits nothing. No visual feedback during revert — no spinner, no terminal output, no confirmation.

**[BUG-15] `commands.rs` — `undo_tweak` performs double revert**
First calls `backup_mgr.restore_tweak_backup()` (currently always empty due to BUG-01), then executes explicit `revert_operations`. Once BUG-01 is fixed, the backup restoration will write the original value, then `revert_operations` will overwrite it with a hardcoded default — silently discarding the user's original value.

**[BUG-16] `commands.rs` — `check_category` spawn_blocking is not awaited (fire-and-forget)**
`tokio::task::spawn_blocking(move || { ... });` — no `.await`. The function always returns `Ok(())` before any checks complete. Internal failures are silently lost.

**[BUG-17] `commands.rs` — `RegistryValue::MultiString` check always returns `false`**
The match arm is: `RegistryValue::MultiString(_) => false`. Any tweak using a MultiString check is always shown as OFF.

**[BUG-18] `commands.rs` — `ScheduledTaskDisable` silently discards the `path` field**
`TweakOperation::ScheduledTaskDisable { path: _path, name }` — the path is prefixed with underscore, intentionally ignored. `Disable-ScheduledTask -TaskName 'name'` without `-TaskPath` is ambiguous for tasks in subfolders (e.g., `\Microsoft\Windows\...`).

**[BUG-19] `security/uac.rs` — `sec_uac_lower` check is always ON on a stock Windows install**
Apply sets `ConsentPromptBehaviorAdmin = 5` AND `PromptOnSecureDesktop = 0`. The check only verifies `ConsentPromptBehaviorAdmin = 5` — which is **already the Windows default**. The tweak appears as ON on every fresh installation before being applied.

**[BUG-20] `security/services.rs` — `sec_disable_remote_services` check is incomplete**
Disables 5 services but the check only verifies `RemoteRegistry`. False positives if other services were already disabled for different reasons.

**[BUG-21] `security/updates.rs` — `sec_disable_windows_update` check is incomplete**
Disables 7 services but the check only verifies `wuauserv`. Same class of bug as BUG-20.

**[BUG-22] `system/monitoring.rs` — `SystemMonitor` spawns an orphaned GPU polling thread**
`SystemMonitor::new()` creates a `thread::spawn` with an infinite `loop {}` that runs `Get-Counter` PowerShell every 2 seconds. No `Arc<AtomicBool>` cancellation channel exists. Thread and PowerShell zombie processes accumulate if the state is ever re-initialized.

**[BUG-23] `activation/mod.rs` — `activation_remove_office` uses PowerShell `return` instead of `exit 1`**
```powershell
if (!(Test-Path $path)) { return }  # exit code = 0 !
```
Rust sees exit code 0 → success → tweak marked "applied", no error shown to user.

**[BUG-24] `input/mouse.rs` — `input_disable_mousetrails` apply and revert are identical**
Both `operations` and `revert_operations` set `MouseTrails = "0"`. The Windows default is already `0`. The check will always show the tweak as ON; the revert restores nothing.

**[BUG-25] `system/monitoring.rs` — `SystemMonitor` Mutex can become permanently poisoned**
`state.lock().unwrap()` inside `spawn_blocking`. If `sysinfo` panics (e.g., denied WMI permissions), the mutex is poisoned and all future monitoring API calls fail permanently until app restart.

***

## 🟡 SEVERITY 2 — Minor Issues

**[BUG-26] Multiple security modules — `RegistryDelete` in revert instead of original value**
`security/exploit.rs` (4×), `security/authentication.rs` (5×), `security/hardening.rs` (3×), `privacy/policies.rs` (4×), `cpu/timer.rs` (1×) use `RegistryDelete` in `revert_operations`. If the key existed before with a non-zero value, the revert deletes it entirely instead of restoring the original.

**[BUG-27] `security/` — `sec_disable_remote_assistance` revert depends on broken backup (BUG-01)**
Registry keys are expected to be restored from the backup system, which never saves to disk.

**[BUG-28] `network/adapter.rs` — `net_disable_checksum_offload` check wildcard is broader than apply**
Check: `*ChecksumOffload*` (catches IPv6, TCP, UDP variants). Apply: only `*IPv4`. False positives when IPv6/UDP variants are independently disabled.

**[BUG-29] `network/adapter.rs` — `net_configure_rss` revert hardcodes wrong default profile**
Revert sets `Set-NetAdapterRss -Profile NUMAStatic`. Default varies per adapter/driver (`ClosestProcessorStatic`, `NUMAScaling`). Original value is never saved.

**[BUG-30] `network/tcp.rs` — `net_tcp_autotuning` revert ignores original value**
Always sets `autotuninglevel=normal` regardless of what the user had before.

**[BUG-31] `network/msi.rs` — `net_msi_nic_high` revert leaves `MSISupported` key orphaned**
Revert runs `Remove-ItemProperty` on MSI values but does not remove the parent key. On some updated drivers the orphaned key causes unexpected interrupt behavior until driver reinstall.

**[BUG-32] `cpu/memory.rs` — `cpu_large_system_cache` is functionally inert**
Sets `LargeSystemCache = 0`. The Windows default (key absent) is already equivalent to `0`. Creates a registry key with no behavioral change.

**[BUG-33] `cpu/power.rs` — `cpu_ultimate_performance` GUID parsing may include trailing characters**
`($guid -split ' ')[-1]` — the last token from `powercfg -duplicatescheme` output may include trailing asterisk or whitespace. Falls back silently to the hardcoded GUID; duplicated scheme creation fails without error.

**[BUG-34] `activation/mod.rs` — `activation_hwid` and `activation_ohook` share the same cache path**
Both download to `$env:LOCALAPPDATA\aio.cmd`. A corrupted or partial download from one activation will be reused by the other without re-downloading.

**[BUG-35] `activation/mod.rs` — MAS script downloaded with no integrity check**
No SHA256 or similar verification of the downloaded file. A modified cached file is executed silently.

**[BUG-36] `privacy/telemetry.rs` — `priv_disable_telemetry` revert hardcodes `AllowTelemetry = 3`**
Always restores to level `3` (Full/Optional), regardless of the user's original setting (e.g., `1` = Basic on managed/enterprise systems).

**[BUG-37] `gaming/mod.rs` — `gaming_disable_gamebar` does not stop background services**
Only disables HKCU overlay flags. `bcastdvr.exe` and `GameBarFTServer.exe` remain running. Missing `ServiceDisable` operations for related Xbox services.

**[BUG-38] `startup/mod.rs` — `toggle_item("EXP:...")` has a hardcoded unsupported error**
Explorer startup items are scanned and displayed in the UI, but every toggle attempt returns `Err("Toggling Explorer items not supported yet")`.

***

***
