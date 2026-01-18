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
        // 1. STYLE TWEAK - Win10 (EP) vs Win11 (Default)
        // TaskbarStyle: 0=Win11, 1=Win10
        // ============================================
        Tweak {
            id: "ep_config_init".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Windows 10 Style".to_string(),
            description: "Use Windows 10 taskbar style (enables position/size options).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "TaskbarStyle".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1), // 1=Win10
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(0), // 0=Win11
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // ============================================
        // Taskbar Position - Top (EP: PrimaryTaskbarLocation=2)
        // ============================================
        Tweak {
            id: "taskbar_top_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Top".to_string(),
            description: "Move taskbar to top of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "PrimaryTaskbarLocation".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            operations: vec![
                // Enforce Win10 style first
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(2), // 2=Top
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(0), // 0=Bottom
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // Taskbar Position - Left (EP: PrimaryTaskbarLocation=1)
        Tweak {
            id: "taskbar_left_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Left".to_string(),
            description: "Move taskbar to left side of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "PrimaryTaskbarLocation".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(1), // 1=Left
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
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
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // Taskbar Position - Right (EP: PrimaryTaskbarLocation=3)
        Tweak {
            id: "taskbar_right_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Right".to_string(),
            description: "Move taskbar to right side of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "PrimaryTaskbarLocation".to_string(),
                expected_value: RegistryValue::DWord(3),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "PrimaryTaskbarLocation".to_string(),
                    value: RegistryValue::DWord(3), // 3=Right
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
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
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // Taskbar Icon Size - Small (EP: TaskbarIconSize=0 for small)
        Tweak {
            id: "taskbar_small_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Small Icons".to_string(),
            description: "Use small taskbar icons.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "TaskbarIconSize".to_string(),
                expected_value: RegistryValue::DWord(0), // 0=Small
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(0), // 0=Small
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(1), // 1=Large (default)
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // Taskbar Icon Size - Large (EP: TaskbarIconSize=1 for large)
        Tweak {
            id: "taskbar_large_ep".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Large Icons".to_string(),
            description: "Use large taskbar icons (default).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "TaskbarIconSize".to_string(),
                expected_value: RegistryValue::DWord(1), // 1=Large
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(1), // 1=Large
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarIconSize".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },

        // Native: Taskbar Alignment (Left vs Center)
        // TaskbarAl: 0=Left, 1=Center
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Icons Left".to_string(),
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
                value: RegistryValue::DWord(0), // Left
            }],
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(1), // Center
            }]),
        },

        // Native: Never Combine
        // TaskbarGlomLevel: 0=Always, 1=WhenFull, 2=Never
        Tweak {
            id: "taskbar_never_combine".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Never Combine".to_string(),
            description: "Show separate buttons for each window.".to_string(),
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
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(2), // Never
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(0), // Always
                },
                TweakOperation::Powershell {
                    script: r#"Start-Process cmd -ArgumentList '/c','taskkill /f /im explorer.exe >nul 2>&1 & timeout /t 2 /nobreak >nul & start explorer.exe' -WindowStyle Hidden"#.to_string(),
                }
            ]),
        },
    ]
}
