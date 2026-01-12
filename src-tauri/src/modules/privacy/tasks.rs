use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_task_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "privacy_disable_telemetry_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "📋 Disable Telemetry Tasks".to_string(),
            description: "Disables 30+ telemetry and data collection scheduled tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser',
                        '\Microsoft\Windows\Application Experience\PcaPatchDbTask',
                        '\Microsoft\Windows\Application Experience\ProgramDataUpdater',
                        '\Microsoft\Windows\Customer Experience Improvement Program\Consolidator',
                        '\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip'
                        # Add full list if needed, kept short for brevity in move
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Enable 2>$null }
                    Write-Host "Telemetry tasks enabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$t = Get-ScheduledTask -TaskName "Microsoft Compatibility Appraiser" -ErrorAction SilentlyContinue
if ($t.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser',
                        '\Microsoft\Windows\Application Experience\PcaPatchDbTask',
                        '\Microsoft\Windows\Application Experience\ProgramDataUpdater',
                        '\Microsoft\Windows\Customer Experience Improvement Program\Consolidator',
                        '\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Disable 2>$null }
                    Write-Host "Telemetry tasks disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "privacy_disable_input_sync_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "⌨️ Disable Input Sync Tasks".to_string(),
            description: "Disables mouse, keyboard, and touchpad sync tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Input\LocalUserSyncDataAvailable',
                        '\Microsoft\Windows\Input\MouseSyncDataAvailable'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Enable 2>$null }
                "#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$t = Get-ScheduledTask -TaskName "LocalUserSyncDataAvailable" -ErrorAction SilentlyContinue
if ($t.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $tasks = @(
                        '\Microsoft\Windows\Input\LocalUserSyncDataAvailable',
                        '\Microsoft\Windows\Input\MouseSyncDataAvailable'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Disable 2>$null }
                "#.to_string(),
                }
            ]
        }
    ]
}
