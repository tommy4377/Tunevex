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
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDeleteKey {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}".to_string(),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe""#.to_string()
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32".to_string(),
                    key: "".to_string(), // Default value
                    value: RegistryValue::String("".to_string()),
                },
                TweakOperation::Powershell {
                    script: r#"Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe""#.to_string()
                }
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
                TweakOperation::Powershell {
                    script: r#"
Remove-Item -Path "HKCR:\*\shell\TakeOwnership" -Recurse -Force -EA 0
Remove-Item -Path "HKCR:\Directory\shell\TakeOwnership" -Recurse -Force -EA 0
"#.to_string(),
                }
            ]),
            operations: vec![
                // File Context Menu
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

                // Folder Context Menu
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
                    // For directories we add /r /d y
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
    ]
}
