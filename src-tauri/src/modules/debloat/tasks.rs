use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_task_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "debloat_disable_misc_tasks".to_string(),
        category: TweakCategory::DebloatTelemetry,
        name: "📅 Disable Misc Scheduled Tasks".to_string(),
        description: "Disables Maps, Speech, Language, Retail Demo, and other unused tasks."
            .to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Maps\MapsToastTask',
                        '\Microsoft\Windows\Maps\MapsUpdateTask',
                        '\Microsoft\Windows\Speech\SpeechModelDownloadTask',
                        '\Microsoft\Windows\LanguageComponentsInstaller\Installation',
                        '\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources',
                        '\Microsoft\Windows\RetailDemo\CleanupOfflineContent',
                        '\Microsoft\Windows\SettingSync\NetworkStateChangeTask',
                        '\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Enable 2>$null }
                    Write-Host "Miscellaneous tasks enabled" -ForegroundColor Green
                "#
            .to_string(),
        }]),
        enabled: false,
        check: None,
        operations: vec![TweakOperation::Powershell {
            script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Maps\MapsToastTask',
                        '\Microsoft\Windows\Maps\MapsUpdateTask',
                        '\Microsoft\Windows\Speech\SpeechModelDownloadTask',
                        '\Microsoft\Windows\LanguageComponentsInstaller\Installation',
                        '\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources',
                        '\Microsoft\Windows\RetailDemo\CleanupOfflineContent',
                        '\Microsoft\Windows\SettingSync\NetworkStateChangeTask',
                        '\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Disable 2>$null }
                    Write-Host "Miscellaneous tasks disabled" -ForegroundColor Green
                "#
            .to_string(),
        }],
    }]
}
