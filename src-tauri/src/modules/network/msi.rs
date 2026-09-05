use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_network_msi_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_msi_nic_high".to_string(),
            category: TweakCategory::Network,
            name: "Enable MSI Mode on NIC (High Priority)".to_string(),
            description: "Enables MSI with Priority 3 on network adapters. Reduces network latency.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MsiEnabledOnNet { priority: 3 }),
            revert_operations: Some(vec![
                TweakOperation::MsiRemoveNet,
            ]),
            operations: vec![
                TweakOperation::MsiSetNet { priority: 3 },
            ]
        },
        Tweak {
            id: "net_msi_nic_normal".to_string(),
            category: TweakCategory::Network,
            name: "Enable MSI Mode on NIC (Normal Priority)".to_string(),
            description: "Enables MSI with Normal priority (2) on supported PCI network adapters. Revert restores saved device values.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MsiEnabledOnNet { priority: 2 }),
            revert_operations: Some(vec![
                TweakOperation::MsiRemoveNet,
            ]),
            operations: vec![
                TweakOperation::MsiSetNet { priority: 2 },
            ]
        },
        Tweak {
            id: "net_msi_additional_vendors".to_string(),
            category: TweakCategory::Network,
            name: "Enable MSI on Additional NICs".to_string(),
            description: "Enables MSI mode for Qualcomm, Broadcom, Marvell, Killer, and MediaTek network adapters with Priority 2.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MsiEnabledOnNet { priority: 2 }),
            revert_operations: Some(vec![
                TweakOperation::MsiRemoveNet,
            ]),
            operations: vec![
                TweakOperation::MsiSetNet { priority: 2 },
            ]
        },
    ]
}
