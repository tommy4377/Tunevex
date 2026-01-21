use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// System Maintenance & Storage Privacy
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Configure Storage Sense (auto cleanup)
        Tweak {
            id: "priv_config_storage_sense".to_string(),
            category: TweakCategory::Privacy,
            name: "Configure Storage Sense".to_string(),
            description: "Enables Storage Sense to auto-clean temp files monthly without touching Downloads or Recycle Bin.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(),
                    key: "01".to_string(),
                    value: RegistryValue::DWord(0), // Disable Storage Sense
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(),
                key: "01".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
# Enable Storage Sense
Set-ItemProperty -Path $path -Name "01" -Value 1 -Type DWord -Force
# Run Storage Sense
Set-ItemProperty -Path $path -Name "1024" -Value 1 -Type DWord -Force
# Run every 30 days
Set-ItemProperty -Path $path -Name "2048" -Value 30 -Type DWord -Force
# Enable temp file cleanup
Set-ItemProperty -Path $path -Name "04" -Value 1 -Type DWord -Force
# Disable Downloads cleanup
Set-ItemProperty -Path $path -Name "32" -Value 0 -Type DWord -Force
# Disable OneDrive cleanup
Set-ItemProperty -Path $path -Name "02" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "128" -Value 0 -Type DWord -Force
# Disable Recycle Bin cleanup
Set-ItemProperty -Path $path -Name "08" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "256" -Value 0 -Type DWord -Force
Write-Host "Storage Sense configured" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // Disable Reserved Storage (DISM)
        Tweak {
            id: "priv_disable_reserved_storage".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Reserved Storage".to_string(),
            description: "Disables Windows reserved storage for updates (saves ~7GB disk space).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Set-ReservedStorageState /State:Enabled 2>$null
Write-Host "Reserved storage enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((dism /Online /Get-ReservedStorageState) -match "Disabled") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Set-ReservedStorageState /State:Disabled 2>$null
Write-Host "Reserved storage disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // Disable Automatic Maintenance WakeUp
        Tweak {
            id: "priv_disable_maintenance_wakeup".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Maintenance Wake-up".to_string(),
            description: "Prevents Windows from waking your PC for automatic maintenance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                    key: "WakeUp".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                key: "WakeUp".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                    key: "WakeUp".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
    ]
}
