//! Storage Power Management Tweaks
//!
//! NVMe and storage power optimization tweaks moved from CPU module

use crate::modules::types::{TweakType, Tweak, TweakCheck, TweakCategory, TweakOperation, WarningLevel, RegistryValue};

/// Returns all storage power management tweaks
pub fn get_storage_power_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NVMe Idle Timeout
        // ============================================
        Tweak {
            id: "storage_nvme_idle_timeout".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NVMe Idle Timeout".to_string(),
            description: "Sets NVMe idle timeout to 0ms to prevent drive from entering low-power mode.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$out = powercfg /query scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d3d55efd-c1ff-424e-9dc3-441be7833010
if ($out -match "Current AC Power Setting Index: 0x00000000") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# NVMe Idle Timeout - Restore defaults (100ms)
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d3d55efd-c1ff-424e-9dc3-441be7833010 100
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d639518a-e56d-4345-8af2-b9f32fb26109 100
# NVME NOPPME - On
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 fc7372b6-ab2d-43ee-8797-15e9841f2cca 1
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# NVMe Idle Timeout - 0ms
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d3d55efd-c1ff-424e-9dc3-441be7833010 0
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d639518a-e56d-4345-8af2-b9f32fb26109 0
# NVME NOPPME - Off
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 fc7372b6-ab2d-43ee-8797-15e9841f2cca 0
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Storage D3 Modern Standby
        // ============================================
        Tweak {
            id: "storage_d3_standby".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable Storage D3 in Modern Standby".to_string(),
            description: "Prevents storage devices from entering D3 cold state during Modern Standby.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                key: "StorageD3InModernStandby".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Re-enable D3 support for storage
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Storage" -Name "StorageD3InModernStandby" -Value 1 -Type DWord -Force -EA 0
# Reset NVMe idle power mode
$nvmePath = "HKLM:\SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device"
Set-ItemProperty -Path $nvmePath -Name "IdlePowerMode" -Value 1 -Type DWord -Force -EA 0
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# D3 support for storage in Modern Standby
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Storage" -Name "StorageD3InModernStandby" -Value 0 -Type DWord -Force -EA 0

# NVMe idle power mode
$nvmePath = "HKLM:\SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device"
if (!(Test-Path $nvmePath)) { New-Item -Path $nvmePath -Force | Out-Null }
Set-ItemProperty -Path $nvmePath -Name "IdlePowerMode" -Value 0 -Type DWord -Force -EA 0
"#.to_string(),
                }
            ]
        },
    ]
}
