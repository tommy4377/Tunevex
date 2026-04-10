//! CPU Power Management Tweaks
//!
//! Based on:
//! - DisablePowerSaving.ps1 from AtlasOS
//! - CPU Idle scripts
//! - Various power optimization sources
//!
//! All operations use native powercfg.exe via TweakOperation::Command and
//! TweakCheck::CommandOutputContains — zero PowerShell spawns.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns all CPU power management tweaks
pub fn get_power_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Ultimate Performance Power Plan
        // ============================================
        Tweak {
            id: "cpu_ultimate_performance".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Ultimate Performance Power Plan".to_string(),
            description: "Enables Windows' hidden Ultimate Performance power plan for maximum performance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Revert: restore Balanced plan (GUID 381b4222-…)
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setactive".to_string(),
                        "381b4222-f694-41f0-9685-ff5bb260df2e".to_string(),
                    ],
                },
            ]),
            // Check: is Ultimate Performance (e9a42b02-…) the active scheme?
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec!["/getactivescheme".to_string()],
                contains: "e9a42b02-d5df-448d-aa00-03f14749eb61".to_string(),
            }),
            // Apply: duplicate + activate Ultimate Performance
            // powercfg will print the new GUID; we then set it active.
            // We attempt activation of the well-known GUID first; if it already
            // exists powercfg returns it directly. If not, duplicate creates it.
            operations: vec![
                // Step 1: duplicate the Ultimate Performance scheme (creates if missing)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/duplicatescheme".to_string(),
                        "e9a42b02-d5df-448d-aa00-03f14749eb61".to_string(),
                    ],
                },
                // Step 2: activate the (now existing) Ultimate Performance scheme
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setactive".to_string(),
                        "e9a42b02-d5df-448d-aa00-03f14749eb61".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Disable USB Selective Suspend
        // ============================================
        Tweak {
            id: "cpu_disable_usb_suspend".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable USB Selective Suspend".to_string(),
            description: "Prevents USB devices from powering down. Fixes mouse/keyboard lag issues.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ]),
            // Check: AC index == 0x00000000 (disabled)
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "SCHEME_CURRENT".to_string(),
                    "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                    "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                ],
                contains: "0x00000000".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ],
        },

        // ============================================
        // Disable PCIe Link State Power Management
        // ============================================
        Tweak {
            id: "cpu_disable_pcie_lpm".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable PCIe Link State Power Management".to_string(),
            description: "Prevents PCIe devices (GPU, NVMe) from entering low-power states.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "501a4d13-42af-4429-9fd1-a8218c268e20".to_string(),
                        "ee12f906-d277-404b-b6da-e5fa1a576df5".to_string(),
                        "2".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "501a4d13-42af-4429-9fd1-a8218c268e20".to_string(),
                        "ee12f906-d277-404b-b6da-e5fa1a576df5".to_string(),
                        "2".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ]),
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "SCHEME_CURRENT".to_string(),
                    "501a4d13-42af-4429-9fd1-a8218c268e20".to_string(),
                    "ee12f906-d277-404b-b6da-e5fa1a576df5".to_string(),
                ],
                contains: "0x00000000".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "501a4d13-42af-4429-9fd1-a8218c268e20".to_string(),
                        "ee12f906-d277-404b-b6da-e5fa1a576df5".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "501a4d13-42af-4429-9fd1-a8218c268e20".to_string(),
                        "ee12f906-d277-404b-b6da-e5fa1a576df5".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ],
        },

        // ============================================
        // Disable CPU Core Parking
        // ============================================
        Tweak {
            id: "cpu_disable_core_parking".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Core Parking".to_string(),
            description: "Prevents Windows from parking CPU cores. Removes latency from unparking cores during gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                        "5".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                        "5".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ]),
            // Check: AC index == 0x00000064 (100 decimal = 100% = no parking)
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "SCHEME_CURRENT".to_string(),
                    "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                    "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                ],
                contains: "0x00000064".to_string(),
            }),
            operations: vec![
                // Make core parking attribute visible in Power Options
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                    key: "Attributes".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Set min cores to 100% (no parking)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setacvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                        "100".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setdcvalueindex".to_string(),
                        "SCHEME_CURRENT".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                        "100".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["-setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ],
        },

        // ============================================
        // CPU Idle Disable
        // ============================================
        Tweak {
            id: "cpu_disable_idle".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Idle States".to_string(),
            description: "Forces CPU to maximum speed always. NOT recommended with HyperThreading/SMT. Ensure good cooling!".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "sub_processor".to_string(),
                        "5d76a2ca-e8c0-402f-a133-2158492d58ad".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ]),
            // Check: AC index == 0x00000001 (idle disabled = value 1)
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "SCHEME_CURRENT".to_string(),
                    "sub_processor".to_string(),
                    "5d76a2ca-e8c0-402f-a133-2158492d58ad".to_string(),
                ],
                contains: "0x00000001".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "sub_processor".to_string(),
                        "5d76a2ca-e8c0-402f-a133-2158492d58ad".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ],
        },

        // ============================================
        // USB 3 Link Power Management
        // ============================================
        Tweak {
            id: "cpu_usb3_link_power".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable USB 3 Link Power Management".to_string(),
            description: "Sets USB 3 Link Power Management to maximum performance. Prevents USB device latency spikes and disconnections.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Check: AC index == 0x00000003 (Maximum Performance)
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "scheme_current".to_string(),
                    "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                    "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                ],
                contains: "0x00000003".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ]),
            operations: vec![
                // USB 3 Link Power → Maximum Performance (3)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                        "3".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                        "3".to_string(),
                    ],
                },
                // USB Selective Suspend → Disabled (0)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "0".to_string(),
                    ],
                },
                // USB Hub Selective Suspend Timeout → 0 ms
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "0853a681-27c8-4100-a2fd-82013e970683".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setdcvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "0853a681-27c8-4100-a2fd-82013e970683".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ],
        },

        // ============================================
        // Disable Throttle States
        // ============================================
        Tweak {
            id: "cpu_disable_throttle_states".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Throttle States".to_string(),
            description: "Disables CPU throttle states (T-states) for consistent performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ]),
            // Check: AC index == 0x00000000 (throttle states off)
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/q".to_string(),
                    "SCHEME_CURRENT".to_string(),
                    "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                    "3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb".to_string(),
                ],
                contains: "0x00000000".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ],
        },

        // ============================================
        // Disable ACPI Power-Saving Devices
        // ============================================
        // Note: PnP device enable/disable cannot be done with powercfg or a simple
        // single-binary call without PInvoke. We use devcon.exe (part of WDK) via
        // Command if present, or gracefully fall back.  The check reads the device
        // status from the registry (HKLM\SYSTEM\...\Enum\ACPI\...) which avoids PS.
        Tweak {
            id: "cpu_disable_acpi_devices".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Power-Saving ACPI Devices".to_string(),
            description: "Disables ACPI Processor Aggregator and other power-saving system devices.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Revert: re-enable the aggregator device via devcon
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "pnputil".to_string(),
                    args: vec![
                        "/enable-device".to_string(),
                        "ACPI\\ACPI000C".to_string(),
                    ],
                },
            ]),
            // Check: the processor aggregator device is NOT present/enabled.
            // Registry key is absent when device is disabled or not present.
            check: Some(TweakCheck::RegistryKeyAbsent {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Enum\\ACPI\\ACPI000C".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "pnputil".to_string(),
                    args: vec![
                        "/disable-device".to_string(),
                        "ACPI\\ACPI000C".to_string(),
                    ],
                },
            ],
        },

        // ============================================
        // Comprehensive Power-Saving Disable
        // ============================================
        Tweak {
            id: "cpu_disable_all_power_saving".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable All Power-Saving Features".to_string(),
            description: "Comprehensive power-saving disable: NVMe idle, USB3 link power, throttle states, device D3, EEE.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            revert_operations: Some(vec![
                // Delete the custom power plan created by duplicatescheme
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/deletescheme".to_string(),
                        "11111111-1111-1111-1111-111111111111".to_string(),
                    ],
                },
                // Best effort: restore Balanced plan
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "-setactive".to_string(),
                        "381b4222-f694-41f0-9685-ff5bb260df2e".to_string(),
                    ],
                },
                // Restore StorageD3InModernStandby to default (1 = enabled)
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                    key: "StorageD3InModernStandby".to_string(),
                    value: RegistryValue::DWord(1),
                },
                // Restore IdlePowerMode (delete to return to default)
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                    key: "IdlePowerMode".to_string(),
                },
            ]),
            // Check: StorageD3InModernStandby == 0 (storage D3 disabled)
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                key: "StorageD3InModernStandby".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                // 1. Duplicate + activate Ultimate Performance scheme
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/duplicatescheme".to_string(),
                        "e9a42b02-d5df-448d-aa00-03f14749eb61".to_string(),
                        "11111111-1111-1111-1111-111111111111".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setactive".to_string(),
                        "11111111-1111-1111-1111-111111111111".to_string(),
                    ],
                },
                // 2. NVMe Idle Timeout → 0 ms
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "d3d55efd-c1ff-424e-9dc3-441be7833010".to_string(),
                        "0".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "d639518a-e56d-4345-8af2-b9f32fb26109".to_string(),
                        "0".to_string(),
                    ],
                },
                // NVMe NOPPME → Off
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "fc7372b6-ab2d-43ee-8797-15e9841f2cca".to_string(),
                        "0".to_string(),
                    ],
                },
                // USB Hub Selective Suspend Timeout → 0 ms
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "0853a681-27c8-4100-a2fd-82013e970683".to_string(),
                        "0".to_string(),
                    ],
                },
                // USB Selective Suspend → Disabled
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226".to_string(),
                        "0".to_string(),
                    ],
                },
                // USB 3 Link Power Management → Off (0)
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "2a737441-1930-4402-8d77-b2bebba308a3".to_string(),
                        "d4e98f31-5ffe-4ce1-be31-1b38b384c009".to_string(),
                        "0".to_string(),
                    ],
                },
                // Allow Throttle States → Off
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb".to_string(),
                        "0".to_string(),
                    ],
                },
                // Processor time check interval → 200 ms
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                        "4d2b0152-7d5c-498b-88e2-34345392a2c5".to_string(),
                        "200".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
                // D3 support for NVMe — disable via registry
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                    key: "StorageD3InModernStandby".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // NVMe idle power mode → 0
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\stornvme\\Parameters\\Device".to_string(),
                    key: "IdlePowerMode".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
    ]
}
