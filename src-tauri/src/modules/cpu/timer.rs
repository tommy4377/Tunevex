//! Timer Resolution and Boot Configuration Tweaks
//!
//! Based on: bcdedit-tweaks.yml, DisablePowerSaving.ps1
//!
//! All checks use TweakCheck::CommandOutputContains (bcdedit /enum {current})
//! or TweakCheck::Registry — zero TweakCheck::Powershell remain.
//! All operations use TweakOperation::Command (bcdedit / powercfg / pnputil)
//! or TweakOperation::RegistrySet — zero TweakOperation::Powershell remain.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_timer_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Global Timer Resolution Requests (registry only)
        // ============================================
        Tweak {
            id: "cpu_timer_resolution".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Global Timer Resolution Requests".to_string(),
            description: "Allows apps to request higher timer resolution (0.5ms) for smoother frametimes.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                    key: "GlobalTimerResolutionRequests".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                key: "GlobalTimerResolutionRequests".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                    key: "GlobalTimerResolutionRequests".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // ============================================
        // Enhanced TSC Synchronization
        // ============================================
        Tweak {
            id: "cpu_tsc_sync".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Enhanced TSC Synchronization".to_string(),
            description: "Forces enhanced TSC sync across CPU cores for better timing accuracy.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "tscsyncpolicy".to_string()],
                },
            ]),
            // bcdedit /enum {current} output contains "tscsyncpolicy    Enhanced"
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{current}".to_string()],
                contains: "Enhanced".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/set".to_string(),
                        "tscsyncpolicy".to_string(),
                        "Enhanced".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Disable Dynamic Tick
        // ============================================
        Tweak {
            id: "cpu_disable_dynamic_tick".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Dynamic Tick".to_string(),
            description: "Forces constant timer interrupts for consistent performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/deletevalue".to_string(),
                        "disabledynamictick".to_string(),
                    ],
                },
            ]),
            // bcdedit /enum {current} contains "disabledynamictick    Yes"
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{current}".to_string()],
                contains: "disabledynamictick".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/set".to_string(),
                        "disabledynamictick".to_string(),
                        "yes".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Disable HPET for Lower Latency
        // ============================================
        Tweak {
            id: "cpu_disable_hpet".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable HPET for Lower Latency".to_string(),
            description: "Disables High Precision Event Timer. Modern TSC is faster. Can improve FPS by 10-20% in games.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Check: useplatformclock is NOT set in BCD (key absent = HPET not forced on)
            // When useplatformclock is absent the bcdedit enum output will NOT contain the word.
            // We invert: applied means the key is absent → use RegistryKeyAbsent on BCD store
            // is impractical; instead we check whether the pnputil HPET device status shows
            // it is disabled by reading its ConfigFlags registry value.
            // ConfigFlags bit 0x1 = disabled; default (enabled) has ConfigFlags absent or 0.
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                // Standard HPET instance path under PnP Enum
                path: "SYSTEM\\CurrentControlSet\\Enum\\ACPI\\PNP0103\\0".to_string(),
                key: "ConfigFlags".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                // Force HPET clock back on in BCD
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/set".to_string(),
                        "useplatformclock".to_string(),
                        "true".to_string(),
                    ],
                },
                // Re-enable HPET in Device Manager via pnputil
                TweakOperation::Command {
                    cmd: "pnputil".to_string(),
                    args: vec![
                        "/enable-device".to_string(),
                        "ACPI\\PNP0103".to_string(),
                    ],
                },
            ]),
            operations: vec![
                // Remove useplatformclock (stops forcing HPET)
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/deletevalue".to_string(),
                        "useplatformclock".to_string(),
                    ],
                },
                // Disable HPET device in Device Manager via pnputil
                TweakOperation::Command {
                    cmd: "pnputil".to_string(),
                    args: vec![
                        "/disable-device".to_string(),
                        "ACPI\\PNP0103".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Legacy Boot Menu
        // ============================================
        Tweak {
            id: "cpu_legacy_boot_menu".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Use Legacy Boot Menu".to_string(),
            description: "Sets legacy boot menu policy for faster boot times.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/set".to_string(),
                        "bootmenupolicy".to_string(),
                        "standard".to_string(),
                    ],
                },
            ]),
            // bcdedit /enum {current} contains "bootmenupolicy    Legacy"
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{current}".to_string()],
                contains: "Legacy".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec![
                        "/set".to_string(),
                        "bootmenupolicy".to_string(),
                        "legacy".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Processor Check Interval
        // ============================================
        Tweak {
            id: "cpu_processor_check_interval".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Optimize Processor Check Interval".to_string(),
            description: "Sets processor performance check interval to 1 (minimum). Reduces latency by checking CPU state more frequently. Windows default is 15.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Check: powercfg /q output contains AC index == 0x00000001
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "scheme_current".to_string(),
                    "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                    "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                ],
                contains: "0x00000001".to_string(),
            }),
            revert_operations: Some(vec![
                // Restore Windows default (15)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                        "15".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                        "15".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ],
        },
    ]
}
