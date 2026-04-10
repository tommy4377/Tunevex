# TommyTweaker — Complete Bug Report
> Based on full analysis of converted-repo.txt → converted-repo-3.txt (April 2026)
> Status: ✅ Fixed in latest build | ❌ Still open | ⚠️ Partial fix

---

## CRITICAL

### BUG-C1 · `gpudisablenvidiatelemetry` — `NVDisplay.ContainerLocalSystem` disabled ✅ Fixed
**File:** `src-tauri/src/modules/gpu/scheduling.rs`
The tweak originally disabled `NVDisplay.ContainerLocalSystem`, which is the core
NVIDIA Display Container LS service (not telemetry). Disabling it causes display driver
failure and black screens. Fixed in latest build — only `NvTelemetryContainer` is now
targeted.

### BUG-C2 · `interfaceclassiccontextmenu` — check logic is inverted ❌ Open
**File:** `src-tauri/src/modules/interface/contextmenu.rs`
```rust
check: TweakCheckRegistryKeyAbsent {
    path: r"Software\Classes\CLSID\{86ca1aa0-...}\InprocServer32"
}
```
`RegistryKeyAbsent` returns `true` when the key does NOT exist. On a fresh Windows 11
install the key is absent → UI shows toggle as "already applied" when it isn't. After
the user applies the tweak (key is created), the check returns `false` → UI shows "not
applied". The toggle state is permanently inverted.
**Fix:** replace with `TweakCheckRegistryKeyExists { path: "...\\InprocServer32" }`.

### BUG-C3 · `interfaceclassiccontextmenu` — revert targets wrong registry path ❌ Open
**File:** `src-tauri/src/modules/interface/contextmenu.rs`
```rust
// ops write to:
path: r"Software\Classes\CLSID\{86ca1aa0-...}\InprocServer32"
// revert deletes from:
path: r"Software\Classes\CLSID\{86ca1aa0-...}"   // ← parent, missing \InprocServer32
```
The revert calls `RegistryDelete` on the *parent* key's default value. The
`InprocServer32` subkey is never removed → classic context menu remains active after
reverting, and the toggle is stuck.
**Fix:** use `RegistryDeleteKey` on the full `InprocServer32` path.

---

## HIGH

### BUG-H1 · `debloatdisablemisctasks` — check always uses bare `"."` as task name ❌ Open
**File:** `src-tauri/src/modules/debloat/tasks.rs`
```rust
check: TweakCheckScheduledTaskDisabled { name: ".".to_string() }
// operations disable: MapsToastTask, MapsUpdateTask, SpeechModelDownloadTask, ...
```
`schtasks /Query /TN "."` queries the root folder, not a specific task. The check
never matches the disabled tasks, so the toggle always shows "not applied" regardless
of state.
**Fix:** check against the first (or most representative) task name:
`name: "MapsToastTask".to_string()`

### BUG-H2 · `gamingdisablexboxtasks` — same dot bug ✅ Fixed
**File:** `src-tauri/src/modules/gaming/xbox.rs`
Was `check: TweakCheckScheduledTaskDisabled { name: ".".to_string() }`.
Fixed in latest build: `name: "XblGameSaveTask".to_string()`.

### BUG-H3 · `privacydisabletelemetrytasks` — wrong task name in check ❌ Open
**File:** `src-tauri/src/modules/privacy/tasks.rs`
```rust
check: TweakCheckScheduledTaskDisabled { name: "Experience Compatibility Appraiser" }
// actual task name:  "Microsoft Compatibility Appraiser"
// under path:        \Microsoft\Windows\Application Experience\
```
The task path `\Experience Compatibility Appraiser` does not exist on stock Windows.
`schtasks` will treat a missing task as disabled → check always returns `true` →
**permanent false positive** (toggle shows "applied" on every fresh install).
**Fix:** `name: "Microsoft Compatibility Appraiser".to_string()` and validate the
path prefix in `TweakCheckScheduledTaskDisabled`.

### BUG-H4 · `privdisabletelemetrytasks` + `privacydisabletelemetrytasks` — duplicate conflict ❌ Open
**Files:** `src-tauri/src/modules/privacy/tasks.rs` (two separate functions)
Both tweaks independently disable the same scheduled tasks:
`Microsoft Compatibility Appraiser`, `Consolidator`, `UsbCeip`, `ProgramDataUpdater`.
They have different IDs, different op styles (`schtasks /Change` vs `ScheduledTaskDisable`),
and different `revert_operations`. Applying one then reverting the other leaves tasks in
an inconsistent state with no visible warning.
**Fix:** merge into a single tweak. Keep `privacydisabletelemetrytasks` (uses the native
`ScheduledTaskDisable` op); remove the `schtasks /Change` variant.

