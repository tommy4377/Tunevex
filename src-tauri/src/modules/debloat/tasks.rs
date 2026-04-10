// src-tauri/src/modules/debloat/tasks.rs
//
// BUG-M4 FIX: check field was TweakCheckScheduledTaskDisabled("") — empty string
// never matches anything, so the tweak always showed as "not applied".
// Fixed by using TweakCheckMultiScheduledTaskDisabled on the first two
// canonical task names (registry-based, no PowerShell).

use crate::modules::types::{Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel};

pub fn get_task_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat-disable-misc-tasks".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Misc Scheduled Tasks".to_string(),
            description: "Disables Maps, Speech, Language, Retail Demo, and other unused tasks."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            // BUG-M4 FIX: was TweakCheckScheduledTaskDisabled { name: "".to_string() }
            // which never matched anything.
            // Now checks the two most representative tasks via schtasks /Query — no empty string.
            check: Some(TweakCheck::MultiScheduledTaskDisabled {
                names: vec![
                    "MapsToastTask".to_string(),
                    "MapsUpdateTask".to_string(),
                ],
            }),

            revert_operations: Some(vec![
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "MapsToastTask".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "MapsUpdateTask".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "SpeechModelDownloadTask".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "Installation".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "ReconcileLanguageResources".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "CleanupOfflineContent".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "NetworkStateChangeTask".to_string(),
                },
                TweakOperation::ScheduledTaskEnable {
                    path: "".to_string(),
                    name: "RemoteAssistanceTask".to_string(),
                },
            ]),

            operations: vec![
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "MapsToastTask".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "MapsUpdateTask".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "SpeechModelDownloadTask".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "Installation".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "ReconcileLanguageResources".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "CleanupOfflineContent".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "NetworkStateChangeTask".to_string(),
                },
                TweakOperation::ScheduledTaskDisable {
                    path: "".to_string(),
                    name: "RemoteAssistanceTask".to_string(),
                },
            ],
        },
    ]
}