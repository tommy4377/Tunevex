//! Storage Power Management Tweaks
//!
//! NVMe and storage power optimization tweaks moved from CPU module

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_storage_power_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "storage_nvme_idle_timeout".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NVMe Idle Timeout".to_string(),
            description:
                "Sets NVMe idle timeout to 0ms to prevent drive from entering low-power mode."
                    .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec![
                    "/query".to_string(),
                    "scheme_current".to_string(),
                    "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                    "d3d55efd-c1ff-424e-9dc3-441be7833010".to_string(),
                ],
                contains: "Current AC Power Setting Index: 0x00000000".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "d3d55efd-c1ff-424e-9dc3-441be7833010".to_string(),
                        "100".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "d639518a-e56d-4345-8af2-b9f32fb26109".to_string(),
                        "100".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec![
                        "/setacvalueindex".to_string(),
                        "scheme_current".to_string(),
                        "0012ee47-9041-4b5d-9b77-535fba8b1442".to_string(),
                        "fc7372b6-ab2d-43ee-8797-15e9841f2cca".to_string(),
                        "1".to_string(),
                    ],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ]),
            operations: vec![
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
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "scheme_current".to_string()],
                },
            ],
        },
        Tweak {
            id: "storage_d3_standby".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable Storage D3 in Modern Standby".to_string(),
            description:
                "Prevents storage devices from entering D3 cold state during Modern Standby."
                    .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::MultiRegistry {
                checks: vec![
                    crate::modules::types::RegistryCheck {
                        root_key: "HKLM".to_string(),
                        path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                        key: "StorageD3InModernStandby".to_string(),
                        expected_value: RegistryValue::DWord(0),
                    },
                    crate::modules::types::RegistryCheck {
                        root_key: "HKLM".to_string(),
                        path: "SYSTEM\\CurrentControlSet\\Services\\stornvme\\Parameters\\Device"
                            .to_string(),
                        key: "IdlePowerMode".to_string(),
                        expected_value: RegistryValue::DWord(0),
                    },
                ],
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                    key: "StorageD3InModernStandby".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\stornvme\\Parameters\\Device"
                        .to_string(),
                    key: "IdlePowerMode".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Storage".to_string(),
                    key: "StorageD3InModernStandby".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\stornvme\\Parameters\\Device"
                        .to_string(),
                    key: "IdlePowerMode".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
    ]
}
