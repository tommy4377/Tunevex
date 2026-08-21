use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_gpu_tweaks() -> Vec<Tweak> {
    vec![
        tweak_enable_vrr(),
        tweak_no_gpu_scaling(),
        tweak_toggle_hdr(),
        tweak_nvidia_low_latency(),
    ]
}

fn tweak_enable_vrr() -> Tweak {
    Tweak {
        id: "display_enable_vrr".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Enable Variable Refresh Rate (VRR/G-SYNC/FreeSync)".to_string(),
        description: "Enables VRR to eliminate screen tearing without V-Sync lag. Requires G-SYNC/FreeSync compatible monitor.".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
            Write-Host "Disabling Variable Refresh Rate (VRR)..." -ForegroundColor Cyan

            $path = "HKCU:\Software\Microsoft\DirectX\UserGpuPreferences"
            $currentValue = Get-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -EA 0

            if ($currentValue) {
                $settings = $currentValue.DirectXUserGlobalSettings -replace "VRROptimizeEnable=1", "VRROptimizeEnable=0"
                Set-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -Value $settings -Type String -Force
            }

            $graphicsPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
            Set-ItemProperty -Path $graphicsPath -Name "EnableVRR" -Value 0 -Type DWord -Force -EA 0

            Write-Host "VRR disabled" -ForegroundColor Green
        "#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
$vrr_enabled = $false
# Check 1: Windows Settings
$path1 = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
$val1 = Get-ItemProperty -Path $path1 -Name "EnableVRR" -EA 0
if ($val1.EnableVRR -eq 1) {
    # Check 2: DirectX Global Settings
    $path2 = "HKCU:\Software\Microsoft\DirectX\UserGpuPreferences"
    $val2 = Get-ItemProperty -Path $path2 -Name "DirectXUserGlobalSettings" -EA 0
    if ($val2.DirectXUserGlobalSettings -match "VRROptimizeEnable=1") {
        $vrr_enabled = $true
    }
}
if ($vrr_enabled) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Enabling Variable Refresh Rate (VRR)..." -ForegroundColor Cyan

# Windows 11 VRR registry setting
$path = "HKCU:\Software\Microsoft\DirectX\UserGpuPreferences"
if (!(Test-Path $path)) {
    New-Item -Path $path -Force | Out-Null
}

# Get current value
$currentValue = Get-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -EA 0
$settings = if ($currentValue) { $currentValue.DirectXUserGlobalSettings } else { "" }

# Add VRROptimizeEnable=1 if not present
if ($settings -notlike "*VRROptimizeEnable=1*") {
    if ($settings -like "*VRROptimizeEnable=0*") {
        $settings = $settings -replace "VRROptimizeEnable=0", "VRROptimizeEnable=1"
    } else {
        $settings += "VRROptimizeEnable=1;"
    }

    Set-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -Value $settings -Type String -Force
    Write-Host "VRR enabled in Windows settings" -ForegroundColor Green
} else {
    Write-Host "VRR already enabled" -ForegroundColor Yellow
}

# Enable in Windows 11 Graphics Settings (if available)
$graphicsPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
if (!(Test-Path $graphicsPath)) { New-Item -Path $graphicsPath -Force | Out-Null }
Set-ItemProperty -Path $graphicsPath -Name "EnableVRR" -Value 1 -Type DWord -Force -EA 0

Write-Host "VRR enabled. Check GPU Control Panel for additional settings." -ForegroundColor Cyan
Write-Host "NVIDIA: Control Panel > Display > Set up G-SYNC > Enable G-SYNC" -ForegroundColor Yellow
Write-Host "AMD: Adrenalin > Display > AMD FreeSync > Enable" -ForegroundColor Yellow
        "#.to_string(),
        }],
    }
}

