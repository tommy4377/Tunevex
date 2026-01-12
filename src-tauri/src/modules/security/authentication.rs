//! Authentication & Login Tweaks
//!
//! Controls for Windows Hello, lock screen, password policies, and auto-login.

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_authentication_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Windows Hello
        Tweak {
            id: "sec_disable_hello".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "👋 Disable Windows Hello".to_string(),
            description: "Disables Windows Hello biometric and PIN sign-in prompts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\PassportForWork".to_string(),
                    key: "Enabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\PassportForWork".to_string(),
                    key: "DisablePostLogonProvisioning".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\PassportForWork".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\PassportForWork".to_string(),
                    key: "DisablePostLogonProvisioning".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
        // Disable Lock Screen
        Tweak {
            id: "sec_disable_lockscreen".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🔓 Disable Lock Screen".to_string(),
            description: "Skips the lock screen and goes directly to login. Faster access."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                key: "NoLockScreen".to_string(),
            }]),
            enabled: false,
            check: None,
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                key: "NoLockScreen".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // Disable Password Reveal Button
        Tweak {
            id: "sec_no_password_reveal".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "👁️ Disable Password Reveal Button".to_string(),
            description: "Hides the 'eye' button that reveals password in text fields.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CredUI".to_string(),
                key: "DisablePasswordReveal".to_string(),
            }]),
            enabled: false,
            check: None,
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CredUI".to_string(),
                key: "DisablePasswordReveal".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // Disable Require Sign-in After Sleep
        Tweak {
            id: "sec_no_signin_sleep".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "😴 Disable Sign-in After Sleep".to_string(),
            description: "Skips password prompt when waking from sleep/hibernate.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            enabled: false,
            check: None,
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
powercfg /SETACVALUEINDEX SCHEME_CURRENT SUB_NONE CONSOLELOCK 1
powercfg /SETDCVALUEINDEX SCHEME_CURRENT SUB_NONE CONSOLELOCK 1
powercfg /SETACTIVE SCHEME_CURRENT
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
powercfg /SETACVALUEINDEX SCHEME_CURRENT SUB_NONE CONSOLELOCK 0
powercfg /SETDCVALUEINDEX SCHEME_CURRENT SUB_NONE CONSOLELOCK 0
powercfg /SETACTIVE SCHEME_CURRENT
Write-Host "Sign-in after sleep disabled" -ForegroundColor Green
"#
                .to_string(),
            }],
        },
        // Disable Automatic Login
        Tweak {
            id: "sec_disable_auto_login".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🔒 Disable Automatic Login".to_string(),
            description: "Ensures Windows requires a password to log in. Improves security."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                // Note: Re-enabling auto-login requires password, so we can't fully revert to "on" if it was on.
                // We'll just leave it disabled or set to 0.
                // A true revert would need to know the previous state, but for security, defaulting to OFF is safer.
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon".to_string(),
                    key: "AutoAdminLogon".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon".to_string(),
                    key: "AutoAdminLogon".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon".to_string(),
                    key: "DefaultPassword".to_string(),
                },
            ],
        },
        // Disable Remote Desktop
        Tweak {
            id: "sec_disable_rdp".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🖥️ Disable Remote Desktop".to_string(),
            description: "Disables Remote Desktop Protocol access to this computer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Terminal Server".to_string(),
                key: "fDenyTSConnections".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            enabled: false,
            check: None,
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Terminal Server".to_string(),
                key: "fDenyTSConnections".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
    ]
}
