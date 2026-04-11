//! Context Menu Tweaks Module

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_context_menu_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Classic Context Menu (Windows 11)
        // ============================================
        Tweak {
            id: "interface_classic_context_menu".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Restore Classic Context Menu".to_string(),
            description: "Restores the Windows 10 style full context menu on right-click.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32".to_string(), key: "".to_string() },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "taskkill".to_string(), "/F".to_string(), "/IM".to_string(), "explorer.exe".to_string(), "&&".to_string(), "start".to_string(), "explorer.exe".to_string()] }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("".to_string()),
                },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "taskkill".to_string(), "/F".to_string(), "/IM".to_string(), "explorer.exe".to_string(), "&&".to_string(), "start".to_string(), "explorer.exe".to_string()] },
            ],
        },

        // ============================================
        // Take Ownership
        // ============================================
        Tweak {
            id: "interface_take_ownership".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Add 'Take Ownership' to Menu".to_string(),
            description: "Adds a context menu option to specific files and folders to take ownership. Uses a safe method that doesn't conflict with 'Run as administrator'.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCR".to_string(),
                path: r"*\shell\TakeOwnership".to_string(),
                key: "MUIVerb".to_string(),
                expected_value: RegistryValue::String("Take Ownership".to_string()),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"*\shell\TakeOwnership\command".to_string(), key: "".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"*\shell\TakeOwnership".to_string(), key: "Icon".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"*\shell\TakeOwnership".to_string(), key: "MUIVerb".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"*\shell\TakeOwnership".to_string(), key: "HasLUAShield".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"*\shell\TakeOwnership".to_string(), key: "IsolatedCommand".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"Directory\shell\TakeOwnership\command".to_string(), key: "".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"Directory\shell\TakeOwnership".to_string(), key: "Icon".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"Directory\shell\TakeOwnership".to_string(), key: "MUIVerb".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"Directory\shell\TakeOwnership".to_string(), key: "HasLUAShield".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCR".to_string(), path: r"Directory\shell\TakeOwnership".to_string(), key: "IsolatedCommand".to_string() },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"*\shell\TakeOwnership".to_string(),
                    key: "MUIVerb".to_string(),
                    value: RegistryValue::String("Take Ownership".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"*\shell\TakeOwnership".to_string(),
                    key: "Icon".to_string(),
                    value: RegistryValue::String("imageres.dll,-78".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"*\shell\TakeOwnership".to_string(),
                    key: "HasLUAShield".to_string(),
                    value: RegistryValue::String("".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"*\shell\TakeOwnership\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("powershell -WindowStyle Hidden -Command \"Start-Process cmd -ArgumentList '/c takeown /f \\\"%1\\\" && icacls \\\"%1\\\" /grant administrators:F' -Verb RunAs\"".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"*\shell\TakeOwnership\command".to_string(),
                    key: "IsolatedCommand".to_string(),
                    value: RegistryValue::String("powershell -WindowStyle Hidden -Command \"Start-Process cmd -ArgumentList '/c takeown /f \\\"%1\\\" && icacls \\\"%1\\\" /grant administrators:F' -Verb RunAs\"".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"Directory\shell\TakeOwnership".to_string(),
                    key: "MUIVerb".to_string(),
                    value: RegistryValue::String("Take Ownership".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"Directory\shell\TakeOwnership".to_string(),
                    key: "Icon".to_string(),
                    value: RegistryValue::String("imageres.dll,-78".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"Directory\shell\TakeOwnership".to_string(),
                    key: "HasLUAShield".to_string(),
                    value: RegistryValue::String("".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"Directory\shell\TakeOwnership\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("powershell -WindowStyle Hidden -Command \"Start-Process cmd -ArgumentList '/c takeown /f \\\"%1\\\" /r /d y && icacls \\\"%1\\\" /grant administrators:F /t' -Verb RunAs\"".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"Directory\shell\TakeOwnership\command".to_string(),
                    key: "IsolatedCommand".to_string(),
                    value: RegistryValue::String("powershell -WindowStyle Hidden -Command \"Start-Process cmd -ArgumentList '/c takeown /f \\\"%1\\\" /r /d y && icacls \\\"%1\\\" /grant administrators:F /t' -Verb RunAs\"".to_string()),
                },
            ],
        },

        // ============================================
        // NEW PART 2 TWEAKS - Remove Context Menu Items
        // ============================================
        Tweak {
            id: "ctx_remove_cast_to_device".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Remove Cast to Device".to_string(),
            description: "Removes the 'Cast to Device' option from the right-click context menu.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}".to_string(),
                expected_value: RegistryValue::String("Cast to Device".to_string()),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}".to_string(),
                value: RegistryValue::String("Cast to Device".to_string()),
            }],
        },
        Tweak {
            id: "ctx_remove_share".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Remove Share".to_string(),
            description: "Removes the 'Share' option from the right-click context menu.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{E2BF9676-5F8F-435C-97EB-11607A5BEDF7}".to_string(),
                expected_value: RegistryValue::String("Share".to_string()),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{E2BF9676-5F8F-435C-97EB-11607A5BEDF7}".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{E2BF9676-5F8F-435C-97EB-11607A5BEDF7}".to_string(),
                value: RegistryValue::String("Share".to_string()),
            }],
        },
        Tweak {
            id: "ctx_remove_troubleshoot_compat".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Remove Troubleshoot Compatibility".to_string(),
            description: "Removes the 'Troubleshoot Compatibility' option from the right-click context menu.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{1d27f844-3a1f-4410-85ac-14651078412d}".to_string(),
                expected_value: RegistryValue::String("Troubleshoot Compat".to_string()),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{1d27f844-3a1f-4410-85ac-14651078412d}".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked".to_string(),
                key: "{1d27f844-3a1f-4410-85ac-14651078412d}".to_string(),
                value: RegistryValue::String("Troubleshoot Compat".to_string()),
            }],
        },

        // ============================================
        // MORE CONTEXT MENU TWEAKS
        // ============================================
        Tweak {
            id: "ctx_run_with_priority".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Add 'Run with Priority' to Context Menu".to_string(),
            description: "Adds a submenu to .exe files to launch them with Realtime, High, Above Normal, or Low CPU priority.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCR".to_string(),
                path: r"exefile\shell\Priority".to_string(),
                key: "".to_string(),
                expected_value: RegistryValue::String("Run with Priority".to_string()),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "reg".to_string(),
                args: vec!["delete".to_string(), r"HKCR\exefile\shell\Priority".to_string(), "/f".to_string()],
            }]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("Run with Priority".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority".to_string(),
                    key: "SubCommands".to_string(),
                    value: RegistryValue::String("".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\01High".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("High Priority".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\01High\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String(r#"cmd.exe /c start /HIGH "%1" %*"#.to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\02AboveNormal".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("Above Normal".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\02AboveNormal\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String(r#"cmd.exe /c start /ABOVENORMAL "%1" %*"#.to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\03Realtime".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("Realtime Priority".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\03Realtime\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String(r#"cmd.exe /c start /REALTIME "%1" %*"#.to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\04Low".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String("Low Priority".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCR".to_string(),
                    path: r"exefile\shell\Priority\shell\04Low\command".to_string(),
                    key: "".to_string(),
                    value: RegistryValue::String(r#"cmd.exe /c start /LOW "%1" %*"#.to_string()),
                },
            ],
        },
    ]
}
