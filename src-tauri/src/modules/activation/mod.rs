//! Windows Activation Module
//!
//! Integration with Microsoft Activation Scripts (MAS) for Windows/Office activation.
//! Refactored for separate Windows/Office management.

use crate::modules::types::{Tweak, TweakCategory, TweakOperation, TweakType, WarningLevel};

/// Returns all Activation tweaks
pub fn get_activation_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // WINDOWS SECTION
        // ============================================

        // 1. Check Windows Status
        Tweak {
            id: "activation_check_windows".to_string(),
            category: TweakCategory::Activation,
            name: "Check Windows Status".to_string(),
            description: "instantly checks Windows activation status via WMI.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
function Get-LicenseStatus($code) {
    switch ($code) {
        0 { "Unlicensed" }
        1 { "Licensed" }
        2 { "OOBE Grace" }
        3 { "Token Store Grace" }
        4 { "Non-Genuine Grace" }
        5 { "Notification" }
        6 { "Extended Grace" }
        default { "Unknown ($code)" }
    }
}

Write-Host "=== WINDOWS STATUS ===" -ForegroundColor Cyan
try {
    $licenses = Get-CimInstance SoftwareLicensingProduct -Filter "PartialProductKey IS NOT NULL" -ErrorAction Stop | Where-Object { $_.Name -like "*Windows*" }
    if ($licenses) {
        foreach ($l in $licenses) {
            $status = Get-LicenseStatus $l.LicenseStatus
            $color = if ($status -eq "Licensed") { "Green" } else { "Red" }
            
            Write-Host "[$($l.Name)]" -ForegroundColor White
            Write-Host "  Status: " -NoNewline; Write-Host $status -ForegroundColor $color
            Write-Host "  Key (Partial): $($l.PartialProductKey)"
            Write-Host "  Description: $($l.Description)"
            Write-Host ""
        }
    } else {
        Write-Host "No active Windows license found." -ForegroundColor Red
    }
} catch {
    Write-Host "WMI Check failed, falling back to legacy slmgr..." -ForegroundColor Red
    cscript //nologo "$env:SystemRoot\System32\slmgr.vbs" /dli
}
"#.to_string(),
            }],
        },

        // 2. Activate Windows (HWID)
        Tweak {
            id: "activation_hwid".to_string(),
            category: TweakCategory::Activation,
            name: "Activate Windows (HWID)".to_string(),
            description: "Permanent digital license. Survives reinstalls. Personal use only.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = "Stop"
$url = "https://raw.githubusercontent.com/massgravel/Microsoft-Activation-Scripts/refs/heads/master/MAS/All-In-One-Version-KL/MAS_AIO.cmd"
$path = "$env:LOCALAPPDATA\TommyTweaker\mas\mas_aio.cmd"

# Force re-download: delete stale cache before downloading
if (Test-Path $path) {
    Remove-Item $path -Force
}
Write-Host "Downloading latest MAS script..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest -Uri $url -OutFile $path
Write-Host "Download complete." -ForegroundColor Green

Write-Host "Starting MAS (HWID)..." -ForegroundColor Green
$process = Start-Process -FilePath "cmd.exe" -ArgumentList "/c `"$path`" /HWID" -PassThru -NoNewWindow -Wait
if ($process.ExitCode -eq 0) {
    Write-Host "Activation process completed." -ForegroundColor Green
} else {
    Write-Host "Activation process failed with code $($process.ExitCode)" -ForegroundColor Red
}
"#.to_string(),
            }],
        },

        // 3. Deactivate Windows (Remove Key)
        Tweak {
            id: "activation_remove_windows".to_string(),
            category: TweakCategory::Activation,
            name: "Deactivate Windows".to_string(),
            description: "Removes validation key and uninstalls product key.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
Write-Host "Deactivating Windows..." -ForegroundColor Yellow
cscript //nologo "$env:SystemRoot\System32\slmgr.vbs" /upk
cscript //nologo "$env:SystemRoot\System32\slmgr.vbs" /cpky
cscript //nologo "$env:SystemRoot\System32\slmgr.vbs" /rearm
Write-Host "Done. Restart required." -ForegroundColor Green
"#.to_string(),
            }],
        },

        // ============================================
        // OFFICE SECTION
        // ============================================

        // 4. Check Office Status
        Tweak {
            id: "activation_check_office".to_string(),
            category: TweakCategory::Activation,
            name: "Check Office Status".to_string(),
            description: "Checks Office activation status.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
Write-Host "=== OFFICE STATUS ===" -ForegroundColor Cyan
$officePaths = @(
    "${env:ProgramFiles}\Microsoft Office\Office16\ospp.vbs",
    "${env:ProgramFiles(x86)}\Microsoft Office\Office16\ospp.vbs"
)
$found = $false
foreach ($path in $officePaths) {
    if (Test-Path $path) {
        $found = $true
        $output = cscript //nologo $path /dstatus 2>&1
        $output | ForEach-Object {
            if ($_ -match "LICENSE STATUS:  ---LICENSED---") { Write-Host $_ -ForegroundColor Green }
            elseif ($_ -match "LICENSE STATUS:") { Write-Host $_ -ForegroundColor Red }
            elseif ($_ -match "PRODUCT ID:" -or $_ -match "SKU ID:") { }
            else { Write-Host $_ }
        }
        break
    }
}
if (-not $found) { Write-Host "Office not installed." -ForegroundColor Gray }
"#.to_string(),
            }],
        },

        // 5. Activate Office (Ohook)
        Tweak {
            id: "activation_ohook".to_string(),
            category: TweakCategory::Activation,
            name: "Activate Office (Ohook)".to_string(),
            description: "Permanent Office activation. Offline. Personal use only.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = "Stop"
$url = "https://raw.githubusercontent.com/massgravel/Microsoft-Activation-Scripts/refs/heads/master/MAS/All-In-One-Version-KL/MAS_AIO.cmd"
$path = "$env:LOCALAPPDATA\TommyTweaker\mas\mas_ohook.cmd"

# Force re-download: delete stale cache before downloading
if (Test-Path $path) {
    Remove-Item $path -Force
}
Write-Host "Downloading latest MAS script..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest -Uri $url -OutFile $path
Write-Host "Download complete." -ForegroundColor Green

Write-Host "Starting MAS (Ohook)..." -ForegroundColor Green
$process = Start-Process -FilePath "cmd.exe" -ArgumentList "/c `"$path`" /Ohook" -PassThru -NoNewWindow -Wait
if ($process.ExitCode -eq 0) {
    Write-Host "Activation process completed." -ForegroundColor Green
} else {
    Write-Host "Activation process failed with code $($process.ExitCode)" -ForegroundColor Red
}
"#.to_string(),
            }],
        },

        // 6. Deactivate Office
        Tweak {
            id: "activation_remove_office".to_string(),
            category: TweakCategory::Activation,
            name: "Deactivate Office".to_string(),
            description: "Removes Ohook and Office activation.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "$env:LOCALAPPDATA\TommyTweaker\mas\mas_aio.cmd"
if (!(Test-Path $path)) {
   Write-Host "MAS script not found. Please run 'Activate Office' check first to download it properly." -ForegroundColor Red
   exit 1
}
Write-Host "Removing Office Activation..." -ForegroundColor Yellow
$process = Start-Process -FilePath "cmd.exe" -ArgumentList "/c `"$path`" /Ohook /Uninstall" -PassThru -NoNewWindow -Wait
Write-Host "Deactivation completed." -ForegroundColor Green
"#.to_string(),
            }],
        },
    ]
}
