use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_gpu_tweaks() -> Vec<Tweak> {
    vec![tweak_enable_vrr(), tweak_no_gpu_scaling()]
}

fn tweak_enable_vrr() -> Tweak {
    Tweak {
        id: "display_enable_vrr".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "🔄 Enable Variable Refresh Rate (VRR/G-SYNC/FreeSync)".to_string(),
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
        enabled: false,
        check: None,
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
        name: "🎯 Disable GPU Scaling".to_string(),
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
        enabled: false,
        check: None,
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
