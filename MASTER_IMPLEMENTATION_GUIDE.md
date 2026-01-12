# TommyTweaker - MASTER IMPLEMENTATION GUIDE

> **Version**: FINAL (Consolidated from all audits)  
> **Date**: January 12, 2026  
> **Total Items**: 54 implementation tasks  
> **Estimated Duration**: 15-20 days  
> **Status**: DEFINITIVE SOURCE OF TRUTH

---

## 📋 TABLE OF CONTENTS

1. [Quick Reference Summary](#1-quick-reference-summary)
2. [Pre-Implementation Setup](#2-pre-implementation-setup)
3. [Phase A: Critical Fixes (15 items)](#3-phase-a-critical-fixes)
4. [Phase B: Feature Integration (21 items)](#4-phase-b-feature-integration)
5. [Phase C: Polish & Optimization (18 items)](#5-phase-c-polish--optimization)
6. [Testing Checklist](#6-testing-checklist)
7. [File Reference Map](#7-file-reference-map)

---

## 1. QUICK REFERENCE SUMMARY

### Total Task Breakdown

| Phase | Priority | Items | Est. Days | Key Focus |
|-------|----------|-------|-----------|-----------|
| **A** | CRITICAL | 15 | 5-6 | Crashes, inverted values, security |
| **B** | HIGH/MEDIUM | 21 | 8-10 | Features, checks, modules |
| **C** | LOW | 18 | 4-5 | UI polish, optimizations |
| **TOTAL** | | **54** | **17-21** | |

### Critical Value Inversions to Fix

| Current Value | Correct Value | File | Impact |
|--------------|---------------|------|--------|
| HPET = ENABLED | DISABLED | `cpu/timer.rs` | -10-20% FPS |
| USB3 Link Power = 0 | 3 | `cpu/power.rs` | Latency issues |
| MSI Priority = 0 | 2 or 3 | `*/msi.rs` | No benefit |
| Processor Check = 200 | 1 | `cpu/timer.rs` | +DPC latency |
| MMCSS GPU revert = 8 | 2 | `gaming/mod.rs` | Wrong default |

### Duplicates to Remove

| Tweak | Remove From | Keep In |
|-------|-------------|---------|
| NetBIOS disable | `security/hardening.rs` | `network/security.rs` |
| LLMNR disable | `security/hardening.rs` | `network/security.rs` |
| WER disable | `privacy/telemetry.rs` | `security/error_reporting.rs` |
| Defender disable | `debloat/apps.rs` | `security/defender.rs` |
| Spectre tweak | `exploit.rs` (sec_disable_kvas) | Merge with sec_disable_spectre |

---

## 2. PRE-IMPLEMENTATION SETUP

### Step 0.1: Create Development Branch
```bash
git checkout -b feature/implementation-roadmap
```

### Step 0.2: Verify File Structure
Confirm these directories exist:
```
src-tauri/src/modules/
├── cpu/          ├── gaming/       ├── privacy/
├── debloat/      ├── gpu/          ├── security/
├── display/      ├── input/        ├── startup/
├── filesystem/   ├── interface/    ├── storage/
├── network/      ├── packages/     ├── system/
└── ui_classic/
```

### Step 0.3: Backup Critical Files
```bash
# Create backup of files being modified
mkdir -p backups/$(date +%Y%m%d)
cp src-tauri/src/commands.rs backups/$(date +%Y%m%d)/
cp src-tauri/src/modules/cpu/timer.rs backups/$(date +%Y%m%d)/
cp src-tauri/src/modules/cpu/power.rs backups/$(date +%Y%m%d)/
# ... continue for all critical files
```

---

## 3. PHASE A: CRITICAL FIXES

**Duration**: 5-6 days  
**Priority**: P0/P1 - Must complete before any other work

---

### A.1 ⛔ Application Freezes During Tweak Application

**Priority**: P0 - Critical  
**Effort**: 4-6 hours

**Files to Modify**:
1. `src-tauri/src/commands.rs` (lines 64-255)
2. `src/components/TweakCard.svelte`

**Step-by-Step**:

1. Open `src-tauri/src/commands.rs`
2. Find the `apply_tweak` function
3. Add async/spawn wrapper:

```rust
#[tauri::command]
pub async fn apply_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Emit progress start
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id.clone(),
        "status": "applying",
        "progress": 0
    }));
    
    // Run in separate thread to prevent UI blocking
    tokio::spawn(async move {
        // ... existing apply logic ...
    }).await.map_err(|e| e.to_string())?
}
```

4. Open `src/components/TweakCard.svelte`
5. Add loading state:

```svelte
<script>
  let isApplying = false;
  
  async function handleToggle() {
    isApplying = true;
    try {
      await invoke('apply_tweak', { id: tweak.id });
    } finally {
      isApplying = false;
    }
  }
</script>

{#if isApplying}
  <div class="loading-bar animate-pulse" />
{/if}
```

**Verification**: Apply 5+ tweaks in sequence without UI freeze

---

### A.2 ⛔ State.json Location (Should be AppData)

**Priority**: P0 - Critical  
**Effort**: 2-3 hours

**Files to Modify**:
1. `src-tauri/src/modules/utils/dirs.rs`
2. `src-tauri/src/modules/utils/state.rs`

**Step-by-Step**:

1. Open `src-tauri/src/modules/utils/dirs.rs`
2. Add or replace the path function:

```rust
pub fn get_state_path() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "APPDATA environment variable not found")?;
    
    let tommy_dir = PathBuf::from(app_data).join("TommyTweaker");
    
    if !tommy_dir.exists() {
        std::fs::create_dir_all(&tommy_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    Ok(tommy_dir.join("state.json"))
}

pub fn get_backup_dir() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "APPDATA environment variable not found")?;
    
    Ok(PathBuf::from(app_data).join("TommyTweaker").join("backups"))
}
```

3. Update all references in `state.rs` to use `get_state_path()`

**Verification**: Check that `%APPDATA%\TommyTweaker\state.json` exists after running app

---

### A.3 ⛔ Request Administrator Permissions on Startup

**Priority**: P0 - Critical  
**Effort**: 1-2 hours

**Files to Modify**:
1. `src-tauri/tauri.conf.json`
2. Create: `src-tauri/tommytweaker.exe.manifest`

**Step-by-Step**:

1. Open `src-tauri/tauri.conf.json`
2. Add to windows configuration (if Tauri 2.0):
```json
{
  "windows": [
    {
      "title": "TommyTweaker",
      "requireAdmin": true
    }
  ]
}
```

3. Create manifest file `src-tauri/tommytweaker.exe.manifest`:
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
```

4. Update `src-tauri/Cargo.toml` to embed manifest

**Verification**: Running EXE shows UAC prompt

---

### A.4 ⛔ Edge Removal Script (Completely Broken)

**Priority**: P1 - High  
**Effort**: 3-4 hours

**Files to Modify**:
1. `src-tauri/src/modules/debloat/apps.rs` or `debloat/edge.rs`

**Step-by-Step**:

1. Locate the edge removal tweak
2. Replace the script with:

```rust
TweakOperation::Powershell {
    script: r#"
Write-Host "Removing Microsoft Edge..." -ForegroundColor Yellow

# Step 1: Check Windows version
$build = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").CurrentBuildNumber
if ([int]$build -ge 26100) {
    Write-Host "Windows 11 24H2+ detected - using updated removal method" -ForegroundColor Cyan
}

# Step 2: Set NoRemove to 0 (REQUIRED first step)
$uninstallPath = "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft Edge"
if (Test-Path $uninstallPath) {
    Set-ItemProperty -Path $uninstallPath -Name "NoRemove" -Value 0 -Force -EA 0
}

# Step 3: Find Edge version and uninstaller
$EdgePath = "C:\Program Files (x86)\Microsoft\Edge\Application"
if (Test-Path $EdgePath) {
    $EdgeVersion = Get-ChildItem $EdgePath -Directory | 
        Where-Object { $_.Name -match '^\d+\.\d+' } | 
        Sort-Object Name -Descending | 
        Select-Object -First 1 -ExpandProperty Name
    
    $UninstallCmd = "$EdgePath\$EdgeVersion\Installer\setup.exe"
    
    if (Test-Path $UninstallCmd) {
        Write-Host "Uninstalling Edge version $EdgeVersion..." -ForegroundColor Yellow
        Start-Process -FilePath $UninstallCmd -ArgumentList "--uninstall --system-level --verbose-logging --force-uninstall" -Wait -NoNewWindow
        Write-Host "Edge uninstalled successfully" -ForegroundColor Green
    } else {
        Write-Host "Edge installer not found at expected path" -ForegroundColor Red
    }
} else {
    Write-Host "Edge not found in Program Files" -ForegroundColor Yellow
}

# Step 4: Prevent Edge reinstallation
$policyPath = "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate"
if (!(Test-Path $policyPath)) { New-Item -Path $policyPath -Force | Out-Null }
Set-ItemProperty -Path $policyPath -Name "DoNotUpdateToEdgeWithChromium" -Value 1 -Type DWord -Force

# Step 5: Disable Edge Update scheduled tasks
Get-ScheduledTask | Where-Object { $_.TaskName -like "*MicrosoftEdgeUpdate*" } | 
    Disable-ScheduledTask -EA 0 | Out-Null

Write-Host "Edge removal complete!" -ForegroundColor Green
"#.to_string(),
}
```

**Verification**: Edge not present in Start Menu after reboot

---

### A.5 ⛔ HPET Logic is INVERTED (Currently HURTS Performance)

**Priority**: P1 - High  
**Effort**: 2-3 hours

**Files to Modify**:
1. `src-tauri/src/modules/cpu/timer.rs`

**Step-by-Step**:

1. Find the `cpu_use_platform_clock` tweak
2. **REPLACE** it entirely with:

```rust
Tweak {
    id: "cpu_disable_hpet".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "⚡ Disable HPET for Lower Latency".to_string(),
    description: "Disables High Precision Event Timer. Modern TSC is faster. Can improve FPS by 10-20% in games.".to_string(),
    warning_level: WarningLevel::Safe,
    requires_restart: true,
    enabled: false,
    check: None,
    revert_operations: Some(vec![
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/set".to_string(), "useplatformclock".to_string(), "true".to_string()],
        },
    ]),
    operations: vec![
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/deletevalue".to_string(), "useplatformclock".to_string()],
        },
        TweakOperation::Powershell {
            script: r#"
# Also disable HPET device in Device Manager
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" } | 
    Disable-PnpDevice -Confirm:$false -EA 0
Write-Host "HPET disabled - also disable in BIOS for full effect" -ForegroundColor Yellow
"#.to_string(),
        },
    ],
}
```

**Verification**: Run LatencyMon before/after - DPC latency should decrease

---

### A.6 ⛔ USB3 Link Power Values are INVERTED

**Priority**: P1 - High  
**Effort**: 1-2 hours

**Files to Modify**:
1. `src-tauri/src/modules/cpu/power.rs` (around line 253)

**Step-by-Step**:

1. Find the `cpu_usb3_link_power` tweak
2. Change the value from 0 to 3:

```rust
// CHANGE FROM:
// powercfg /setacvalueindex ... d4e98f31-5ffe-4ce1-be31-1b38b384c009 0

// TO:
script: r#"
# USB 3 Link Power Management - Maximum Performance (3), not 0
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 3
powercfg /setactive scheme_current
"#.to_string(),
```

3. Fix revert to use default (1 = Moderate):
```rust
revert_operations: Some(vec![
    TweakOperation::Powershell {
        script: r#"
# Restore default (Moderate Power Savings = 1)
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 1
powercfg /setactive scheme_current
"#.to_string(),
    }
]),
```

**Verification**: USB devices maintain consistent low latency

---

### A.7 ⛔ Apps & Bloatware Crashes

**Priority**: P1 - High  
**Effort**: 3-4 hours  
**Depends On**: A.1

**Files to Modify**:
1. `src-tauri/src/modules/debloat/apps.rs`

**Step-by-Step**:

1. Find bloatware removal scripts
2. Fix PowerShell command:
   - WRONG: `Remove-ProvisionedAppxPackage`
   - RIGHT: `Remove-AppxProvisionedPackage`

3. Replace with better error handling:

```rust
TweakOperation::Powershell {
    script: r#"
$apps = @('Microsoft.BingNews', 'Microsoft.GetHelp', 'Microsoft.Getstarted')
$removed = 0
$failed = 0

foreach ($app in $apps) {
    try {
        $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA Stop
        if ($pkg) {
            $pkg | Remove-AppxPackage -AllUsers -EA Stop
            $removed++
        }
        
        $prov = Get-AppxProvisionedPackage -Online | Where-Object { $_.DisplayName -like "*$app*" }
        if ($prov) {
            $prov | Remove-AppxProvisionedPackage -Online -EA Stop
        }
    } catch {
        $failed++
        Write-Host "Failed: $app - $_" -ForegroundColor Red
    }
}

Write-Host "Removed: $removed, Failed: $failed" -ForegroundColor $(if ($failed -eq 0) { 'Green' } else { 'Yellow' })
"#.to_string(),
}
```

**Verification**: Remove 3+ bloatware apps without crash

---

### A.8 ⛔ Classic UI Module Non-Functional

**Priority**: P1 - High  
**Effort**: 4-6 hours  
**Depends On**: A.2

**Files to Modify**:
1. `src-tauri/src/modules/ui_classic/setup.rs`
2. Create: `src-tauri/resources/secureuxtheme/`

**Step-by-Step**:

1. Download SecureUxTheme DLLs from GitHub releases
2. Create resource folder and add DLLs:
   - `SecureUxTheme_x64.dll`
   - `SecureUxTheme_x86.dll`
   - `SecureUxTheme_ARM64.dll`

3. Add RustEmbed to bundle:

```rust
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/secureuxtheme"]
struct SecureUxAssets;

pub fn deploy_secureuxtheme_assets() -> Result<(), String> {
    let target = std::path::PathBuf::from("C:\\ProgramData\\TommyTweaker\\SecureUxTheme");
    std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    
    for file in SecureUxAssets::iter() {
        if let Some(content) = SecureUxAssets::get(file.as_ref()) {
            let dest = target.join(file.as_ref());
            std::fs::write(&dest, content.data.as_ref()).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
```

**Verification**: Custom .msstyles theme applies after restart

---

### A.9 🔨 ADD: VBS (Virtualization Based Security) Disable

**Priority**: P1 - High  
**Effort**: 2-3 hours

**Files to Modify**:
1. `src-tauri/src/modules/cpu/scheduling.rs`

**Step-by-Step**:

1. Add new tweak to the module:

```rust
Tweak {
    id: "cpu_disable_vbs".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "🔒 Disable VBS (Virtualization Based Security)".to_string(),
    description: "Disables VBS and Memory Integrity. Provides 5-15% FPS boost. Reduces security - only for gaming systems.".to_string(),
    warning_level: WarningLevel::Careful,
    requires_restart: true,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$vbs = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\DeviceGuard" -Name "EnableVirtualizationBasedSecurity" -EA 0
if ($vbs.EnableVirtualizationBasedSecurity -eq 0) { "True" } else { "False" }
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::RegistryDelete {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard".to_string(),
            key: "EnableVirtualizationBasedSecurity".to_string(),
        },
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/deletevalue".to_string(), "hypervisorlaunchtype".to_string()],
        },
    ]),
    operations: vec![
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard".to_string(),
            key: "EnableVirtualizationBasedSecurity".to_string(),
            value: RegistryValue::DWord(0),
        },
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity".to_string(),
            key: "Enabled".to_string(),
            value: RegistryValue::DWord(0),
        },
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/set".to_string(), "hypervisorlaunchtype".to_string(), "off".to_string()],
        },
    ],
}
```

**Verification**: Run `msinfo32.exe`, "Virtualization-based security" shows "Not enabled"

---

### A.10 🔨 ADD: Network Throttling Disable

**Priority**: P1 - High  
**Effort**: 2-3 hours

**Files to Modify**:
1. `src-tauri/src/modules/gaming/mod.rs`

**Step-by-Step**:

1. Add the network throttling tweak (see IMPLEMENTATION_ROADMAP.md A.10 for full code)

**Verification**: Ping test shows lower latency in games

---

### A.11 ⛔ Startup Module - Add Toggle for IFEO/AppInit

**Priority**: P0 - Critical  
**Effort**: 3-4 hours

**Files to Modify**:
1. `src-tauri/src/modules/startup/boot.rs`
2. `src-tauri/src/modules/startup/mod.rs`

**Step-by-Step**:

1. Add `toggle_boot_item()` function to boot.rs (see IMPLEMENTATION_ROADMAP.md A.11 for full code)
2. Update `toggle_item()` in mod.rs to route IFEO/APPINIT calls

**Verification**: Toggle IFEO item off successfully

---

### A.12 ⛔ Network MSI Mode Shows "No NIC Found"

**Priority**: P1 - High  
**Effort**: 2-3 hours

**Files to Modify**:
1. `src-tauri/src/modules/network/msi.rs`

**Step-by-Step**:

1. Improve NIC detection logic
2. Add WMI fallback
3. Better error messaging

**Verification**: Network MSI mode shows detected NICs

---

### A.13 ⛔ NEW: Processor Check Interval Value is WRONG

**Priority**: P1 - High  
**Effort**: 1-2 hours

**Files to Modify**:
1. `src-tauri/src/modules/cpu/timer.rs`

**Step-by-Step**:

1. Find the powercfg command with `4d2b0152-7d5c-498b-88e2-34345392a2c5`
2. Change the value:
   - FROM: 200
   - TO: 1

3. Fix revert value:
   - FROM: whatever it is
   - TO: 15

```rust
// APPLY - set to 1 for minimum latency
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 1

// REVERT - Windows default is 15
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
```

**Verification**: Run LatencyMon, DPC latency should decrease

---

### A.14 ⛔ NEW: Prefetch Cleanup HURTS Performance

**Priority**: P0 - Critical  
**Effort**: 1 hour

**Files to Modify**:
1. `src-tauri/src/modules/system/maintenance.rs`

**Step-by-Step**:

1. Search for any prefetch cleanup code
2. Either REMOVE it entirely, OR change to:

```rust
Tweak {
    id: "system_clean_prefetch",
    name: "⚠️ Clean Prefetch (NOT RECOMMENDED)",
    description: "Clears Windows Prefetch cache. WARNING: This HURTS performance! Prefetch improves app launch times by 15-30%. Only use if troubleshooting corrupted prefetch data. Space saved: ~50MB.",
    warning_level: WarningLevel::Dangerous, // MUST be Dangerous
    // ...
}
```

**Verification**: Prefetch cleanup removed OR has Dangerous warning

---

### A.15 ⛔ NEW: Spectre/Meltdown Registry Conflict

**Priority**: P1 - High  
**Effort**: 2 hours

**Files to Modify**:
1. `src-tauri/src/modules/security/exploit.rs`

**Step-by-Step**:

1. Find `sec_disable_spectre` and `sec_disable_kvas`
2. **MERGE** them into single tweak:

```rust
Tweak {
    id: "sec_disable_cpu_mitigations",
    name: "⚡ Disable CPU Mitigations (Spectre/Meltdown/KVAS)",
    description: "Disables all CPU vulnerability mitigations. Provides 2-8% performance gain. DANGEROUS: Leaves system vulnerable.",
    warning_level: WarningLevel::Dangerous,
    operations: vec![
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverride".to_string(),
            value: RegistryValue::DWord(3),
        },
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverrideMask".to_string(),
            value: RegistryValue::DWord(3),
        },
    ],
}
```

3. **DELETE** the separate `sec_disable_kvas` tweak

**Verification**: Only ONE CPU mitigations tweak exists

---

## 4. PHASE B: FEATURE INTEGRATION

**Duration**: 8-10 days  
**Priority**: HIGH/MEDIUM

---

### B.1 🔨 GPU TDR Delay

**Files**: `src-tauri/src/modules/gpu/scheduling.rs`  
**Action**: Add TdrDelay tweak (see IMPLEMENTATION_ROADMAP.md B.1)

---

### B.2 🔨 Tamper Protection Toggle Warning

**Files**: `src-tauri/src/modules/security/defender.rs`  
**Action**: Add pre-check for Tamper Protection status

---

### B.3 🔨 Mouse Acceleration Disable

**Files**: `src-tauri/src/modules/input/mouse.rs`  
**Action**: Add mouse acceleration disable tweak

---

### B.4 🔨 NVIDIA Telemetry Disable (GPU Module)

**Files**: `src-tauri/src/modules/gpu/mod.rs`  
**Action**: Add telemetry service disable tweak

---

### B.5a 🔨 Check Functions - Security Module

**Files**: `security/defender.rs`, `security/firewall.rs`, `security/uac.rs`  
**Scope**: 15 tweaks  
**Action**: Add check logic to each tweak

---

### B.5b 🔨 Check Functions - Gaming Module

**Files**: `gaming/mod.rs`, `gaming/xbox.rs`  
**Scope**: 7 tweaks  
**Action**: Add check logic to each tweak

---

### B.5c 🔨 Check Functions - CPU/GPU Modules

**Files**: `cpu/power.rs`, `cpu/timer.rs`, `cpu/scheduling.rs`, `gpu/scheduling.rs`  
**Scope**: 24 tweaks  
**Action**: Add check logic to each tweak

---

### B.5d 🔨 Check Functions - Remaining Modules

**Scope**: 157 remaining tweaks across 44 files  
**Action**: Implement incrementally, prioritize most-used tweaks

---

### B.6 🔨 Nagle's Algorithm Disable

**Files**: `src-tauri/src/modules/network/tcp.rs`  
**Action**: Add TcpAckFrequency and TCPNoDelay tweak

---

### B.7 🔨 Windows Services Manager (by Risk Level)

**Files**: `src-tauri/src/modules/system/services.rs`  
**Action**: Create Safe/Careful/Dangerous service categories

---

### B.8 🔨 Remove Duplicate Tweaks

**Files**: Multiple  
**Action**: 
- Remove NetBIOS from `security/hardening.rs`
- Remove LLMNR from `security/hardening.rs`
- Remove WER from `privacy/telemetry.rs`
- Remove "From Atlas" text references

---

### B.9 🔨 Startup Module - Missing Registry Locations

**Files**: `startup/logon.rs`, `startup/boot.rs`  
**Action**: Add Group Policy Run, RunOnceEx, WinNT Run scanning

---

### B.10 🔨 Chrome/Edge Extension Scanning

**Files**: `src-tauri/src/modules/startup/browser.rs`  
**Action**: Add Chrome/Edge extension folder scanning

---

### B.11 🔨 Programs Module Complete Overhaul

**Files**: `src-tauri/src/modules/packages/winget.rs`  
**Action**:
1. Fix check_package_status() - check stdout
2. Add list_available_packages()
3. Add bulk operations
4. Add import/export

---

### B.12 🔨 Display Module Enhancements

**Files**: `src-tauri/src/modules/display/gpu.rs`  
**Action**: Add HDR toggle, NVIDIA Reflex, GPU Scaling fix

---

### B.13 🔨 Input Module Cleanup

**Files**: `input/mouse.rs`, `input/usb.rs`  
**Action**: Move keyboard tweaks, remove USB/CPU overlap

---

### B.14 🔨 Classic Start Menu with Open-Shell

**Files**: `src-tauri/src/modules/ui_classic/tweaks.rs`  
**Action**: Add Windows version detection with Open-Shell fallback

---

### B.15 🔨 NEW: MSI Priority Device-Specific

**Files**: `system/msi.rs`, `network/msi.rs`, `gpu/msi.rs`  
**Action**: 
- Change Priority 0 to 2 (Normal) or 3 (High)
- Remove USB from device class list

---

### B.16 🔨 NEW: Expand MSI Vendor Support

**Files**: `src-tauri/src/modules/network/msi.rs`  
**Action**: Add Qualcomm Atheros, Broadcom, Marvell, Killer vendors

---

### B.17 🔨 NEW: Fast Startup Disable

**Files**: `src-tauri/src/modules/system/maintenance.rs`  
**Action**: Add HiberbootEnabled registry tweak

---

### B.18 🔨 NEW: Remove Defender from Debloat

**Files**: `src-tauri/src/modules/debloat/apps.rs`  
**Action**: Remove any Defender disable functionality

---

## 5. PHASE C: POLISH & OPTIMIZATION

**Duration**: 4-5 days  
**Priority**: LOW

---

### C.1-C.6: UI & Programs Fixes
- C.1: UI Button States (Toggle vs Action)
- C.2: UI Visual Glitches
- C.3: Programs Module Enhancement (merged with B.11)
- C.4: OEM Bloatware Patterns
- C.5: Native NVMe Driver (24H2+)
- C.6: FSO Description Update

---

### C.7-C.9: Storage & Package Tweaks
- C.7: Storage Write-Cache Toggle
- C.8: Disable Scheduled Defrag
- C.9: Package Search Function

---

### C.10-C.13: UI Polish
- C.10: Square Window Corners
- C.11: Show File Extensions
- C.12: Compact File Explorer
- C.13: Remove "Safe Tweaks" from UAC Section

---

### C.14-C.18: NEW Items from Triple-Check
- C.14: Update SysMain Description
- C.15: DNS Cache Size Optimization
- C.16: TCP Initial RTO
- C.17: Fix MMCSS GPU Priority Revert (8→2)
- C.18: Add SMBv1 Disable

---

## 6. TESTING CHECKLIST

### Phase A Verification

- [ ] A.1: Apply 5+ tweaks without UI freeze
- [ ] A.2: State.json in `%APPDATA%\TommyTweaker\`
- [ ] A.3: App shows UAC prompt on launch
- [ ] A.4: Edge removal works on Win11 24H2
- [ ] A.5: LatencyMon shows lower DPC after HPET disable
- [ ] A.6: USB devices consistent after power fix
- [ ] A.7: Remove 3+ bloatware without crash
- [ ] A.8: Custom theme applies after restart
- [ ] A.9: msinfo32 shows VBS "Not enabled"
- [ ] A.10: Lower ping after network throttling disable
- [ ] A.11: Can toggle IFEO items off
- [ ] A.12: Network MSI shows detected NICs
- [ ] A.13: Processor Check Interval = 1
- [ ] A.14: Prefetch cleanup removed/warned
- [ ] A.15: Only ONE CPU mitigations tweak

### Phase B Verification

- [ ] B.5a: Security tweaks show correct state
- [ ] B.9: Startup finds Policy\Run items
- [ ] B.10: Chrome/Edge extensions appear
- [ ] B.11: Package catalog works
- [ ] B.12: HDR toggle works

### Phase C Verification

- [ ] C.1: "Run" buttons for actions
- [ ] C.10: Square corners work
- [ ] C.17: MMCSS revert = 2

---

## 7. FILE REFERENCE MAP

### Core Files
| Purpose | Path |
|---------|------|
| Command Engine | `src-tauri/src/commands.rs` |
| Type Definitions | `src-tauri/src/modules/types.rs` |
| Config Paths | `src-tauri/src/modules/utils/dirs.rs` |
| State Management | `src-tauri/src/modules/utils/state.rs` |

### Module Files
| Module | Directory | Key Files |
|--------|-----------|-----------|
| CPU | `modules/cpu/` | `power.rs`, `timer.rs`, `scheduling.rs` |
| GPU | `modules/gpu/` | `mod.rs`, `msi.rs`, `scheduling.rs` |
| Gaming | `modules/gaming/` | `mod.rs`, `xbox.rs` |
| Network | `modules/network/` | `tcp.rs`, `msi.rs`, `security.rs` |
| Security | `modules/security/` | `defender.rs`, `exploit.rs`, `hardening.rs` |
| Debloat | `modules/debloat/` | `apps.rs`, `edge.rs` |
| Storage | `modules/storage/` | `ntfs.rs`, `power.rs` |
| System | `modules/system/` | `services.rs`, `maintenance.rs`, `msi.rs` |
| Input | `modules/input/` | `mouse.rs`, `keyboard.rs`, `usb.rs` |
| Startup | `modules/startup/` | `boot.rs`, `logon.rs`, `browser.rs` |
| UI Classic | `modules/ui_classic/` | `setup.rs`, `tweaks.rs` |
| Packages | `modules/packages/` | `winget.rs` |

### Frontend Files
| Purpose | Path |
|---------|------|
| Tweak Card | `src/components/TweakCard.svelte` |
| Main CSS | `src/app.css` |
| Type Definitions | `src/lib/types.ts` |

---

## IMPLEMENTATION ORDER SUMMARY

### Week 1: Phase A (Critical)
1. Day 1: A.1 (async), A.2 (paths), A.3 (admin)
2. Day 2: A.4 (Edge), A.5 (HPET), A.6 (USB3)
3. Day 3: A.7 (bloatware), A.8 (UI classic)
4. Day 4: A.9 (VBS), A.10 (throttling), A.11 (IFEO)
5. Day 5: A.12 (NIC), A.13-A.15 (value fixes)
6. Day 6: Testing & bug fixes

### Week 2-3: Phase B (Features)
7. Days 7-9: B.1-B.4 (new tweaks)
8. Days 10-12: B.5a-B.5d (check functions)
9. Days 13-14: B.6-B.11 (modules)
10. Days 15-16: B.12-B.18 (cleanup)

### Week 3-4: Phase C (Polish)
11. Days 17-18: C.1-C.6 (UI)
12. Days 19-20: C.7-C.18 (remaining)
13. Day 21: Final testing

---

*Master Implementation Guide v4.0 - January 12, 2026*  
*This document supersedes all previous roadmaps and audits*
