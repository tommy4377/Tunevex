//! Hardware-Specific GPU Tweaks with Auto-Detection
//!
//! Contains Nvidia, AMD, and Intel specific tweaks that are conditionally shown
//! based on detected hardware.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Detect if Nvidia GPU is present
fn is_nvidia_gpu() -> bool {
    if let Ok(output) = std::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", "Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like '*NVIDIA*' } | Select-Object -First 1 -ExpandProperty Name"])
        .output()
    {
        let name = String::from_utf8_lossy(&output.stdout);
        return !name.trim().is_empty();
    }
    false
}

/// Detect if AMD GPU is present
fn is_amd_gpu() -> bool {
    if let Ok(output) = std::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", "Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like '*AMD*' -or $_.Name -like '*Radeon*' } | Select-Object -First 1 -ExpandProperty Name"])
        .output()
    {
        let name = String::from_utf8_lossy(&output.stdout);
        return !name.trim().is_empty();
    }
    false
}

/// Returns hardware-specific GPU tweaks based on detected vendor
pub fn get_vendor_gpu_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // Nvidia-specific tweaks
    if is_nvidia_gpu() {
        tweaks.extend(get_nvidia_tweaks());
    }

    // AMD-specific tweaks
    if is_amd_gpu() {
        tweaks.extend(get_amd_gpu_tweaks());
    }

    tweaks
}

/// Nvidia-specific GPU tweaks
fn get_nvidia_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gpu_nvidia_dynamic_pstate".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "[NVIDIA] Disable Dynamic P-States".to_string(),
            description: "Disables NVIDIA dynamic P-state switching.

Forces the GPU to stay at higher performance states.
May reduce frame time variance in games.
Note: Slightly increases idle power consumption."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\NVTweak".to_string(),
                key: "DisableDynamicPstate".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\NVTweak".to_string(),
                key: "DisableDynamicPstate".to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "HKLM:\SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\NVTweak"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "DisableDynamicPstate" -Value 1 -Type DWord -Force
Write-Host "NVIDIA Dynamic P-States disabled" -ForegroundColor Green
"#
                .to_string(),
            }],
        },
        Tweak {
            id: "gpu_nvidia_clean_cache".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "[NVIDIA] Clear Driver Profile Cache".to_string(),
            description: "Clears NVIDIA driver profile database files.

Can fix performance issues from corrupted profiles.
Profiles will be automatically recreated."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None, // One-time action, no revert
            operations: vec![TweakOperation::Powershell {
                script: r#"
$paths = @(
    "$env:ProgramData\NVIDIA Corporation\Drs\nvdrsdb0.bin",
    "$env:ProgramData\NVIDIA Corporation\Drs\nvdrsdb1.bin",
    "$env:LOCALAPPDATA\NVIDIA\*"
)
$deleted = 0
foreach ($p in $paths) {
    if (Test-Path $p) {
        Remove-Item -Path $p -Force -Recurse -EA 0
        $deleted++
    }
}
if ($deleted -gt 0) {
    Write-Host "NVIDIA cache cleared ($deleted items)" -ForegroundColor Green
} else {
    Write-Host "No NVIDIA cache files found" -ForegroundColor Yellow
}
"#
                .to_string(),
            }],
        },
    ]
}

/// AMD-specific GPU tweaks
fn get_amd_gpu_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gpu_amd_ulps".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "[AMD] Disable Ultra Low Power State".to_string(),
            description: "Disables AMD ULPS (Ultra Low Power State).

Prevents the GPU from entering deep sleep states.
Can fix issues with multi-monitor and CrossFire setups.
May slightly increase idle power consumption.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$key = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000" -Name "EnableUlps" -EA 0
if ($key.EnableUlps -eq 0) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
# Re-enable ULPS on all AMD adapters
Get-ChildItem "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}" | ForEach-Object {
    $props = Get-ItemProperty $_.PSPath -EA 0
    if ($props.DriverDesc -like "*AMD*" -or $props.DriverDesc -like "*Radeon*") {
        Set-ItemProperty -Path $_.PSPath -Name "EnableUlps" -Value 1 -Type DWord -EA 0
    }
}
Write-Host "AMD ULPS re-enabled" -ForegroundColor Green
"#.to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
# Disable ULPS on all AMD adapters
Get-ChildItem "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}" | ForEach-Object {
    $props = Get-ItemProperty $_.PSPath -EA 0
    if ($props.DriverDesc -like "*AMD*" -or $props.DriverDesc -like "*Radeon*") {
        Set-ItemProperty -Path $_.PSPath -Name "EnableUlps" -Value 0 -Type DWord -EA 0
        Write-Host "Disabled ULPS on: $($props.DriverDesc)" -ForegroundColor Green
    }
}
"#.to_string(),
            }],
        },
    ]
}
