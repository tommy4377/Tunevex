use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_xbox_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gaming_disable_xbox_services".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Xbox Services".to_string(),
            description: "Disables Xbox-related services. WARNING: May affect some games!"
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode {
                    name: "XboxGipSvc".to_string(),
                    mode: "demand".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "XblAuthManager".to_string(),
                    mode: "demand".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "XboxNetApiSvc".to_string(),
                    mode: "demand".to_string(),
                },
                TweakOperation::ServiceSetMode {
                    name: "XblGameSave".to_string(),
                    mode: "demand".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec!["XboxGipSvc".to_string(), "XblAuthManager".to_string()],
            }),
            operations: vec![
                TweakOperation::ServiceDisable {
                    name: "XboxGipSvc".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "XblAuthManager".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "XboxNetApiSvc".to_string(),
                },
                TweakOperation::ServiceDisable {
                    name: "XblGameSave".to_string(),
                },
            ],
        },
        Tweak {
            id: "gaming_disable_xbox_tasks".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Xbox Scheduled Tasks".to_string(),
            description: "Disables Xbox game save and related tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::ScheduledTaskEnable {
                path: "\\Microsoft\\XblGameSave".to_string(),
                name: "XblGameSaveTask".to_string(),
            }]),
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::ScheduledTaskDisabled {
                name: "\\Microsoft\\XblGameSave\\XblGameSaveTask".to_string(),
            }),
            operations: vec![TweakOperation::ScheduledTaskDisable {
                path: "\\Microsoft\\XblGameSave".to_string(),
                name: "XblGameSaveTask".to_string(),
            }],
        },
    ]
}
