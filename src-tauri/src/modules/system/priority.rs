//! System Priority Tweaks
//!
//! Optimizations for process scheduling, IRQ priority, and game mode settings.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_priority_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Win32PrioritySeparation
        // ============================================
        Tweak {
            id: "sys_priority_separation".to_string(),
            category: TweakCategory::System,
            name: "Optimize Process Scheduling".to_string(),
            description: "Sets Win32PrioritySeparation to 26 (short quantum, foreground priority boost).

This gives foreground applications (games) more responsive scheduling.
Value 26 = Short quantum, variable, high foreground boost.
Default Windows value is typically 2 (long quantum, no boost).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "Win32PrioritySeparation".to_string(),
                expected_value: RegistryValue::DWord(26),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "Win32PrioritySeparation".to_string(),
                value: RegistryValue::DWord(2), // Windows default
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "Win32PrioritySeparation".to_string(),
                value: RegistryValue::DWord(26),
            }],
        },
        // ============================================
        // IRQ8 Priority (Real Time Clock)
        // ============================================
        Tweak {
            id: "sys_irq8_priority".to_string(),
            category: TweakCategory::System,
            name: "Boost RTC IRQ Priority".to_string(),
            description: "Sets IRQ8 (Real Time Clock) to high priority.

May slightly reduce timer latency on older systems.
Modern systems may not see significant improvement.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "IRQ8Priority".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "IRQ8Priority".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\PriorityControl".to_string(),
                key: "IRQ8Priority".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // ============================================
        // Game Mode Priority Settings
        // ============================================
        Tweak {
            id: "sys_game_priority".to_string(),
            category: TweakCategory::System,
            name: "Optimize Game Priority Settings".to_string(),
            description: "Configures Windows Multimedia Class Scheduler Service (MMCSS) for games:
- GPU Priority: 8 (high)
- Priority: 6 (above normal)
- Scheduling Category: High
- SFIO Priority: High

Ensures games get priority access to GPU and CPU resources.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                key: "GPU Priority".to_string(),
                expected_value: RegistryValue::DWord(8),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "GPU Priority".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Priority".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Scheduling Category".to_string(),
                    value: RegistryValue::String("Medium".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "SFIO Priority".to_string(),
                    value: RegistryValue::String("Normal".to_string()),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "GPU Priority".to_string(),
                    value: RegistryValue::DWord(8),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Priority".to_string(),
                    value: RegistryValue::DWord(6),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Scheduling Category".to_string(),
                    value: RegistryValue::String("High".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "SFIO Priority".to_string(),
                    value: RegistryValue::String("High".to_string()),
                },
            ],
        },
        // ============================================
        // System Responsiveness
        // ============================================
        Tweak {
            id: "sys_responsiveness".to_string(),
            category: TweakCategory::System,
            name: "Maximize System Responsiveness".to_string(),
            description: "Sets SystemResponsiveness to 0 (minimum reserved CPU for background tasks).

Default is 20% reserved for background. Setting to 0 gives games more CPU time.
Note: May slightly affect background task performance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "SystemResponsiveness".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "SystemResponsiveness".to_string(),
                value: RegistryValue::DWord(20), // Windows default
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "SystemResponsiveness".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        // ============================================
        // NoLazyMode for MMCSS
        // ============================================
        Tweak {
            id: "sys_no_lazy_mode".to_string(),
            category: TweakCategory::System,
            name: "Disable MMCSS Lazy Mode".to_string(),
            description: "Disables lazy mode in Multimedia Class Scheduler.

When enabled, MMCSS always provides maximum priority to registered tasks.
May slightly increase CPU usage but improves audio/video smoothness.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "NoLazyMode".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "NoLazyMode".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "NoLazyMode".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
    ]
}
