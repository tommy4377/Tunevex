//! App removal tweaks (Microsoft, Third-Party, OEM bloatware)

use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakOperation, WarningLevel, TweakCheck, RegistryValue};

fn create_bloatware_removal_script(apps: &[&str]) -> String {
    let app_list = apps.iter()
        .map(|a| format!("'{}'", a))
        .collect::<Vec<_>>()
        .join(", ");
    
    format!(r#"
$ErrorActionPreference = "SilentlyContinue"
$apps = @({})
$removed = 0
$failed = 0
$notFound = 0

Write-Host "Starting bloatware removal..." -ForegroundColor Cyan

foreach ($appPattern in $apps) {{
    Write-Host "Processing: $appPattern" -ForegroundColor White
    
    # Try to remove installed package
    $packages = Get-AppxPackage -AllUsers | Where-Object {{ $_.Name -like "*$appPattern*" }}
    
    if ($packages) {{
        foreach ($pkg in $packages) {{
            try {{
                Remove-AppxPackage -Package $pkg.PackageFullName -AllUsers -ErrorAction Stop
                $removed++
                Write-Host "  ✓ Removed: $($pkg.Name)" -ForegroundColor Green
            }} catch {{
                $failed++
                Write-Host "  ✗ Failed: $($pkg.Name) - $($_.Exception.Message)" -ForegroundColor Red
            }}
        }}
    }} else {{
        $notFound++
    }}
    
    # Also remove provisioned package (prevents reinstall for new users)
    $provisioned = Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like "*$appPattern*" }}
    
    if ($provisioned) {{
        foreach ($prov in $provisioned) {{
            try {{
                Remove-AppxProvisionedPackage -Online -PackageName $prov.PackageName -ErrorAction Stop | Out-Null
                Write-Host "  ✓ Deprovisioned: $($prov.DisplayName)" -ForegroundColor Green
            }} catch {{
                Write-Host "  ✗ Deprovision failed: $($prov.DisplayName)" -ForegroundColor Yellow
            }}
        }}
    }}
}}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Removed: $removed" -ForegroundColor Green
Write-Host "Failed: $failed" -ForegroundColor $(if ($failed -gt 0) {{ "Red" }} else {{ "Green" }})
Write-Host "Not found: $notFound" -ForegroundColor Yellow
"#, app_list)
}

pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Microsoft Bloatware
        // ============================================
        Tweak {
            id: "debloat_ms_common".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🧹 Remove Common Microsoft Bloatware".to_string(),
            description: "Removes Clipchamp, 3DBuilder, BingNews, Solitaire, Skype, Zune, StickyNotes, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Microsoft apps must be reinstalled from Microsoft Store manually." -ForegroundColor Yellow
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*Solitaire*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "Clipchamp.Clipchamp","Microsoft.3DBuilder","Microsoft.549981C3F5F10",
                        "Microsoft.Windows.Ai.Copilot.Provider","Microsoft.WindowsBackup","Microsoft.Paint",
                        "Microsoft.BingFinance","Microsoft.BingFoodAndDrink","Microsoft.BingHealthAndFitness",
                        "Microsoft.BingNews","Microsoft.BingSports","Microsoft.BingTranslator",
                        "Microsoft.BingTravel","Microsoft.BingWeather","Microsoft.GetHelp","Microsoft.Getstarted",
                        "Microsoft.Messaging","Microsoft.Microsoft3DViewer","Microsoft.MicrosoftJournal",
                        "Microsoft.MicrosoftOfficeHub","Microsoft.MicrosoftPowerBIForWindows",
                        "Microsoft.MicrosoftSolitaireCollection","Microsoft.MicrosoftStickyNotes",
                        "Microsoft.MixedReality.Portal","Microsoft.NetworkSpeedTest","Microsoft.News",
                        "Microsoft.Office.OneNote","Microsoft.Office.Sway","Microsoft.OneConnect",
                        "Microsoft.PowerAutomateDesktop","Microsoft.Print3D","Microsoft.SkypeApp",
                        "Microsoft.Todos","Microsoft.Windows.DevHome","Microsoft.WindowsAlarms",
                        "Microsoft.WindowsFeedbackHub","Microsoft.WindowsMaps","Microsoft.WindowsSoundRecorder",
                        "Microsoft.XboxApp","Microsoft.ZuneMusic","Microsoft.ZuneVideo","MicrosoftCorporationII.MicrosoftFamily",
                        "MicrosoftCorporationII.QuickAssist","MicrosoftTeams","MSTeams"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_ms_comm".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Remove Mail, Calendar & People".to_string(),
            description: "Removes Windows Mail, Calendar, and People apps.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*windowscommunicationsapps*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "microsoft.windowscommunicationsapps",
                        "Microsoft.People"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_ms_outlook".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Remove New Outlook for Windows".to_string(),
            description: "Removes the new Outlook app.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*OutlookForWindows*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "Microsoft.OutlookForWindows"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_ms_phonelink".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Remove Phone Link".to_string(),
            description: "Removes Phone Link (Your Phone) app.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*YourPhone*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "Microsoft.YourPhone",
                        "MicrosoftWindows.CrossDevice"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_onedrive".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "☁️ Remove OneDrive".to_string(),
            description: "Completely removes Microsoft OneDrive from the system.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "OneDrive can be reinstalled from:" -ForegroundColor Yellow
Write-Host "https://www.microsoft.com/en-us/microsoft-365/onedrive/download" -ForegroundColor Cyan
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Test-Path "$env:LOCALAPPDATA\Microsoft\OneDrive\OneDrive.exe")) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Removing OneDrive..." -ForegroundColor Yellow

# Stop OneDrive
taskkill /f /im OneDrive.exe 2>$null

# Uninstall OneDrive
if (Test-Path "$env:SystemRoot\System32\OneDriveSetup.exe") {
    & "$env:SystemRoot\System32\OneDriveSetup.exe" /uninstall
} elseif (Test-Path "$env:SystemRoot\SysWOW64\OneDriveSetup.exe") {
    & "$env:SystemRoot\SysWOW64\OneDriveSetup.exe" /uninstall
}

# Remove leftover folders
Remove-Item "$env:USERPROFILE\OneDrive" -Recurse -Force -EA 0
Remove-Item "$env:LOCALAPPDATA\Microsoft\OneDrive" -Recurse -Force -EA 0
Remove-Item "$env:PROGRAMDATA\Microsoft OneDrive" -Recurse -Force -EA 0
Remove-Item "C:\OneDriveTemp" -Recurse -Force -EA 0

# Remove from Explorer
reg delete "HKCR\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}" /f 2>$null
reg delete "HKCR\Wow6432Node\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}" /f 2>$null

