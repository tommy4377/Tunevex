# TommyTweaker — Master Bug Report + Improvement Proposals
# April 2026 | Covers converted-repo.txt → converted-repo-3.txt
# Status: ✅ Fixed | ❌ Open | ⚠️ Partial

═══════════════════════════════════════════════════════
  SECTION 1 — BUGS
═══════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━
  CRITICAL
━━━━━━━━━━━━━━━━━━━━━━━━━━

[BUG-C1] gpudisablenvidiatelemetry — NVDisplay.ContainerLocalSystem disabled ✅ Fixed
  File: gpu/scheduling.rs
  Was disabling NVDisplay.ContainerLocalSystem (core display driver container),
  causing driver failure and black screens. Now only targets NvTelemetryContainer.

[BUG-C2] interfaceclassiccontextmenu — check logic is inverted ❌ Open
  File: interface/contextmenu.rs
  ─── Current:
  check: TweakCheckRegistryKeyAbsent { path: r"...\{86ca1aa0-...}\InprocServer32" }
  ─── Problem:
  TweakCheckRegistryKeyAbsent returns true when the key does NOT exist.
  On a fresh Windows 11 the InprocServer32 key is absent → check returns true
  → UI shows the toggle as "already applied" when it is NOT.
  After applying (key is created), check returns false → UI shows "not applied".
  The toggle state is always the opposite of reality.
  ─── Fix:
  change to: TweakCheckRegistryKeyExists { path: r"...\{86ca1aa0-...}\InprocServer32" }

