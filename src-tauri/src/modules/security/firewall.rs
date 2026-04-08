//! Windows Firewall Tweaks
//!
//! Controls for Windows Firewall profiles, notifications, and rules.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_firewall_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Firewall Notifications
        Tweak {
            id: "sec_fw_notifications".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Firewall Notifications".to_string(),
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
            name: "Disable Windows Firewall".to_string(),
            description: "Completely disables Windows Firewall for all network profiles. DANGEROUS: System exposed to network attacks.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "netsh".to_string(),
                args: vec!["advfirewall".to_string(), "show".to_string(), "allprofiles".to_string(), "state".to_string()],
                contains: "OFF".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "set".to_string(), "allprofiles".to_string(), "state".to_string(), "on".to_string()],
                }
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "set".to_string(), "allprofiles".to_string(), "state".to_string(), "off".to_string()],
                }
            ],
        },

        // Block Outbound by Default (Whitelist Mode)
        Tweak {
            id: "sec_fw_whitelist".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Block Outbound by Default".to_string(),
            description: "Sets firewall to block all outbound connections unless explicitly allowed. Very restrictive.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "netsh".to_string(),
                args: vec!["advfirewall".to_string(), "show".to_string(), "allprofiles".to_string(), "firewallpolicy".to_string()],
                contains: "BlockOutbound".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "set".to_string(), "allprofiles".to_string(), "firewallpolicy".to_string(), "BlockInbound,AllowOutbound".to_string()],
                }
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "set".to_string(), "allprofiles".to_string(), "firewallpolicy".to_string(), "BlockInbound,BlockOutbound".to_string()],
                }
            ],
        },

        // Disable Remote Desktop in Firewall
        Tweak {
            id: "sec_fw_block_rdp".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Block Remote Desktop".to_string(),
            description: "Disables Remote Desktop firewall rules to prevent remote access.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "netsh".to_string(),
                args: vec!["advfirewall".to_string(), "firewall".to_string(), "show".to_string(), "rule".to_string(), "group=Remote Desktop".to_string()],
                contains: "Enabled:                            No".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "firewall".to_string(), "set".to_string(), "rule".to_string(), "group=Remote Desktop".to_string(), "new".to_string(), "enable=Yes".to_string()],
                }
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["advfirewall".to_string(), "firewall".to_string(), "set".to_string(), "rule".to_string(), "group=Remote Desktop".to_string(), "new".to_string(), "enable=No".to_string()],
                }
            ],
        },
    ]
}
