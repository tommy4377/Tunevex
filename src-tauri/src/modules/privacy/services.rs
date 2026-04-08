use crate::modules::types::{Tweak, TweakCategory, TweakOperation, TweakType, WarningLevel};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "privacy_disable_telemetry_services".to_string(),
        category: TweakCategory::Privacy,
        name: "Disable Telemetry Services".to_string(),
        description:
            "Disables DiagTrack, dmwappushservice, diagnosticshub, and other telemetry services."
                .to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: true,
        revert_operations: Some(vec![
            TweakOperation::ServiceSetMode {
                name: "DiagTrack".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::ServiceSetMode {
                name: "dmwappushservice".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::ServiceSetMode {
                name: "diagnosticshub.standardcollector.service".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::ServiceSetMode {
                name: "WerSvc".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::ServiceSetMode {
                name: "wercplsupport".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::ServiceSetMode {
                name: "PcaSvc".to_string(),
                mode: "Auto".to_string(),
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec!["start".to_string(), "DiagTrack".to_string()],
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec!["start".to_string(), "dmwappushservice".to_string()],
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec![
                    "start".to_string(),
                    "diagnosticshub.standardcollector.service".to_string(),
                ],
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec!["start".to_string(), "WerSvc".to_string()],
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec!["start".to_string(), "wercplsupport".to_string()],
            },
            TweakOperation::Command {
                cmd: "sc".to_string(),
                args: vec!["start".to_string(), "PcaSvc".to_string()],
            },
        ]),
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(crate::modules::types::TweakCheck::MultiServiceDisabled {
            names: vec![
                "DiagTrack".to_string(),
                "dmwappushservice".to_string(),
                "diagnosticshub.standardcollector.service".to_string(),
                "WerSvc".to_string(),
                "wercplsupport".to_string(),
                "PcaSvc".to_string(),
            ],
        }),
        operations: vec![
            TweakOperation::ServiceDisable {
                name: "DiagTrack".to_string(),
            },
            TweakOperation::ServiceDisable {
                name: "dmwappushservice".to_string(),
            },
            TweakOperation::ServiceDisable {
                name: "diagnosticshub.standardcollector.service".to_string(),
            },
            TweakOperation::ServiceDisable {
                name: "WerSvc".to_string(),
            },
            TweakOperation::ServiceDisable {
                name: "wercplsupport".to_string(),
            },
            TweakOperation::ServiceDisable {
                name: "PcaSvc".to_string(),
            },
        ],
    }]
}
