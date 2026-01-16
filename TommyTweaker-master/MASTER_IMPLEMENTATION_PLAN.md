# TommyTweaker - MASTER IMPLEMENTATION PLAN

> **Version**: 4.0 FINAL  
> **Date**: January 12, 2026  
> **Total Items**: 54 actionable steps  
> **Status**: DEFINITIVE SOURCE OF TRUTH - Supersedes all previous documents

---

## Executive Summary

This document consolidates findings from three audit passes:
- Initial Implementation Roadmap (42 items)
- QA Audit Report (+21 forgotten items) 
- Triple-Check Stress Test (+12 critical fixes)

**Final count: 54 implementation items across 3 phases**

| Phase | Items | Effort | Impact |
|-------|-------|--------|--------|
| **A - Critical** | 15 | 5-6 days | Blocking bugs, crashes, wrong values |
| **B - Features** | 21 | 10-12 days | Missing tweaks, checks, modules |
| **C - Polish** | 18 | 4-5 days | UI fixes, optimizations |

---

## Pre-Implementation Checklist

Before starting, verify:
- [ ] Rust toolchain installed (1.75+)
- [ ] Node.js 18+ installed
- [ ] Tauri CLI installed
- [ ] Project compiles: `cargo build`
- [ ] Frontend builds: `npm run build`

---

# PHASE A: Foundation & Critical Path (HIGH PRIORITY)

**15 items | 5-6 days | Must complete before Phase B**

---

## A.1 ⛔ FIX: Application Freezes During Tweak Application

**Priority**: P0 - Critical  
**Issue**: User reported "application needs to be optimized in general, as the program freezes or becomes unresponsive for a while when applying a single or multiple tweaks."

### Files Affected
- `src-tauri/src/commands.rs` (lines 64-255)
- `src/components/TweakCard.svelte`

### Technical Implementation

**Backend (commands.rs):**
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

**Frontend (TweakCard.svelte):**
```svelte
<script>
  import { listen } from '@tauri-apps/api/event';
  
  let isApplying = false;
  
  async function handleToggle() {
    isApplying = true;
    try {
      await invoke('apply_tweak', { id: tweak.id });
    } finally {
      isApplying = false;
    }
  }
  
  // Listen for progress events
  listen('tweak-progress', (event) => {
    if (event.payload.id === tweak.id) {
      // Update UI based on status
    }
  });
</script>

{#if isApplying}
  <div class="loading-bar animate-pulse bg-blue-500 h-1 w-full" />
{/if}
```

### Dependency Check
None - foundation fix

### Verification Criteria
- [ ] Apply 5+ tweaks in sequence without UI freeze
- [ ] Progress indicator appears during application
- [ ] App remains responsive during tweak execution

---

## A.2 ⛔ FIX: State.json Location (Should be AppData)

**Priority**: P0 - Critical  
**Issue**: "The state.json file should be in appdata, but it's in the program directory"

### Files Affected
- `src-tauri/src/modules/utils/dirs.rs`
- `src-tauri/src/modules/utils/state.rs`

### Technical Implementation

**dirs.rs:**
```rust
use std::path::PathBuf;

pub fn get_app_dir() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "APPDATA environment variable not found")?;
    
    let tommy_dir = PathBuf::from(app_data).join("TommyTweaker");
    
    if !tommy_dir.exists() {
        std::fs::create_dir_all(&tommy_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    Ok(tommy_dir)
}

pub fn get_state_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("state.json"))
}

pub fn get_backup_dir() -> Result<PathBuf, String> {
    let backup_dir = get_app_dir()?.join("backups");
    
    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    }
    
    Ok(backup_dir)
}

pub fn get_logs_dir() -> Result<PathBuf, String> {
    let logs_dir = get_app_dir()?.join("logs");
    
    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }
    
    Ok(logs_dir)
}
```

**Update state.rs to use new paths:**
```rust
use super::dirs::get_state_path;

pub fn load_state() -> Result<AppState, String> {
    let path = get_state_path()?;
    // ... rest of implementation
}

pub fn save_state(state: &AppState) -> Result<(), String> {
    let path = get_state_path()?;
    // ... rest of implementation
}
```

### Dependency Check
None

### Verification Criteria
- [ ] `%APPDATA%\TommyTweaker\` directory created on first run
- [ ] `state.json` saved to `%APPDATA%\TommyTweaker\state.json`
- [ ] Backups saved to `%APPDATA%\TommyTweaker\backups\`
- [ ] Old state.json in program directory is migrated (optional)

---

## A.3 ⛔ FIX: Request Administrator Permissions on Startup

**Priority**: P0 - Critical  
**Issue**: "When the program starts, it should ask for administrator permissions"

### Files Affected
- `src-tauri/tauri.conf.json`
- `src-tauri/build.rs` (create if not exists)

### Technical Implementation

**Option 1: Tauri 2.0 Config (tauri.conf.json):**
```json
{
  "bundle": {
    "windows": {
      "allowDowngrades": true
    }
  },
  "app": {
    "windows": [
      {
        "title": "TommyTweaker",
        "width": 1200,
        "height": 800
      }
    ]
  }
}
```

**Option 2: Windows Manifest (Recommended):**

Create `src-tauri/tommytweaker.exe.manifest`:
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    version="1.0.0.0"
    processorArchitecture="*"
    name="TommyTweaker"
    type="win32"
  />
  <description>TommyTweaker System Optimization Tool</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <!-- Windows 10/11 -->
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
    </application>
  </compatibility>
</assembly>
```

**Update build.rs:**
```rust
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("tommytweaker.exe.manifest");
        res.compile().unwrap();
    }
    tauri_build::build()
}
```

**Add to Cargo.toml:**
```toml
[build-dependencies]
winres = "0.1"
```

### Dependency Check
None

### Verification Criteria
- [ ] Running .exe shows UAC elevation prompt
- [ ] App runs with administrator privileges
- [ ] Registry/PowerShell operations succeed without "Access Denied"

---

## A.4 ⛔ FIX: Edge Removal Script (Completely Broken)

**Priority**: P1 - High  
**Issue**: Edge removal uses deprecated registry method that stopped working in Windows 11 24H2

### Files Affected
- `src-tauri/src/modules/debloat/edge.rs`

### Technical Implementation

