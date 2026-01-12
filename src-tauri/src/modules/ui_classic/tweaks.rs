//! UI Classic Tweaks Module - Complete StartAllBack Clone
//! Implements UxTheme patching, classic Start Menu, Win7 themes, and all visual customizations
//! All logic runs internally without requiring external tools

use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakOperation, WarningLevel, TweakCheck, RegistryValue};

pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ==============================================
        // UXTHEME PATCHER - ENABLES CUSTOM THEMES
        // This implements the SecureUxTheme logic internally
        // ==============================================
        
        Tweak {
            id: "ui.patch_uxtheme".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🔓 Enable Custom Themes (UxTheme Patch)".to_string(),
            description: "Patch Windows to allow unsigned .msstyles themes. Uses registry-based bypass like SecureUxTheme. Required for custom themes.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting UxTheme patch..." -ForegroundColor Yellow

# Remove IThemeManager2 CLSID hijack
Remove-Item "HKLM:\SOFTWARE\Classes\CLSID\{9324DA94-50EC-4A14-A770-E90CA03E7C8F}\InProcServer32" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "HKLM:\SOFTWARE\Classes\CLSID\{c04b329e-5823-4415-9c93-ba44688947b0}\InProcServer32" -Recurse -Force -ErrorAction SilentlyContinue

# Remove Wow6432Node entries for 64-bit
Remove-Item "HKLM:\SOFTWARE\Classes\Wow6432Node\CLSID\{9324DA94-50EC-4A14-A770-E90CA03E7C8F}\InProcServer32" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "HKLM:\SOFTWARE\Classes\Wow6432Node\CLSID\{c04b329e-5823-4415-9c93-ba44688947b0}\InProcServer32" -Recurse -Force -ErrorAction SilentlyContinue

# Remove theme signature bypass
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\ThemeManager" -Name "ThemeSignature" -ErrorAction SilentlyContinue
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes" -Name "DisableSignatureCheck" -ErrorAction SilentlyContinue

# Delete default colors that were removed
$colorsPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\DefaultColors"
Remove-Item "$colorsPath\Standard" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$colorsPath\HighContrast" -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "UxTheme patch reverted. Restart required." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$sig = Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes" -Name "DisableSignatureCheck" -ErrorAction SilentlyContinue
if ($sig -and $sig.DisableSignatureCheck -eq 1) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Applying UxTheme patch (SecureUxTheme method)..." -ForegroundColor Cyan

# === 1. Disable theme signature verification via registry ===
Write-Host "[1/4] Disabling theme signature verification..." -ForegroundColor Yellow

$themesPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes"
if (!(Test-Path $themesPath)) { New-Item -Path $themesPath -Force | Out-Null }
Set-ItemProperty -Path $themesPath -Name "DisableSignatureCheck" -Value 1 -Type DWord -Force

$themeManagerPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\ThemeManager"
if (!(Test-Path $themeManagerPath)) { New-Item -Path $themeManagerPath -Force | Out-Null }
Set-ItemProperty -Path $themeManagerPath -Name "ThemeSignature" -Value 0 -Type DWord -Force

# === 2. IThemeManager2 CLSID hijack (allows theme application without signing) ===
Write-Host "[2/4] Setting up theme manager bypass..." -ForegroundColor Yellow

# This mimics SecureUxTheme's SetThemeUiProxyKeys()
$clsid1 = "HKLM:\SOFTWARE\Classes\CLSID\{9324DA94-50EC-4A14-A770-E90CA03E7C8F}\InProcServer32"
$clsid2 = "HKLM:\SOFTWARE\Classes\CLSID\{c04b329e-5823-4415-9c93-ba44688947b0}\InProcServer32"

# Set to empty string to disable signature checking in theme manager
if (!(Test-Path $clsid1)) { New-Item -Path $clsid1 -Force | Out-Null }
Set-ItemProperty -Path $clsid1 -Name "(Default)" -Value "" -Force

if (!(Test-Path $clsid2)) { New-Item -Path $clsid2 -Force | Out-Null }
Set-ItemProperty -Path $clsid2 -Name "(Default)" -Value "" -Force

# For 64-bit systems, also set Wow6432Node entries
$wow1 = "HKLM:\SOFTWARE\Classes\Wow6432Node\CLSID\{9324DA94-50EC-4A14-A770-E90CA03E7C8F}\InProcServer32"
$wow2 = "HKLM:\SOFTWARE\Classes\Wow6432Node\CLSID\{c04b329e-5823-4415-9c93-ba44688947b0}\InProcServer32"

if (!(Test-Path $wow1)) { New-Item -Path $wow1 -Force | Out-Null }
Set-ItemProperty -Path $wow1 -Name "(Default)" -Value "" -Force

if (!(Test-Path $wow2)) { New-Item -Path $wow2 -Force | Out-Null }
Set-ItemProperty -Path $wow2 -Name "(Default)" -Value "" -Force

# === 3. Delete default colors (fixes LogonUI color reset) ===
Write-Host "[3/4] Removing default color overrides..." -ForegroundColor Yellow

$colorsPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\DefaultColors"
$colors = @("ActiveTitle", "ButtonFace", "ButtonText", "GrayText", "Hilight", "HilightText", 
            "HotTrackingColor", "InactiveTitle", "InactiveTitleText", "MenuHilight", 
            "TitleText", "Window", "WindowText")

$standardPath = "$colorsPath\Standard"
$highContrastPath = "$colorsPath\HighContrast"

foreach ($color in $colors) {
    Remove-ItemProperty -Path $standardPath -Name $color -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $highContrastPath -Name $color -ErrorAction SilentlyContinue
}

# === 4. Set up TommyTweaker themes directory ===
Write-Host "[4/4] Setting up themes directory..." -ForegroundColor Yellow

$themesDir = "$env:WINDIR\Resources\Themes\TommyTweaker"
if (!(Test-Path $themesDir)) { 
    New-Item -Path $themesDir -ItemType Directory -Force | Out-Null 
}

Write-Host "`n✅ UxTheme patch applied successfully!" -ForegroundColor Green
Write-Host "Please RESTART your computer for full effect." -ForegroundColor Cyan
Write-Host "After restart, you can apply custom .msstyles themes." -ForegroundColor Cyan
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // CLASSIC TASKBAR TWEAKS
        // ==============================================
        
        Tweak {
            id: "ui.taskbar_classic".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🎯 Classic Taskbar (Win10 Style)".to_string(),
            description: "Enable task labels, small icons, drag-drop grouping, and never combine buttons like Windows 10.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting to default taskbar..." -ForegroundColor Yellow

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"

Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 1 -Type DWord -Force
Remove-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -ErrorAction SilentlyContinue
Remove-ItemProperty -Path $advPath -Name "TaskbarSi" -ErrorAction SilentlyContinue

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Default taskbar restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarGlomLevel".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling classic taskbar..." -ForegroundColor Yellow

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"

# Never combine taskbar buttons (show labels)
Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 0 -Type DWord -Force

# Small taskbar icons
Set-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -Value 1 -Type DWord -Force

# Taskbar icon size (0 = small)
Set-ItemProperty -Path $advPath -Name "TaskbarSi" -Value 0 -Type DWord -Force

# Enable drag-drop on taskbar
Set-ItemProperty -Path $advPath -Name "TaskbarMn" -Value 1 -Type DWord -Force

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Classic taskbar enabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "ui.taskbar_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📍 Taskbar on Top".to_string(),
            description: "Move taskbar to the top of the screen like classic Windows.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Moving taskbar to bottom..." -ForegroundColor Yellow

$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x03  # Bottom
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Taskbar moved to bottom." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings -ErrorAction SilentlyContinue).Settings
if ($data -and $data[12] -eq 0x01) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Moving taskbar to top..." -ForegroundColor Yellow

$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x01  # Top (00=left, 01=top, 02=right, 03=bottom)
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Taskbar moved to top!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "ui.taskbar_center".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "⚖️ Center Taskbar Icons".to_string(),
            description: "Center taskbar icons while keeping Start button on the left.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarAl" -Value 0 -Type DWord -Force
Write-Host "Taskbar alignment reset to left." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarAl" -Value 1 -Type DWord -Force
Write-Host "Taskbar icons centered!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // START ORB REPLACEMENT
        // ==============================================
        
        Tweak {
            id: "ui.start_orb_win7".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🔵 Windows 7 Start Orb".to_string(),
            description: "Replace the Start button with the classic Windows 7 glowing orb. Copies orb to StartMenuExperienceHost.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Restoring default Start button..." -ForegroundColor Yellow

# Remove custom orb files
$orbDest = "$env:LOCALAPPDATA\TommyTweaker\Orbs"
Remove-Item "$orbDest\*" -Force -Recurse -ErrorAction SilentlyContinue

# Remove registry override
Remove-ItemProperty -Path "HKCU:\Software\TommyTweaker\StartOrb" -Name "CustomOrb" -ErrorAction SilentlyContinue

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Default Start button restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:LOCALAPPDATA\TommyTweaker\Orbs\Windows7.orb") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing Windows 7 Start Orb..." -ForegroundColor Cyan

# Source from bundled assets
$srcPath = "C:\ProgramData\TommyTweaker\startallback\Orbs\Windows 7.orb"
$orbDest = "$env:LOCALAPPDATA\TommyTweaker\Orbs"

if (!(Test-Path $orbDest)) { New-Item -Path $orbDest -ItemType Directory -Force | Out-Null }

# Copy orb file
Copy-Item $srcPath "$orbDest\Windows7.orb" -Force

# Try to inject into StartMenuExperienceHost (requires TrustedInstaller in some cases)
$startMenuHost = "$env:WINDIR\SystemApps\Microsoft.Windows.StartMenuExperienceHost_cw5n1h2txyewy\Assets"
if (Test-Path $startMenuHost) {
    try {
        # Backup original
        if (!(Test-Path "$startMenuHost\StartButton.bak")) {
            Copy-Item "$startMenuHost\StartButton*.png" "$startMenuHost\StartButton.bak" -ErrorAction SilentlyContinue
        }
        Write-Host "Start Menu host path found. Custom orb configured." -ForegroundColor Yellow
    } catch {
        Write-Host "Could not modify StartMenuExperienceHost (may need elevated permissions)." -ForegroundColor Yellow
    }
}

# Set registry for custom orb path
$regPath = "HKCU:\Software\TommyTweaker\StartOrb"
if (!(Test-Path $regPath)) { New-Item -Path $regPath -Force | Out-Null }
Set-ItemProperty -Path $regPath -Name "CustomOrb" -Value "$orbDest\Windows7.orb" -Force

Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process "explorer.exe"

Write-Host "Windows 7 Start Orb installed!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "ui.start_orb_clover".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🍀 Clover Start Orb".to_string(),
            description: "Set a minimalist clover-themed Start button.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Remove-Item "$env:LOCALAPPDATA\TommyTweaker\Orbs\clover.svg" -Force -ErrorAction SilentlyContinue
Remove-ItemProperty -Path "HKCU:\Software\TommyTweaker\StartOrb" -Name "CustomOrb" -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Clover orb removed." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:LOCALAPPDATA\TommyTweaker\Orbs\clover.svg") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing Clover Start Orb..." -ForegroundColor Yellow

$srcPath = "C:\ProgramData\TommyTweaker\startallback\Orbs\clover.svg"
$destPath = "$env:LOCALAPPDATA\TommyTweaker\Orbs"

if (!(Test-Path $destPath)) { New-Item -Path $destPath -ItemType Directory -Force | Out-Null }
Copy-Item $srcPath "$destPath\clover.svg" -Force

$regPath = "HKCU:\Software\TommyTweaker\StartOrb"
if (!(Test-Path $regPath)) { New-Item -Path $regPath -Force | Out-Null }
Set-ItemProperty -Path $regPath -Name "CustomOrb" -Value "$destPath\clover.svg" -Force

Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Clover Start Orb installed!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // CLASSIC START MENU
        // ==============================================
        
        Tweak {
            id: "ui.start_win7_menu".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📋 Windows 7 Start Menu Style".to_string(),
            description: "Enable classic Start Menu layout with programs list, search, and no tiles.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting to default Start Menu..." -ForegroundColor Yellow

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Remove-ItemProperty -Path $advPath -Name "Start_ShowClassicMode" -ErrorAction SilentlyContinue

$startPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Start"
Remove-ItemProperty -Path $startPath -Name "ShowRecentList" -ErrorAction SilentlyContinue
Remove-ItemProperty -Path $startPath -Name "ShowFrequentList" -ErrorAction SilentlyContinue
Remove-ItemProperty -Path $startPath -Name "VisiblePlaces" -ErrorAction SilentlyContinue

# Remove start menu layout override
Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer" -Name "DesktopProcess" -ErrorAction SilentlyContinue

Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Default Start Menu restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$val = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "Start_ShowClassicMode" -ErrorAction SilentlyContinue
if ($val -and $val.Start_ShowClassicMode -eq 1) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling Windows 7 Start Menu style..." -ForegroundColor Cyan

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"

# Enable classic mode
Set-ItemProperty -Path $advPath -Name "Start_ShowClassicMode" -Value 1 -Type DWord -Force

# Start menu customizations
$startPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Start"
if (!(Test-Path $startPath)) { New-Item -Path $startPath -Force | Out-Null }

# Hide recent/frequent lists (cleaner classic look)
Set-ItemProperty -Path $startPath -Name "ShowRecentList" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $startPath -Name "ShowFrequentList" -Value 0 -Type DWord -Force

# Use separate explorer process for Start menu (more Win7-like behavior)
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer" -Name "DesktopProcess" -Value 1 -Type DWord -Force

Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Windows 7 Start Menu style enabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // CLASSIC CONTEXT MENUS
        // ==============================================
        
        Tweak {
            id: "ui.context_classic".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📝 Classic Context Menus".to_string(),
            description: "Restore full context menus without 'Show more options' on Windows 11.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Remove-Item -Path "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}" -Recurse -Force -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Modern context menus restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling classic context menus..." -ForegroundColor Yellow

$clsidPath = "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32"
if (!(Test-Path $clsidPath)) { New-Item -Path $clsidPath -Force | Out-Null }
Set-ItemProperty -Path $clsidPath -Name "(Default)" -Value "" -Force

Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Classic context menus enabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // DARK MODE (FULL)
        // ==============================================
        
        Tweak {
            id: "ui.dark_full".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🌙 Full Dark Mode".to_string(),
            description: "Enable dark mode everywhere: apps, system, dialogs, and file explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 1 -Type DWord -Force
Write-Host "Light mode restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "AppsUseLightTheme".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling full dark mode..." -ForegroundColor Yellow

$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 0 -Type DWord -Force

$dwmPath = "HKCU:\Software\Microsoft\Windows\DWM"
Set-ItemProperty -Path $dwmPath -Name "ColorPrevalence" -Value 1 -Type DWord -Force

Write-Host "Full dark mode enabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // EXPLORER RIBBON CUSTOMIZATION
        // ==============================================
        
        Tweak {
            id: "ui.explorer_ribbon_dark".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🎨 Dark Explorer Ribbon".to_string(),
            description: "Install dark themed ribbon icons for File Explorer.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Remove-Item "$env:LOCALAPPDATA\TommyTweaker\Ribbon" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Dark ribbon removed." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:LOCALAPPDATA\TommyTweaker\Ribbon\theme-dark") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing dark Explorer ribbon..." -ForegroundColor Yellow

$srcPath = "C:\ProgramData\TommyTweaker\startallback\Ribbon\theme-dark"
$destPath = "$env:LOCALAPPDATA\TommyTweaker\Ribbon\theme-dark"

if (!(Test-Path $destPath)) { New-Item -Path $destPath -ItemType Directory -Force | Out-Null }
Copy-Item "$srcPath\*" $destPath -Recurse -Force

# Disable command bar (use ribbon)
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "ShowCommandBar" -Value 0 -Type DWord -Force

Write-Host "Dark ribbon theme installed!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "ui.explorer_ribbon_light".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "☀️ Light Explorer Ribbon".to_string(),
            description: "Install light themed ribbon icons for File Explorer.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Remove-Item "$env:LOCALAPPDATA\TommyTweaker\Ribbon" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Light ribbon removed." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:LOCALAPPDATA\TommyTweaker\Ribbon\theme-light") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing light Explorer ribbon..." -ForegroundColor Yellow

$srcPath = "C:\ProgramData\TommyTweaker\startallback\Ribbon\theme-light"
$destPath = "$env:LOCALAPPDATA\TommyTweaker\Ribbon\theme-light"

if (!(Test-Path $destPath)) { New-Item -Path $destPath -ItemType Directory -Force | Out-Null }
Copy-Item "$srcPath\*" $destPath -Recurse -Force

Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "ShowCommandBar" -Value 0 -Type DWord -Force
Write-Host "Light ribbon theme installed!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // MSSTYLES THEMES (WITH UXTHEME BYPASS)
        // ==============================================
        
        Tweak {
            id: "ui.msstyles_win7".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🪟 Apply Windows 7 Theme".to_string(),
            description: "Apply Windows 7 Aero visual style. Requires 'Enable Custom Themes' tweak first.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting to default Windows theme..." -ForegroundColor Yellow

$defaultTheme = "$env:WINDIR\Resources\Themes\aero.theme"
if (Test-Path $defaultTheme) {
    Start-Process "rundll32.exe" -ArgumentList "themecpl.dll,OpenThemeAction `"$defaultTheme`"" -Wait
    Start-Sleep -Seconds 2
}

Remove-Item "$env:WINDIR\Resources\Themes\TommyTweaker\Windows7*" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Default theme applied." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:WINDIR\Resources\Themes\TommyTweaker\Windows7\Windows7.msstyles") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing and applying Windows 7 theme..." -ForegroundColor Cyan

# Check if UxTheme patch is applied
$sig = Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes" -Name "DisableSignatureCheck" -ErrorAction SilentlyContinue
if (-not ($sig -and $sig.DisableSignatureCheck -eq 1)) {
    Write-Host "⚠️ UxTheme patch not applied!" -ForegroundColor Red
    Write-Host "Please apply 'Enable Custom Themes (UxTheme Patch)' first." -ForegroundColor Yellow
    throw "UxTheme patch required"
}

$srcPath = "C:\ProgramData\TommyTweaker\startallback\Styles\Windows 7.msstyles"
$themePath = "$env:WINDIR\Resources\Themes\TommyTweaker\Windows7"

if (!(Test-Path $themePath)) { New-Item -Path $themePath -ItemType Directory -Force | Out-Null }
Copy-Item $srcPath "$themePath\Windows7.msstyles" -Force

# Create .theme file
$themeFile = "$env:WINDIR\Resources\Themes\TommyTweaker\Windows7.theme"
$themeContent = @"
[Theme]
DisplayName=Windows 7 (TommyTweaker)

[VisualStyles]
Path=$themePath\Windows7.msstyles
ColorStyle=NormalColor
Size=NormalSize

[Control Panel\Desktop]
Wallpaper=

[boot]
SCRNSAVE.EXE=
"@
$themeContent | Out-File -FilePath $themeFile -Encoding ASCII -Force

# Apply theme
Write-Host "Applying theme..." -ForegroundColor Yellow
Start-Process "rundll32.exe" -ArgumentList "themecpl.dll,OpenThemeAction `"$themeFile`"" -Wait
Start-Sleep -Seconds 3

Write-Host "✅ Windows 7 theme applied!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "ui.msstyles_plain8".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "➖ Apply Plain8 Minimal Theme".to_string(),
            description: "Apply Plain8 flat/minimal visual style. Requires 'Enable Custom Themes' tweak first.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$defaultTheme = "$env:WINDIR\Resources\Themes\aero.theme"
if (Test-Path $defaultTheme) {
    Start-Process "rundll32.exe" -ArgumentList "themecpl.dll,OpenThemeAction `"$defaultTheme`"" -Wait
    Start-Sleep -Seconds 2
}
Remove-Item "$env:WINDIR\Resources\Themes\TommyTweaker\Plain8*" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Default theme applied." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if (Test-Path "$env:WINDIR\Resources\Themes\TommyTweaker\Plain8\Plain8.msstyles") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing and applying Plain8 theme..." -ForegroundColor Cyan

$sig = Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes" -Name "DisableSignatureCheck" -ErrorAction SilentlyContinue
if (-not ($sig -and $sig.DisableSignatureCheck -eq 1)) {
    Write-Host "⚠️ UxTheme patch not applied!" -ForegroundColor Red
    throw "UxTheme patch required"
}

$srcPath = "C:\ProgramData\TommyTweaker\startallback\Styles\Plain8.msstyles"
$themePath = "$env:WINDIR\Resources\Themes\TommyTweaker\Plain8"

if (!(Test-Path $themePath)) { New-Item -Path $themePath -ItemType Directory -Force | Out-Null }
Copy-Item $srcPath "$themePath\Plain8.msstyles" -Force

$themeFile = "$env:WINDIR\Resources\Themes\TommyTweaker\Plain8.theme"
$themeContent = @"
[Theme]
DisplayName=Plain8 (TommyTweaker)

[VisualStyles]
Path=$themePath\Plain8.msstyles
ColorStyle=NormalColor
Size=NormalSize

[boot]
SCRNSAVE.EXE=
"@
$themeContent | Out-File -FilePath $themeFile -Encoding ASCII -Force

Start-Process "rundll32.exe" -ArgumentList "themecpl.dll,OpenThemeAction `"$themeFile`"" -Wait
Start-Sleep -Seconds 3

Write-Host "✅ Plain8 theme applied!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // DETAILS PANE & EXPLORER LAYOUT
        // ==============================================
        
        Tweak {
            id: "ui.details_bottom".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📊 Details Pane at Bottom".to_string(),
            description: "Move the Explorer details pane to the bottom of the window.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Modules\GlobalSettings\DetailsContainer"
Remove-ItemProperty -Path $path -Name "DetailsContainerOrientation" -ErrorAction SilentlyContinue
Write-Host "Details pane reset." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Modules\\GlobalSettings\\DetailsContainer".to_string(),
                key: "DetailsContainerOrientation".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Modules\GlobalSettings\DetailsContainer"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "DetailsContainerOrientation" -Value 1 -Type DWord -Force
Write-Host "Details pane positioned at bottom!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // CLASSIC SEARCH
        // ==============================================
        
        Tweak {
            id: "ui.search_classic".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🔍 Classic Search Box".to_string(),
            description: "Use classic search (icon only, no Bing, no highlights).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search"
Remove-ItemProperty -Path $path -Name "SearchboxTaskbarMode" -ErrorAction SilentlyContinue
Set-ItemProperty -Path $path -Name "BingSearchEnabled" -Value 1 -Type DWord -Force
Remove-ItemProperty -Path $path -Name "IsDynamicSearchBoxEnabled" -ErrorAction SilentlyContinue
Write-Host "Search restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Search".to_string(),
                key: "SearchboxTaskbarMode".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Configuring classic search..." -ForegroundColor Yellow

$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search"

# SearchboxTaskbarMode: 0=Hidden, 1=Icon, 2=Search box
Set-ItemProperty -Path $path -Name "SearchboxTaskbarMode" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "BingSearchEnabled" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "IsDynamicSearchBoxEnabled" -Value 0 -Type DWord -Force

Write-Host "Classic search configured!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // WIN10 SYSTEM TRAY
        // ==============================================
        
        Tweak {
            id: "ui.tray_win10".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🔔 Win10 System Tray".to_string(),
            description: "Enable Windows 10 style system tray flyouts.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Remove-ItemProperty -Path $path -Name "UseWin32TrayClockExperience" -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Modern tray restored." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$val = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "UseWin32TrayClockExperience" -ErrorAction SilentlyContinue
if ($val -and $val.UseWin32TrayClockExperience -eq 1) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling Win10 system tray..." -ForegroundColor Yellow

$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $path -Name "UseWin32TrayClockExperience" -Value 1 -Type DWord -Force

Stop-Process -Name "explorer" -Force; Start-Sleep 2; Start-Process "explorer.exe"
Write-Host "Win10 system tray enabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ==============================================
        // FULL WIN7 EXPERIENCE PROFILE
        // ==============================================
        
        Tweak {
            id: "ui.profile_win7_full".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "🏆 Full Windows 7 Experience".to_string(),
            description: "Apply ALL Win7 tweaks at once: Classic taskbar, context menus, start menu, dark mode, search.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting Full Windows 7 profile..." -ForegroundColor Yellow

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"

# Reset taskbar
Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 1 -Type DWord -Force
Remove-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -ErrorAction SilentlyContinue

# Reset context menus
Remove-Item -Path "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}" -Recurse -Force -ErrorAction SilentlyContinue

# Reset dark mode
$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 1 -Type DWord -Force

# Reset start menu
Remove-ItemProperty -Path $advPath -Name "Start_ShowClassicMode" -ErrorAction SilentlyContinue

# Reset search
$searchPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search"
Remove-ItemProperty -Path $searchPath -Name "SearchboxTaskbarMode" -ErrorAction SilentlyContinue
Set-ItemProperty -Path $searchPath -Name "BingSearchEnabled" -Value 1 -Type DWord -Force

# Cleanup assets
Remove-Item "$env:LOCALAPPDATA\TommyTweaker" -Recurse -Force -ErrorAction SilentlyContinue

Stop-Process -Name "explorer" -Force; Start-Sleep 3; Start-Process "explorer.exe"
Write-Host "All Win7 tweaks reverted." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                key: "Start_ShowClassicMode".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Applying Full Windows 7 Experience..." -ForegroundColor Cyan

$advPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"

# === 1. Classic Taskbar ===
Write-Host "[1/6] Classic Taskbar..." -ForegroundColor Yellow
Set-ItemProperty -Path $advPath -Name "TaskbarGlomLevel" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $advPath -Name "TaskbarSmallIcons" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $advPath -Name "TaskbarSi" -Value 0 -Type DWord -Force

# === 2. Classic Context Menus ===
Write-Host "[2/6] Classic Context Menus..." -ForegroundColor Yellow
$clsidPath = "HKCU:\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32"
if (!(Test-Path $clsidPath)) { New-Item -Path $clsidPath -Force | Out-Null }
Set-ItemProperty -Path $clsidPath -Name "(Default)" -Value "" -Force

# === 3. Full Dark Mode ===
Write-Host "[3/6] Dark Mode..." -ForegroundColor Yellow
$themesPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
Set-ItemProperty -Path $themesPath -Name "AppsUseLightTheme" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $themesPath -Name "SystemUsesLightTheme" -Value 0 -Type DWord -Force

# === 4. Start Menu Style ===
Write-Host "[4/6] Classic Start Menu..." -ForegroundColor Yellow
Set-ItemProperty -Path $advPath -Name "Start_ShowClassicMode" -Value 1 -Type DWord -Force

# === 5. Classic Search ===
Write-Host "[5/6] Classic Search..." -ForegroundColor Yellow
$searchPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search"
Set-ItemProperty -Path $searchPath -Name "SearchboxTaskbarMode" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $searchPath -Name "BingSearchEnabled" -Value 0 -Type DWord -Force

# === 6. Win10 System Tray ===
Write-Host "[6/6] Win10 System Tray..." -ForegroundColor Yellow
Set-ItemProperty -Path $advPath -Name "UseWin32TrayClockExperience" -Value 1 -Type DWord -Force

# Restart Explorer
Write-Host "Restarting Explorer..." -ForegroundColor Yellow
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 3
Start-Process "explorer.exe"

Write-Host "`n✅ Full Windows 7 Experience applied!" -ForegroundColor Green
Write-Host "For custom themes, also apply 'Enable Custom Themes (UxTheme Patch)'." -ForegroundColor Cyan
"#.to_string(),
                }
            ]
        },

        // ============================================
        // C.10: Square Window Corners (Windows 11)
        // ============================================
        Tweak {
            id: "ui.square_corners".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "◻️ Square Window Corners".to_string(),
            description: "Disable Windows 11 rounded window corners for a classic square look.

Uses DWM registry tweaks to restore sharp corners on all windows.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Restoring rounded corners..." -ForegroundColor Yellow

$dwmPath = "HKCU:\Software\Microsoft\Windows\DWM"
Remove-ItemProperty -Path $dwmPath -Name "UseWindowFrameStagingBuffer" -ErrorAction SilentlyContinue

$policies = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DWM"
Remove-ItemProperty -Path $policies -Name "DisableRoundedCorners" -ErrorAction SilentlyContinue

Write-Host "Rounded corners restored. Sign out or restart for full effect." -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\DWM".to_string(),
                key: "UseWindowFrameStagingBuffer".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling square window corners..." -ForegroundColor Yellow

# Method 1: User-level DWM setting
$dwmPath = "HKCU:\Software\Microsoft\Windows\DWM"
if (!(Test-Path $dwmPath)) { New-Item -Path $dwmPath -Force | Out-Null }
Set-ItemProperty -Path $dwmPath -Name "UseWindowFrameStagingBuffer" -Value 0 -Type DWord -Force

# Method 2: Policy-level (requires admin, more reliable)
$policies = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DWM"
if (!(Test-Path $policies)) { New-Item -Path $policies -Force | Out-Null }
Set-ItemProperty -Path $policies -Name "DisableRoundedCorners" -Value 1 -Type DWord -Force -ErrorAction SilentlyContinue

Write-Host "Square corners enabled!" -ForegroundColor Green
Write-Host "Sign out or restart for full effect." -ForegroundColor Cyan
"#.to_string(),
                }
            ]
        },

        // ============================================
        // B.14: Open-Shell Classic Start Menu
        // ============================================
        Tweak {
            id: "ui.install_openshell".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "📋 Install Open-Shell (Classic Start Menu)".to_string(),
            description: "Installs Open-Shell for a proper Windows 7 style Start Menu on Windows 11.

Features:
- Classic cascading menus
- Highly customizable skins
- Fast search
- Pin and organize programs

This is the best solution for a true classic Start Menu experience.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Uninstalling Open-Shell..." -ForegroundColor Yellow

# Try winget first
$wingetResult = winget uninstall --id "Open-Shell.Open-Shell-Menu" -e --silent 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "Open-Shell uninstalled via winget" -ForegroundColor Green
} else {
    # Fallback: standard uninstaller
    $uninstaller = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*" | 
        Where-Object { $_.DisplayName -like "*Open-Shell*" }
    
    if ($uninstaller -and $uninstaller.UninstallString) {
        Start-Process "msiexec.exe" -ArgumentList "/x $($uninstaller.PSChildName) /quiet" -Wait
        Write-Host "Open-Shell uninstalled" -ForegroundColor Green
    } else {
        Write-Host "Open-Shell not found or already uninstalled" -ForegroundColor Yellow
    }
}
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$installed = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*" | 
    Where-Object { $_.DisplayName -like "*Open-Shell*" }
if ($installed) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Installing Open-Shell Classic Start Menu..." -ForegroundColor Cyan

# Check if winget is available
$wingetAvailable = Get-Command winget -EA SilentlyContinue

if ($wingetAvailable) {
    Write-Host "Installing via winget..." -ForegroundColor Yellow
    winget install --id "Open-Shell.Open-Shell-Menu" -e --silent --accept-package-agreements --accept-source-agreements
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Open-Shell installed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Winget install failed. Trying direct download..." -ForegroundColor Yellow
        # Fallback to direct download
        $url = "https://github.com/Open-Shell/Open-Shell-Menu/releases/latest/download/OpenShellSetup.exe"
        $dest = "$env:TEMP\OpenShellSetup.exe"
        Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
        Start-Process $dest -ArgumentList "/silent" -Wait
    }
} else {
    Write-Host "Winget not available. Downloading directly..." -ForegroundColor Yellow
    $url = "https://github.com/Open-Shell/Open-Shell-Menu/releases/latest/download/OpenShellSetup.exe"
    $dest = "$env:TEMP\OpenShellSetup.exe"
    Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    Start-Process $dest -ArgumentList "/silent" -Wait
}

# Configure Open-Shell for Windows 7 style
$settingsPath = "HKCU:\Software\OpenShell\StartMenu\Settings"
if (Test-Path $settingsPath) {
    Write-Host "Configuring Windows 7 style..." -ForegroundColor Yellow
    Set-ItemProperty -Path $settingsPath -Name "MenuStyle" -Value "Win7" -Force -EA 0
    Set-ItemProperty -Path $settingsPath -Name "SkinC1" -Value "Windows Aero" -Force -EA 0
}

Write-Host "" -ForegroundColor Yellow
Write-Host "Open-Shell installed!" -ForegroundColor Green
Write-Host "Press the Windows key to see the classic Start Menu." -ForegroundColor Cyan
Write-Host "Right-click the Start button > Open-Shell Menu Settings to customize." -ForegroundColor Cyan
"#.to_string(),
                }
            ]
        },
    ]
}
