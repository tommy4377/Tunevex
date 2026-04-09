use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_system_msi_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "system_msi_global_safe".to_string(),
            category: TweakCategory::System,
            name: "Enable MSI Mode Globally (Safe Priority)".to_string(),
            description: "Enables MSI with Priority 0 on all supported PCI devices. Safest option for broad compatibility.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MsiEnabledGlobally { priority: 0 }),
            revert_operations: Some(vec![
                TweakOperation::MsiRemove { class: "Display".to_string() },
                TweakOperation::MsiRemove { class: "SCSIAdapter".to_string() },
                TweakOperation::MsiRemove { class: "Net".to_string() },
                TweakOperation::MsiRemove { class: "USB".to_string() },
                TweakOperation::MsiRemove { class: "HDC".to_string() },
            ]),
            operations: vec![
                TweakOperation::MsiSet { class: "Display".to_string(), priority: 0 },
                TweakOperation::MsiSet { class: "SCSIAdapter".to_string(), priority: 0 },
                TweakOperation::MsiSet { class: "Net".to_string(), priority: 0 },
                TweakOperation::MsiSet { class: "USB".to_string(), priority: 0 },
                TweakOperation::MsiSet { class: "HDC".to_string(), priority: 0 },
            ]
        },
    ]
}
