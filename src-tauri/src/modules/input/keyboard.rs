use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Keyboard Optimization Tweaks
pub fn get_keyboard_tweaks() -> Vec<Tweak> {
    vec![
        // Fast keyboard repeat rate
        Tweak {
            id: "input_keyboard_speed".to_string(),
            category: TweakCategory::MouseInput,
            name: "Maximize Keyboard Repeat Speed".to_string(),
            description: "Sets keyboard repeat rate to maximum (31) and delay to minimum (0) for faster typing.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true, // Needs restart/logoff
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "KeyboardSpeed".to_string(),
                    value: RegistryValue::String("31".to_string()), // Default usually 31
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "KeyboardDelay".to_string(),
                    value: RegistryValue::String("1".to_string()), // Default usually 1
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Keyboard".to_string(),
                key: "KeyboardDelay".to_string(),
                expected_value: RegistryValue::String("0".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "KeyboardSpeed".to_string(),
                    value: RegistryValue::String("31".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "KeyboardDelay".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]
        },
        
        // Disable Sticky Keys popup
        Tweak {
            id: "input_disable_sticky_keys".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Sticky Keys Popup".to_string(),
            description: "Prevents annoying Sticky Keys popup when pressing Shift 5 times (common in gaming).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\StickyKeys".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("510".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Accessibility\\StickyKeys".to_string(),
                key: "Flags".to_string(),
                expected_value: RegistryValue::String("506".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\StickyKeys".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("506".to_string()),
                },
            ]
        },
        
        // Disable Filter Keys popup
        Tweak {
            id: "input_disable_filter_keys".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Filter Keys Popup".to_string(),
            description: "Prevents Filter Keys popup when holding Shift for 8 seconds.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\Keyboard Response".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("126".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Accessibility\\Keyboard Response".to_string(),
                key: "Flags".to_string(),
                expected_value: RegistryValue::String("122".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\Keyboard Response".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("122".to_string()),
                },
            ]
        },
        
        // Disable Toggle Keys popup
        Tweak {
            id: "input_disable_toggle_keys".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Toggle Keys Popup".to_string(),
            description: "Prevents Toggle Keys popup when holding NumLock for 5 seconds.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\ToggleKeys".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("62".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Accessibility\\ToggleKeys".to_string(),
                key: "Flags".to_string(),
                expected_value: RegistryValue::String("58".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Accessibility\\ToggleKeys".to_string(),
                    key: "Flags".to_string(),
                    value: RegistryValue::String("58".to_string()),
                },
            ]
        },
        
        // Disable Touch Keyboard Features
        Tweak {
            id: "input_disable_touch_keyboard".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Touch Keyboard Features".to_string(),
            description: "Disables unnecessary touch keyboard audio feedback and auto-shift features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\TabletTip\\1.7".to_string(),
                    key: "EnableAutoShiftEngage".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\TabletTip\\1.7".to_string(),
                    key: "EnableKeyAudioFeedback".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\TabletTip\\1.7".to_string(),
                key: "EnableAutoShiftEngage".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\TabletTip\\1.7".to_string(),
                    key: "EnableAutoShiftEngage".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\TabletTip\\1.7".to_string(),
                    key: "EnableKeyAudioFeedback".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Enable NumLock on Startup
        Tweak {
            id: "input_numlock_startup".to_string(),
            category: TweakCategory::MouseInput,
            name: "Enable NumLock on Startup".to_string(),
            description: "Ensures NumLock is always enabled when Windows starts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "InitialKeyboardIndicators".to_string(),
                    value: RegistryValue::String("2".to_string()), // Default 2 often
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Keyboard".to_string(),
                key: "InitialKeyboardIndicators".to_string(),
                expected_value: RegistryValue::String("2".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Keyboard".to_string(),
                    key: "InitialKeyboardIndicators".to_string(),
                    value: RegistryValue::String("2".to_string()),
                },
            ]
        },

        // NEW TWEAK: Keyboard Data Queue Size
        Tweak {
            id: "input_keyboard_data_queue_size".to_string(),
            category: TweakCategory::MouseInput,
            name: "Setup Keyboard Data Queue Size".to_string(),
            description: "Increases keyboard data queue size to 50 (decimal) to handle more inputs and reduce potential latency.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters".to_string(),
                    key: "KeyboardDataQueueSize".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters".to_string(),
                key: "KeyboardDataQueueSize".to_string(),
                expected_value: RegistryValue::DWord(50),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters".to_string(),
                    key: "KeyboardDataQueueSize".to_string(),
                    value: RegistryValue::DWord(50),
                },
            ]
        },
    ]
}
