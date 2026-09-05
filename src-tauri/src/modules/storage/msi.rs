use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_storage_msi_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "storage_msi_nvme_high".to_string(),
        category: TweakCategory::FileSystem,
        name: "Enable MSI Mode on NVMe (High Priority)".to_string(),
        description: "Enables MSI with Priority 3 on NVMe drives. Reduces storage latency."
            .to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: true,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::MsiEnabledForClass { class: "NVMe".into(), priority: 3 }),
        revert_operations: Some(vec![TweakOperation::MsiRemove {
            class: "NVMe".to_string(),
        }]),
        operations: vec![TweakOperation::MsiSet {
            class: "NVMe".to_string(),
            priority: 3,
        }],
    }]
}