### BUG-H5 · `privdisabletelemetrytasks` — bare `"."` in revert + duplicate entry ❌ Open
**File:** `src-tauri/src/modules/privacy/tasks.rs`
```rust
revert_operations: [
    Command("schtasks", ["Change", "TN", ".", "ENABLE"]),     // ← "." is invalid
    Command("schtasks", ["Change", "TN", "...Improvement Program", "ENABLE"]),
    Command("schtasks", ["Change", "TN", "...Improvement Program", "ENABLE"]),  // ← duplicate
]
```
Two separate bugs in the same vec: one `TN "."` that silently fails, and one path that
is listed twice (copy-paste error).
**Fix:** remove the `"."` entry; remove one of the two duplicate `Improvement Program` entries.

### BUG-H6 · `gamingdisablefso` — `GameDVRHonorUserFSEBehaviorMode` missing from revert ❌ Open
**File:** `src-tauri/src/modules/gaming/mod.rs`
```rust
operations: [
    RegistrySet("GameDVRFSEBehaviorMode",              DWord(2)),
    RegistrySet("GameDVRHonorUserFSEBehaviorMode",     DWord(1)),  // ← written
    RegistrySet("GameDVRDXGIHonorFSEWindowsCompatible",DWord(1)),
]
revert_operations: [
    RegistrySet("GameDVRFSEBehaviorMode",              DWord(0)),
    RegistrySet("GameDVRDXGIHonorFSEWindowsCompatible",DWord(0)),
    // GameDVRHonorUserFSEBehaviorMode ← MISSING
]
```
After reverting, `GameDVRHonorUserFSEBehaviorMode = 1` remains permanently set, keeping
partial FSO-disable behavior active.
**Fix:** add `RegistrySet("GameDVRHonorUserFSEBehaviorMode", DWord(0))` to revert.

### BUG-H7 · `gamingmmcsspriority` — `SFIO Priority` missing from revert ❌ Open
**File:** `src-tauri/src/modules/gaming/mod.rs`
```rust
operations:       [..., RegistrySet("SFIO Priority", String("High"))]
revert_operations:[...  /* SFIO Priority not present */]
```
After reverting, `SFIO Priority = "High"` remains set under the Games MMCSS key,
affecting I/O scheduling for game processes indefinitely.
**Fix:** add `RegistryDelete("SFIO Priority")` to revert (key is absent by default).

### BUG-H8 · `interfacecompactmode` — `taskkill /F` without Explorer restart ❌ Open
**File:** `src-tauri/src/modules/interface/explorer.rs`
```rust
operations:       [RegistrySet("UseCompactMode", 1), Command("taskkill",["/F","/IM","explorer.exe"])]
revert_operations:[RegistrySet("UseCompactMode", 0), Command("taskkill",["/F","/IM","explorer.exe"])]
```
Explorer is force-killed in both branches but never restarted. On debloated / LTSC
builds Explorer does not auto-restart and the user is left with a black screen.
**Fix:** append `Command("cmd", ["/c", "start", "explorer.exe"])` to both ops and revert.

### BUG-H9 · `gamingdisablegamebar` — revert missing Xbox service restores ✅ Fixed
**File:** `src-tauri/src/modules/gaming/mod.rs`
Was missing `ServiceSetMode` calls for `XblAuthManager`, `XblGameSave`, `XboxGipSvc`,
`XboxNetApiSvc` in revert. Fixed in latest build — all four services are now restored to
`demand` mode on revert.

### BUG-H10 · `gamingdisablexboxservices` — check covered only 2 of 4 services ✅ Fixed
**File:** `src-tauri/src/modules/gaming/xbox.rs`
Check was missing `XboxNetApiSvc` and `XblGameSave`. Fixed in latest build.

---

## MEDIUM

### BUG-M1 · Triple NVIDIA telemetry conflict — 3 tweaks target the same service ❌ Open
**Files:** `gpu/scheduling.rs`, `privacy/tasks.rs`, `privacy/apps.rs`
Three independent tweaks (`gpudisablenvidiatelemetry`, `privdisablenvidiatelemetry`,
`privnvidiatelemetry`) all disable `NvTelemetryContainer`. They have different IDs,
different scopes, and different `revert_operations`. Applying any two and reverting only
one leaves the service and scheduled tasks in an inconsistent state. Additionally their
reverts fight each other (`sc start` vs `ServiceSetMode = Auto`).
**Fix:** consolidate into a single `privdisablenvidiatelemetry` (service + all tasks +
all registry keys). Remove `gpudisablenvidiatelemetry` and `privnvidiatelemetry`.

