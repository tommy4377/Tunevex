//! Hardware-Specific CPU Tweaks with Auto-Detection
//!
//! Contains Intel and AMD specific tweaks that are conditionally shown
//! based on detected hardware.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Detect if CPU is Intel
/// Detect if CPU is Intel
fn is_intel_cpu() -> bool {
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(&["-NoProfile", "-Command", "(Get-ItemProperty 'HKLM:\\HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0').VendorIdentifier"])
            .output()
        {
            let vendor = String::from_utf8_lossy(&output.stdout);
            return vendor.contains("GenuineIntel");
        }
    }
    false
}

/// Detect if CPU is AMD
fn is_amd_cpu() -> bool {
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(&["-NoProfile", "-Command", "(Get-ItemProperty 'HKLM:\\HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0').VendorIdentifier"])
            .output()
        {
            let vendor = String::from_utf8_lossy(&output.stdout);
            return vendor.contains("AuthenticAMD");
        }
    }
    false
}

/// Returns hardware-specific CPU tweaks based on detected vendor
pub fn get_hardware_specific_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // Intel-specific tweaks
    if is_intel_cpu() {
        tweaks.extend(get_intel_tweaks());
    }

    // AMD-specific tweaks
    if is_amd_cpu() {
        tweaks.extend(get_amd_tweaks());
    }

    tweaks
}

/// Intel-specific CPU tweaks
fn get_intel_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "cpu_intel_ppm_mode".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "[Intel] Enable Processor Power Management".to_string(),
            description: "Ensures Intel PPM driver is enabled for proper power management.

Required for Turbo Boost and P-state transitions.
Set to Automatic (Start=3) for best balance."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\intelppm".to_string(),
                key: "Start".to_string(),
                expected_value: RegistryValue::DWord(3), // Automatic
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\intelppm".to_string(),
                key: "Start".to_string(),
                value: RegistryValue::DWord(1), // Only if user broke it
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\intelppm".to_string(),
                key: "Start".to_string(),
                value: RegistryValue::DWord(3),
            }],
        },
        Tweak {
            id: "cpu_intel_disable_tsx".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "[Intel] Disable TSX (Transactional Synchronization)".to_string(),
            description: "Disables Intel TSX feature that has known security vulnerabilities.

Also slightly improves performance on affected CPUs.
Recommended for Haswell through Coffee Lake."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                key: "DisableTsx".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                key: "DisableTsx".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                key: "DisableTsx".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
    ]
}

/// AMD-specific CPU tweaks
fn get_amd_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "cpu_amd_ppm_mode".to_string(),
        category: TweakCategory::CpuPerformance,
        name: "[AMD] Enable Processor Power Management".to_string(),
        description: "Ensures AMD PPM driver is enabled for proper power management.

Required for Precision Boost and P-state transitions on Ryzen."
            .to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: true,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::Registry {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Services\\amdppm".to_string(),
            key: "Start".to_string(),
            expected_value: RegistryValue::DWord(3),
        }),
        revert_operations: Some(vec![TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Services\\amdppm".to_string(),
            key: "Start".to_string(),
            value: RegistryValue::DWord(1),
        }]),
        operations: vec![TweakOperation::RegistrySet {
            root_key: "HKLM".to_string(),
            path: "SYSTEM\\CurrentControlSet\\Services\\amdppm".to_string(),
            key: "Start".to_string(),
            value: RegistryValue::DWord(3),
        }],
    }]
}
