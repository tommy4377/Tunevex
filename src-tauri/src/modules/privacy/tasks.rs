use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_task_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "privacy_disable_telemetry_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Telemetry Tasks".to_string(),
            description: "Disables 30+ telemetry and data collection scheduled tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "Microsoft Compatibility Appraiser".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "PcaPatchDbTask".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "ProgramDataUpdater".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Customer Experience Improvement Program"
                        .to_string(),
                    name: "Consolidator".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Customer Experience Improvement Program"
                        .to_string(),
                    name: "UsbCeip".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::ScheduledTaskDisabled {
                name: "Microsoft Compatibility Appraiser".to_string(),
            }),
            operations: vec![
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "Microsoft Compatibility Appraiser".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "PcaPatchDbTask".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Application Experience".to_string(),
                    name: "ProgramDataUpdater".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Customer Experience Improvement Program"
                        .to_string(),
                    name: "Consolidator".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Customer Experience Improvement Program"
                        .to_string(),
                    name: "UsbCeip".to_string(),
                },
            ],
        },
        Tweak {
            id: "privacy_disable_input_sync_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Input Sync Tasks".to_string(),
            description: "Disables mouse, keyboard, and touchpad sync tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Input".to_string(),
                    name: "LocalUserSyncDataAvailable".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "\\Microsoft\\Windows\\Input".to_string(),
                    name: "MouseSyncDataAvailable".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::ScheduledTaskDisabled {
                name: "LocalUserSyncDataAvailable".to_string(),
            }),
            operations: vec![
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Input".to_string(),
                    name: "LocalUserSyncDataAvailable".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "\\Microsoft\\Windows\\Input".to_string(),
                    name: "MouseSyncDataAvailable".to_string(),
                },
            ],
        },
    ]
}