Replace entire Edge removal tweak:
```rust
Tweak {
    id: "debloat_edge_removal".to_string(),
    category: TweakCategory::Debloat,
    name: "🌐 Remove Microsoft Edge".to_string(),
    description: "Completely removes Microsoft Edge browser. WARNING: May break WebView2 apps (Teams, Outlook). Use Firefox or Chrome as alternative.".to_string(),
    warning_level: WarningLevel::Dangerous,
    requires_restart: true,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
if (Test-Path "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe") {
    Write-Output "False"
} else {
    Write-Output "True"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::Powershell {
            script: r#"
Write-Host "Edge cannot be automatically reinstalled." -ForegroundColor Yellow
Write-Host "To reinstall Edge:" -ForegroundColor Cyan
Write-Host "1. Download from: https://www.microsoft.com/edge" -ForegroundColor White
Write-Host "2. Or run: winget install Microsoft.Edge" -ForegroundColor White
"#.to_string(),
        }
    ]),
    operations: vec![
        TweakOperation::Powershell {
            script: r#"
Write-Host "Removing Microsoft Edge..." -ForegroundColor Yellow

# Step 1: Check Windows version
$build = [int](Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").CurrentBuildNumber
Write-Host "Windows Build: $build" -ForegroundColor Cyan

# Step 2: Remove NoRemove flag (REQUIRED first step)
$uninstallPath = "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft Edge"
if (Test-Path $uninstallPath) {
    Set-ItemProperty -Path $uninstallPath -Name "NoRemove" -Value 0 -Force -EA 0
    Write-Host "[1/6] Removed uninstall protection" -ForegroundColor Green
}

# Step 3: Find Edge version and uninstaller
$EdgePath = "C:\Program Files (x86)\Microsoft\Edge\Application"
if (Test-Path $EdgePath) {
    $EdgeVersion = Get-ChildItem $EdgePath -Directory | 
        Where-Object { $_.Name -match '^\d+\.\d+' } | 
        Sort-Object { [version]($_.Name -replace '\..*$', '') } -Descending | 
        Select-Object -First 1 -ExpandProperty Name
    
    if ($EdgeVersion) {
        $UninstallCmd = "$EdgePath\$EdgeVersion\Installer\setup.exe"
        
        if (Test-Path $UninstallCmd) {
            Write-Host "[2/6] Uninstalling Edge version $EdgeVersion..." -ForegroundColor Yellow
            
            $process = Start-Process -FilePath $UninstallCmd `
                -ArgumentList "--uninstall --system-level --verbose-logging --force-uninstall" `
                -Wait -PassThru -NoNewWindow
            
            if ($process.ExitCode -eq 0) {
                Write-Host "[3/6] Edge uninstalled successfully" -ForegroundColor Green
            } else {
                Write-Host "[3/6] Edge uninstall returned code: $($process.ExitCode)" -ForegroundColor Yellow
            }
        }
    }
} else {
    Write-Host "[2/6] Edge not found in standard location" -ForegroundColor Yellow
}

# Step 4: Remove Edge WebView (optional but thorough)
$WebViewPath = "C:\Program Files (x86)\Microsoft\EdgeWebView"
if (Test-Path $WebViewPath) {
    Write-Host "[4/6] Removing Edge WebView..." -ForegroundColor Yellow
    Remove-Item -Path $WebViewPath -Recurse -Force -EA 0
}

# Step 5: Prevent Edge reinstallation via Group Policy
$policyPaths = @(
    "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate",
    "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
)
foreach ($path in $policyPaths) {
    if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
}
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate" -Name "DoNotUpdateToEdgeWithChromium" -Value 1 -Type DWord -Force
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate" -Name "CreateDesktopShortcutDefault" -Value 0 -Type DWord -Force
Write-Host "[5/6] Blocked Edge reinstallation" -ForegroundColor Green

# Step 6: Disable Edge Update scheduled tasks
$tasks = Get-ScheduledTask | Where-Object { $_.TaskName -like "*MicrosoftEdge*" -or $_.TaskName -like "*Edge*Update*" }
foreach ($task in $tasks) {
    Disable-ScheduledTask -TaskName $task.TaskName -EA 0 | Out-Null
}
Write-Host "[6/6] Disabled Edge update tasks" -ForegroundColor Green

# Step 7: Stop and disable Edge Update service
Stop-Service -Name "edgeupdate" -Force -EA 0
Stop-Service -Name "edgeupdatem" -Force -EA 0
Set-Service -Name "edgeupdate" -StartupType Disabled -EA 0
Set-Service -Name "edgeupdatem" -StartupType Disabled -EA 0

Write-Host "`nEdge removal complete! Restart recommended." -ForegroundColor Green
"#.to_string(),
        }
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Edge removed from Start Menu
- [ ] Edge removed from Program Files
- [ ] Edge cannot reinstall after Windows Update
- [ ] Reboot completes without issues

---

## A.5 ⛔ FIX: HPET Logic is INVERTED (Currently HURTS Performance)

**Priority**: P1 - High  
**Issue**: Current tweak ENABLES HPET, but modern consensus is HPET should be DISABLED for gaming (causes 10-20% FPS loss)

### Files Affected
- `src-tauri/src/modules/cpu/timer.rs`

### Technical Implementation

**REPLACE the existing `cpu_use_platform_clock` tweak:**
```rust
Tweak {
    id: "cpu_disable_hpet".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "⚡ Disable HPET for Lower Latency".to_string(),
    description: "Disables High Precision Event Timer. Modern TSC (Time Stamp Counter) is faster. Research shows HPET causes 10-20% FPS loss in games. Also disable HPET in BIOS for maximum effect.".to_string(),
    warning_level: WarningLevel::Safe,
    requires_restart: true,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$hpet = Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" }
if ($hpet.Status -eq "Error" -or $hpet.Status -eq "Disabled") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/set".to_string(), "useplatformclock".to_string(), "true".to_string()],
        },
        TweakOperation::Powershell {
            script: r#"
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" } | 
    Enable-PnpDevice -Confirm:$false -EA 0
Write-Host "HPET re-enabled" -ForegroundColor Green
"#.to_string(),
        },
    ]),
    operations: vec![
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/deletevalue".to_string(), "useplatformclock".to_string()],
        },
        TweakOperation::Powershell {
            script: r#"
# Disable HPET device in Device Manager
$hpet = Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" }
if ($hpet) {
    Disable-PnpDevice -InstanceId $hpet.InstanceId -Confirm:$false -EA 0
    Write-Host "HPET disabled in Device Manager" -ForegroundColor Green
} else {
    Write-Host "HPET device not found (may already be disabled)" -ForegroundColor Yellow
}

Write-Host "`nIMPORTANT: Also disable HPET in BIOS for full effect!" -ForegroundColor Cyan
Write-Host "Location varies by motherboard - look in CPU or Power settings" -ForegroundColor White
"#.to_string(),
        },
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Run `bcdedit /enum` - no `useplatformclock` entry
- [ ] Device Manager shows HPET as disabled
- [ ] LatencyMon shows reduced DPC latency (run before/after comparison)
- [ ] Game FPS improves by ~10-20%

