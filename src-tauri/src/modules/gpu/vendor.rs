//! Hardware-Specific GPU Tweaks with Auto-Detection
//!
//! Contains Nvidia, AMD, and Intel specific tweaks that are conditionally shown
//! based on detected hardware.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

use winreg::enums::*;
use winreg::RegKey;

/// Detect if Nvidia GPU is present by checking registry
fn is_nvidia_gpu() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let video_path =
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";

    if let Ok(class_key) = hklm.open_subkey(video_path) {
        for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
            if let Ok(subkey) = class_key.open_subkey(&subkey_name) {
                if let Ok(driver_desc) = subkey.get_value::<String, _>("DriverDesc") {
                    if driver_desc.to_lowercase().contains("nvidia") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Detect if AMD GPU is present by checking registry
fn is_amd_gpu() -> bool {
    find_amd_gpu_instance().is_some()
}

/// Find the AMD GPU instance ID (e.g., "0000", "0001")
fn find_amd_gpu_instance() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let video_path =
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";

    if let Ok(class_key) = hklm.open_subkey(video_path) {
        for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
            if let Ok(subkey) = class_key.open_subkey(&subkey_name) {
                if let Ok(driver_desc) = subkey.get_value::<String, _>("DriverDesc") {
                    let lower = driver_desc.to_lowercase();
                    if lower.contains("amd") || lower.contains("radeon") {
                        return Some(subkey_name);
                    }
                }
            }
        }
    }
    None
}

/// Returns hardware-specific GPU tweaks based on detected vendor
pub fn get_vendor_gpu_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // Nvidia-specific tweaks
    if is_nvidia_gpu() {
        tweaks.extend(get_nvidia_tweaks());
    }

    // AMD-specific tweaks
    if is_amd_gpu() {
        tweaks.extend(get_amd_gpu_tweaks());
    }

    tweaks
}

/// Nvidia-specific GPU tweaks
fn get_nvidia_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gpu_nvidia_dynamic_pstate".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "[NVIDIA] Disable Dynamic P-States".to_string(),
            description: "Disables NVIDIA dynamic P-state switching.

Forces the GPU to stay at higher performance states.
May reduce frame time variance in games.
Note: Slightly increases idle power consumption."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\NVTweak".to_string(),
                key: "DisableDynamicPstate".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\NVTweak".to_string(),
                key: "DisableDynamicPstate".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\NVTweak".to_string(),
                key: "DisableDynamicPstate".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "gpu_nvidia_clean_cache".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "[NVIDIA] Clear Driver Profile Cache".to_string(),
            description: "Clears NVIDIA driver profile database files.

Can fix performance issues from corrupted profiles.
Profiles will be automatically recreated."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        "del /Q /F \"%ProgramData%\\NVIDIA Corporation\\Drs\\nvdrsdb0.bin\" 2>nul"
                            .to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        "del /Q /F \"%ProgramData%\\NVIDIA Corporation\\Drs\\nvdrsdb1.bin\" 2>nul"
                            .to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        "rmdir /S /Q \"%LOCALAPPDATA%\\NVIDIA\\DXCache\" 2>nul".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        "rmdir /S /Q \"%LOCALAPPDATA%\\NVIDIA\\GLCache\" 2>nul".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        "rmdir /S /Q \"%LOCALAPPDATA%\\NVIDIA\\ShaderCache\" 2>nul".to_string(),
                    ],
                },
            ],
        },
    ]
}

/// AMD-specific GPU tweaks
fn get_amd_gpu_tweaks() -> Vec<Tweak> {
    let amd_instance = match find_amd_gpu_instance() {
        Some(id) => id,
        None => return vec![],
    };

    let amd_path = format!(
        "SYSTEM\\CurrentControlSet\\Control\\Class\\{{4d36e968-e325-11ce-bfc1-08002be10318}}\\{}",
        amd_instance
    );

    vec![Tweak {
        id: "gpu_amd_ulps".to_string(),
        category: TweakCategory::GpuOptimization,
        name: "[AMD] Disable Ultra Low Power State".to_string(),
        description: "Disables AMD ULPS (Ultra Low Power State).

Prevents the GPU from entering deep sleep states.
Can fix issues with multi-monitor and CrossFire setups.
May slightly increase idle power consumption."
            .to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: true,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::Registry {
            root_key: "HKLM".to_string(),
            path: amd_path.clone(),
            key: "EnableUlps".to_string(),
            expected_value: RegistryValue::DWord(0),
        }),
        revert_operations: Some(vec![TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: amd_path.clone(),
            key: "EnableUlps".to_string(),
            value: RegistryValue::DWord(1),
        }]),
        operations: vec![TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: amd_path,
            key: "EnableUlps".to_string(),
            value: RegistryValue::DWord(0),
        }],
    }]
}
