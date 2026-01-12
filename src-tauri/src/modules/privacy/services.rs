use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "privacy_disable_telemetry_services".to_string(),
            category: TweakCategory::Privacy,
            name: "🔒 Disable Telemetry Services".to_string(),
            description: "Disables DiagTrack, dmwappushservice, diagnosticshub, and other telemetry services.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("DiagTrack", "dmwappushservice", "diagnosticshub.standardcollector.service", "WerSvc", "wercplsupport", "PcaSvc")
                    foreach ($svc in $services) {
                        Set-Service -Name $svc -StartupType Automatic -EA 0
                        Start-Service -Name $svc -EA 0
                    }
                    Write-Host "Telemetry services enabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-Service DiagTrack -ErrorAction SilentlyContinue).StartType -match 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("DiagTrack", "dmwappushservice", "diagnosticshub.standardcollector.service", "WerSvc", "wercplsupport", "PcaSvc")
                    foreach ($svc in $services) {
                        Stop-Service -Name $svc -Force -EA 0
                        Set-Service -Name $svc -StartupType Disabled -EA 0
                    }
                    Write-Host "Telemetry services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        }
    ]
}
