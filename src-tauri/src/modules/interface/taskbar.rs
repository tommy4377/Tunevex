//! Taskbar Tweaks Module (Native Windows 11 Tweaks)
//!
//! Contains only native Windows 11 taskbar tweaks.
//! Explorer Patcher functionality has been removed.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // ALIGNMENT - LEFT (Native, works immediately)
        // ============================================
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Left".to_string(),
            description: "Align taskbar icons to the left.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(0),
            }],
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(1),
            }]),
        },

        // ============================================
        // END TASK IN TASKBAR (Windows 11 Developer Feature)
        // ============================================
        Tweak {
            id: "interface_taskbar_end_task".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Enable End Task in Taskbar".to_string(),
            description: "Adds 'End Task' option when right-clicking apps on the taskbar. A Windows 11 developer feature.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings".to_string(),
                key: "TaskbarEndTask".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings".to_string(),
                    key: "TaskbarEndTask".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings".to_string(),
                    key: "TaskbarEndTask".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]),
        },
    ]
}
