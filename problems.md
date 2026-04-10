# TommyTweaker — Full Code Audit
> Generated: 2026-04-10 | Source: `converted-repo.txt`

***

## 🔴 CRITICAL — Logic Bugs (broken behavior)

### BUG-01 · `debloatdisableprinterservices` — operations identical to revert
`operations` and `revertoperations` are byte-for-byte identical: both run
`ServiceSetMode(Spooler, Auto)` + `sc start Spooler`.  
**Applying this tweak starts and re-enables the Spooler instead of stopping it.**  
`check: TweakCheckServiceDisabled(Spooler)` will never return `true`.

```rust
// Current (WRONG) — both blocks are the same
revert:  ServiceSetMode("Spooler", "Auto") + Command("sc", ["start", "Spooler"])
ops:     ServiceSetMode("Spooler", "Auto") + Command("sc", ["start", "Spooler"])

// Fix
ops:     ServiceDisable("Spooler") + Command("sc", ["stop", "Spooler"])
```

***

### BUG-02 · `debloatdisablebluetoothservices` — operations = revert
Both `operations` and `revertoperations` set `BTAGService` and `bthserv` to **Manual**.  
Applying the tweak and reverting it produce the exact same result.  
`check: TweakCheckMultiServiceDisabled` will never return `true` (Manual ≠ Disabled).

```rust
// Fix
ops:  ServiceSetMode("BTAGService", "Disabled") + ServiceSetMode("bthserv", "Disabled")
```

***

### BUG-03 · `debloatdisablemiscservices` — check/ops semantic mismatch
`check: TweakCheckMultiServiceDisabled(["WMPNetworkSvc", "MapsBroker", ...])` checks
for `Disabled` startup type, but `operations` sets all services to **Manual**, not Disabled.  
The toggle will always appear as "not applied" even after the tweak runs.

```rust
// Fix: either change ops to ServiceDisable(...) for each service,
// or change check to TweakCheckMultiServiceMode(..., "Manual")
```

***

### BUG-04 · `debloatdisablemisctasks` — check references wrong task name
`check: Some(TweakCheckScheduledTaskDisabled { name: "." })` — the task name
being checked is a bare `"."` (dot), not any of the 8 tasks listed in `operations`.  
The check will always fail or return an error, so the toggle never reflects reality.

```rust
// Fix: use a representative task that is always present, e.g.
check: TweakCheckScheduledTaskDisabled { name: "MapsToastTask" }
```

***

### BUG-05 · `debloatedgeautostart` — check is None, revert is incomplete
`check: None` means the toggle button never shows as active/applied in the UI.  
`operations` deletes `MicrosoftEdgeAutoLaunch` via `reg delete`, but `revertoperations`
only deletes `PreventFirstRunPage` — the auto-launch key is never restored.

***

### BUG-06 · `gamingdisablemmcss` — check still uses PowerShell
The `operations` block correctly uses native `ServiceDisable("MMCSS")`, but  
`check` still spawns PowerShell to call `Get-Service MMCSS`.  
Should be `TweakCheckServiceDisabled { name: "MMCSS" }`.

***

## 🟠 FRONTEND — Tweaks loaded in Rust but invisible in the UI

### CPU Dashboard (`CpuDashboard.svelte`)
Active section filters: `priority|fth|svchost|pagecombining|sleepstudy` /
`power|performance|usb|pcie|coreparking|idle|acpi|throttle` /
`ntfs|memory|cache|paging|prefetch` / `timer|tsc|tick|clock|boot|processorcheckinterval`

| Tweak ID | Missing keyword | Impact |
|---|---|---|
| `cpudisablevbs` | `"vbs"` not in any filter | **High** — advertised as 5-15 FPS boost |
| `cpudisablehpet` | `"hpet"` not in timer filter | **High** — major latency tweak |
| `cpuintelppmmode` | `"intel"` not in any filter | Medium |
| `cpuinteldisabletsx` | `"intel"` not in any filter | Medium |
| `cpuamdppmmode` | `"amd"` not in any filter | Medium |

**Fix:** add `'vbs' | 'hpet'` to the timer/scheduling filter keywords, and add an
`"intel" | "amd"` vendor sub-section card to CpuDashboard.

***

### GPU Dashboard (`GpuDashboard.svelte`)
Active section filters: `hags|preemption|mpo|priority` / `msi`

All tweaks from `gpu/vendor.rs` are invisible because none contain the above keywords:

| Tweak ID | Missing keyword |
|---|---|
| `gpunvidiadynamicpstate` | `"nvidia"` |
| `gpunvidiacleancache` | `"nvidia"` |
| `gpudisablenvidiatelemetry` | `"nvidia"` |
| `gpuincreasetdrdelay` | `"tdr"` |
| `gpudisableoverlaytestmode` | no keyword match |
| all AMD vendor GPU tweaks | `"amd"` |

**Fix:** add a third card/section "Vendor Tweaks" in GpuDashboard using
`.id.includes('nvidia') || .id.includes('amd') || .id.includes('tdr')`.

***

### Display Dashboard (`DisplayDashboard.svelte`)
Uses a **hardcoded ID whitelist** (not keyword-based):
```ts
monitorTweaks = [...].filter(t => ['displaymaxrefreshrate','displaydpi100','display8bitcolor'].includes(t.id))
gpuTweaks     = [...].filter(t => ['displayenablevrr','displaynogpuscaling'].includes(t.id))
systemTweaks  = [...].filter(t => ['displaytimerresolution'].includes(t.id))
```
Any new tweak added to `display/gpu.rs` or `display/monitor.rs` is silently dropped
unless its exact ID is manually added to this whitelist.