fn tweak_no_gpu_scaling() -> Tweak {
    Tweak {
        id: "display_no_gpu_scaling".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Disable GPU Scaling".to_string(),
        description: "Uses display (monitor) scaling instead of GPU scaling to reduce input latency. Best for native resolution gaming.".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: true,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
            Write-Host "Re-enabling GPU scaling..." -ForegroundColor Cyan

            # NVIDIA
            $nvidiaPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Video\*\0000" -Name "DriverDesc" -EA 0 |
                Where-Object { $_.DriverDesc -like "*NVIDIA*" }

            foreach ($nv in $nvidiaPaths) {
                Remove-ItemProperty -Path $nv.PSPath -Name "NV_GPUScaling" -Force -EA 0
            }

            # AMD
            $amdPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*" -Name "DriverDesc" -EA 0 |
                Where-Object { $_.DriverDesc -like "*AMD*" -or $_.DriverDesc -like "*Radeon*" }

            foreach ($amd in $amdPaths) {
                Remove-ItemProperty -Path $amd.PSPath -Name "EnableGPUScaling" -Force -EA 0
                Remove-ItemProperty -Path $amd.PSPath -Name "GPUScaling" -Force -EA 0
            }

            Write-Host "GPU scaling settings reset to default" -ForegroundColor Green
        "#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
$nv = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Video\*\0000" -Name "NV_GPUScaling" -EA 0 | Where-Object { $_.NV_GPUScaling -eq 0 }
$amd = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*" -Name "EnableGPUScaling" -EA 0 | Where-Object { $_.EnableGPUScaling -eq 0 }
if ($nv -or $amd) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Disabling GPU scaling..." -ForegroundColor Cyan

$changed = $false

# NVIDIA: Disable GPU scaling
$nvidiaPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Video\*\0000" -Name "DriverDesc" -EA 0 |
    Where-Object { $_.DriverDesc -like "*NVIDIA*" }

foreach ($nv in $nvidiaPaths) {
    $nvPath = $nv.PSPath
    Set-ItemProperty -Path $nvPath -Name "NV_GPUScaling" -Value 0 -Type DWord -Force -EA 0
    Write-Host "NVIDIA GPU scaling disabled" -ForegroundColor Green
    $changed = $true
}

# AMD: Disable GPU scaling
$amdPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*" -Name "DriverDesc" -EA 0 |
    Where-Object { $_.DriverDesc -like "*AMD*" -or $_.DriverDesc -like "*Radeon*" }

foreach ($amd in $amdPaths) {
    $amdPath = $amd.PSPath
    Set-ItemProperty -Path $amdPath -Name "EnableGPUScaling" -Value 0 -Type DWord -Force -EA 0
    Set-ItemProperty -Path $amdPath -Name "GPUScaling" -Value 0 -Type DWord -Force -EA 0
    Write-Host "AMD GPU scaling disabled" -ForegroundColor Green
    $changed = $true
}

if (!$changed) {
    Write-Host "No NVIDIA/AMD GPU detected or registry paths not found" -ForegroundColor Yellow
}

Write-Host "" -ForegroundColor Yellow
Write-Host "For best results, also configure in GPU Control Panel:" -ForegroundColor Yellow
Write-Host "NVIDIA: Control Panel > Adjust desktop size and position > Perform scaling on: Display" -ForegroundColor Cyan
Write-Host "AMD: Adrenalin > Display > GPU Scaling: Off" -ForegroundColor Cyan
        "#.to_string(),
        }],
    }
}

// B.12: HDR Toggle
fn tweak_toggle_hdr() -> Tweak {
    Tweak {
        id: "display_enable_hdr".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Enable HDR (High Dynamic Range)".to_string(),
        description: "Enables HDR for supported monitors. Provides better colors and contrast in games and media.

Requires:
- HDR-capable monitor
- HDR-capable GPU
- DisplayPort 1.4+ or HDMI 2.0a+ cable".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Disabling HDR..." -ForegroundColor Cyan

# Method 1: Settings app toggle via DisplayConfigSetDeviceInfo (requires restart of apps)
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\VideoSettings"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "EnableHDR" -Value 0 -Type DWord -Force

# Method 2: Per-display HDR settings
$displayPath = "HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Configuration"
Get-ChildItem $displayPath -Recurse -EA 0 | ForEach-Object {
    Set-ItemProperty -Path $_.PSPath -Name "SetToHDR" -Value 0 -EA 0
    Set-ItemProperty -Path $_.PSPath -Name "AdvancedColorEnabled" -Value 0 -EA 0
}

Write-Host "HDR disabled. Some apps may need restart." -ForegroundColor Green
"#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Registry {
            root_key: "HKCU".to_string(),
            path: "Software\\Microsoft\\Windows\\CurrentVersion\\VideoSettings".to_string(),
            key: "EnableHDR".to_string(),
            expected_value: RegistryValue::DWord(1),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Enabling HDR (High Dynamic Range)..." -ForegroundColor Cyan

# Check if HDR is supported
$hdrSupported = $false
$displays = Get-WmiObject -Namespace root\wmi -Class WmiMonitorBasicDisplayParams -EA 0
if ($displays) {
    Write-Host "Monitor(s) detected: $($displays.Count)" -ForegroundColor Cyan
    $hdrSupported = $true
}

if (!$hdrSupported) {
    Write-Host "Warning: Could not verify HDR support. Proceeding anyway..." -ForegroundColor Yellow
}

# Method 1: User settings
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\VideoSettings"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "EnableHDR" -Value 1 -Type DWord -Force

# Method 2: System-wide HDR
$displayPath = "HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Configuration"
Get-ChildItem $displayPath -Recurse -EA 0 | ForEach-Object {
    Set-ItemProperty -Path $_.PSPath -Name "SetToHDR" -Value 1 -EA 0
    Set-ItemProperty -Path $_.PSPath -Name "AdvancedColorEnabled" -Value 1 -EA 0
}

Write-Host "HDR enabled!" -ForegroundColor Green
Write-Host "Note: Some games need 'Use HDR' enabled in their settings." -ForegroundColor Yellow
Write-Host "Windows Settings > System > Display > HDR to verify" -ForegroundColor Cyan
"#.to_string(),
        }],
    }
}

