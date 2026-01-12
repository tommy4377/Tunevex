//! App removal tweaks (Microsoft, Third-Party, OEM bloatware)

use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
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
)
Write-Host "Removing common Microsoft bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Removed $removed apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @("microsoft.windowscommunicationsapps", "Microsoft.People")
foreach ($app in $apps) {
    Get-AppxPackage -Name "*$app*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Mail, Calendar & People removed" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-AppxPackage -Name "*Microsoft.OutlookForWindows*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*OutlookForWindows*" } | 
    Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
Write-Host "New Outlook removed" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @("Microsoft.YourPhone", "MicrosoftWindows.CrossDevice")
foreach ($app in $apps) {
    Get-AppxPackage -Name "*$app*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Phone Link removed" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
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
)
Write-Host "Removing third-party bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Removed $removed third-party apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "AD2F1837.HPAIExperienceCenter","AD2F1837.HPConnectedMusic",
    "AD2F1837.HPConnectedPhotopoweredbySnapfish","AD2F1837.HPDesktopSupportUtilities",
    "AD2F1837.HPEasyClean","AD2F1837.HPFileViewer","AD2F1837.HPJumpStarts",
    "AD2F1837.HPPCHardwareDiagnosticsWindows","AD2F1837.HPPowerManager",
    "AD2F1837.HPPrinterControl","AD2F1837.HPPrivacySettings","AD2F1837.HPQuickDrop",
    "AD2F1837.HPQuickTouch","AD2F1837.HPRegistration","AD2F1837.HPSupportAssistant",
    "AD2F1837.HPSureShieldAI","AD2F1837.HPSystemInformation","AD2F1837.HPWelcome",
    "AD2F1837.HPWorkWell","AD2F1837.myHP"
)
Write-Host "Removing HP bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Removed $removed HP apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "DellInc.DellSupportAssistforPCs","DellInc.DellDigitalDelivery",
    "DellInc.DellCommandUpdate","DellInc.DellPowerManager",
    "DellInc.PartnerPromo","DellInc.DellCustomerConnect"
)
Write-Host "Removing Dell bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
}
Write-Host "Removed $removed Dell apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "E046963F.LenovoCompanion","E046963F.LenovoSettings",
    "E0469640.LenovoUtility","LenovoCorporation.LenovoID",
    "LenovoCorporation.LenovoVantage","LenovoCorporation.LenovoSettings"
)
Write-Host "Removing Lenovo bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
}
Write-Host "Removed $removed Lenovo apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "B9ECED6F.ArmouryCrate","B9ECED6F.ASUSROGLiveService",
    "ASUSTeK COMPUTER INC.MyASUS","B9ECED6F.arabormonicux"
)
Write-Host "Removing ASUS bloatware..." -ForegroundColor Cyan
$removed = 0
foreach ($app in $apps) {
    $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
    if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0; $removed++ }
}
Write-Host "Removed $removed ASUS apps" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$apps = @(
    "Microsoft.GamingApp","Microsoft.XboxGameOverlay","Microsoft.XboxGamingOverlay",
    "Microsoft.XboxIdentityProvider","Microsoft.XboxSpeechToTextOverlay"
)
Write-Host "Removing Xbox gaming apps..." -ForegroundColor Yellow
Write-Host "WARNING: This may break some games!" -ForegroundColor Red
foreach ($app in $apps) {
    Get-AppxPackage -Name "*$app*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Xbox apps removed" -ForegroundColor Green
"#.to_string(),
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
            enabled: false,
            check: None,
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
            enabled: false,
            check: None,
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
    ]
}