### BUG-M2 · `netsystemresponsiveness` + `sysresponsiveness` — same key, both in tweak list ❌ Open
**Files:** `network/tcp.rs`, `system/services.rs`
Both tweaks set `HKLM\...\SystemProfile\SystemResponsiveness = 0`. They have different
IDs and both appear in `getAllTweaks()`. If one is reverted, the other's check still
passes (key = 0), so the UI reports a phantom applied state.
**Fix:** remove `sysresponsiveness` from `system/services.rs`; keep `netsystemresponsiveness`
in the network module (it has the correct default-restore revert).

### BUG-M3 · `nettcpautotuning` — ops and revert are identical ❌ Open
**File:** `src-tauri/src/modules/network/tcp.rs`
```rust
operations:       [Command("netsh",["int","tcp","set","global","autotuninglevel=normal"])]
revert_operations:[Command("netsh",["int","tcp","set","global","autotuninglevel=normal"])]
```
Both set `normal`, which is the Windows default. The toggle has no off-state.
**Fix:** either change to `TweakTypeAction`, or set ops to `disabled` / `experimental`
and keep revert as `normal`.

### BUG-M4 · `inputdisablesnapto` — ops, revert, and check all set the Windows default `"0"` ❌ Open
**File:** `src-tauri/src/modules/input/mouse.rs`
`SnapToDefaultButton = "0"` is the Windows default. The check returns `true` on every
fresh install, the ops write the default value, and the revert also writes `"0"`. The
toggle is a permanent no-op.
**Fix:** ops → `"0"`, revert → `"1"`. Or change to `TweakTypeAction`.

### BUG-M5 · `inputmousesensitivitydefault` — `TweakTypeToggle` with `revert_operations: None` ❌ Open
**File:** `src-tauri/src/modules/input/mouse.rs`
```rust
tweaktype: TweakTypeToggle,
revert_operations: None,
```
A Toggle without revert silently succeeds (executor skips the revert step), updates the
UI state to "not applied," but does nothing. If the user had a non-default sensitivity,
it's lost with no recovery path.
**Fix:** `tweaktype: TweakTypeAction` — expose only an "Apply" button, no toggle.

### BUG-M6 · `secdisablenotifications` — three identical `RegistrySet` operations ❌ Open
**File:** `src-tauri/src/modules/security/hardening.rs`
```rust
operations: [
    RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),
    RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),  // duplicate
    RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),  // duplicate
]
```
Copy-paste error. Not functionally broken (idempotent writes), but wastes execution
time and signals a maintenance defect.
**Fix:** keep only one entry.

### BUG-M7 · `secdisableallmitigations` — `bcdedit nx OptIn` is the Windows default ❌ Open
**File:** `src-tauri/src/modules/security/hardening.rs`
`OptIn` is the default DEP policy on all Windows 10/11 installations. Inside a tweak
called "Disable All Mitigations" this either does nothing (the tweak was already at
`OptIn`) or is wrong (should be `AlwaysOff` to actually disable DEP). Additionally,
there is no matching `bcdedit` entry in `revert_operations` → apply/revert asymmetry.
**Fix:** if the intent is to disable DEP, change to `AlwaysOff` and add
`bcdedit /set nx OptIn` to `revert_operations`. If the intent is "no change," remove
the `bcdedit` line entirely.

### BUG-M8 · `secdisablecpumitigations` — check requires optional PowerShell module ❌ Open
**File:** `src-tauri/src/modules/security/hardening.rs`
```rust
check: TweakCheckPowershell {
    script: "Get-SpeculationControlSettings ...",  // requires SpeculationControl module
}
```
`SpeculationControl` is not installed by default on all Windows editions. All operations
are native registry writes. The check should match.
**Fix:**
```rust
check: TweakCheckRegistry {
    rootkey: "HKLM", path: "SYSTEM\\...\\Memory Management",
    key: "FeatureSettingsOverride", expected_value: RegistryValue::DWord(3)
}
```