// B.12: NVIDIA Low Latency Mode (Reflex)
fn tweak_nvidia_low_latency() -> Tweak {
    Tweak {
        id: "display_nvidia_low_latency".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "NVIDIA Low Latency Mode (Ultra)".to_string(),
        description: "Enables NVIDIA Ultra Low Latency Mode globally for reduced input lag.

Equivalent to setting 'Low Latency Mode' to 'Ultra' in NVIDIA Control Panel.

Best for competitive gaming. May slightly reduce FPS.

Requires NVIDIA GPU.".to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Disabling NVIDIA Low Latency Mode..." -ForegroundColor Cyan

$nvidiaPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*" -Name "DriverDesc" -EA 0 |
    Where-Object { $_.DriverDesc -like "*NVIDIA*" }

$changed = $false
foreach ($nv in $nvidiaPaths) {
    Remove-ItemProperty -Path $nv.PSPath -Name "LowLatencyMode" -Force -EA 0
    $changed = $true
}

# Also remove from NVIDIA profile
$nvcplPath = "HKCU:\Software\NVIDIA Corporation\Global\NVTweak"
Remove-ItemProperty -Path $nvcplPath -Name "LowLatencyMode" -EA 0

if ($changed) {
    Write-Host "Low Latency Mode disabled (reverted to Application Controlled)" -ForegroundColor Green
} else {
    Write-Host "No NVIDIA GPU found" -ForegroundColor Yellow
}
"#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
$val = Get-ItemProperty -Path "HKCU:\Software\NVIDIA Corporation\Global\NVTweak" -Name "LowLatencyMode" -EA 0
if ($val.LowLatencyMode -eq 2) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Enabling NVIDIA Ultra Low Latency Mode..." -ForegroundColor Cyan

$nvidiaPaths = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*" -Name "DriverDesc" -EA 0 |
    Where-Object { $_.DriverDesc -like "*NVIDIA*" }

$changed = $false
foreach ($nv in $nvidiaPaths) {
    # LowLatencyMode: 0=Off, 1=On, 2=Ultra
    Set-ItemProperty -Path $nv.PSPath -Name "LowLatencyMode" -Value 2 -Type DWord -Force -EA 0
    Write-Host "Set Ultra Low Latency on: $($nv.DriverDesc)" -ForegroundColor Green
    $changed = $true
}

# Also set in NVIDIA profile
$nvcplPath = "HKCU:\Software\NVIDIA Corporation\Global\NVTweak"
if (!(Test-Path $nvcplPath)) { New-Item -Path $nvcplPath -Force | Out-Null }
Set-ItemProperty -Path $nvcplPath -Name "LowLatencyMode" -Value 2 -Type DWord -Force

if ($changed) {
    Write-Host "" -ForegroundColor Yellow
    Write-Host "Ultra Low Latency Mode enabled!" -ForegroundColor Green
    Write-Host "For games with NVIDIA Reflex support, enable Reflex in-game for best results." -ForegroundColor Cyan
} else {
    Write-Host "No NVIDIA GPU detected. This tweak only works with NVIDIA GPUs." -ForegroundColor Red
}
"#.to_string(),
        }],
    }
}