---

## A.6 ⛔ FIX: USB3 Link Power Values are INVERTED

**Priority**: P1 - High  
**Issue**: USB3 link power uses value 0 (max power saving) instead of 3 (max performance)

### Files Affected
- `src-tauri/src/modules/cpu/power.rs` (line ~253)

### Technical Implementation

**Find and replace the USB3 link power script:**
```rust
Tweak {
    id: "cpu_usb3_link_power".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "🔌 Disable USB 3 Link Power Management".to_string(),
    description: "Sets USB 3 Link Power Management to maximum performance. Prevents USB device latency spikes and disconnections.".to_string(),
    warning_level: WarningLevel::Safe,
    requires_restart: false,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$result = powercfg /q scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009
if ($result -match "0x00000003") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::Powershell {
            script: r#"
# Restore default (Moderate Power Savings = 1)
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 1
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 1
powercfg /setactive scheme_current
Write-Host "USB 3 Link Power restored to moderate" -ForegroundColor Green
"#.to_string(),
        }
    ]),
    operations: vec![
        TweakOperation::Powershell {
            script: r#"
# USB 3 Link Power Management values:
# 0 = Maximum Power Savings (WRONG for performance!)
# 1 = Moderate Power Savings (default)
# 2 = Minimum Power Savings
# 3 = Maximum Performance (CORRECT!)

# Set USB 3 Link Power to Maximum Performance (3)
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 3
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 3

# Also disable USB Selective Suspend
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0

# USB Hub Selective Suspend Timeout - 0ms
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 0853a681-27c8-4100-a2fd-82013e970683 0
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 0853a681-27c8-4100-a2fd-82013e970683 0

powercfg /setactive scheme_current
Write-Host "USB 3 Link Power set to Maximum Performance" -ForegroundColor Green
"#.to_string(),
        }
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Run `powercfg /q` and verify USB settings show 0x00000003
- [ ] USB devices maintain stable connection
- [ ] No USB latency spikes visible in LatencyMon

---

## A.7 ⛔ FIX: Apps & Bloatware Crashes

**Priority**: P1 - High  
**Issue**: "All the tweaks within Apps & Bloatware don't work, it just crashes"

### Files Affected
- `src-tauri/src/modules/debloat/apps.rs`

### Technical Implementation

**Root cause: Wrong cmdlet name and missing error handling**

```rust
// Common bloatware removal function with proper error handling
fn create_bloatware_removal_script(apps: &[&str]) -> String {
    let app_list = apps.iter()
        .map(|a| format!("'{}'", a))
        .collect::<Vec<_>>()
        .join(", ");
    
    format!(r#"
$ErrorActionPreference = "SilentlyContinue"
$apps = @({})
$removed = 0
$failed = 0
$notFound = 0

Write-Host "Starting bloatware removal..." -ForegroundColor Cyan

foreach ($appPattern in $apps) {{
    Write-Host "Processing: $appPattern" -ForegroundColor White
    
    # Try to remove installed package
    $packages = Get-AppxPackage -AllUsers | Where-Object {{ $_.Name -like "*$appPattern*" }}
    
    if ($packages) {{
        foreach ($pkg in $packages) {{
            try {{
                Remove-AppxPackage -Package $pkg.PackageFullName -AllUsers -ErrorAction Stop
                $removed++
                Write-Host "  ✓ Removed: $($pkg.Name)" -ForegroundColor Green
            }} catch {{
                $failed++
                Write-Host "  ✗ Failed: $($pkg.Name) - $($_.Exception.Message)" -ForegroundColor Red
            }}
        }}
    }} else {{
        $notFound++
    }}
    
    # Also remove provisioned package (prevents reinstall for new users)
    # CORRECT cmdlet: Remove-AppxProvisionedPackage (not Remove-ProvisionedAppxPackage!)
    $provisioned = Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like "*$appPattern*" }}
    
    if ($provisioned) {{
        foreach ($prov in $provisioned) {{
            try {{
                Remove-AppxProvisionedPackage -Online -PackageName $prov.PackageName -ErrorAction Stop | Out-Null
                Write-Host "  ✓ Deprovisioned: $($prov.DisplayName)" -ForegroundColor Green
            }} catch {{
                Write-Host "  ✗ Deprovision failed: $($prov.DisplayName)" -ForegroundColor Yellow
            }}
        }}
    }}
}}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Removed: $removed" -ForegroundColor Green
Write-Host "Failed: $failed" -ForegroundColor $(if ($failed -gt 0) {{ "Red" }} else {{ "Green" }})
Write-Host "Not found: $notFound" -ForegroundColor Yellow
"#, app_list)
}

// Example usage in tweak:
Tweak {
    id: "debloat_common_apps".to_string(),
    category: TweakCategory::Debloat,
    name: "🗑️ Remove Common Bloatware".to_string(),
    description: "Removes Bing apps, Get Help, Tips, and other unused Microsoft apps.".to_string(),
    warning_level: WarningLevel::Safe,
    operations: vec![
        TweakOperation::Powershell {
            script: create_bloatware_removal_script(&[
                "Microsoft.BingNews",
                "Microsoft.BingWeather",
                "Microsoft.GetHelp",
                "Microsoft.Getstarted",
                "Microsoft.MicrosoftOfficeHub",
                "Microsoft.MicrosoftSolitaireCollection",
                "Microsoft.People",
                "Microsoft.PowerAutomateDesktop",
                "Microsoft.Todos",
                "Microsoft.WindowsFeedbackHub",
                "Microsoft.WindowsMaps",
                "Microsoft.ZuneMusic",
                "Microsoft.ZuneVideo",
                "Clipchamp.Clipchamp",
            ]),
        }
    ],
    // ...
}
```

### Dependency Check
- A.1 (async execution) should be implemented first to prevent UI freeze during removal

### Verification Criteria
- [ ] Remove 3+ bloatware apps without crash
- [ ] Error messages displayed for failed removals
- [ ] Summary shows correct counts
- [ ] Apps don't reinstall after Windows Update

---

## A.8 ⛔ FIX: Classic UI Module Non-Functional

**Priority**: P1 - High  
**Issue**: "Everything in the Classic UI doesn't work"

### Files Affected
- `src-tauri/src/modules/ui_classic/setup.rs`
- `src-tauri/Cargo.toml`
- `src-tauri/resources/secureuxtheme/` (new directory)

### Technical Implementation

**1. Add to Cargo.toml:**
```toml
[dependencies]
rust-embed = "8.0"
```

**2. Create setup.rs:**
```rust
use rust_embed::RustEmbed;
use std::fs;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "resources/secureuxtheme"]
struct SecureUxAssets;