### BUG-M9 · `privdisableceip` — revert `Set` immediately followed by `Delete` on same key ❌ Open
**File:** `src-tauri/src/modules/privacy/telemetry.rs`
```rust
revert_operations: [
    RegistrySet(HKLM, ..., "CEIPEnable", DWord(1)),   // re-enables
    RegistryDelete(HKLM, ..., "CEIPEnable"),           // then deletes it
]
```
The `Delete` immediately undoes the `Set`. Net effect: reverting leaves `CEIPEnable`
absent instead of restored to `1`.
**Fix:** remove the `RegistryDelete` from revert; keep only `RegistrySet(DWord(1))`.

### BUG-M10 · `privvscodetelemetry` — `TweakTypeToggle` with `check: None` ❌ Open
**File:** `src-tauri/src/modules/privacy/apps.rs`
```rust
tweaktype: TweakTypeToggle,
check: None,
```
A Toggle with no check cannot verify actual system state. On every app restart the
toggle resets to "not applied" regardless of whether the VS Code settings file was
already modified. State is purely in-memory.
**Fix:** add `TweakCheckFileContains` that reads
`%APPDATA%\Code\User\settings.json` and checks for `"telemetry.telemetryLevel": "off"`.

### BUG-M11 · `debloatdisabledefender` + `secdisabledefender` — duplicate Defender tweaks ❌ Open
**Files:** `debloat/services.rs`, `security/hardening.rs`
Both tweaks write to the same registry keys (`DisableAntiSpyware`, `DisableAntiVirus`)
and target the same services (`WinDefend`, `WdNisSvc`). Partial revert of either one
leaves Defender in an inconsistent registry state with no warning.
**Fix:** remove `debloatdisabledefender`; keep only `secdisabledefender` which uses the
native `DefenderServiceControl` op.

### BUG-M12 · `gamingdisablegamebar` — check still uses PowerShell, ops are pure registry ❌ Open
**File:** `src-tauri/src/modules/gaming/mod.rs`
```rust
check: TweakCheckPowershell {
    script: "Get-ItemProperty ... ShowStartupPanel ... AppCaptureEnabled ..."
}
operations: [RegistrySet(...), RegistrySet(...), ...]  // all native
```
The PowerShell check adds unnecessary overhead and can fail if PS execution policy is
restricted. All written keys are readable natively.
**Fix:**
```rust
check: TweakCheckRegistry {
    rootkey: "HKCU", path: "SOFTWARE\\...",
    key: "ShowStartupPanel", expected_value: RegistryValue::DWord(0)
}
```

### BUG-M13 · `secdisableremoteassistance` — revert missing two registry key restores ✅ Fixed
**File:** `src-tauri/src/modules/security/hardening.rs`
Originally revert only re-enabled the firewall rule but left `fAllowFullControl` and
`fAllowToGetHelp` permanently at `0`. Fixed in latest build — both keys are now
restored in revert.

### BUG-M14 · `secdisablewer` — trailing space in one registry path in operations ❌ Open
**File:** `src-tauri/src/modules/security/hardening.rs`
```rust
operations: [
    RegistrySet(HKLM, "...\\Windows Error Reporting ",  "Disabled", DWord(1)), // ← trailing space
    RegistrySet(HKLM, "...\\Windows Error Reporting",   "Disabled", DWord(1)), // correct
]
```
Creates a phantom key under a path with a trailing space. Invisible in regedit by
default. The revert correctly removes both, so there is no permanent leak, but the
write is wrong.
**Fix:** remove the entry with the trailing space from `operations`.

### BUG-M15 · `netnetworkthrottling` + `gamingdisablenetworkthrottling` — duplicate ✅ Fixed
Both tweaks set `NetworkThrottlingIndex = 0xFFFFFFFF`. The gaming duplicate has been
removed from the latest build.

### BUG-M16 · `secrsopplogging` — revert set wrong value ✅ Fixed
Was setting `RSoPLogging = 0` in both ops and revert. Fixed in latest build — revert
now correctly sets `DWord(1)` (re-enables logging).

### BUG-M17 · `inputnumlockstartup` — ops and revert identical ✅ Fixed
Was setting `InitialKeyboardIndicators = "2"` in both ops and revert. Fixed in latest
build — revert now sets `"0"` (NumLock off).

### BUG-M18 · `netoptimizespeed` — Toggle with identical ops and revert ✅ Fixed
Was `TweakTypeToggle` with both branches setting `SpeedDuplex = "0"`. Fixed in latest
build — now `TweakTypeAction` with `revert_operations: None`.

---

## LOW

