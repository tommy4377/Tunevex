use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_task_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "debloat_disable_misc_tasks".to_string(),
        category: TweakCategory::DebloatTelemetry,
        name: "Disable Misc Scheduled Tasks".to_string(),
        description: "Disables Maps, Speech, Language, Retail Demo, and other unused tasks."
            .to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\Maps".to_string(),
                name: "MapsToastTask".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\Maps".to_string(),
                name: "MapsUpdateTask".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\Speech".to_string(),
                name: "SpeechModelDownloadTask".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\LanguageComponentsInstaller".to_string(),
                name: "Installation".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\LanguageComponentsInstaller".to_string(),
                name: "ReconcileLanguageResources".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\RetailDemo".to_string(),
                name: "CleanupOfflineContent".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\SettingSync".to_string(),
                name: "NetworkStateChangeTask".to_string(),
            },
            TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\Windows\\RemoteAssistance".to_string(),
                name: "RemoteAssistanceTask".to_string(),
            },
        ]),
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::ScheduledTaskDisabled {
            name: "\\Microsoft\\Windows\\Maps\\MapsToastTask".to_string(),
        }),
        operations: vec![
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\Maps".to_string(),
                name: "MapsToastTask".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\Maps".to_string(),
                name: "MapsUpdateTask".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\Speech".to_string(),
                name: "SpeechModelDownloadTask".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\LanguageComponentsInstaller".to_string(),
                name: "Installation".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\LanguageComponentsInstaller".to_string(),
                name: "ReconcileLanguageResources".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\RetailDemo".to_string(),
                name: "CleanupOfflineContent".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\SettingSync".to_string(),
                name: "NetworkStateChangeTask".to_string(),
            },
            TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\Windows\\RemoteAssistance".to_string(),
                name: "RemoteAssistanceTask".to_string(),
            },
        ],
    }]
}
