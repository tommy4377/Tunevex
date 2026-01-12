//! Windows Defender Tweaks
//!
//! Controls for Windows Defender real-time protection, cloud features,
//! sample submission, and exclusions.

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_defender_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Real-time Protection
        Tweak {
            id: "sec_disable_realtime".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🛡️ Disable Real-time Protection".to_string(),
            description: "Disables Windows Defender real-time scanning. WARNING: Leaves system vulnerable to malware.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableRealtimeMonitoring".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableBehaviorMonitoring".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableOnAccessProtection".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableScanOnRealtimeEnable".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableRealtimeMonitoring".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableBehaviorMonitoring".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableOnAccessProtection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection".to_string(),
                    key: "DisableScanOnRealtimeEnable".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // Disable Cloud Protection
        Tweak {
            id: "sec_disable_cloud".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "☁️ Disable Cloud-Delivered Protection".to_string(),
            description: "Disables cloud-based threat detection. Reduces network usage but may miss new threats.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SpynetReporting".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\MpEngine".to_string(),
                    key: "MpCloudBlockLevel".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SpynetReporting".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\MpEngine".to_string(),
                    key: "MpCloudBlockLevel".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable Sample Submission
        Tweak {
            id: "sec_disable_samples".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "📤 Disable Sample Submission".to_string(),
            description: "Stops Defender from sending file samples to Microsoft for analysis.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SubmitSamplesConsent".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SubmitSamplesConsent".to_string(),
                    value: RegistryValue::DWord(2), // 2 = Never send
                },
            ],
        },

        // Add Developer Exclusions
        Tweak {
            id: "sec_dev_exclusions".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "💻 Add Developer Folder Exclusions".to_string(),
            description: "Excludes common developer folders from Defender scanning (improves build times).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,

            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$exclusions = @(
    "$env:USERPROFILE\source",
    "$env:USERPROFILE\repos",
    "$env:USERPROFILE\.cargo",
    "$env:USERPROFILE\.rustup",
    "$env:USERPROFILE\node_modules",
    "$env:USERPROFILE\.npm",
    "$env:USERPROFILE\go",
    "$env:APPDATA\npm",
    "C:\Users\*\AppData\Local\Temp\rust*"
)
foreach ($path in $exclusions) {
    Remove-MpPreference -ExclusionPath $path -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$exclusions = @(
    "$env:USERPROFILE\source",
    "$env:USERPROFILE\repos",
    "$env:USERPROFILE\.cargo",
    "$env:USERPROFILE\.rustup",
    "$env:USERPROFILE\node_modules",
    "$env:USERPROFILE\.npm",
    "$env:USERPROFILE\go",
    "$env:APPDATA\npm",
    "C:\Users\*\AppData\Local\Temp\rust*"
)
foreach ($path in $exclusions) {
    Add-MpPreference -ExclusionPath $path -ErrorAction SilentlyContinue
}
Write-Host "Developer exclusions added to Windows Defender" -ForegroundColor Green
"#.to_string(),
                }
            ],
        },

        // Disable PUA Protection
        Tweak {
            id: "sec_disable_pua".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "📦 Disable PUA Protection".to_string(),
            description: "Disables detection of Potentially Unwanted Applications. Useful for tools like cracks, keygens.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "PUAProtection".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "PUAProtection".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable Defender Completely
        Tweak {
            id: "sec_disable_defender".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "⛔ Disable Windows Defender Completely".to_string(),
            description: "Completely disables Windows Defender antivirus. EXTREME RISK. NOTE: You MUST disable 'Tamper Protection' manually in Windows Security settings first.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Re-enable Defender services
$services = @('WinDefend', 'WdNisSvc', 'Sense')
foreach ($svc in $services) {
    Set-Service -Name $svc -StartupType Automatic -ErrorAction SilentlyContinue
    Start-Service -Name $svc -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiSpyware".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiVirus".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: r#"
# Disable Defender services
$services = @('WinDefend', 'WdNisSvc', 'Sense')
foreach ($svc in $services) {
    Stop-Service -Name $svc -Force -ErrorAction SilentlyContinue
    Set-Service -Name $svc -StartupType Disabled -ErrorAction SilentlyContinue
}
Write-Host "Windows Defender disabled (reboot required). Ensure Tamper Protection is OFF." -ForegroundColor Yellow
"#.to_string(),
                }
            ],
        },
    ]
}
