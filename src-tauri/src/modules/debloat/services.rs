use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat_disable_misc_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Miscellaneous Services".to_string(),
            description: "Disables unused services: WMP, Maps, Fax, RetailDemo, Wallet, Phone, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("WMPNetworkSvc", "MapsBroker", "Fax", "RetailDemo", "WalletService", "PhoneSvc", "TapiSrv")
                    foreach ($svc in $services) { Set-Service -Name $svc -StartupType Manual -EA 0 }
                    "#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$fax = Get-Service Fax -ErrorAction SilentlyContinue
$wmp = Get-Service WMPNetworkSvc -ErrorAction SilentlyContinue
if (($fax.StartType -eq 'Disabled') -and ($wmp.StartType -eq 'Disabled')) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("WMPNetworkSvc", "MapsBroker", "Fax", "RetailDemo", "WalletService", "PhoneSvc", "TapiSrv")
                    foreach ($svc in $services) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
                    Write-Host "Misc services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ],
            tweak_type: TweakType::Toggle,
            enabled: false,
        },
        Tweak {
            id: "debloat_disable_edge_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Update Services".to_string(),
            description: "Disables Microsoft Edge update services.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "MicrosoftEdgeElevationService" -StartupType Manual -EA 0
                    Set-Service -Name "edgeupdate" -StartupType Automatic -EA 0
                    Set-Service -Name "edgeupdatem" -StartupType Manual -EA 0
                    "#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false, check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-Service edgeupdate -ErrorAction SilentlyContinue).StartType -match 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("MicrosoftEdgeElevationService", "edgeupdate", "edgeupdatem")
                    foreach ($svc in $services) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
                    Write-Host "Edge services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "debloat_disable_printer_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Printer Services".to_string(),
            description: "Disables Print Spooler. Only if not using printers!".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "Spooler" -StartupType Automatic -EA 0
                    Start-Service -Name "Spooler" -EA 0
                    "#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false, check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-Service Spooler -ErrorAction SilentlyContinue).StartType -match 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Stop-Service -Name "Spooler" -Force -EA 0; Set-Service -Name "Spooler" -StartupType Disabled -EA 0
                    Write-Host "Printer services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "debloat_disable_bluetooth_services".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Bluetooth Services".to_string(),
            description: "Disables Bluetooth services. Only if not using Bluetooth!".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("BTAGService", "bthserv")
                    foreach ($svc in $services) { Set-Service -Name $svc -StartupType Manual -EA 0 }
                    "#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false, check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-Service bthserv -ErrorAction SilentlyContinue).StartType -match 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    $services = @("BTAGService", "bthserv")
                    foreach ($svc in $services) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
                    Write-Host "Bluetooth services disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        }
    ]
}
