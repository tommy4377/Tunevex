use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat_disable_misc_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Miscellaneous Services".to_string(),
            description:
                "Disables unused services: WMP, Maps, Fax, RetailDemo, Wallet, Phone, etc."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode {
                    name: "WMPNetworkSvc".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "MapsBroker".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "Fax".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "RetailDemo".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "WalletService".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "PhoneSvc".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "TapiSrv".to_string(),
                    mode: "Manual".to_string(),
                },
            ]),
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec![
                    "WMPNetworkSvc".to_string(),
                    "MapsBroker".to_string(),
                    "Fax".to_string(),
                    "RetailDemo".to_string(),
                    "WalletService".to_string(),
                    "PhoneSvc".to_string(),
                    "TapiSrv".to_string(),
                ],
            }),
            operations: vec![
                TweakOperation::ServiceDisable {
                    name: "WMPNetworkSvc".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "MapsBroker".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "Fax".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "RetailDemo".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "WalletService".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "PhoneSvc".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "TapiSrv".to_string(),
                },
            ],
            tweak_type: TweakType::Toggle,
            enabled: false,
        },
        Tweak {
            id: "debloat_disable_edge_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Update Services".to_string(),
            description: "Disables Microsoft Edge update services.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode {
                    name: "MicrosoftEdgeElevationService".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "edgeupdate".to_string(),
                    mode: "Auto".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "edgeupdatem".to_string(),
                    mode: "Manual".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec![
                    "MicrosoftEdgeElevationService".to_string(),
                    "edgeupdate".to_string(),
                    "edgeupdatem".to_string(),
                ],
            }),
            operations: vec![
                TweakOperation::ServiceDisable {
                    name: "MicrosoftEdgeElevationService".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "edgeupdate".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "edgeupdatem".to_string(),
                },
            ],
        },
        Tweak {
            id: "debloat_disable_bluetooth_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Bluetooth Services".to_string(),
            description: "Disables Bluetooth services. Only if not using Bluetooth!".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode {
                    name: "BTAGService".to_string(),
                    mode: "Manual".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "bthserv".to_string(),
                    mode: "Manual".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec!["BTAGService".to_string(), "bthserv".to_string()],
            }),
            operations: vec![
                TweakOperation::ServiceDisable {
                    name: "BTAGService".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "bthserv".to_string(),
                },
            ],
        },
        Tweak {
            id: "debloat_disable_printer_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Printer Services".to_string(),
            description: "Disables Print Spooler. Only if not using printers!".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode {
                    name: "Spooler".to_string(),
                    mode: "Auto".to_string(),
                },
                TweakOperation::Command {
                    cmd: "sc".to_string(),
                    args: vec!["start".to_string(), "Spooler".to_string()],
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::ServiceDisabled {
                name: "Spooler".to_string(),
            }),
            operations: vec![
                TweakOperation::ServiceDisable {
                    name: "Spooler".to_string(),
                },
                TweakOperation::Command {
                    cmd: "sc".to_string(),
                    args: vec!["stop".to_string(), "Spooler".to_string()],
                },
            ],
        },
    ]
}