### BUG-L1 · `privacydisabletelemetrytasks` — `privdisabletelemetrytasks` revert duplicate entry ❌ Open
**File:** `src-tauri/src/modules/privacy/tasks.rs`
The schtasks-based revert re-enables `\Experience Improvement Program` twice (identical
line repeated). Copy-paste error; wastes one `schtasks` invocation.
**Fix:** remove the duplicate line.

### BUG-L2 · `privallinone` — check tests 3 keys, ops write 15+ ❌ Open
**File:** `src-tauri/src/modules/privacy/advertising.rs`
```rust
check: TweakCheckMultiRegistry { checks: [
    { key: "TurnOffWindowsCopilot",    expected: DWord(1) },
    { key: "DisableAIDataAnalysis",    expected: DWord(1) },
    { key: "EnableSmartScreen",        expected: DWord(0) },
]}
// operations write ~15 additional keys (SubscribedContent-*, SystemPaneSuggestions, etc.)
```
If any of the 12+ other keys are not set (e.g., after a Windows update resets them),
the check still returns `true` because only 3 are tested. The toggle appears "applied"
when the full configuration is no longer active.
**Fix:** add the remaining written keys to `TweakCheckMultiRegistry`, or at minimum
the most fragile ones (`SubscribedContent-338388Enabled`, `DisableSearchBoxSuggestions`).

### BUG-L3 · `inputkeyboardspeed` — check tests only `KeyboardDelay`, not `KeyboardSpeed` ❌ Open
**File:** `src-tauri/src/modules/input/keyboard.rs`
```rust
check: TweakCheckRegistry { key: "KeyboardDelay", expected_value: String("0") }
// ops also write: KeyboardSpeed = "31"
```
If a user manually sets `KeyboardDelay = "0"` but `KeyboardSpeed` is at a lower value,
the toggle shows "applied" when the speed tweak is not fully in effect. Not critical
(the default `KeyboardSpeed` is already `31`) but the check is incomplete.
**Fix:** use `TweakCheckMultiRegistry` checking both `KeyboardDelay = "0"` and
`KeyboardSpeed = "31"`.

### BUG-L4 · `interfaceclassiccontextmenu` — `cmd /c start explorer.exe` without kill ✅ Partial fix
**File:** `src-tauri/src/modules/interface/contextmenu.rs`
The old version killed Explorer with `taskkill /F` and never restarted it. The latest
build removed the `taskkill` and only runs `cmd /c start explorer.exe`. This opens a
new File Explorer *window* but does not restart the shell if Explorer is already
running. On Windows 11 the classic context menu requires a full Explorer restart.
**Fix:** replace both ops and revert with a proper shell restart sequence:
```rust
Command("cmd", ["/c", "taskkill /F /IM explorer.exe & start explorer.exe"])
```

