//! Explorer Tweaks Module

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_explorer_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "interface_show_file_extensions".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Show File Extensions".to_string(),
            description: "Shows file extensions in File Explorer (e.g. .exe, .txt).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "HideFileExt".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        Tweak {
            id: "interface_compact_mode".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Enable Compact Mode".to_string(),
            description: "Reduces padding between items in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "UseCompactMode".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "UseCompactMode".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe""#.to_string()
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "UseCompactMode".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe""#.to_string()
                }
            ],
        },
        Tweak {
            id: "interface_show_hidden_files".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Show Hidden Files".to_string(),
            description: "Shows hidden files and folders in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "Hidden".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(2), // 2 = Don't show
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(1), // 1 = Show
            }],
        },
        Tweak {
            id: "interface_show_system_files".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Show System Files".to_string(),
            description:
                "Shows protected operating system files. WARNING: Modification can break Windows."
                    .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "ShowSuperHidden".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
    ]
}
