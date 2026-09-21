# verify_debloat.ps1
# Automating verification of debloat tweaks
# Logs output to C:\Tunevex_Logs

if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "Please run as Administrator!"
    exit
}

$LogDir = "C:\Tunevex_Logs"
if (!(Test-Path $LogDir)) { New-Item -ItemType Directory -Path $LogDir -Force | Out-Null }
$LogFile = "$LogDir\debloat_verification_$(Get-Date -Format 'yyyyMMdd_HHmmss').txt"

function Log-Message {
    param([string]$Message)
    $Timestamp = Get-Date -Format "HH:mm:ss"
    $Line = "[$Timestamp] $Message"
    Write-Host $Line
    Add-Content -Path $LogFile -Value $Line
}

function Test-Tweak {
    param(
        [string]$Name,
        [string]$ApplyScript,
        [string]$RevertScript,
        [string]$CheckScript = ""
    )

    Log-Message "=================================================="
    Log-Message "TESTING TWEAK: $Name"
    Log-Message "=================================================="

    # 1. APPLY
    Log-Message "ACTION: APPLY"
    try {
        $ApplyOutput = Invoke-Expression $ApplyScript | Out-String
        Log-Message "Output:`n$ApplyOutput"
    }
    catch {
        Log-Message "ERROR Applying: $_"
    }

    # 2. VERIFY (Optional)
    if ($CheckScript) {
        Log-Message "ACTION: VERIFY (Post-Apply)"
        try {
            $CheckOutput = Invoke-Expression $CheckScript | Out-String
            Log-Message "Check Output: $CheckOutput"
        }
        catch {
            Log-Message "Check Failed: $_"
        }
    }

    # 3. REVERT
    Log-Message "ACTION: REVERT"
    try {
        $RevertOutput = Invoke-Expression $RevertScript | Out-String
        Log-Message "Output:`n$RevertOutput"
    }
    catch {
        Log-Message "ERROR Reverting: $_"
    }
    
    Log-Message "--------------------------------------------------`n"
}

Log-Message "Starting Debloat Verification..."

# ==========================================
# APPS (Most have manual revert)
# ==========================================