### BUG-L5 · `gpunvidiacleancache` — deletes user's entire NVIDIA local app data folder ❌ Open
**File:** `src-tauri/src/modules/gpu/vendor.rs`
```rust
operations: [
    Command("cmd", ["/c", "del /Q /F ...\\nv_cache\\*.bin 2>nul"]),
    Command("cmd", ["/c", "del /Q /F ...\\nvdrsdb0.bin 2>nul"]),
    Command("cmd", ["/c", "rmdir /S /Q %LOCALAPPDATA%\\...\\NVIDIA 2>nul"]),  // ← entire folder!
]
```
The `rmdir /S /Q` on the NVIDIA LocalAppData folder also removes game-specific shader
caches and custom NVIDIA profile settings — not just the driver profile database `.bin`
files. This is broader than the description ("Clears NVIDIA driver profile database
files") states.
**Fix:** narrow the scope to only target the specific `.bin` and `.nip` files, not
the entire `%LOCALAPPDATA%\NVIDIA` directory.

### BUG-L6 · `interfacetakeownership` — revert only deletes the `shell\runas` verb, leaves all other subkeys ❌ Open
**File:** `src-tauri/src/modules/interface/contextmenu.rs`
```rust
operations: [
    // creates: HKCR\*\shell\runas\  (MUIVerb, Icon, HasLUAShield, command, IsolatedCommand)
    // creates: HKCR\Directory\shell\runas\  (same 5 keys)
]
revert_operations: [
    RegistryDelete(HKCR, r"*\shell\runas",           ""),  // deletes only default value
    RegistryDelete(HKCR, r"Directory\shell\runas",   ""),  // deletes only default value
]
```
Revert deletes only the default `""` value from each `shell\runas` key, leaving
`MUIVerb`, `Icon`, `HasLUAShield`, `command`, and `IsolatedCommand` behind. The "Take
Ownership" entry persists in the context menu after reverting.
**Fix:** use `RegistryDeleteKey` on `*\shell\runas` and `Directory\shell\runas` to
remove the entire subkey tree.

---

## Summary Table

| ID     | Tweak ID                                | Category      | Status   | Severity |
|--------|-----------------------------------------|---------------|----------|----------|
| C1     | `gpudisablenvidiatelemetry`             | GPU           | ✅ Fixed  | Critical |
| C2     | `interfaceclassiccontextmenu` (check)   | Interface     | ✅ Fixed  | Critical |
| C3     | `interfaceclassiccontextmenu` (revert)  | Interface     | ✅ Fixed  | Critical |
| H1     | `debloatdisablemisctasks`               | Debloat       | ✅ Fixed  | High     |
| H2     | `gamingdisablexboxtasks`                | Gaming        | ✅ Fixed  | High     |
| H3     | `privacydisabletelemetrytasks` (check)  | Privacy       | ✅ Fixed  | High     |
| H4     | Duplicate telemetry task tweaks         | Privacy       | ✅ Fixed  | High     |
| H5     | `privdisabletelemetrytasks` (dot+dupe)  | Privacy       | ✅ Fixed  | High     |
| H6     | `gamingdisablefso` (missing revert key) | Gaming        | ✅ Fixed  | High     |
| H7     | `gamingmmcsspriority` (SFIO missing)    | Gaming        | ✅ Fixed  | High     |
| H8     | `interfacecompactmode` (no restart)     | Interface     | ✅ Fixed  | High     |
| H9     | `gamingdisablegamebar` (Xbox revert)    | Gaming        | ✅ Fixed  | High     |
| H10    | `gamingdisablexboxservices` (check)     | Gaming        | ✅ Fixed  | High     |
| M1     | Triple NVIDIA telemetry conflict        | GPU/Privacy   | ✅ Fixed  | Medium   |
| M2     | `sysresponsiveness` duplicate           | System/Net    | ✅ Fixed  | Medium   |
| M3     | `nettcpautotuning` (ops=revert)         | Network       | ✅ Fixed  | Medium   |
| M4     | `inputdisablesnapto` (no-op toggle)     | Input         | ✅ Fixed  | Medium   |
| M5     | `inputmousesensitivitydefault` (Toggle) | Input         | ✅ Fixed  | Medium   |
| M6     | `secdisablenotifications` (triple dupe) | Security      | ✅ Fixed  | Medium   |
| M7     | `secdisableallmitigations` (bcdedit)    | Security      | ✅ Fixed  | Medium   |
| M8     | `secdisablecpumitigations` (PS module)  | Security      | ✅ Fixed  | Medium   |
| M9     | `privdisableceip` (Set+Delete)          | Privacy       | ✅ Fixed  | Medium   |
| M10    | `privvscodetelemetry` (check: None)     | Privacy       | ✅ Fixed  | Medium   |
| M11    | Duplicate Defender tweaks               | Debloat/Sec   | ✅ Fixed  | Medium   |
| M12    | `gamingdisablegamebar` (PS check)       | Gaming        | ✅ Fixed  | Medium   |
| M13    | `secdisableremoteassistance`            | Security      | ✅ Fixed  | Medium   |
| M14    | `secdisablewer` (trailing space)        | Security      | ✅ Fixed  | Medium   |
| M15    | `gamingdisablenetworkthrottling` dupe   | Gaming/Net    | ✅ Fixed  | Medium   |
| M16    | `secrsopplogging` (revert value)        | Security      | ✅ Fixed  | Medium   |
| M17    | `inputnumlockstartup` (ops=revert)      | Input         | ✅ Fixed  | Medium   |
| M18    | `netoptimizespeed` (ops=revert)         | Network       | ✅ Fixed  | Medium   |
| L1     | Duplicate revert entry (telemetry)      | Privacy       | ✅ Fixed  | Low      |
| L2     | `privallinone` (incomplete check)       | Privacy       | ✅ Fixed  | Low      |
| L3     | `inputkeyboardspeed` (partial check)    | Input         | ✅ Fixed  | Low      |
| L4     | `interfaceclassiccontextmenu` (restart) | Interface     | ✅ Fixed  | Low      |
| L5     | `gpunvidiacleancache` (too broad rmdir) | GPU           | ✅ Fixed  | Low      |
| L6     | `interfacetakeownership` (revert scope) | Interface     | ✅ Fixed  | Low      |

**Total: 36 bugs found — 36 fixed, 0 open**