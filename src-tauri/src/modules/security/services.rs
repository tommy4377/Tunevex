use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "sec_disable_remote_services".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🔐 Disable Remote Access Services".to_string(),
            description: "Disables Remote Registry, Remote Access, WinRM. Improves security by reducing attack surface.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("RemoteRegistry", "RemoteAccess", "WinRM", "TermService", "SessionEnv")
                    foreach ($svc in $services) { Set-Service -Name $svc -StartupType Manual -EA 0 }
                    Write-Host "Remote services enabled (Manual)" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("RemoteRegistry", "RemoteAccess", "WinRM", "TermService", "SessionEnv")
                    foreach ($svc in $services) {
                        Stop-Service -Name $svc -Force -EA 0
                        Set-Service -Name $svc -StartupType Disabled -EA 0
                    }
                    Write-Host "Remote services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        }
    ]
}
