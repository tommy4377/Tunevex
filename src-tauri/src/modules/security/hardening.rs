//! Security Hardening Tweaks
//!
//! Network security hardening, SAM enumeration blocking, remote assistance disable.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_hardening_tweaks() -> Vec<Tweak> {
    vec![
        // Block Anonymous SAM Enumeration
        Tweak {
            id: "sec_block_sam_enum".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Block Anonymous SAM Enumeration".to_string(),
            description: "Prevents anonymous users from enumerating SAM accounts. Security hardening.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                    key: "RestrictAnonymousSAM".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                key: "RestrictAnonymousSAM".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                    key: "RestrictAnonymousSAM".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // Disable Remote Assistance
        Tweak {
            id: "sec_disable_remote_assistance".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Remote Assistance".to_string(),
            description: "Disables Windows Remote Assistance feature and blocks related firewall rules.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Remote Assistance".to_string(),
                key: "fAllowToGetHelp".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
netsh advfirewall firewall set rule group="Remote Assistance" new enable=yes
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Remote Assistance".to_string(),
                    key: "fAllowFullControl".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Remote Assistance".to_string(),
                    key: "fAllowToGetHelp".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"
netsh advfirewall firewall set rule group="Remote Assistance" new enable=no
Write-Host "Remote Assistance disabled" -ForegroundColor Green
"#.to_string(),
                }
            ],
        },

        // Disable SMBv1
        Tweak {
            id: "sec_disable_smbv1".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable SMBv1".to_string(),
            description: "Disables vulnerable SMBv1 protocol. Protects against WannaCry-style attacks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$smb1 = Get-WindowsOptionalFeature -Online -FeatureName SMB1Protocol -EA 0
if ($smb1.State -eq 'Disabled') { 'True' } else { 'False' }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Enable-WindowsOptionalFeature -Online -FeatureName SMB1Protocol -NoRestart -ErrorAction SilentlyContinue
Set-SmbServerConfiguration -EnableSMB1Protocol $true -Force
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Disable-WindowsOptionalFeature -Online -FeatureName SMB1Protocol -NoRestart -ErrorAction SilentlyContinue
Set-SmbServerConfiguration -EnableSMB1Protocol $false -Force
Write-Host "SMBv1 disabled" -ForegroundColor Green
"#.to_string(),
                }
            ],
        },

        // Enable SMB Signing
        Tweak {
            id: "sec_enable_smb_signing".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Enable SMB Signing".to_string(),
            description: "Requires SMB packet signing. Prevents man-in-the-middle attacks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanmanServer\\Parameters".to_string(),
                    key: "RequireSecuritySignature".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters".to_string(),
                    key: "RequireSecuritySignature".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\LanmanServer\\Parameters".to_string(),
                key: "RequireSecuritySignature".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanmanServer\\Parameters".to_string(),
                    key: "RequireSecuritySignature".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters".to_string(),
                    key: "RequireSecuritySignature".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // Disable Delivery Optimization (P2P Updates)
        Tweak {
            id: "sec_disable_delivery_opt".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Delivery Optimization".to_string(),
            description: "Stops P2P sharing of Windows Updates. Downloads only from Microsoft servers.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization".to_string(),
                    key: "DODownloadMode".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization".to_string(),
                key: "DODownloadMode".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization".to_string(),
                    key: "DODownloadMode".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable Windows Update Medic Service
        Tweak {
            id: "sec_disable_update_medic".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Update Medic Service".to_string(),
            description: "Disables WaaSMedicSvc which re-enables Windows Update. May break updates.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\WaaSMedicSvc".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(3),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\WaaSMedicSvc".to_string(),
                key: "Start".to_string(),
                expected_value: RegistryValue::DWord(4),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\WaaSMedicSvc".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(4),
                },
            ],
        },

        // Disable UAC Virtualization
        Tweak {
            id: "sec_disable_uac_virtualization".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable UAC Virtualization".to_string(),
            description: "Disables file/registry virtualization for legacy apps. May break old software.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableVirtualization".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                key: "EnableVirtualization".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableVirtualization".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable Automatic Maintenance
        Tweak {
            id: "sec_disable_auto_maintenance".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Automatic Maintenance".to_string(),
            description: "Stops Windows from running automatic maintenance tasks (updates, defrag, etc).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\Maintenance".to_string(),
                    key: "MaintenanceDisabled".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\Maintenance".to_string(),
                key: "MaintenanceDisabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\Maintenance".to_string(),
                    key: "MaintenanceDisabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
    ]
}
