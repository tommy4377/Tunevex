use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_usb_msi_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "input_msi_usb_normal".to_string(),
        category: TweakCategory::MouseInput,
        name: "Enable MSI Mode on USB Controllers (Normal)".to_string(),
        description: "Enables MSI with Priority 1 on USB host controllers. Reduces USB latency."
            .to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: true,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::MsiEnabledGlobally { priority: 1 }),
        revert_operations: Some(vec![TweakOperation::MsiRemove {
            class: "USB".to_string(),
        }]),
        operations: vec![TweakOperation::MsiSet {
            class: "USB".to_string(),
            priority: 1,
        }],
    }]
}
