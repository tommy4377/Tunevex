//! Windows Firewall Tweaks
//!
//! Controls for Windows Firewall profiles, notifications, and rules.

use crate::modules::types::{TweakType, 
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel,
};

pub fn get_firewall_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Firewall Notifications
        Tweak {
            id: "sec_fw_notifications".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🔔 Disable Firewall Notifications".to_string(),
            description: "Stops Windows Firewall from showing popup notifications when blocking apps.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\\\CurrentControlSet\\\\Services\\\\SharedAccess\\\\Parameters\\\\FirewallPolicy\\\\DomainProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\\\CurrentControlSet\\\\Services\\\\SharedAccess\\\\Parameters\\\\FirewallPolicy\\\\StandardProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\\\CurrentControlSet\\\\Services\\\\SharedAccess\\\\Parameters\\\\FirewallPolicy\\\\PublicProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\StandardProfile".to_string(),
                key: "DisableNotifications".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\DomainProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\StandardProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\PublicProfile".to_string(),
                    key: "DisableNotifications".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // Disable Firewall (All Profiles)
        Tweak {
            id: "sec_disable_firewall".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🔥 Disable Windows Firewall".to_string(),
            description: "Completely disables Windows Firewall for all network profiles. DANGEROUS: System exposed to network attacks.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$profile = Get-NetFirewallProfile -Profile Private
if ($profile.Enabled -eq "False") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled True
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled False
Write-Host "Windows Firewall disabled for all profiles" -ForegroundColor Yellow
"#.to_string(),
                }
            ],
        },

        // Block Outbound by Default (Whitelist Mode)
        Tweak {
            id: "sec_fw_whitelist".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🚫 Block Outbound by Default".to_string(),
            description: "Sets firewall to block all outbound connections unless explicitly allowed. Very restrictive.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$profile = Get-NetFirewallProfile -Profile Private
if ($profile.DefaultOutboundAction -eq "Block") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-NetFirewallProfile -Profile Domain,Public,Private -DefaultOutboundAction Allow
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Set-NetFirewallProfile -Profile Domain,Public,Private -DefaultOutboundAction Block
Write-Host "Firewall set to block outbound by default" -ForegroundColor Cyan
"#.to_string(),
                }
            ],
        },

        // Disable Remote Desktop in Firewall
        Tweak {
            id: "sec_fw_block_rdp".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🖥️ Block Remote Desktop".to_string(),
            description: "Disables Remote Desktop firewall rules to prevent remote access.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$rules = Get-NetFirewallRule -DisplayGroup "Remote Desktop" -Enabled True -ErrorAction SilentlyContinue
if (-not $rules) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Enable-NetFirewallRule -DisplayGroup "Remote Desktop" -ErrorAction SilentlyContinue
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Disable-NetFirewallRule -DisplayGroup "Remote Desktop" -ErrorAction SilentlyContinue
Write-Host "Remote Desktop firewall rules disabled" -ForegroundColor Green
"#.to_string(),
                }
            ],
        },
    ]
}
