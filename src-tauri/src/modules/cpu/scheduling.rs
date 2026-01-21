//! CPU Scheduling & Process Priority Tweaks
//!
//! Based on:
//! - win32-priority-separation.yml
//! - config-mmcss.yml
//! - disable-fth.yml
//! - disable-service-host-split.yml

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns all CPU scheduling and priority tweaks
pub fn get_scheduling_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // From: win32-priority-separation.yml
        // ============================================
        Tweak {
            id: "cpu_win32_priority".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Prioritize Foreground Applications".to_string(),
            description: "Sets Win32PrioritySeparation to 0x26 (38 decimal) for short quantum, variable, high foreground boost. Essential for gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                    key: "Win32PrioritySeparation".to_string(),
                    value: RegistryValue::DWord(2), // Default
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "Win32PrioritySeparation".to_string(),
                expected_value: RegistryValue::DWord(38),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                    key: "Win32PrioritySeparation".to_string(),
                    value: RegistryValue::DWord(38), // 0x26
                }
            ]
        },
        

        
        // ============================================
        // From: disable-fth.yml
        // ============================================
        Tweak {
            id: "cpu_disable_fth".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Fault Tolerant Heap".to_string(),
            description: "Disables FTH which applies mitigations to crashing apps but causes performance hits.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\FTH".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(1), // Enable FTH
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\FTH".to_string(),
                key: "Enabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\FTH".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]
        },
        
        // ============================================
        // Disable Power Throttling
        // ============================================
        Tweak {
            id: "cpu_disable_power_throttling".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Power Throttling".to_string(),
            description: "Prevents Windows from throttling CPU performance to save power.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling".to_string(),
                    key: "PowerThrottlingOff".to_string(),
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling".to_string(),
                key: "PowerThrottlingOff".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling".to_string(),
                    key: "PowerThrottlingOff".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        
        // ============================================
        // From: disable-service-host-split.yml
        // ============================================
        Tweak {
            id: "cpu_disable_svchost_split".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Service Host Splitting".to_string(),
            description: "Combines services into fewer svchost processes for lower RAM usage. Excludes Xbox services.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services' |
    Where-Object { $_.Name -notmatch 'Xbl|Xbox' } |
    ForEach-Object {
        Remove-ItemProperty -Path "Registry::$_" -Name 'SvcHostSplitDisable' -ErrorAction SilentlyContinue
    }
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$svc = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation" -Name "SvcHostSplitDisable" -EA 0
if ($svc.SvcHostSplitDisable -eq 1) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services' |
    Where-Object { $_.Name -notmatch 'Xbl|Xbox' } |
    ForEach-Object {
        if ($null -ne (Get-ItemProperty -Path "Registry::$_" -EA 0).Start) {
            Set-ItemProperty -Path "Registry::$_" -Name 'SvcHostSplitDisable' -Type DWORD -Value 1 -Force -EA 0
        }
    }
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // NEW: From disable-paging.yml - Disable Page Combining
        // ============================================
        Tweak {
            id: "cpu_disable_page_combining".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Page Combining".to_string(),
            description: "Disables memory page combining for improved stability and reduced CPU overhead.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                    key: "DisablePageCombining".to_string(),
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "DisablePageCombining".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                    key: "DisablePageCombining".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        
        // ============================================
        // NEW: Disable Sleep Study Diagnostics
        // ============================================
        Tweak {
            id: "cpu_disable_sleep_study".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Sleep Study Diagnostics".to_string(),
            description: "Disables Windows sleep study and power diagnostics to reduce background overhead.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                    key: "SleepStudyDisabled".to_string(),
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                key: "SleepStudyDisabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                    key: "SleepStudyDisabled".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        
        // ============================================
        // NEW: Disable VBS (Virtualization Based Security)
        // ============================================
        Tweak {
            id: "cpu_disable_vbs".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable VBS (Virtualization Based Security)".to_string(),
            description: "Disables VBS and Memory Integrity. Provides 5-15% FPS boost. Reduces security - only for gaming systems.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
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
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "hypervisorlaunchtype".to_string(), "off".to_string()],
                },
            ],
        },
    ]
}