Write-Host "OneDrive removed" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Third-Party Bloatware
        // ============================================
        Tweak {
            id: "debloat_thirdparty".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🎮 Remove Third-Party Bloatware".to_string(),
            description: "Removes Spotify, Netflix, Disney+, TikTok, Facebook, Instagram, Candy Crush, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*Spotify*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "CorsairiCUE","ACGMediaPlayer","ActiproSoftwareLLC","AdobeSystemsIncorporated.AdobePhotoshopExpress",
                        "Amazon.com.Amazon","AmazonVideo.PrimeVideo","Asphalt8Airborne","AutodeskSketchBook",
                        "CaesarsSlotsFreeCasino","COOKINGFEVER","CyberLinkMediaSuiteEssentials",
                        "DisneyMagicKingdoms","Disney","DrawboardPDF","Duolingo-LearnLanguagesforFree",
                        "EclipseManager","Facebook","FarmVille2CountryEscape","fitbit","Flipboard",
                        "HiddenCity","HULULLC.HULUPLUS","iHeartRadio","Instagram",
                        "king.com.BubbleWitch3Saga","king.com.CandyCrushSaga","king.com.CandyCrushSodaSaga",
                        "LinkedInforWindows","MarchofEmpires","Netflix","NYTCrossword","OneCalendar",
                        "PandoraMediaInc","PhototasticCollage","PicsArt-PhotoStudio","Plex",
                        "PolarrPhotoEditorAcademicEdition","Royal Revolt","Shazam","Sidia.LiveWallpaper",
                        "SlingTV","Spotify","TikTok","TuneInRadio","Twitter","Viber","WinZipUniversal",
                        "Wunderlist","XING"
                    ]),
                }
            ]
        },
        
        // ============================================
        // OEM Bloatware
        // ============================================
        Tweak {
            id: "debloat_hp".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove HP Bloatware".to_string(),
            description: "Removes HP pre-installed apps: Support Assistant, JumpStarts, MyHP, WorkWell, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "HP apps can be reinstalled from HP website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*HPSupportAssistant*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "AD2F1837.HPAIExperienceCenter","AD2F1837.HPConnectedMusic",
                        "AD2F1837.HPConnectedPhotopoweredbySnapfish","AD2F1837.HPDesktopSupportUtilities",
                        "AD2F1837.HPEasyClean","AD2F1837.HPFileViewer","AD2F1837.HPJumpStarts",
                        "AD2F1837.HPPCHardwareDiagnosticsWindows","AD2F1837.HPPowerManager",
                        "AD2F1837.HPPrinterControl","AD2F1837.HPPrivacySettings","AD2F1837.HPQuickDrop",
                        "AD2F1837.HPQuickTouch","AD2F1837.HPRegistration","AD2F1837.HPSupportAssistant",
                        "AD2F1837.HPSureShieldAI","AD2F1837.HPSystemInformation","AD2F1837.HPWelcome",
                        "AD2F1837.HPWorkWell","AD2F1837.myHP"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_dell".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove Dell Bloatware".to_string(),
            description: "Removes Dell pre-installed apps: SupportAssist, Digital Delivery, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Dell apps can be reinstalled from Dell website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*DellSupportAssistforPCs*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "DellInc.DellSupportAssistforPCs","DellInc.DellDigitalDelivery",
                        "DellInc.DellCommandUpdate","DellInc.DellPowerManager",
                        "DellInc.PartnerPromo","DellInc.DellCustomerConnect"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_lenovo".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove Lenovo Bloatware".to_string(),
            description: "Removes Lenovo pre-installed apps: Vantage, Companion, etc.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Lenovo apps can be reinstalled from Lenovo website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*LenovoVantage*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "E046963F.LenovoCompanion","E046963F.LenovoSettings",
                        "E0469640.LenovoUtility","LenovoCorporation.LenovoID",
                        "LenovoCorporation.LenovoVantage","LenovoCorporation.LenovoSettings"
                    ]),
                }
            ]
        },
        
        Tweak {
            id: "debloat_asus".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove ASUS Bloatware".to_string(),
            description: "Removes ASUS pre-installed apps: Armoury Crate, ROG, MyASUS, etc.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "ASUS apps can be reinstalled from ASUS website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*ArmouryCrate*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "B9ECED6F.ArmouryCrate","B9ECED6F.ASUSROGLiveService",
                        "ASUSTeK COMPUTER INC.MyASUS","B9ECED6F.arabormonicux"
                    ]),
                }
            ]
        },

        // MSI Bloatware
        Tweak {
            id: "debloat_msi".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove MSI Bloatware".to_string(),
            description: "Removes MSI Center, Dragon Center, Live Update, SDK, Sound Tune, True Color.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall MSI apps from MSI website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*MSICenter*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "MSI.CenterPortal","MicroStarINT","MicroStarInt.MSICenter",
    "MSI Center","MSI Dragon Center","MSI Live Update",
    "MSI SDK","MSI Sound Tune","MSI True Color"
)
Write-Host "Removing MSI bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    # AppX
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
    
    # Win32 (Basic search)
    Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | ForEach-Object { 
        $_.Uninstall(); $removed++ 
    }
}
Write-Host "Removed $removed MSI apps" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },

        // Acer Bloatware
        Tweak {
            id: "debloat_acer".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove Acer Bloatware".to_string(),
            description: "Removes Acer Care Center, Quick Access, Portal, Collection, Product Registration.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall Acer apps from Acer website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*AcerCareCenter*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "AcerIncorporated","Acer Care Center","Acer Configuration Manager",
    "Acer Quick Access","Acer Portal","Acer Collection","Acer Product Registration"
)
Write-Host "Removing Acer bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    # AppX
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
    
    # Win32
    Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | ForEach-Object { 
        $_.Uninstall(); $removed++ 
    }
}
Write-Host "Removed $removed Acer apps" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },

        // Razer Bloatware
        Tweak {
            id: "debloat_razer".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💻 Remove Razer Bloatware".to_string(),
            description: "Removes Razer Synapse, Cortex, Central, Chroma (keeps drivers intact).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall Razer software from Razer website" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Test-Path "$env:ProgramFiles\Razer") -and !(Test-Path "${env:ProgramFiles(x86)}\Razer")) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "Razer Synapse","Razer Cortex","RazerCortex","Razer Central","Razer Chroma"
)
Write-Host "Removing Razer bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    # Win32 (Razer software is mostly Win32)
    Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | ForEach-Object { 
        $_.Uninstall(); $removed++ 
    }
}
Write-Host "Removed $removed Razer apps" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },

        // Security Bloatware - McAfee
        Tweak {
            id: "debloat_mcafee".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🛡️ Remove McAfee Antivirus".to_string(),
            description: "Removes McAfee trial software using official removal tool (MCPR).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall McAfee from website if desired" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-Service -DisplayName "*McAfee*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Downloading McAfee Removal Tool (MCPR)..." -ForegroundColor Cyan
$url = "https://download.mcafee.com/molbin/iss-loc/SupportTools/MCPR/MCPR.exe"
$dest = "$env:TEMP\MCPR.exe"
try {
    Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    Write-Host "Running MCPR..." -ForegroundColor Cyan
    # Run with silent flags if supported, typically MCPR is interactive but we can try basic silent args
    $args = "-p StopServices,MFSY,PEF,MXD,CSP,Sustainability,MOCP,MFP,APPSTATS,Auth,EMproxy,FWdiver,HW,MAS,MAT,MBK,MCPR,McProxy,McSvcHost,VUL,MHN,MNA,MOBK,MPFP,MPFPCU,MPS,SHRED,MPSCU,MQC,MQCCU,MSAD,MSHR,MSK,MSKCU,MWL,NMC,RedirSvc,VS,REMEDIATION,MSC,YAP,TRUEKEY,LAM,PCB,Symlink,SafeConnect,MGS,WMIRemover,RESIDUE -v -s"
    Start-Process -FilePath $dest -ArgumentList $args -Wait
    Write-Host "McAfee removal completed. Restart required." -ForegroundColor Green
} catch {
    Write-Host "Failed to download/run MCPR: $_" -ForegroundColor Red
}
"#.to_string(),
                }
            ]
        },

        // Security Bloatware - Norton
        Tweak {
            id: "debloat_norton".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🛡️ Remove Norton Security".to_string(),
            description: "Removes Norton/Symantec trial software.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Reinstall Norton from website if desired" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-Service -DisplayName "*Norton*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @("NortonLifeLock", "Norton Security", "Norton 360", "Symantec")
Write-Host "Removing Norton software..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | ForEach-Object { 
        $_.Uninstall(); $removed++ 
    }
}
Write-Host "Removed $removed Norton apps" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },

        // Teams Chat Taskbar
        Tweak {
            id: "debloat_teams_chat_taskbar".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "💬 Remove Teams Chat from Taskbar".to_string(),
            description: "Hides Teams Chat icon from Windows 11 taskbar and removes integration.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarMn" -Value 1 -Type DWord -Force -EA 0
Write-Host "Teams Chat restored (may require re-login)" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                key: "TaskbarMn".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarMn" -Value 0 -Type DWord -Force -EA 0
Get-AppxPackage -Name "*MicrosoftTeams*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
Write-Host "Teams Chat removed from taskbar" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_gaming".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🎮 Remove Xbox Gaming Apps".to_string(),
            description: "Removes Xbox apps. WARNING: May break some games that require Xbox integration!".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell { script: r#"Write-Host "Xbox apps must be reinstalled from Microsoft Store" -ForegroundColor Yellow"#.to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (!(Get-AppxPackage -Name "*GamingApp*" -ErrorAction SilentlyContinue)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: create_bloatware_removal_script(&[
                        "Microsoft.GamingApp","Microsoft.XboxGameOverlay","Microsoft.XboxGamingOverlay",
                        "Microsoft.XboxIdentityProvider","Microsoft.XboxSpeechToTextOverlay"
                    ]),
                }
            ]
        },
        
        // Widgets removal
        Tweak {
            id: "debloat_widgets".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "📰 Remove Widgets".to_string(),
            description: "Removes Widgets app and hides taskbar button.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarDa" -Value 1 -Type DWord -Force -EA 0
Write-Host "Widgets enabled (reinstall from Store if needed)" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                key: "TaskbarDa".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarDa" -Value 0 -Type DWord -Force -EA 0
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Dsh"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "AllowNewsAndInterests" -Value 0 -Type DWord -Force
Get-AppxPackage -Name "*WebExperience*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
Write-Host "Widgets removed" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // AppX Prevention
        Tweak {
            id: "debloat_prevent_reinstall".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🚫 Prevent App Reinstallation".to_string(),
            description: "Prevents Windows from automatically reinstalling removed apps.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned"
if (Test-Path $path) { Remove-Item -Path $path -Recurse -Force -EA 0 }
Write-Host "AppX deprovisioning protection disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Write-Host "AppX reinstallation prevention enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // DANGEROUS: Full Microsoft Edge Removal
        // ============================================
        Tweak {
            id: "debloat_remove_edge_full".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🗑️ Remove Microsoft Edge (Full)".to_string(),
            description: "DANGEROUS: Completely removes Microsoft Edge browser. WARNING: May break WebView2 apps (Teams, Outlook). Use Firefox or Chrome as alternative.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Edge cannot be automatically reinstalled." -ForegroundColor Yellow
Write-Host "To reinstall Edge:" -ForegroundColor Cyan
Write-Host "1. Download from: https://www.microsoft.com/edge" -ForegroundColor White
Write-Host "2. Or run: winget install Microsoft.Edge" -ForegroundColor White
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(crate::modules::types::TweakCheck::Powershell {
                script: r#"
if (Test-Path "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe") {
    Write-Output "False"
} else {
    Write-Output "True"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Removing Microsoft Edge..." -ForegroundColor Yellow

# Step 1: Check Windows version
$build = [int](Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").CurrentBuildNumber
Write-Host "Windows Build: $build" -ForegroundColor Cyan

# Step 2: Remove NoRemove flag (REQUIRED first step)
$uninstallPath = "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft Edge"
if (Test-Path $uninstallPath) {
    Set-ItemProperty -Path $uninstallPath -Name "NoRemove" -Value 0 -Force -EA 0
    Write-Host "[1/6] Removed uninstall protection" -ForegroundColor Green
}

# Step 3: Find Edge version and uninstaller
$EdgePath = "C:\Program Files (x86)\Microsoft\Edge\Application"
if (Test-Path $EdgePath) {
    $EdgeVersion = Get-ChildItem $EdgePath -Directory | 
        Where-Object { $_.Name -match '^\d+\.\d+' } | 
        Sort-Object { [version]($_.Name -replace '\..*$', '') } -Descending | 
        Select-Object -First 1 -ExpandProperty Name
    
    if ($EdgeVersion) {
        $UninstallCmd = "$EdgePath\$EdgeVersion\Installer\setup.exe"
        
        if (Test-Path $UninstallCmd) {
            Write-Host "[2/6] Uninstalling Edge version $EdgeVersion..." -ForegroundColor Yellow
            
            $process = Start-Process -FilePath $UninstallCmd `
                -ArgumentList "--uninstall --system-level --verbose-logging --force-uninstall" `
                -Wait -PassThru -NoNewWindow
            
            if ($process.ExitCode -eq 0) {
                Write-Host "[3/6] Edge uninstalled successfully" -ForegroundColor Green
            } else {
                Write-Host "[3/6] Edge uninstall returned code: $($process.ExitCode)" -ForegroundColor Yellow
            }
        }
    }
} else {
    Write-Host "[2/6] Edge not found in standard location" -ForegroundColor Yellow
}

# Step 4: Remove Edge WebView (optional but thorough)
$WebViewPath = "C:\Program Files (x86)\Microsoft\EdgeWebView"
if (Test-Path $WebViewPath) {
    Write-Host "[4/6] Removing Edge WebView..." -ForegroundColor Yellow
    Remove-Item -Path $WebViewPath -Recurse -Force -EA 0
}

# Step 5: Prevent Edge reinstallation via Group Policy
$policyPaths = @(
    "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate",
    "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
)
foreach ($path in $policyPaths) {
    if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
}
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate" -Name "DoNotUpdateToEdgeWithChromium" -Value 1 -Type DWord -Force
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate" -Name "CreateDesktopShortcutDefault" -Value 0 -Type DWord -Force
Write-Host "[5/6] Blocked Edge reinstallation" -ForegroundColor Green

# Step 6: Disable Edge Update scheduled tasks
$tasks = Get-ScheduledTask | Where-Object { $_.TaskName -like "*MicrosoftEdge*" -or $_.TaskName -like "*Edge*Update*" }
foreach ($task in $tasks) {
    Disable-ScheduledTask -TaskName $task.TaskName -EA 0 | Out-Null
}
Write-Host "[6/6] Disabled Edge update tasks" -ForegroundColor Green

# Step 7: Stop and disable Edge Update service
Stop-Service -Name "edgeupdate" -Force -EA 0
Stop-Service -Name "edgeupdatem" -Force -EA 0
Set-Service -Name "edgeupdate" -StartupType Disabled -EA 0
Set-Service -Name "edgeupdatem" -StartupType Disabled -EA 0

Write-Host "`nEdge removal complete! Restart recommended." -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // DANGEROUS: Disable Windows Defender
        // ============================================
        Tweak {
            id: "debloat_disable_defender".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🛡️ Disable Windows Defender".to_string(),
            description: "DANGEROUS: Disables Windows Defender real-time protection and services via registry policies. Does NOT remove files. Requires reboot.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Re-enabling Windows Defender..." -ForegroundColor Yellow

# Remove policy overrides
$policies = @(
    "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender",
    "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
    "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender\Spynet"
)
foreach ($path in $policies) {
    if (Test-Path $path) { Remove-Item $path -Recurse -Force -EA 0 }
}

# Re-enable services
$services = @("WinDefend", "WdNisSvc", "SecurityHealthService")
foreach ($svc in $services) {
    Set-Service -Name $svc -StartupType Automatic -EA 0
    Start-Service -Name $svc -EA 0
}

# Enable real-time protection
Set-MpPreference -DisableRealtimeMonitoring $false -EA 0

Write-Host "Defender re-enabled. Please reboot." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(crate::modules::types::TweakCheck::Powershell {
                script: r#"
$disabled = Get-ItemProperty "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender" -Name "DisableAntiSpyware" -EA 0
if ($disabled -and $disabled.DisableAntiSpyware -eq 1) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Disabling Windows Defender..." -ForegroundColor Yellow

# Set main policy
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "DisableAntiSpyware" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "DisableAntiVirus" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "ServiceKeepAlive" -Value 0 -Type DWord -Force

# Disable real-time protection
$rtPath = "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"
if (!(Test-Path $rtPath)) { New-Item -Path $rtPath -Force | Out-Null }
Set-ItemProperty -Path $rtPath -Name "DisableRealtimeMonitoring" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $rtPath -Name "DisableBehaviorMonitoring" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $rtPath -Name "DisableOnAccessProtection" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $rtPath -Name "DisableIOAVProtection" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $rtPath -Name "DisableScanOnRealtimeEnable" -Value 1 -Type DWord -Force

# Disable SpyNet/MAPS
$spyPath = "HKLM:\SOFTWARE\Policies\Microsoft\Windows Defender\Spynet"
if (!(Test-Path $spyPath)) { New-Item -Path $spyPath -Force | Out-Null }
Set-ItemProperty -Path $spyPath -Name "SpynetReporting" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $spyPath -Name "SubmitSamplesConsent" -Value 2 -Type DWord -Force

# Disable services
$services = @("WinDefend", "WdNisSvc", "SecurityHealthService")
foreach ($svc in $services) {
    Stop-Service -Name $svc -Force -EA 0
    Set-Service -Name $svc -StartupType Disabled -EA 0
}

# Try to disable via MpPreference
Set-MpPreference -DisableRealtimeMonitoring $true -EA 0

Write-Host "Defender disabled. Please reboot." -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // CAREFUL: Remove Microsoft Store
        // ============================================
        Tweak {
            id: "debloat_remove_store".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🏪 Remove Microsoft Store".to_string(),
            description: "Removes the Microsoft Store application. Use the restore checks or `Get-AppxPackage` to reinstall if needed.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reinstalling Microsoft Store..." -ForegroundColor Yellow

# Remove deprovisioned marker
$store = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore"
Remove-Item "$store\Deprovisioned\Microsoft.WindowsStore*" -Recurse -Force -EA 0
Remove-Item "$store\EndOfLife\*\Microsoft.WindowsStore*" -Recurse -Force -EA 0

# Reset store via wsreset
Start-Process wsreset -Wait -EA 0

# Try to reinstall from existing package
$pkg = Get-AppxPackage -AllUsers Microsoft.WindowsStore -EA 0
if ($pkg) {
    Add-AppxPackage -Register "$($pkg.InstallLocation)\AppXManifest.xml" -DisableDevelopmentMode -EA 0
} else {
    # Fallback: download and install
    wsreset -i
}

Write-Host "Store reinstall initiated" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(crate::modules::types::TweakCheck::Powershell {
                script: r#"
$store = Get-AppxPackage -AllUsers Microsoft.WindowsStore -EA 0
if ($store) { "False" } else { "True" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Removing Microsoft Store..." -ForegroundColor Yellow

# Get all store packages
$pkgs = Get-AppxPackage -AllUsers *WindowsStore* -EA 0

foreach ($pkg in $pkgs) {
    # Mark as deprovisioned
    $store = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore"
    New-Item "$store\Deprovisioned\$($pkg.PackageFamilyName)" -Force | Out-Null
    New-Item "$store\EndOfLife\S-1-5-18\$($pkg.PackageFullName)" -Force | Out-Null
    
    # Set non-removable policy to 0
    dism /Online /Set-NonRemovableAppPolicy /PackageFamily:$($pkg.PackageFamilyName) /NonRemovable:0 2>&1 | Out-Null
    
    # Remove package
    Remove-AppxPackage -Package $pkg.PackageFullName -AllUsers -EA 0
}

# Remove provisioned packages
Get-AppxProvisionedPackage -Online | Where-Object { $_.PackageName -like "*WindowsStore*" } | Remove-AppxProvisionedPackage -Online -EA 0

Write-Host "Store removal complete" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