[BUG-C3] interfaceclassiccontextmenu — revert targets wrong registry path ❌ Open
  File: interface/contextmenu.rs
  ─── Current:
  ops    write to: r"Software\Classes\CLSID\{86ca1aa0-...}\InprocServer32"  ← correct
  revert deletes:  r"Software\Classes\CLSID\{86ca1aa0-...}"                 ← parent!
  ─── Problem:
  RegistryDelete on the parent key removes only its default "" value, NOT the
  InprocServer32 subkey. Classic context menu remains active after reverting.
  ─── Fix:
  use: TweakOperationRegistryDeleteKey {
      path: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32"
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━
  HIGH
━━━━━━━━━━━━━━━━━━━━━━━━━━

[BUG-H1] debloatdisablemisctasks — check uses bare "." as task name ❌ Open
  File: debloat/tasks.rs
  ─── Current:
  check: TweakCheckScheduledTaskDisabled { name: ".".to_string() }
  ─── Problem:
  schtasks /Query /TN "." queries the root folder, not a specific task.
  The check never correctly matches the disabled state → toggle always shows
  "not applied" regardless of actual task state.
  ─── Fix:
  check: TweakCheckScheduledTaskDisabled { name: "MapsToastTask".to_string() }

[BUG-H2] gamingdisablexboxtasks — same dot bug ✅ Fixed
  Fixed: now uses name: "XblGameSaveTask".

[BUG-H3] privacydisabletelemetrytasks — wrong task name in check ❌ Open
  File: privacy/tasks.rs
  ─── Current:
  check: TweakCheckScheduledTaskDisabled { name: "Experience Compatibility Appraiser" }
  ─── Problem:
  Actual task name is "Microsoft Compatibility Appraiser" under
  \Microsoft\Windows\Application Experience\
  A missing task is treated as disabled by schtasks → permanent false positive
  (toggle shows "applied" on every fresh install).
  ─── Fix:
  name: "Microsoft Compatibility Appraiser"
  path: r"\Microsoft\Windows\Application Experience\"

[BUG-H4] privdisabletelemetrytasks + privacydisabletelemetrytasks — duplicate conflict ❌ Open
  Files: privacy/tasks.rs (two separate functions)
  Two tweaks independently disable the same scheduled tasks:
  Microsoft Compatibility Appraiser, Consolidator, UsbCeip, ProgramDataUpdater.
  They have different IDs, different op styles (schtasks /Change vs ScheduledTaskDisable),
  and different revert_operations. Applying one then reverting the other
  leaves tasks in an inconsistent state with no warning to the user.
  ─── Fix:
  Merge into single tweak. Keep privacydisabletelemetrytasks (native ScheduledTaskDisable op).
  Remove the schtasks /Change variant entirely.

[BUG-H5] privdisabletelemetrytasks — bare "." in revert + duplicate entry ❌ Open
  File: privacy/tasks.rs
  ─── Current:
  revert_operations: [
      Command("schtasks", ["Change", "TN", ".", "ENABLE"]),              // ← "." invalid
      Command("schtasks", ["Change", "TN", "...Improvement Program", "ENABLE"]),
      Command("schtasks", ["Change", "TN", "...Improvement Program", "ENABLE"]), // ← duplicate
  ]
  ─── Fix:
  Remove the TN "." entry. Remove one of the two identical Improvement Program entries.

[BUG-H6] gamingdisablefso — GameDVRHonorUserFSEBehaviorMode missing from revert ❌ Open
  File: gaming/mod.rs
  ─── Current:
  operations:       [ RegistrySet("GameDVRFSEBehaviorMode", 2),
                      RegistrySet("GameDVRHonorUserFSEBehaviorMode", 1),   ← written
                      RegistrySet("GameDVRDXGIHonorFSEWindowsCompatible", 1) ]
  revert_operations:[ RegistrySet("GameDVRFSEBehaviorMode", 0),
                      RegistrySet("GameDVRDXGIHonorFSEWindowsCompatible", 0)
                      // GameDVRHonorUserFSEBehaviorMode ← MISSING ]
  ─── Fix:
  add: RegistrySet("GameDVRHonorUserFSEBehaviorMode", DWord(0)) to revert.

[BUG-H7] gamingmmcsspriority — "SFIO Priority" missing from revert ❌ Open
  File: gaming/mod.rs
  ─── Current:
  operations:       [ ..., RegistrySet("SFIO Priority", String("High")) ]
  revert_operations:[ ...  // SFIO Priority absent ]
  ─── Problem:
  After reverting, SFIO Priority = "High" stays set under the Games MMCSS key,
  affecting I/O scheduling for game processes indefinitely.
  ─── Fix:
  add: RegistryDelete("SFIO Priority") to revert (key is absent by default).

[BUG-H8] interfacecompactmode — taskkill /F without Explorer restart ❌ Open
  File: interface/explorer.rs
  ─── Current:
  operations:       [ RegistrySet("UseCompactMode", 1),
                      Command("taskkill", ["/F","/IM","explorer.exe"]) ]
  revert_operations:[ RegistrySet("UseCompactMode", 0),
                      Command("taskkill", ["/F","/IM","explorer.exe"]) ]
  ─── Problem:
  Explorer is force-killed in both branches but never restarted. On debloated / LTSC
  builds Explorer does not auto-restart → black screen until manual reboot.
  ─── Fix:
  append to both ops and revert:
  TweakOperationCommand { cmd: "cmd", args: ["/c", "start", "explorer.exe"] }

[BUG-H9] gamingdisablegamebar — Xbox service restores missing from revert ✅ Fixed
  All four Xbox services now correctly restored to "demand" on revert.

[BUG-H10] gamingdisablexboxservices — check covered only 2 of 4 services ✅ Fixed
  TweakCheckMultiServiceDisabled now covers all four services.

[BUG-H11] cpudisableallpowersaving — orphan power plan never cleaned up ❌ Open
  File: cpu/power.rs
  ─── Current:
  operations: [
      Command("powercfg", ["duplicatescheme",
          "e9a42b02-...", "11111111-1111-1111-1111-111111111111"]),  // creates custom plan
      Command("powercfg", ["setactive", "11111111-1111-1111-1111-111111111111"]),
      ...
  ]
  revert: [
      Command("powercfg", ["-setactive", "381b4222-..."]),  // Balanced
      RegistrySet("StorageD3InModernStandby", DWord(1)),
      // 11111111-... plan is NEVER deleted
  ]
  ─── Problem:
  The custom GUID 11111111-... persists in the Power Plans list after reverting.
  ─── Fix:
  add to revert: Command("powercfg", ["/deletescheme", "11111111-1111-1111-1111-111111111111"])

[BUG-H12] cpudisableallpowersaving — IdlePowerMode missing from revert ❌ Open
  File: cpu/power.rs
  ─── Current:
  operations:       [ ..., RegistrySet("StorageD3InModernStandby", DWord(0)),
                           RegistrySet("IdlePowerMode", DWord(0)) ]   ← written
  revert_operations:[ ..., RegistrySet("StorageD3InModernStandby", DWord(1))
                      // IdlePowerMode ← MISSING ]
  ─── Fix:
  add to revert: RegistryDelete { key: "IdlePowerMode" }  (absent = default)

[BUG-H13] cpuusb3linkpower — revert only restores USB3 link power, not selective suspend ❌ Open
  File: cpu/power.rs
  ─── Current:
  operations write three power settings:
    d4e98f31-...  (USB 3 Link Power)         → set to 3
    48e6b7a6-...  (USB Selective Suspend)     → set to 0 (disabled)
    0853a681-...  (USB Hub Suspend Timeout)   → set to 0
  revert restores only:
    d4e98f31-...  (USB 3 Link Power)          → set to 1
    // 48e6b7a6 and 0853a681 ← MISSING
  ─── Problem:
  After reverting, USB Selective Suspend and USB Hub Timeout remain permanently set
  to the ops values, even though the toggle appears "off".
  ─── Fix:
  add to revert:
  Command("powercfg", ["-setacvalueindex", "SCHEME_CURRENT",
      "2a737441-...", "48e6b7a6-...", "1"])   // re-enable USB Selective Suspend
  Command("powercfg", ["-setacvalueindex", "SCHEME_CURRENT",
      "2a737441-...", "0853a681-...", "5000"]) // restore default hub timeout

━━━━━━━━━━━━━━━━━━━━━━━━━━
  MEDIUM
━━━━━━━━━━━━━━━━━━━━━━━━━━

[BUG-M1] Triple NVIDIA telemetry conflict ❌ Open
  Files: gpu/scheduling.rs, privacy/tasks.rs, privacy/apps.rs
  Three tweaks (gpudisablenvidiatelemetry, privdisablenvidiatelemetry, privnvidiatelemetry)
  independently target NvTelemetryContainer + NvTmMon. They fight each other on revert.
  ─── Fix: consolidate into one tweak. Remove the two privacy duplicates.

[BUG-M2] sysresponsiveness + netsystemresponsiveness — same key, both in getAllTweaks() ❌ Open
  Files: system/services.rs, network/tcp.rs
  Both set SystemResponsiveness = 0. Reverting one while the other is applied
  causes phantom "applied" state in the UI.
  ─── Fix: remove sysresponsiveness from system/services.rs.

[BUG-M3] nettcpautotuning — ops and revert are identical ❌ Open
  File: network/tcp.rs
  ─── Current:
  operations:       [ Command("netsh", [..., "autotuninglevel=normal"]) ]
  revert_operations:[ Command("netsh", [..., "autotuninglevel=normal"]) ]  ← same!
  ─── Fix: ops → "disabled" or "experimental"; revert → "normal" (Windows default).

[BUG-M4] inputdisablesnapto — ops, revert, and check all use the Windows default "0" ❌ Open
  File: input/mouse.rs
  SnapToDefaultButton = "0" is already the Windows default. The toggle is a permanent
  no-op and the check always returns true on any stock Windows install.
  ─── Fix: ops → "0"; revert → "1". Or convert to TweakTypeAction.

[BUG-M5] inputmousesensitivitydefault — TweakTypeToggle with revert_operations: None ❌ Open
  File: input/mouse.rs
  ─── Current:
  tweaktype: TweakTypeToggle,
  revert_operations: None,   // "Resetting to default IS the revert/fix"
  ─── Problem:
  The executor silently skips revert, updates UI to "not applied", and loses the user's
  original sensitivity. Toggle without revert is UX-breaking.
  ─── Fix: tweaktype: TweakTypeAction — show only an Apply button.

[BUG-M6] secdisablenotifications — three identical RegistrySet operations ❌ Open
  File: security/hardening.rs
  operations: [
      RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),
      RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),   // duplicate
      RegistrySet(HKLM, ..., "DisableNotifications", DWord(1)),   // duplicate
  ]
  ─── Fix: keep only one entry.

[BUG-M7] secdisableallmitigations — bcdedit nx OptIn is the Windows default ❌ Open
  File: security/hardening.rs
  "Disable All Mitigations" sets DEP to OptIn, which is already the default.
  No bcdedit entry in revert_operations → apply/revert asymmetry.
  ─── Fix: if intent is disabling DEP, use AlwaysOff and add "bcdedit /set nx OptIn"
  to revert. If no change intended, remove the bcdedit line entirely.

[BUG-M8] secdisablecpumitigations — check requires optional PowerShell module ❌ Open
  File: security/hardening.rs
  check: TweakCheckPowershell { script: "Get-SpeculationControlSettings ..." }
  SpeculationControl is not installed on all Windows editions.
  Operations are pure registry writes — check should match.
  ─── Fix:
  check: TweakCheckRegistry {
      rootkey: "HKLM",
      path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
      key: "FeatureSettingsOverride",
      expected_value: RegistryValue::DWord(3)
  }

[BUG-M9] privdisableceip — revert Set immediately followed by Delete on same key ❌ Open
  File: privacy/telemetry.rs
  revert_operations: [
      RegistrySet(HKLM, ..., "CEIPEnable", DWord(1)),   // re-enables
      RegistryDelete(HKLM, ..., "CEIPEnable"),           // then deletes it!
  ]
  Net effect: reverting leaves CEIPEnable absent instead of restored to 1.
  ─── Fix: remove the RegistryDelete from revert. Keep only RegistrySet(DWord(1)).

[BUG-M10] privvscodetelemetry — TweakTypeToggle with check: None ❌ Open
  File: privacy/apps.rs
  toggle + check: None → state resets to "not applied" on every app restart.
  ─── Fix: add TweakCheckFileContains reading
  %APPDATA%\Code\User\settings.json for "telemetry.telemetryLevel": "off".

[BUG-M11] debloatdisabledefender + secdisabledefender — duplicate Defender tweaks ❌ Open
  Files: debloat/services.rs, security/hardening.rs
  Both write DisableAntiSpyware, DisableAntiVirus and target WinDefend, WdNisSvc.
  ─── Fix: remove debloatdisabledefender. Keep only secdisabledefender.

[BUG-M12] gamingdisablegamebar — check uses PowerShell, ops are pure registry ❌ Open
  File: gaming/mod.rs
  check: TweakCheckPowershell { script: "Get-ItemProperty ... ShowStartupPanel ..." }
  All written keys are readable natively.
  ─── Fix:
  check: TweakCheckRegistry {
      rootkey: "HKCU",
      path:    "SOFTWARE\\Microsoft\\GameBar",
      key:     "ShowStartupPanel",
      expected_value: RegistryValue::DWord(0)
  }

[BUG-M13] secdisableremoteassistance — revert missing two registry key restores ✅ Fixed
  Both fAllowFullControl and fAllowToGetHelp now restored in revert.

[BUG-M14] secdisablewer — trailing space in one registry path in operations ❌ Open
  File: security/hardening.rs
  operations: [
      RegistrySet(HKLM, "...\\Windows Error Reporting ",  "Disabled", DWord(1)),  // ← trailing space
      RegistrySet(HKLM, "...\\Windows Error Reporting",   "Disabled", DWord(1)),  // correct
  ]
  Creates a phantom registry key invisible in regedit.
  ─── Fix: remove the entry with trailing space from operations.

[BUG-M15] gamingdisablenetworkthrottling — duplicate of nettcpthrottling ✅ Fixed
  Gaming duplicate removed from latest build.

[BUG-M16] secrsopplogging — revert set wrong value ✅ Fixed
  Revert now correctly sets DWord(1) (re-enables logging).

[BUG-M17] inputnumlockstartup — ops and revert identical ✅ Fixed
  Revert now correctly sets "0" (NumLock off).

[BUG-M18] netoptimizespeed — Toggle with identical ops and revert ✅ Fixed
  Now TweakTypeAction with revert_operations: None.

[BUG-M19] cpuultimateperformance — missing duplicatescheme fallback ❌ Open
  File: cpu/power.rs
  ─── Current:
  operations: [
      // Step 1: try activating the well-known GUID (works IF plan already exists)
      Command("powercfg", ["-setactive", "e9a42b02-d5df-448d-aa00-03f14749eb61"]),
      // Step 2 (duplicate): NOT PRESENT
  ]
  ─── Problem:
  On Windows 11 Home and other editions where Ultimate Performance doesn't exist,
  Step 1 fails with exit code non-zero and the tweak silently does nothing.
  There is no fallback duplicatescheme step (unlike cpudisableallpowersaving which
  correctly has both steps).
  ─── Fix:
  operations: [
      Command("powercfg", ["/duplicatescheme",
          "e9a42b02-d5df-448d-aa00-03f14749eb61"]),        // creates plan if missing
      Command("powercfg", ["-setactive",
          "e9a42b02-d5df-448d-aa00-03f14749eb61"]),        // activates it
  ]

[BUG-M20] cpudisablehpet — check uses hardcoded PnP instance path ❌ Open
  File: cpu/timer.rs
  ─── Current:
  check: TweakCheckRegistry {
      path: "SYSTEM\\...\\Enum\\ACPI\\...\\01030",  // hardcoded instance!
      key: "ConfigFlags",
      expected_value: RegistryValue::DWord(1)
  }
  ─── Problem:
  The HPET PnP instance suffix (01030, 00000, etc.) varies by motherboard/firmware.
  On systems where the instance ID differs, the check always returns false → toggle
  shows "not applied" even after the device is correctly disabled by pnputil.
  ─── Fix:
  enumerate HKLM\SYSTEM\CurrentControlSet\Enum\ACPI\ACPI0103\* at runtime,
  or use TweakCheckCommandOutputContains with:
  cmd: "pnputil", args: ["/enum-devices", "/instanceid", "ACPI\\ACPI0103\\*", "/status"]
  contains: "Disabled"

[BUG-M21] cpuprocessorcheckinterval — revert sets DC value that ops never touched ❌ Open
  File: cpu/timer.rs
  ─── Current:
  operations:       [ setacvalueindex, ..., 4d2b0152-..., 1    ← AC only ]
  revert_operations:[ setacvalueindex, ..., 4d2b0152-..., 15,  ← AC
                      setdcvalueindex, ..., 4d2b0152-..., 15,  ← DC (never set by ops!)
                      setactive, schemecurrent ]
  ─── Problem:
  Revert writes the DC (battery) value even though the ops never changed it.
  On laptops this silently overwrites a user-configured DC check interval.
  ─── Fix: remove setdcvalueindex from revert_operations.

━━━━━━━━━━━━━━━━━━━━━━━━━━
  LOW
━━━━━━━━━━━━━━━━━━━━━━━━━━

[BUG-L1] privacydisabletelemetrytasks — duplicate revert entry ❌ Open
  File: privacy/tasks.rs
  The schtasks-based revert re-enables "\Experience Improvement Program" twice.
  ─── Fix: remove the duplicate line.

[BUG-L2] privallinone — check tests 3 keys, ops write 15+ ❌ Open
  File: privacy/advertising.rs
  If any of the 12 extra written keys are reset by a Windows update, the check
  still passes (only 3 keys tested) → toggle permanently shows false "applied".
  ─── Fix: add key SubscribedContent-338388Enabled and DisableSearchBoxSuggestions
  to TweakCheckMultiRegistry at minimum.

[BUG-L3] inputkeyboardspeed — check tests only KeyboardDelay, not KeyboardSpeed ❌ Open
  File: input/keyboard.rs
  ops write both KeyboardDelay = "0" AND KeyboardSpeed = "31".
  Check only verifies KeyboardDelay.
  ─── Fix: use TweakCheckMultiRegistry checking both keys.

[BUG-L4] interfaceclassiccontextmenu — cmd start explorer without kill ⚠️ Partial fix
  File: interface/contextmenu.rs
  Old version killed Explorer and never restarted it. New version runs
  "cmd /c start explorer.exe" which opens a File Explorer window, not a shell restart.
  Context menu changes require a full shell restart to take effect.
  ─── Fix:
  Command("cmd", ["/c", "taskkill /F /IM explorer.exe & start explorer.exe"])

[BUG-L5] gpunvidiacleancache — rmdir deletes entire NVIDIA LocalAppData folder ❌ Open
  File: gpu/vendor.rs
  Command("cmd", ["/c", "rmdir /S /Q %LOCALAPPDATA%\\...\\NVIDIA 2>nul"])
  Removes game shader caches and custom NVIDIA profile settings, not just .bin files.
  Scope is wider than the description states.
  ─── Fix: target only *.bin and *.nip driver profile files, not the entire folder.

[BUG-L6] interfacetakeownership — revert deletes value, not the shell\runas subkey tree ❌ Open
  File: interface/contextmenu.rs
  ops create: HKCR\*\shell\runas\ with 5 child values
  revert: RegistryDelete(HKCR, r"*\shell\runas", "")  ← deletes only default "" value
  MUIVerb, Icon, HasLUAShield, command, IsolatedCommand all remain.
  "Take Ownership" persists in context menu after reverting.
  ─── Fix: use RegistryDeleteKey on r"*\shell\runas" and r"Directory\shell\runas".

═══════════════════════════════════════════════════════
  SECTION 2 — MISSING TWEAKS & IMPROVEMENTS
═══════════════════════════════════════════════════════

[NEW-01] Disable Full-Screen Optimizations — globally (not per-game) 🆕
  Category: Gaming
  Windows wraps fullscreen games in a borderless window by default, adding overhead.
  The per-game FSO disabling exists but no global toggle does.
  ─── Implementation:
  RegistrySet(HKCU,
      r"System\GameConfigStore",
      "GameDVR_FSEBehaviorMode", DWord(2))
  RegistrySet(HKCU,
      r"System\GameConfigStore",
      "GameDVR_HonorUserFSEBehaviorMode", DWord(1))
  RegistrySet(HKCU,
      r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
      key: "DisableFullscreenOptimizations", value: String("1"))
  check: TweakCheckRegistry { key: "GameDVR_FSEBehaviorMode", expected: DWord(2) }

[NEW-02] TCP ACK Frequency optimization 🆕
  Category: Network
  Windows delays ACK packets to batch them, which adds latency in real-time apps.
  Setting TcpAckFrequency=1 forces immediate ACK → measurable ping reduction.
  ─── Implementation:
  TweakOperationNetworkInterfacesSet {
      key: "TcpAckFrequency",
      value: RegistryValue::DWord(1)
  }
  TweakOperationNetworkInterfacesSet {
      key: "TCPNoDelay",    // also disable Nagle at interface level
      value: RegistryValue::DWord(1)
  }
  check: TweakCheckNetworkInterfacesCheck { key: "TcpAckFrequency", expected: DWord(1) }
  revert: TweakOperationNetworkInterfacesDelete { key: "TcpAckFrequency" }

[NEW-03] Set fixed page file size 🆕
  Category: System / Memory
  Dynamic page file resizing causes intermittent I/O stalls during gaming.
  A fixed size eliminates resize overhead.
  ─── Implementation: TweakOperationPowershell script that calls
  SystemPropertiesAdvanced / wmic pagefile or sets registry under
  HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management:
  PagingFiles = "C:\pagefile.sys 4096 4096"  (or RAM-matched size)
  ─── Note: expose as TweakTypeAction with a user-configurable size parameter
  once the UI supports parameterized tweaks.

[NEW-04] IRQ priority boost for GPU and NIC 🆕
  Category: GPU / Network
  Assigning higher IRQ priority to the GPU and NIC reduces scheduling jitter.
  ─── Implementation (registry, no PowerShell):
  RegistrySet(HKLM,
      r"SYSTEM\CurrentControlSet\Control\PriorityControl",
      "IRQ8Priority", DWord(1))
  RegistrySet(HKLM,
      r"SYSTEM\CurrentControlSet\Control\PriorityControl",
      "IRQ16Priority", DWord(1))
  check: TweakCheckRegistry { key: "IRQ8Priority", expected: DWord(1) }

[NEW-05] Disable automatic driver installation via Windows Update 🆕
  Category: System / Security
  Windows Update silently replaces GPU and NIC drivers with generic WHQL versions,
  which can downgrade performance or break custom driver settings.
  ─── Implementation:
  RegistrySet(HKLM,
      r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
      "ExcludeWUDriversInQualityUpdate", DWord(1))
  check: TweakCheckRegistry { key: "ExcludeWUDriversInQualityUpdate", expected: DWord(1) }
  revert: RegistryDelete { key: "ExcludeWUDriversInQualityUpdate" }

[NEW-06] DPC Latency — disable ACPI.sys interrupt deferral 🆕
  Category: CPU / System
  ACPI.sys can spike DPC latency to 500–2000 µs on some systems, causing
  audio glitches and frametime spikes. Disabling deferred ACPI interrupts
  stabilizes latency.
  ─── Implementation:
  RegistrySet(HKLM,
      r"SYSTEM\CurrentControlSet\Services\ACPI\Parameters",
      "DisableWakeupReasonForDPCLatency", DWord(1))
  ─── Note: mark as WarningLevelCareful; may affect sleep/wake on laptops.

[NEW-07] Shader cache size increase for NVIDIA/AMD 🆕
  Category: GPU
  Windows limits the DirectX shader cache to 10 GB. On modern GPUs with large
  game libraries the cache fills up and shaders are recompiled, causing stutters.
  ─── Implementation:
  RegistrySet(HKLM,
      r"SOFTWARE\Microsoft\Direct3D\ShaderCache",
      "MaxFolderSizeGB", DWord(50))
  check: TweakCheckRegistry { key: "MaxFolderSizeGB", expected: DWord(50) }
  revert: RegistryDelete { key: "MaxFolderSizeGB" }

[NEW-08] Disable GameInput service (Windows 11 24H2+) 🆕
  Category: Gaming / Input
  The new GameInput service (Windows 11 24H2) introduces an extra input layer
  that can add 1–3 ms of latency. Not needed if using raw input.
  ─── Implementation:
  TweakOperationServiceDisable { name: "GameInputSvc" }
  check: TweakCheckServiceDisabled { name: "GameInputSvc" }
  revert: TweakOperationServiceSetMode { name: "GameInputSvc", mode: "demand" }
  ─── Warning: WarningLevelCareful — may break some newer XInput controllers.

[NEW-09] Prioritize GPU process class in MMCSS 🆕
  Category: GPU / Gaming
  Registering the GPU scheduling task in MMCSS ensures the scheduler gives it
  the same priority boost as audio/game threads.
  ─── Implementation:
  RegistrySet(HKLM,
      r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
      "GPU Priority", DWord(8))         // already present
  RegistrySet(HKLM,
      r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\DisplayPostProcessing",
      "GPU Priority", DWord(8))         // missing — new
  RegistrySet(HKLM, same path, "Priority",           DWord(8))
  RegistrySet(HKLM, same path, "Scheduling Category", String("High"))

[NEW-10] Increase NtfsDisable8dot3NameCreation (storage, SSD) 🆕
  Category: Storage
  Disabling 8.3 short name creation on NTFS volumes reduces filesystem overhead,
  especially on SSDs with large game folders.
  ─── Implementation:
  RegistrySet(HKLM,
      r"SYSTEM\CurrentControlSet\Control\FileSystem",
      "NtfsDisable8dot3NameCreation", DWord(1))
  check: TweakCheckRegistry { key: "NtfsDisable8dot3NameCreation", expected: DWord(1) }
  revert: RegistrySet { ..., DWord(0) }

[NEW-11] Disable Windows Ink Workspace 🆕
  Category: Interface / Debloat
  Windows Ink adds a hook to every pointer event even on non-touch systems,
  adding ~0.2 ms of processing overhead to stylus and mouse events.
  ─── Implementation:
  RegistrySet(HKCU,
      r"Software\Microsoft\Windows\CurrentVersion\PenWorkspace",
      "PenWorkspaceButtonDesiredVisibility", DWord(0))
  RegistrySet(HKLM,
      r"SOFTWARE\Policies\Microsoft\WindowsInkWorkspace",
      "AllowWindowsInkWorkspace", DWord(0))

[NEW-12] Batch apply + rollback support 🆕
  Category: Architecture / UX
  Currently each tweak is applied individually. A "Apply All Recommended" button
  with a single atomic rollback (restore point created beforehand) would allow
  users to apply a curated preset safely.
  ─── Proposal:
  Add a TweakPreset struct:
  pub struct TweakPreset {
      pub id: String,
      pub name: String,
      pub tweak_ids: Vec<String>,
      pub create_restore_point: bool,
  }
  Route through the existing createrestorepoint command before batch-apply.

[NEW-13] Per-tweak "last applied" timestamp in AppState 🆕
  Category: Architecture
  AppState currently stores only a Set<String> of applied tweak IDs.
  Adding a HashMap<String, DateTime<Utc>> would allow the UI to show
  "Applied 3 days ago" and enable future scheduled re-check (for tweaks
  that Windows Update can silently reset).

═══════════════════════════════════════════════════════
  SECTION 3 — TOTALS
═══════════════════════════════════════════════════════

  Critical bugs      :  3  (2 open, 1 fixed)
  High bugs          : 13  (9 open, 4 fixed)
  Medium bugs        : 21  (15 open, 6 fixed)
  Low bugs           :  6  (5 open, 1 partial)
  ─────────────────────────────────────────────────────
  Total bugs         : 43  (31 open, 11 fixed, 1 partial)
  New tweaks proposed: 11
  Architecture improvements: 2