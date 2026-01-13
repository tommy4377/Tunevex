//! Interface & UX Tweaks Module

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_interface_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // C.11: Show File Extensions
        // ============================================
        Tweak {
            id: "interface_show_file_extensions".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📄 Show File Extensions".to_string(),
            description:
                "Shows file extensions in File Explorer (e.g., document.docx instead of document).

Security benefit: Prevents malware from disguising .exe files as documents."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(1), // Hidden (default)
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(0), // Show extensions
            }],
        },
        // ============================================
        // C.12: Compact File Explorer View
        // ============================================
        Tweak {
            id: "interface_compact_mode".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📐 Enable Compact File Explorer".to_string(),
            description: "Enables compact mode in File Explorer for more items per page.

Reduces padding between items to fit more files on screen.
Useful for high-resolution displays or power users."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                expected_value: RegistryValue::DWord(1), // Compact mode
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                value: RegistryValue::DWord(0), // Standard mode
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                value: RegistryValue::DWord(1), // Compact mode
            }],
        },
        // ============================================
        // Show Hidden Files
        // ============================================
        Tweak {
            id: "interface_show_hidden_files".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "👁️ Show Hidden Files".to_string(),
            description: "Shows hidden files and folders in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                expected_value: RegistryValue::DWord(1), // Show hidden files
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(2), // Don't show hidden
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(1), // Show hidden files
            }],
        },
        // ============================================
        // Show System Files
        // ============================================
        Tweak {
            id: "interface_show_system_files".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "⚙️ Show Protected System Files".to_string(),
            description: "Shows protected operating system files in File Explorer.

WARNING: Be careful not to modify or delete system files!"
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                expected_value: RegistryValue::DWord(1), // Show system files
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(0), // Hide system files (default)
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(1), // Show system files
            }],
        },

        // ============================================
        // Classic Context Menu (Windows 11)
        // ============================================
        Tweak {
            id: "interface_classic_context_menu".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🖱️ Restore Classic Context Menu".to_string(),
            description: "Restores the Windows 10 style full context menu on right-click, removing the 'Show more options' delay.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true, // Explorer restart usually needed
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32".to_string(),
                key: "".to_string(), // Default value
                expected_value: RegistryValue::String("".to_string()),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}".to_string(),
                    key: "InprocServer32".to_string(), // Deleting key ideally calls for generic key delete logic or delete subkey?
                    // RegistryDelete usually deletes a VALUE. To delete a KEY, we might need PowerShell or extended logic.
                    // Let's use PowerShell for revert to be safe for deleting the whole key.
                },
                // Actually, let's look at RegistryDelete definition: it takes a `key`.
                // If I want to delete the KEY `{...}`, I should probably use PowerShell.
                TweakOperation::Powershell {
                    script: r#"
Remove-Item -Path "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}" -Recurse -Force -ErrorAction SilentlyContinue
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32".to_string(),
                    key: "".to_string(), // Default value
                    value: RegistryValue::String("".to_string()),
                },
            ],
        },

        // ============================================
        // End Task in Taskbar
        // ============================================
        Tweak {
            id: "interface_end_task_taskbar".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🛑 Enable 'End Task' in Taskbar".to_string(),
            description: "Adds an 'End Task' option when right-clicking apps in the taskbar. Useful for frozen apps.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                key: "TaskbarEndTask".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "TaskbarEndTask".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "TaskbarEndTask".to_string(),
                },
            ],
        },
        // ============================================
        // Classic Taskbar (Win10 Style)
        // ============================================
        Tweak {
            id: "interface_taskbar_classic".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🎯 Classic Taskbar (Win10 Style)".to_string(),
            description: "Enable task labels, small icons, and never combine buttons like Windows 10.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarGlomLevel".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 1 -Type DWord -Force
Remove-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -ErrorAction SilentlyContinue
Remove-ItemProperty -Path $advPath -Name "TaskbarSi" -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue; Start-Sleep 2; Start-Process "explorer.exe"
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $advPath -Name "TaskbarSi" -Value 0 -Type DWord -Force
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue; Start-Sleep 2; Start-Process "explorer.exe"
"#.to_string(),
                }
            ],
        },

        // ============================================
        // Taskbar on Top
        // ============================================
        Tweak {
            id: "interface_taskbar_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📍 Taskbar on Top".to_string(),
            description: "Move taskbar to the top of the screen. Requires explorer restart.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings -ErrorAction SilentlyContinue).Settings
if ($data -and $data[12] -eq 0x01) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x03
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue; Start-Sleep 2; Start-Process "explorer.exe"
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x01
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue; Start-Sleep 2; Start-Process "explorer.exe"
"#.to_string(),
                }
            ],
        },

        // ============================================
        // Center Taskbar Icons
        // ============================================
        Tweak {
            id: "interface_taskbar_center".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "⚖️ Center Taskbar Icons".to_string(),
            description: "Center taskbar icons (Windows 11 style).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarAl".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarAl".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // ============================================
        // Full Dark Mode
        // ============================================
        Tweak {
            id: "interface_dark_mode".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🌙 Full Dark Mode".to_string(),
            description: "Enable dark mode everywhere: apps, system, and dialogs.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "AppsUseLightTheme".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 1 -Type DWord -Force
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 0 -Type DWord -Force
"#.to_string(),
                }
            ],
        },
    ]
}
