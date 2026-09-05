use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_usb_msi_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "input_msi_usb_normal".to_string(),
        category: TweakCategory::MouseInput,
        name: "Enable MSI Mode on USB Controllers (Normal)".to_string(),
        description: "Enables MSI with Normal priority (2) on supported PCI USB host controllers. Restores saved values on revert."
            .to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: true,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::MsiEnabledForClass { class: "USB".into(), priority: 2 }),
        revert_operations: Some(vec![TweakOperation::MsiRemove {
            class: "USB".to_string(),
        }]),
        operations: vec![TweakOperation::MsiSet {
            class: "USB".to_string(),
            priority: 2,
        }],
    }]
}
