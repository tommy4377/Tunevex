use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

pub fn get_update_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "sec_disable_windows_update".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "⚠️ Disable Windows Update Services".to_string(),
            description: "Completely disables Windows Update services and tasks. WARNING: You will receive NO security updates!".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Write-Host "Enabling Windows Update..."
                    $services = @("wuauserv", "UsoSvc", "WaaSMedicSvc", "BITS", "DoSvc", "uhssvc", "InstallService")
                    foreach ($svc in $services) { Set-Service -Name $svc -StartupType Manual -EA 0; Start-Service -Name $svc -EA 0 }
                    $tasks = @(
                        '\Microsoft\Windows\InstallService\ScanForUpdates',
                        '\Microsoft\Windows\InstallService\ScanForUpdatesAsUser',
                        '\Microsoft\Windows\InstallService\SmartRetry',
                        '\Microsoft\Windows\UpdateOrchestrator\Schedule Scan',
                        '\Microsoft\Windows\WindowsUpdate\Scheduled Start'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Enable 2>$null }
                    $wuPath = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate"
                    Remove-ItemProperty -Path $wuPath -Name "DisableWindowsUpdateAccess" -ErrorAction SilentlyContinue
                    Write-Host "Windows Update enabled!" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$svc = Get-Service -Name "wuauserv" -EA 0
if ($svc.StartType -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Write-Host "Disabling Windows Update..."
                    $services = @("wuauserv", "UsoSvc", "WaaSMedicSvc", "BITS", "DoSvc", "uhssvc", "InstallService")
                    foreach ($svc in $services) {
                        Stop-Service -Name $svc -Force -EA 0
                        Set-Service -Name $svc -StartupType Disabled -EA 0
                    }
                    $tasks = @(
                        '\Microsoft\Windows\InstallService\ScanForUpdates',
                        '\Microsoft\Windows\InstallService\ScanForUpdatesAsUser',
                        '\Microsoft\Windows\InstallService\SmartRetry',
                        '\Microsoft\Windows\UpdateOrchestrator\Schedule Scan',
                        '\Microsoft\Windows\WindowsUpdate\Scheduled Start'
                    )
                    foreach ($task in $tasks) { schtasks /Change /TN $task /Disable 2>$null }
                    $wuPath = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate"
                    if (!(Test-Path $wuPath)) { New-Item -Path $wuPath -Force | Out-Null }
                    Set-ItemProperty -Path $wuPath -Name "DisableWindowsUpdateAccess" -Value 1 -Type DWord -Force
                    Write-Host "Windows Update disabled!" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        }
    ]
}