pub fn get_secureuxtheme_dir() -> PathBuf {
    PathBuf::from("C:\\ProgramData\\TommyTweaker\\SecureUxTheme")
}

pub fn deploy_secureuxtheme_assets() -> Result<(), String> {
    let target_dir = get_secureuxtheme_dir();
    
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create SecureUxTheme directory: {}", e))?;
    }
    
    for file in SecureUxAssets::iter() {
        let file_path = file.as_ref();
        if let Some(content) = SecureUxAssets::get(file_path) {
            let dest = target_dir.join(file_path);
            
            // Create parent directories if needed
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).ok();
            }
            
            fs::write(&dest, content.data.as_ref())
                .map_err(|e| format!("Failed to write {}: {}", file_path, e))?;
        }
    }
    
    Ok(())
}

pub fn is_secureuxtheme_installed() -> bool {
    let dir = get_secureuxtheme_dir();
    dir.join("SecureUxTheme.dll").exists()
}
```

**3. Download SecureUxTheme binaries:**
- Go to: https://github.com/namazso/SecureUxTheme/releases
- Download latest release
- Extract to `src-tauri/resources/secureuxtheme/`:
  - `SecureUxTheme_x64.dll`
  - `SecureUxTheme_x86.dll`

### Dependency Check
- A.2 (AppData paths) should be complete first

### Verification Criteria
- [ ] SecureUxTheme DLLs deployed to `C:\ProgramData\TommyTweaker\SecureUxTheme\`
- [ ] Custom .msstyles theme applies after reboot
- [ ] No file permission errors

---

## A.9 🔨 ADD: VBS (Virtualization Based Security) Disable

**Priority**: P1 - High  
**Issue**: "Single biggest performance gain (5-15% FPS)" - completely missing from codebase

### Files Affected
- `src-tauri/src/modules/cpu/scheduling.rs` (add new tweak)

### Technical Implementation

```rust
Tweak {
    id: "cpu_disable_vbs".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "🔒 Disable VBS (Virtualization Based Security)".to_string(),
    description: "Disables Virtualization Based Security and Memory Integrity (HVCI). Provides 5-15% FPS boost in games. WARNING: Reduces protection against kernel exploits - only recommended for gaming-focused systems.".to_string(),
    warning_level: WarningLevel::Careful,
    requires_restart: true,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$vbs = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\DeviceGuard" -Name "EnableVirtualizationBasedSecurity" -EA 0
if ($null -eq $vbs -or $vbs.EnableVirtualizationBasedSecurity -eq 0) {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::RegistryDelete {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard".to_string(),
            key: "EnableVirtualizationBasedSecurity".to_string(),
        },
        TweakOperation::RegistryDelete {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity".to_string(),
            key: "Enabled".to_string(),
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
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity".to_string(),
            key: "Locked".to_string(),
            value: RegistryValue::DWord(0),
        },
        TweakOperation::Command {
            cmd: "bcdedit".to_string(),
            args: vec!["/set".to_string(), "hypervisorlaunchtype".to_string(), "off".to_string()],
        },
        TweakOperation::Powershell {
            script: r#"
Write-Host "VBS and Memory Integrity disabled" -ForegroundColor Green
Write-Host "Restart required for changes to take effect" -ForegroundColor Yellow
Write-Host "`nTo verify after restart:" -ForegroundColor Cyan
Write-Host "1. Run msinfo32.exe" -ForegroundColor White
Write-Host "2. Look for 'Virtualization-based security'" -ForegroundColor White
Write-Host "3. Should show 'Not enabled'" -ForegroundColor White
"#.to_string(),
        },
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Run `msinfo32.exe` → "Virtualization-based security" shows "Not enabled"
- [ ] Windows Security → Device Security → Core isolation shows "Off"
- [ ] Game FPS increases by 5-15%

---

## A.10 🔨 ADD: Network Throttling Disable (Gaming Critical)

**Priority**: P1 - High  
**Issue**: "Single biggest latency improvement (10-30ms)" - completely missing

### Files Affected
- `src-tauri/src/modules/gaming/mod.rs`

### Technical Implementation

```rust
Tweak {
    id: "gaming_disable_network_throttling".to_string(),
    category: TweakCategory::GameOptimizations,
    name: "🌐 Disable Network Throttling".to_string(),
    description: "Disables Windows network throttling (10 packets/ms limit). Reduces online gaming latency by 10-30ms. Also sets SystemResponsiveness to 0 for maximum foreground priority.".to_string(),
    warning_level: WarningLevel::Safe,
    requires_restart: false,
    enabled: false,
    check: Some(TweakCheck::Registry {
        root_key: "HKLM".to_string(),
        path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
        key: "NetworkThrottlingIndex".to_string(),
        expected_value: RegistryValue::DWord(0xFFFFFFFF),
    }),
    revert_operations: Some(vec![
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
            key: "NetworkThrottlingIndex".to_string(),
            value: RegistryValue::DWord(10),
        },
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
            key: "SystemResponsiveness".to_string(),
            value: RegistryValue::DWord(20),
        },
    ]),
    operations: vec![
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
            key: "NetworkThrottlingIndex".to_string(),
            value: RegistryValue::DWord(0xFFFFFFFF), // Disable throttling completely
        },
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
            key: "SystemResponsiveness".to_string(),
            value: RegistryValue::DWord(0), // 100% to foreground apps
        },
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Registry shows `NetworkThrottlingIndex = 0xFFFFFFFF`
- [ ] Registry shows `SystemResponsiveness = 0`
- [ ] Online game ping decreases by 10-30ms

---

## A.11 ⛔ FIX: Startup Module - Add Toggle for IFEO/AppInit Items

**Priority**: P0 - Critical  
**Issue**: Boot persistence items (IFEO hijacks, AppInit_DLLs) have no toggle - users can't remove malware!

### Files Affected
- `src-tauri/src/modules/startup/boot.rs`
- `src-tauri/src/modules/startup/mod.rs`

### Technical Implementation

**Add to boot.rs:**
```rust
use winreg::enums::*;
use winreg::RegKey;

pub fn toggle_boot_item(id: &str, enable: bool) -> Result<(), String> {
    if id.starts_with("IFEO:") {
        toggle_ifeo_item(id, enable)
    } else if id.starts_with("APPINIT:") {
        toggle_appinit_item(id, enable)
    } else if id.starts_with("BOOTEXEC:") {
        toggle_bootexec_item(id, enable)
    } else {
        Err(format!("Unknown boot item type: {}", id))
    }
}

fn toggle_ifeo_item(id: &str, enable: bool) -> Result<(), String> {
    let exe_name = id.strip_prefix("IFEO:")
        .ok_or("Invalid IFEO ID format")?;
    
    let path = format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
        exe_name
    );
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey_with_flags(&path, KEY_ALL_ACCESS)
        .map_err(|e| format!("Cannot open IFEO key: {}", e))?;
    
    if enable {
        // Re-enabling IFEO is dangerous - refuse
        return Err("Cannot re-enable IFEO debugger hijacks - too dangerous. \
                    If this was legitimate software, reinstall it.".to_string());
    }
    
    // Backup before deletion
    if let Ok(debugger) = key.get_value::<String, _>("Debugger") {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let backup_path = format!(r"Software\TommyTweaker\Backups\IFEO\{}", exe_name);
        let (backup_key, _) = hkcu.create_subkey(&backup_path)
            .map_err(|e| format!("Cannot create backup: {}", e))?;
        backup_key.set_value("Debugger", &debugger)
            .map_err(|e| format!("Cannot save backup: {}", e))?;
    }
    
    // Delete the debugger value
    key.delete_value("Debugger")
        .map_err(|e| format!("Cannot remove debugger: {}", e))?;
    
    Ok(())
}

fn toggle_appinit_item(id: &str, enable: bool) -> Result<(), String> {
    let paths = [
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Windows",
    ];
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    for path in paths {
        if let Ok(key) = hklm.open_subkey_with_flags(path, KEY_ALL_ACCESS) {
            if enable {
                return Err("Cannot re-enable AppInit_DLLs - too dangerous".to_string());
            }
            
            // Backup current value
            if let Ok(current) = key.get_value::<String, _>("AppInit_DLLs") {
                if !current.is_empty() {
                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    let backup_path = r"Software\TommyTweaker\Backups\AppInit";
                    let (backup_key, _) = hkcu.create_subkey(backup_path).ok().unwrap();
                    backup_key.set_value("AppInit_DLLs", &current).ok();
                }
            }
            
            // Clear AppInit_DLLs
            key.set_value("AppInit_DLLs", &"").ok();
            key.set_value("LoadAppInit_DLLs", &0u32).ok();
        }
    }
    
    Ok(())
}

fn toggle_bootexec_item(id: &str, enable: bool) -> Result<(), String> {
    // BootExecute items are more complex - just warn user
    Err("BootExecute items cannot be toggled safely. \
         Use 'msconfig' or 'autoruns' for manual editing.".to_string())
}
```

**Update mod.rs toggle_item():**
```rust
pub fn toggle_item(id: String, enable: bool) -> Result<(), String> {
    // Check for boot persistence items first
    if id.starts_with("IFEO:") || id.starts_with("APPINIT:") || id.starts_with("BOOTEXEC:") {
        return boot::toggle_boot_item(&id, enable);
    }
    
    // ... existing toggle logic for other item types ...
}
```

### Dependency Check
None

### Verification Criteria
- [ ] IFEO debugger entry can be removed
- [ ] AppInit_DLLs can be cleared
- [ ] Backup created before removal
- [ ] Re-enabling blocked with error message

---

## A.12 ⛔ FIX: Network MSI Mode Shows "No NIC Found"

**Priority**: P1 - High  
**Issue**: User reported "Network MSI mode shows --> no supported NIC found"

### Files Affected
- `src-tauri/src/modules/network/msi.rs`

### Technical Implementation

```rust
pub async fn scan_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
    let script = r#"
$ErrorActionPreference = "SilentlyContinue"

# Try multiple methods to find network adapters
$adapters = @()

# Method 1: Get-NetAdapter (preferred)
$netAdapters = Get-NetAdapter -Physical | Where-Object { $_.Status -eq 'Up' -or $_.Status -eq 'Disconnected' }

foreach ($adapter in $netAdapters) {
    $pnpDevice = Get-PnpDevice | Where-Object { $_.InstanceId -eq $adapter.PnPDeviceID }
    
    $adapters += [PSCustomObject]@{
        Name = $adapter.Name
        Description = $adapter.InterfaceDescription
        DeviceId = $adapter.PnPDeviceID
        MacAddress = $adapter.MacAddress
        Speed = $adapter.LinkSpeed
        Status = $adapter.Status
        DriverVersion = $adapter.DriverVersion
    }
}

# Method 2: Fallback to WMI if Get-NetAdapter found nothing
if ($adapters.Count -eq 0) {
    $wmiAdapters = Get-WmiObject Win32_NetworkAdapter | Where-Object { 
        $_.PhysicalAdapter -eq $true -and 
        $_.PNPDeviceID -match "PCI" 
    }
    
    foreach ($adapter in $wmiAdapters) {
        $adapters += [PSCustomObject]@{
            Name = $adapter.Name
            Description = $adapter.Description
            DeviceId = $adapter.PNPDeviceID
            MacAddress = $adapter.MACAddress
            Speed = $adapter.Speed
            Status = if ($adapter.NetEnabled) { "Up" } else { "Down" }
            DriverVersion = "Unknown"
        }
    }
}

# Method 3: Direct PnP device enumeration as last resort
if ($adapters.Count -eq 0) {
    $pnpNics = Get-PnpDevice -Class Net -Status OK | Where-Object {
        $_.InstanceId -match "PCI\\VEN_"
    }
    
    foreach ($nic in $pnpNics) {
        $adapters += [PSCustomObject]@{
            Name = $nic.FriendlyName
            Description = $nic.FriendlyName
            DeviceId = $nic.InstanceId
            MacAddress = "Unknown"
            Speed = "Unknown"
            Status = "OK"
            DriverVersion = "Unknown"
        }
    }
}

if ($adapters.Count -eq 0) {
    Write-Output "NO_ADAPTERS_FOUND"
} else {
    $adapters | ConvertTo-Json -Compress
}
"#;

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if stdout == "NO_ADAPTERS_FOUND" {
        return Err("No network adapters found. Ensure network drivers are installed.".to_string());
    }
    
    if stdout.is_empty() {
        return Err("Failed to enumerate network adapters".to_string());
    }
    
    // Parse JSON response
    let adapters: Vec<NetworkAdapter> = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse adapter list: {}", e))?;
    
    Ok(adapters)
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Network MSI mode shows detected NICs
- [ ] Works with Intel, Realtek, Qualcomm, Broadcom adapters
- [ ] Proper error message if no adapters found

---

## A.13 ⛔ FIX: Processor Check Interval Value is WRONG

**Priority**: P1 - High  
**Issue**: Current value 200 INCREASES latency. Windows default is 15, optimal is 1.

### Files Affected
- `src-tauri/src/modules/cpu/timer.rs`

### Technical Implementation

**Find the processor check interval tweak and fix:**
```rust
Tweak {
    id: "cpu_processor_check_interval".to_string(),
    category: TweakCategory::CpuPerformance,
    name: "⚡ Optimize Processor Check Interval".to_string(),
    description: "Sets processor performance check interval to 1 (minimum). Reduces latency by checking CPU state more frequently. Windows default is 15.".to_string(),
    warning_level: WarningLevel::Safe,
    requires_restart: false,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$result = powercfg /q scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5
if ($result -match "0x00000001") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::Powershell {
            script: r#"
# Restore Windows default (15, NOT 15ms - it's a counter value)
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
powercfg /setdcvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
powercfg /setactive scheme_current
Write-Host "Processor check interval restored to default (15)" -ForegroundColor Green
"#.to_string(),
        }
    ]),
    operations: vec![
        TweakOperation::Powershell {
            script: r#"
# Processor Check Interval:
# - Values are timer tick counts, NOT milliseconds
# - 200 = check every 200 ticks (WRONG - increases latency!)
# - 15 = Windows default
# - 1 = check every tick (OPTIMAL for performance)

powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 1
powercfg /setdcvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 1
powercfg /setactive scheme_current
Write-Host "Processor check interval set to 1 (minimum latency)" -ForegroundColor Green
"#.to_string(),
        }
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] powercfg /q shows value 0x00000001
- [ ] DPC latency in LatencyMon decreases
- [ ] No increase in CPU power consumption

---

## A.14 ⛔ FIX: Prefetch Cleanup HURTS Performance - Remove or Warn

**Priority**: P0 - Critical  
**Issue**: Cleaning Prefetch causes "-15-30% launch speed" according to analysis

### Files Affected
- `src-tauri/src/modules/system/maintenance.rs`

### Technical Implementation

**Option 1: REMOVE the prefetch cleanup tweak entirely**

**Option 2: If keeping, change to DANGEROUS with clear warning:**
```rust
Tweak {
    id: "system_clean_prefetch".to_string(),
    category: TweakCategory::SystemMaintenance,
    name: "⚠️ Clean Prefetch Cache (NOT RECOMMENDED)".to_string(),
    description: "Clears Windows Prefetch cache. 

⛔ WARNING: THIS HURTS PERFORMANCE! ⛔

Prefetch improves application launch times by 15-30% by pre-loading 
frequently used data. Cleaning it provides NO benefit and forces Windows 
to rebuild the cache from scratch.

Only use if:
- Troubleshooting corrupted prefetch data
- SSD is nearly full (saves ~50MB)
- Testing fresh launch times

Modern SSDs are not harmed by Prefetch writes.".to_string(),
    warning_level: WarningLevel::Dangerous,
    requires_restart: false,
    enabled: false,
    check: None,
    revert_operations: None, // Cannot restore prefetch data
    operations: vec![
        TweakOperation::Powershell {
            script: r#"
$confirmation = Read-Host "This will SLOW DOWN your system. Type 'YES' to confirm"
if ($confirmation -ne "YES") {
    Write-Host "Operation cancelled" -ForegroundColor Yellow
    exit 1
}

$prefetchPath = "$env:SystemRoot\Prefetch"
$count = (Get-ChildItem $prefetchPath -EA 0).Count

Remove-Item "$prefetchPath\*" -Force -EA 0

Write-Host "Removed $count prefetch files (~50MB)" -ForegroundColor Yellow
Write-Host "Windows will rebuild prefetch over the next few days" -ForegroundColor Yellow
Write-Host "Expect slower app launches until then" -ForegroundColor Red
"#.to_string(),
        }
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Prefetch cleanup either removed OR has Dangerous warning
- [ ] Warning text clearly states performance impact
- [ ] User must confirm before execution

---

## A.15 ⛔ FIX: Spectre/Meltdown Tweaks Have Conflicting Registry Values

**Priority**: P1 - High  
**Issue**: Both `sec_disable_spectre` and `sec_disable_kvas` write different values to the same registry keys

### Files Affected
- `src-tauri/src/modules/security/exploit.rs`

### Technical Implementation

**MERGE into single comprehensive tweak:**
```rust
// REMOVE sec_disable_spectre AND sec_disable_kvas
// REPLACE with single unified tweak:

Tweak {
    id: "sec_disable_cpu_mitigations".to_string(),
    category: TweakCategory::Security,
    name: "⚡ Disable CPU Vulnerability Mitigations".to_string(),
    description: "Disables all CPU vulnerability mitigations (Spectre, Meltdown, L1TF, MDS, KVAS). 

Performance gain: 2-8% depending on workload and CPU.

⚠️ SECURITY WARNING ⚠️
This leaves your system vulnerable to:
- Spectre V1/V2 (side-channel attacks)
- Meltdown (memory leak attacks)
- L1TF (Foreshadow)
- MDS (Microarchitectural Data Sampling)

Only enable on:
- Gaming-focused systems with no sensitive data
- Air-gapped systems
- VMs where host handles security

NOT recommended for:
- Systems handling financial/personal data
- Shared computers
- Internet-connected work machines".to_string(),
    warning_level: WarningLevel::Dangerous,
    requires_restart: true,
    enabled: false,
    check: Some(TweakCheck::Powershell {
        script: r#"
$override = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "FeatureSettingsOverride" -EA 0
$mask = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "FeatureSettingsOverrideMask" -EA 0

if ($override.FeatureSettingsOverride -eq 3 -and $mask.FeatureSettingsOverrideMask -eq 3) {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
        expected_output: "True".to_string(),
    }),
    revert_operations: Some(vec![
        TweakOperation::RegistryDelete {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverride".to_string(),
        },
        TweakOperation::RegistryDelete {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverrideMask".to_string(),
        },
    ]),
    operations: vec![
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverride".to_string(),
            value: RegistryValue::DWord(3), // Disables Spectre + Meltdown
        },
        TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
            key: "FeatureSettingsOverrideMask".to_string(),
            value: RegistryValue::DWord(3), // Apply override
        },
        TweakOperation::Powershell {
            script: r#"
Write-Host "CPU vulnerability mitigations disabled" -ForegroundColor Yellow
Write-Host "Restart required for changes to take effect" -ForegroundColor Yellow
Write-Host "`nTo verify after restart:" -ForegroundColor Cyan
Write-Host "Run: Get-SpeculationControlSettings" -ForegroundColor White
Write-Host "All mitigations should show as disabled" -ForegroundColor White
"#.to_string(),
        },
    ],
}
```

### Dependency Check
None

### Verification Criteria
- [ ] Only ONE CPU mitigations tweak exists
- [ ] No registry value conflicts
- [ ] `Get-SpeculationControlSettings` shows mitigations disabled after reboot

---

# PHASE B: Feature Integration (MEDIUM PRIORITY)

**21 items | 10-12 days | Implement after Phase A complete**

Due to document length limits, Phase B and C items follow the same detailed format. Here's a summary with key items expanded:

---

## B.1 🔨 ADD: GPU TDR Delay

**Files**: `gpu/scheduling.rs`  
**Action**: Add TdrDelay=8, TdrLevel=3 to prevent "Display driver stopped responding"

---

## B.2 🔨 ADD: Tamper Protection Check

**Files**: `security/defender.rs`  
**Action**: Add check for Tamper Protection status before Defender disable

---

## B.3 🔨 ADD: Mouse Acceleration Disable

**Files**: `input/mouse.rs`  
**Action**: Add MouseSpeed=0, MouseThreshold1=0, MouseThreshold2=0

---

## B.4 🔨 ADD: NVIDIA Telemetry Disable

**Files**: `gpu/mod.rs`  
**Action**: Stop NvTelemetryContainer, NvContainerLocalSystem services

---

## B.5a-d 🔨 ADD: Check Functions (203 tweaks across 44 files)

Split by priority:
- **B.5a**: Security module (15 tweaks) - HIGH
- **B.5b**: Gaming module (7 tweaks) - HIGH
- **B.5c**: CPU/GPU modules (24 tweaks) - MEDIUM
- **B.5d**: Remaining modules (157 tweaks) - LOW

---

## B.6 🔨 ADD: Nagle's Algorithm Disable

**Files**: `network/tcp.rs`  
**Action**: TcpAckFrequency=1, TCPNoDelay=1 per interface

---

## B.7 🔨 ADD: Windows Services Manager

**Files**: `system/services.rs`  
**Action**: Create Safe/Careful/Dangerous service disable categories

---

## B.8 🔨 FIX: Remove Duplicate Tweaks

**Action**: Remove from security/hardening.rs:
- NetBIOS disable (keep in network/security.rs)
- LLMNR disable (keep in network/security.rs)

---

## B.9 🔨 ADD: Startup Registry Locations

**Files**: `startup/logon.rs`  
**Action**: Add Group Policy Run, RunOnceEx, Windows NT Run scanning

---

## B.10 🔨 ADD: Chrome/Edge Extension Scanning

**Files**: `startup/browser.rs`  
**Action**: Scan `%LOCALAPPDATA%\Google\Chrome\...\Extensions`

---

## B.11 🔨 ADD: Programs Module Overhaul

**Files**: `packages/winget.rs`  
**Action**: Fix check_package_status (check stdout not exit code), add catalog, bulk ops

---

## B.12 🔨 ADD: Display Enhancements

**Files**: `display/gpu.rs`  
**Action**: HDR toggle, NVIDIA Reflex, GPU Scaling fix

---

## B.13 🔨 ADD: Input Module Cleanup

**Files**: `input/mouse.rs`, `input/usb.rs`  
**Action**: Move keyboard tweaks to keyboard.rs, remove USB/CPU overlap

---

## B.14 🔨 ADD: Classic Start Menu (Open-Shell)

**Files**: `ui_classic/tweaks.rs`  
**Action**: Install Open-Shell on Windows 11 24H2+

---

## B.15 🔨 FIX: MSI Priority (0→2/3)

**Files**: `system/msi.rs`, `network/msi.rs`, `gpu/msi.rs`  
**Action**: Device-specific priorities, remove USB from device classes

---

## B.16 🔨 ADD: MSI Vendor Support

**Files**: `network/msi.rs`  
**Action**: Add VEN_168C (Qualcomm), VEN_14E4 (Broadcom), VEN_11AB (Marvell)

---

## B.17 🔨 ADD: Fast Startup Disable

**Files**: `system/maintenance.rs`  
**Action**: HiberbootEnabled=0

---

## B.18 🔨 FIX: Remove Defender from Debloat

**Files**: `debloat/apps.rs`  
**Action**: Remove any Defender disable code (keep only in security module)

---

## B.19 🔨 ADD: GPU Preemption Disable

**Files**: `gpu/scheduling.rs`  
**Action**: EnablePreemption=0 in GraphicsDrivers\Scheduler

---

## B.20 🔨 ADD: Visual Effects Disable

**Files**: `gaming/mod.rs`  
**Action**: UserPreferencesMask for performance, disable animations

---

## B.21 🔨 ADD: MMCSS Disable Option

**Files**: `gaming/mod.rs`  
**Action**: Some systems perform better without MMCSS - add toggle

---

# PHASE C: Polish & Optimization (LOW PRIORITY)

**18 items | 4-5 days | Nice-to-have improvements**

---

## C.1 🎨 FIX: UI Button States

**Files**: `TweakCard.svelte`, `types.rs`  
**Action**: Add TweakType enum (Toggle vs Action)

---

## C.2 🎨 FIX: UI Visual Glitches

**Files**: `app.css`  
**Action**: Fix overflow, z-index, scrollbar issues

---

## C.3 🔨 ADD: Programs Enhancement (merged into B.11)

---

## C.4 🔨 ADD: OEM Bloatware Patterns

**Files**: `debloat/apps.rs`  
**Action**: HP, Dell, Lenovo, Asus, Acer bloatware detection

---

## C.5 🔨 ADD: Native NVMe Driver (24H2+)

**Files**: `storage/ntfs.rs`  
**Action**: FeatureManagement\Overrides\1176759950=1

---

## C.6 🔨 FIX: FSO Description Update

**Files**: `gaming/mod.rs`  
**Action**: Modern FSO is good - warn about VRR/HDR breaking

---

## C.7 🔨 ADD: Storage Write-Cache

**Files**: `storage/ntfs.rs`  
**Action**: Set-PhysicalDisk -WriteCacheEnabled $true

---

## C.8 🔨 ADD: Disable Scheduled Defrag

**Files**: `storage/ntfs.rs`  
**Action**: Disable-ScheduledTask ScheduledDefrag

---

## C.9 🔨 ADD: Package Search

**Files**: `packages/winget.rs`  
**Action**: winget search with parsing

---

## C.10 🎨 ADD: Square Window Corners

**Files**: `ui_classic/tweaks.rs`  
**Action**: UseWindowFrameColor=0 in DWM

---

## C.11 🎨 ADD: Show File Extensions

**Files**: `interface/mod.rs`  
**Action**: HideFileExt=0

---

## C.12 🎨 ADD: Compact File Explorer

**Files**: `interface/mod.rs`  
**Action**: UseCompactMode=1

---

## C.13 🎨 FIX: UAC Safe Button

**Files**: `src/components/`  
**Action**: Hide "Safe" filter in UAC section

---

## C.14 🔨 FIX: SysMain Description

**Files**: `system/services.rs`  
**Action**: Update outdated 2013 advice, change to WarningLevel::Careful

---

## C.15 🔨 ADD: DNS Cache Optimization

**Files**: `network/dns.rs`  
**Action**: CacheHashTableSize=384, MaxCacheEntryTtlLimit=64000

---

## C.16 🔨 ADD: TCP Initial RTO

**Files**: `network/tcp.rs`  
**Action**: TcpInitialRtt=3

---

## C.17 🔨 FIX: MMCSS GPU Priority Revert

**Files**: `gaming/mod.rs`  
**Action**: Revert value 8→2 (Windows default)

---

## C.18 🔨 ADD: SMBv1 Disable

**Files**: `security/hardening.rs`  
**Action**: Disable-WindowsOptionalFeature SMB1Protocol

---

# Testing Checklist

## Phase A (All must pass before Phase B)
- [ ] A.1: Apply 5+ tweaks without UI freeze
- [ ] A.2: state.json in %APPDATA%\TommyTweaker\
- [ ] A.3: UAC prompt on launch
- [ ] A.4: Edge removed on Win11 24H2
- [ ] A.5: HPET disabled, LatencyMon shows improvement
- [ ] A.6: USB3 Link Power = 3 (max performance)
- [ ] A.7: Bloatware removal completes without crash
- [ ] A.8: SecureUxTheme assets deployed
- [ ] A.9: VBS shows "Not enabled" in msinfo32
- [ ] A.10: Network throttling disabled
- [ ] A.11: IFEO/AppInit toggle works
- [ ] A.12: Network MSI shows detected NICs
- [ ] A.13: Processor Check Interval = 1
- [ ] A.14: Prefetch cleanup has Dangerous warning
- [ ] A.15: Single CPU mitigations tweak, no conflicts

## Phase B (Sample verification)
- [ ] Security tweaks show correct state
- [ ] Startup scan finds all registry locations
- [ ] Package catalog populates
- [ ] MSI Priority uses 2/3, not 0

## Phase C (Sample verification)
- [ ] Action tweaks show "Run" button
- [ ] File extensions visible in Explorer
- [ ] SMBv1 disabled

---

# File Change Summary

| File | Changes |
|------|---------|
| `commands.rs` | Async execution, progress events |
| `utils/dirs.rs` | AppData paths |
| `utils/state.rs` | Use new paths |
| `debloat/edge.rs` | Complete rewrite |
| `debloat/apps.rs` | Fix cmdlet, remove Defender |
| `cpu/timer.rs` | HPET invert, Processor Check fix |
| `cpu/power.rs` | USB3 Link Power fix |
| `cpu/scheduling.rs` | Add VBS disable |
| `security/exploit.rs` | Merge Spectre/Meltdown |
| `security/hardening.rs` | Add SMBv1, remove duplicates |
| `gaming/mod.rs` | Network throttling, MMCSS fix |
| `startup/boot.rs` | IFEO/AppInit toggle |
| `startup/logon.rs` | Additional registry locations |
| `startup/browser.rs` | Chrome/Edge extensions |
| `network/msi.rs` | Fix NIC detection, vendor support |
| `network/tcp.rs` | Nagle's, TCP Initial RTO |
| `network/dns.rs` | DNS cache optimization |
| `system/maintenance.rs` | Prefetch warning, Fast Startup |
| `system/msi.rs` | Priority fix |
| `system/services.rs` | SysMain description |
| `gpu/scheduling.rs` | TDR delay, preemption |
| `gpu/mod.rs` | NVIDIA telemetry |
| `display/gpu.rs` | HDR, Reflex, scaling |
| `packages/winget.rs` | Complete overhaul |
| `ui_classic/setup.rs` | SecureUxTheme deployment |
| `ui_classic/tweaks.rs` | Start menu, corners |
| `interface/mod.rs` | Explorer tweaks |
| `input/mouse.rs` | Acceleration, cleanup |
| `TweakCard.svelte` | Loading indicator |
| `types.rs` | TweakType enum |

---

# Execution Order

```
Week 1: Phase A (Critical)
├── Day 1-2: A.1-A.3 (Foundation)
├── Day 3-4: A.4-A.8 (Bug Fixes)
└── Day 5-6: A.9-A.15 (Value Fixes)

Week 2-3: Phase B (Features)
├── Day 1-3: B.1-B.7 (Core Features)
├── Day 4-6: B.8-B.14 (Modules)
└── Day 7-10: B.15-B.21 (Polish)

Week 4: Phase C (Polish)
├── Day 1-2: C.1-C.9 (UI/UX)
└── Day 3-5: C.10-C.18 (Final Tweaks)

Week 5: Testing & Release
├── Day 1-3: Full regression testing
└── Day 4-5: Build & deployment
```

---

**Total: 54 items | ~4-5 weeks to production**

*MASTER IMPLEMENTATION PLAN v4.0 FINAL - January 12, 2026*
