use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakOperation, WarningLevel, TweakCheck, RegistryValue};

/// Mouse Optimization Tweaks
pub fn get_mouse_tweaks() -> Vec<Tweak> {
    vec![
        // Disable Mouse Acceleration
        Tweak {
            id: "input_disable_mouse_accel".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Mouse Acceleration".to_string(),
            description: "Disables 'Enhance Pointer Precision' (mouse acceleration) for 1:1 mouse movement. Critical for FPS gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true, // Often requires logoff/restart to fully apply globally
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseSpeed".to_string(),
                    value: RegistryValue::String("1".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseThreshold1".to_string(),
                    value: RegistryValue::String("6".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseThreshold2".to_string(),
                    value: RegistryValue::String("10".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$speed = Get-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -ErrorAction SilentlyContinue
$thresh1 = Get-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -ErrorAction SilentlyContinue
$thresh2 = Get-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -ErrorAction SilentlyContinue

if (($speed.MouseSpeed -eq 0) -and ($thresh1.MouseThreshold1 -eq 0) -and ($thresh2.MouseThreshold2 -eq 0)) {
    "True"
} else {
    "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseSpeed".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseThreshold1".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseThreshold2".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]
        },
        
        // Mouse Sensitivity Default
        Tweak {
            id: "input_mouse_sensitivity_default".to_string(),
            category: TweakCategory::MouseInput,
            name: "Set Mouse Sensitivity to Default (10)".to_string(),
            description: "Resets Windows mouse sensitivity to default 10. Let games handle DPI scaling.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, // Resetting to default IS the revert/fix
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Mouse".to_string(),
                key: "MouseSensitivity".to_string(),
                expected_value: RegistryValue::String("10".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseSensitivity".to_string(),
                    value: RegistryValue::String("10".to_string()),
                },
            ]
        },
        
        // Minimize Mouse Hover Time
        Tweak {
            id: "input_mouse_hover_time".to_string(),
            category: TweakCategory::MouseInput,
            name: "Minimize Mouse Hover Time".to_string(),
            description: "Reduces hover time from 400ms to 20ms for instant tooltips in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseHoverTime".to_string(),
                    value: RegistryValue::String("400".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Mouse".to_string(),
                key: "MouseHoverTime".to_string(),
                expected_value: RegistryValue::String("20".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseHoverTime".to_string(),
                    value: RegistryValue::String("20".to_string()),
                },
            ]
        },
        
        // Disable Mouse Trails
        Tweak {
            id: "input_disable_mouse_trails".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Mouse Trails".to_string(),
            description: "Removes legacy mouse trail effect.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseTrails".to_string(),
                    value: RegistryValue::String("0".to_string()), // 0 is actually disabled/default too
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Mouse".to_string(),
                key: "MouseTrails".to_string(),
                expected_value: RegistryValue::String("0".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "MouseTrails".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]
        },
        
        // Disable Snap To Default Button
        Tweak {
            id: "input_disable_snap_to".to_string(),
            category: TweakCategory::MouseInput,
            name: "Disable Snap to Default Button".to_string(),
            description: "Prevents cursor from auto-moving to dialog buttons.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "SnapToDefaultButton".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\Mouse".to_string(),
                key: "SnapToDefaultButton".to_string(),
                expected_value: RegistryValue::String("0".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\Mouse".to_string(),
                    key: "SnapToDefaultButton".to_string(),
                    value: RegistryValue::String("0".to_string()),
                },
            ]
        },
    ]
}
