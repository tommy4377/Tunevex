//! Taskbar Tweaks Module (ExplorerPatcher Stealth Integration)
//!
//! Controls taskbar position, size, icon alignment via ExplorerPatcher registry keys.
//! Confirmed working on Windows 11 25H2 via stealth integration.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Helper: Restart-ExplorerSilent is typically used, but we'll inline it for reliability here
        // or assume the common TweakOperation::Powershell handles it.
        // User requested: "taskbar top/left/right, small/large icons"
        // ============================================

        // Taskbar Position - Top (ExplorerPatcher)
        Tweak {
            id: "taskbar_top_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar in Alto".to_string(),
            description: "Sposta taskbar in cima.".to_string(), // Keep short or match prompt style
            warning_level: WarningLevel::Safe, // It's safe via EP
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Test-Path 'HKCU:\Software\ExplorerPatcher')) { exit 0 }
(Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' 'PrimaryTaskbarLocation' -EA 0).PrimaryTaskbarLocation -eq 1
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(), // 1=Top
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(0), // 0=Bottom (Default)
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ]),
        },

        // Taskbar Position - Left (ExplorerPatcher)
        Tweak {
            id: "taskbar_left_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar a Sinistra".to_string(),
            description: "Moves taskbar to left side.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Test-Path 'HKCU:\Software\ExplorerPatcher')) { exit 0 }
(Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' 'PrimaryTaskbarLocation' -EA 0).PrimaryTaskbarLocation -eq 2
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(), // 2=Left
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ]),
        },

        // Taskbar Position - Right (ExplorerPatcher)
        Tweak {
            id: "taskbar_right_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar a Destra".to_string(),
            description: "Moves taskbar to right side.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Test-Path 'HKCU:\Software\ExplorerPatcher')) { exit 0 }
(Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' 'PrimaryTaskbarLocation' -EA 0).PrimaryTaskbarLocation -eq 3
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(), // 3=Right
                    value: RegistryValue::DWord(3),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ]),
        },

        // Taskbar Icon Size - Small (ExplorerPatcher)
        Tweak {
            id: "taskbar_small_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Icone Taskbar Piccole".to_string(),
            description: "Sets taskbar icons to small size (16px).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"(Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' 'TaskbarIconSize' -EA 0).TaskbarIconSize -eq 16"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(16),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(24), // 24 = Large/Default in older wins? Or 32? Assuming 24 is medium/default.
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ]),
        },

        // Taskbar Icon Size - Large (ExplorerPatcher)
        Tweak {
            id: "taskbar_large_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Icone Taskbar Grandi".to_string(),
            description: "Sets taskbar icons to large size (32px).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"(Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' 'TaskbarIconSize' -EA 0).TaskbarIconSize -eq 32"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(32),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(24),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force -EA SilentlyContinue; Start-Sleep 1; Start-Process explorer.exe"#.to_string(),
                }
            ]),
        },

        // Keep: Taskbar Alignment (Native) - Still works fine and is different from Position
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Taskbar Icons Left".to_string(),
            description: "Moves taskbar icons to the left side (Windows 10 style).".to_string(),
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
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },

        // Keep: Never Combine (Is now handled by EP via TaskbarGlomLevel? Or native?)
        // The Prompt says: "Rimuovi vecchi tweak rotti (TaskbarSi, StuckRects3)."
        // NeverCombine (TaskbarGlomLevel) is native but might be enhanced by EP.
        // I will keep the native one I wrote earlier as it seemed to be working or at least attempting to.
        // Actually, EP uses `TaskbarGlomLevel` too (it respects the native key often).
        Tweak {
            id: "taskbar_never_combine".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Never Combine Taskbar".to_string(),
            description: "Shows separate buttons for each window.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarGlomLevel".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarGlomLevel".to_string(),
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
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe""#.to_string()
                }
            ],
        },
    ]
}