**Fix:** switch to keyword-based filtering or at minimum document the whitelist requirement.

***

### Debloat Dashboard (`DebloatDashboard.svelte`)
Three sections: Apps (explicit ID list), Features (`printerfeatures|ie|mediaplayer|wordpad`),
Edge (`startsWith('debloatedge')`).

| Tweak ID | Why invisible |
|---|---|
| `debloatdisablemiscservices` | not in any section filter |
| `debloatdisableedgeservices` | starts with `debloatdisable`, not `debloatedge` |
| `debloatdisableprinterservices` | not in any section filter |
| `debloatdisablebluetoothservices` | not in any section filter |
| `debloatdisablemisctasks` | not in any section filter |

**Fix:** add a "Services & Tasks" section card in DebloatDashboard.

***

### UI / Interface Dashboard (`UIDashboard.svelte`)
`sectionsMeta` contains only 3 sections: **Context Menu**, **File Explorer**, **Appearance**.  
There is no **Taskbar** section. All tweaks from `interface/taskbar.rs` are registered
via `get_interface_tweaks()` in `lib.rs` but are never reachable from any UI section.

**Fix:** add a `{ id: 'taskbar', title: 'Taskbar', tweaks: taskbarTweaks }` entry to
`sectionsMeta` and a matching filter `t.id.includes('taskbar')`.

***

## 🟡 DUPLICATES — Same function, different IDs

| Pair | Problem |
|---|---|
| `netdisablenetbios` + `secdisablenetbios` | Both appear in SecurityDashboard `hardeningTweaks`; user sees two identical-looking tweaks |
| `netdisablellmnr` + `secdisablellmnr` | Same situation in hardeningTweaks |
| `privdisablewer` + `secdisablewer` (if exists) | Both disable WER; one in Privacy, one in Security |
| `cpudisablevbs` (CpuPerformance) + `secdisablevbs` (SecurityPrivacy) | VBS disabled in two separate tweaks with different registry paths; applying both may conflict or partially revert each other |
| `cpuultimateperformance` + `cpuatlaspowerscheme` | Both activate an Ultimate Performance power plan variant; `cpuatlaspowerscheme` uses a custom GUID `11111111-...` while `cpuultimateperformance` uses the canonical `e9a42b02-...`; applying both causes the active scheme to toggle unpredictably |

***

## ⚪ USELESS / QUESTIONABLE TWEAKS

### `cpuintelppmmode` / `cpuamdppmmode`
Sets the PPM driver `Start` registry value to `3` (Automatic).  
`3` is already the Windows default. `check` will always return `true` on a stock system,
making the tweak appear permanently applied without doing anything.

### `cpunvidiacleancache`
Deletes the NVIDIA shader/driver cache directory.  
The cache is fully regenerated on the next GPU workload. This is a one-shot maintenance
action, not a persistent optimization. Modelling it as a `TweakTypeToggle` is incorrect;
it should be `TweakTypeAction` with no `check` or `revert`.

### `cpuatlaspowerscheme`
Functionally redundant with `cpuultimateperformance`. Using a custom fixed GUID
`11111111-...` means it also bypasses the natural `powercfg` duplicate mechanism.
One of the two power plan tweaks should be removed.

***

## 🔵 REMAINING POWERSHELL (known, confirmed)

| File | PS usage | Status |
|---|---|---|
| `debloat/apps.rs` | 28+ tweaks (AppX via `Get-AppxPackage`) | Intentionally kept — needs windows-rs upgrade to remove |
| `activation/mod.rs` | 6 tweaks (MAS download + execution) | Intentionally kept — external tool dependency |
| `display/gpu.rs` | 4 tweaks (VRR, GPU Scaling, HDR, NVIDIA Low Latency) | Kept — complex string manipulation on registry binary values |
| `gaming/mmcss.rs` | 1 check only (`TweakCheckPowershell`) | Easy fix → `TweakCheckServiceDisabled` (see BUG-06) |
| `cpu/vendor.rs` | `is_intel_cpu()` + `is_amd_cpu()` detection at startup | Low risk (read-only, runs once at init) — can be replaced with `winreg` read |

***

## ✅ CONFIRMED WORKING (no issues found)

- All `TweakOperationCommand` (bcdedit, powercfg, pnputil, sc, schtasks, dism) — executor handles non-zero exit gracefully
- `TweakOperationRegistrySet / Delete` — backup-before-write implemented in `apply_tweak`
- `TweakOperationServiceDisable` + `TweakOperationServiceSetMode` — correct `sc stop` + `sc config` sequence
- `TweakOperationScheduledTaskDisable / Enable` — correct `schtasks /Change /TN ... /Disable|Enable`
- `TweakOperationSvcHostSplitAll` — custom Rust implementation in `commands.rs`, Xbox exclusion confirmed
- `TweakOperationMsiSet / MsiRemove` — custom registry writer in `commands.rs`
- `TweakOperationNetAdapterProperty` — native NIC property writer
- `kill_tweak_process` — correct `taskkill /F /PID /T` via `ProcessManager` PID registry
- Revert engine — backup-first restore implemented, skips already-restored keys, falls through to explicit `revert_operations`
- Activation dashboard — separate `ActivationDashboard.svelte` with correct `applytweak` (no undo) for `TweakTypeAction`
- State persistence — `applied_tweaks` HashSet saved to `state.json` on every apply/undo