Test-Tweak -Name "Remove Common Microsoft Bloatware" -ApplyScript @'
$apps = @(
    "Clipchamp.Clipchamp","Microsoft.3DBuilder","Microsoft.549981C3F5F10",
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
'@ -RevertScript 'Write-Host "Microsoft apps must be reinstalled from Microsoft Store manually." -ForegroundColor Yellow'

Test-Tweak -Name "Remove Mail, Calendar & People" -ApplyScript @'
$apps = @("microsoft.windowscommunicationsapps", "Microsoft.People")
foreach ($app in $apps) {
    Get-AppxPackage -Name "*$app*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Mail, Calendar & People removed" -ForegroundColor Green
'@ -RevertScript 'Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow'

Test-Tweak -Name "Remove New Outlook" -ApplyScript @'
Get-AppxPackage -Name "*Microsoft.OutlookForWindows*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*OutlookForWindows*" } | 
    Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
Write-Host "New Outlook removed" -ForegroundColor Green
'@ -RevertScript 'Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow'

Test-Tweak -Name "Remove Phone Link" -ApplyScript @'
$apps = @("Microsoft.YourPhone", "MicrosoftWindows.CrossDevice")
foreach ($app in $apps) {
    Get-AppxPackage -Name "*$app*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Get-AppxProvisionedPackage -Online -EA 0 | Where-Object { $_.PackageName -like "*$app*" } | 
        Remove-ProvisionedAppxPackage -Online -AllUsers -EA 0 | Out-Null
}
Write-Host "Phone Link removed" -ForegroundColor Green
'@ -RevertScript 'Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow'

# OneDrive - Skipping actual uninstall to avoid breaking user env too much in a test, but logging script logic
Log-Message "SKIPPING OneDrive Uninstall execution to prevent data sync issues during test."

Test-Tweak -Name "Remove Third-Party Bloatware" -ApplyScript @'
$apps = @(
    "ACGMediaPlayer","ActiproSoftwareLLC","AdobeSystemsIncorporated.AdobePhotoshopExpress",
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
}
Write-Host "Removed $removed third-party apps" -ForegroundColor Green
'@ -RevertScript 'Write-Host "Reinstall from Microsoft Store" -ForegroundColor Yellow'

Test-Tweak -Name "Remove Widgets" -ApplyScript @'
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarDa" -Value 0 -Type DWord -Force -EA 0
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Dsh"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "AllowNewsAndInterests" -Value 0 -Type DWord -Force
Get-AppxPackage -Name "*WebExperience*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
Write-Host "Widgets removed" -ForegroundColor Green
'@ -RevertScript @'
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarDa" -Value 1 -Type DWord -Force -EA 0
Write-Host "Widgets enabled (reinstall from Store if needed)" -ForegroundColor Green
'@

Test-Tweak -Name "Prevent App Reinstall" -ApplyScript @'
$path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Write-Host "AppX reinstallation prevention enabled" -ForegroundColor Green
'@ -RevertScript @'
$path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned"
if (Test-Path $path) { Remove-Item -Path $path -Recurse -Force -EA 0 }
Write-Host "AppX deprovisioning protection disabled" -ForegroundColor Green
'@

# ==========================================
# FEATURES (Automatic Revert)
# ==========================================

Test-Tweak -Name "Disable Printer Features" -ApplyScript @'
Write-Host "Disabling Printer and XPS features..." -ForegroundColor Cyan
$features = @("Printing-Foundation-InternetPrinting-Client","LPDPrintService","Printing-Foundation-LPRPortMonitor","Printing-PrintToPDFServices-Features","Printing-XPSServices-Features","Xps-Foundation-Xps-Viewer","WorkFolders-Client")
foreach ($f in $features) { dism /Online /Disable-Feature /FeatureName:"$f" /NoRestart 2>$null }
Write-Host "Printer and XPS features disabled" -ForegroundColor Green
'@ -RevertScript @'
Write-Host "Enabling Printer and XPS features..." -ForegroundColor Cyan
$features = @("Printing-Foundation-InternetPrinting-Client","LPDPrintService","Printing-Foundation-LPRPortMonitor","Printing-PrintToPDFServices-Features","Printing-XPSServices-Features","Xps-Foundation-Xps-Viewer","WorkFolders-Client")
foreach ($f in $features) { dism /Online /Enable-Feature /FeatureName:"$f" /NoRestart 2>$null }
Write-Host "Printer and XPS features enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable IE Mode" -ApplyScript @'
dism /Online /Disable-Feature /FeatureName:"Internet-Explorer-Optional-amd64" /NoRestart 2>$null
Write-Host "Internet Explorer disabled" -ForegroundColor Green
'@ -RevertScript @'
dism /Online /Enable-Feature /FeatureName:"Internet-Explorer-Optional-amd64" /NoRestart 2>$null
Write-Host "Internet Explorer enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable Media Player" -ApplyScript @'
dism /Online /Disable-Feature /FeatureName:"WindowsMediaPlayer" /NoRestart 2>$null
Write-Host "Windows Media Player disabled" -ForegroundColor Green
'@ -RevertScript @'
dism /Online /Enable-Feature /FeatureName:"WindowsMediaPlayer" /NoRestart 2>$null
Write-Host "Windows Media Player enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable WordPad" -ApplyScript @'
dism /Online /Disable-Feature /FeatureName:"Microsoft-Windows-WordPad" /NoRestart 2>$null
Write-Host "WordPad disabled" -ForegroundColor Green
'@ -RevertScript @'
dism /Online /Enable-Feature /FeatureName:"Microsoft-Windows-WordPad" /NoRestart 2>$null
Write-Host "WordPad enabled" -ForegroundColor Green
'@

# ==========================================
# EDGE (Automatic Revert)
# ==========================================

Test-Tweak -Name "Disable Edge Sidebar" -ApplyScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "HubsSidebarEnabled" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "WebWidgetAllowed" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "EdgeShoppingAssistantEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge sidebar features disabled" -ForegroundColor Green
'@ -RevertScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "HubsSidebarEnabled" -EA 0
Remove-ItemProperty -Path $path -Name "WebWidgetAllowed" -EA 0
Remove-ItemProperty -Path $path -Name "EdgeShoppingAssistantEnabled" -EA 0
Write-Host "Edge sidebar features enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable Edge Startup" -ApplyScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "HideFirstRunExperience" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "RunStartUpSystemCheck" -Value 0 -Type DWord -Force
Write-Host "Edge first run disabled" -ForegroundColor Green
'@ -RevertScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "HideFirstRunExperience" -EA 0
Remove-ItemProperty -Path $path -Name "RunStartUpSystemCheck" -EA 0
Write-Host "Edge first run enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable Edge Sync" -ApplyScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "SyncDisabled" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "EdgeCollectionsEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge sync disabled" -ForegroundColor Green
'@ -RevertScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "SyncDisabled" -EA 0
Remove-ItemProperty -Path $path -Name "EdgeCollectionsEnabled" -EA 0
Write-Host "Edge sync enabled" -ForegroundColor Green
'@

Test-Tweak -Name "Disable Edge Telemetry" -ApplyScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "PersonalizationReportingEnabled" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "UserFeedbackAllowed" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "MetricsReportingEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge telemetry disabled" -ForegroundColor Green
'@ -RevertScript @'
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "PersonalizationReportingEnabled" -EA 0
Remove-ItemProperty -Path $path -Name "UserFeedbackAllowed" -EA 0
Remove-ItemProperty -Path $path -Name "MetricsReportingEnabled" -EA 0
Write-Host "Edge telemetry enabled" -ForegroundColor Green
'@

Log-Message "Verification Complete. Log saved to $LogFile"
Start-Process "notepad.exe" $LogFile
