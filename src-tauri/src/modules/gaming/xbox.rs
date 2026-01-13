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
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
                    $services = @("XboxGipSvc", "XblAuthManager", "XboxNetApiSvc", "XblGameSave")
                    foreach ($svc in $services) { Set-Service -Name $svc -StartupType Manual -EA 0 }
                    Write-Host "Xbox services enabled (Manual)" -ForegroundColor Green
                "#
                .to_string(),
            }]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$gip = Get-Service -Name "XboxGipSvc" -EA 0
$auth = Get-Service -Name "XblAuthManager" -EA 0
if (($gip.StartType -eq 'Disabled') -and ($auth.StartType -eq 'Disabled')) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![TweakOperation::Powershell {
                script: r#"
                    $services = @("XboxGipSvc", "XblAuthManager", "XboxNetApiSvc", "XblGameSave")
                    foreach ($svc in $services) {
                        Stop-Service -Name $svc -Force -EA 0
                        Set-Service -Name $svc -StartupType Disabled -EA 0
                    }
                    Write-Host "Xbox services disabled" -ForegroundColor Green
                "#
                .to_string(),
            }],
        },
        Tweak {
            id: "gaming_disable_xbox_tasks".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Xbox Scheduled Tasks".to_string(),
            description: "Disables Xbox game save and related tasks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
                    schtasks /Change /TN '\Microsoft\XblGameSave\XblGameSaveTask' /Enable 2>$null
                    Write-Host "Xbox tasks enabled" -ForegroundColor Green
                "#
                .to_string(),
            }]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$task = Get-ScheduledTask -TaskName "XblGameSaveTask" -TaskPath "\Microsoft\XblGameSave\" -EA 0
if ($task.State -eq 'Disabled') { "True" } else { "False" }
"#
                .to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![TweakOperation::Powershell {
                script: r#"
                    schtasks /Change /TN '\Microsoft\XblGameSave\XblGameSaveTask' /Disable 2>$null
                    Write-Host "Xbox tasks disabled" -ForegroundColor Green
                "#
                .to_string(),
            }],
        },
    ]
}
