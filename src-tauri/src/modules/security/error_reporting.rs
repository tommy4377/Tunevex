//! Windows Error Reporting Tweaks
//!
//! Controls for crash reporting, memory dumps, and WER service.

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_error_reporting_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Windows Error Reporting
        Tweak {
            id: "sec_disable_wer".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "📊 Disable Windows Error Reporting".to_string(),
            description: "Stops Windows from collecting and sending crash reports to Microsoft."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting"
                        .to_string(),
                    key: "Disabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting"
                        .to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
        // Disable Automatic Crash Dumps
        Tweak {
            id: "sec_disable_crash_dumps".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "💾 Disable Automatic Crash Dumps".to_string(),
            description: "Prevents Windows from creating memory dumps when apps crash.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "DontSendAdditionalData".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "DontShowUI".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "DontSendAdditionalData".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "DontShowUI".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
        // Disable WER Service
        Tweak {
            id: "sec_disable_wer_service".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "⚙️ Disable WER Service".to_string(),
            description: "Stops the Windows Error Reporting Service from running.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            enabled: false,
            check: None,
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
Set-Service WerSvc -StartupType Manual -ErrorAction SilentlyContinue
Start-Service WerSvc -ErrorAction SilentlyContinue
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
Stop-Service WerSvc -Force -ErrorAction SilentlyContinue
Set-Service WerSvc -StartupType Disabled -ErrorAction SilentlyContinue
Write-Host "Windows Error Reporting Service disabled" -ForegroundColor Green
"#
                .to_string(),
            }],
        },
        // Disable Problem Reporting Dialog
        Tweak {
            id: "sec_disable_problem_dialog".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "💬 Disable Problem Reporting Dialog".to_string(),
            description: "Disables the 'Windows is checking for a solution' dialog.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                key: "DontShowUI".to_string(),
            }]),
            enabled: false,
            check: None,
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                key: "DontShowUI".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // Disable Corporate Error Reporting
        Tweak {
            id: "sec_disable_corp_wer".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🏢 Disable Corporate Error Reporting".to_string(),
            description: "Disables error reporting for enterprise environments.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\PCHealth\\ErrorReporting".to_string(),
                key: "DoReport".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            enabled: false,
            check: None,
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\PCHealth\\ErrorReporting".to_string(),
                key: "DoReport".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
    ]
}
