use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_monitor_tweaks() -> Vec<Tweak> {
    vec![
        tweak_max_refresh_rate(),
        tweak_dpi_100(),
        tweak_8bit_color(),
    ]
}

fn tweak_max_refresh_rate() -> Tweak {
    Tweak {
        id: "display_max_refresh_rate".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Set Monitor to Maximum Refresh Rate".to_string(),
        description: "Automatically sets your monitor to its maximum supported refresh rate (120Hz/144Hz/165Hz/240Hz/360Hz).".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
Write-Host "This tweak cannot be reverted automatically - it only sets to maximum." -ForegroundColor Yellow
Write-Host "Your monitor is now at its maximum supported refresh rate." -ForegroundColor Cyan
Write-Host "To change: Settings > Display > Advanced display > Choose a refresh rate" -ForegroundColor Cyan
"#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class DisplayConfig {
    [DllImport("user32.dll")]
    public static extern int EnumDisplaySettings(string deviceName, int modeNum, ref DEVMODE devMode);
    
    public const int ENUM_CURRENT_SETTINGS = -1;
    
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct DEVMODE {
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmDeviceName;
        public short dmSpecVersion;
        public short dmDriverVersion;
        public short dmSize;
        public short dmDriverExtra;
        public int dmFields;
        public int dmPositionX;
        public int dmPositionY;
        public int dmDisplayOrientation;
        public int dmDisplayFixedOutput;
        public short dmColor;
        public short dmDuplex;
        public short dmYResolution;
        public short dmTTOption;
        public short dmCollate;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmFormName;
        public short dmLogPixels;
        public int dmBitsPerPel;
        public int dmPelsWidth;
        public int dmPelsHeight;
        public int dmDisplayFlags;
        public int dmDisplayFrequency;
        public int dmICMMethod;
        public int dmICMIntent;
        public int dmMediaType;
        public int dmDitherType;
        public int dmReserved1;
        public int dmReserved2;
        public int dmPanningWidth;
        public int dmPanningHeight;
    }
}
"@

$devmode = New-Object DisplayConfig+DEVMODE
$devmode.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($devmode)

# Get current display settings
[DisplayConfig]::EnumDisplaySettings($null, [DisplayConfig]::ENUM_CURRENT_SETTINGS, [ref]$devmode)
$currentHz = $devmode.dmDisplayFrequency

# Find max Hz for current resolution
$maxHz = $currentHz
$modeNum = 0
while ($true) {
    $testMode = New-Object DisplayConfig+DEVMODE
    $testMode.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($testMode)
    
    $result = [DisplayConfig]::EnumDisplaySettings($null, $modeNum, [ref]$testMode)
    if ($result -eq 0) { break }
    
    if ($testMode.dmPelsWidth -eq $devmode.dmPelsWidth -and 
        $testMode.dmPelsHeight -eq $devmode.dmPelsHeight -and
        $testMode.dmBitsPerPel -eq $devmode.dmBitsPerPel) {
        
        if ($testMode.dmDisplayFrequency -gt $maxHz) {
            $maxHz = $testMode.dmDisplayFrequency
        }
    }
    $modeNum++
}

if ($currentHz -ge $maxHz) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class DisplayConfig {
    [DllImport("user32.dll")]
    public static extern int EnumDisplaySettings(string deviceName, int modeNum, ref DEVMODE devMode);
    
    [DllImport("user32.dll")]
    public static extern int ChangeDisplaySettings(ref DEVMODE devMode, int flags);
    
    public const int ENUM_CURRENT_SETTINGS = -1;
    public const int CDS_UPDATEREGISTRY = 0x01;
    public const int DISP_CHANGE_SUCCESSFUL = 0;
    
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct DEVMODE {
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmDeviceName;
        public short dmSpecVersion;
        public short dmDriverVersion;
        public short dmSize;
        public short dmDriverExtra;
        public int dmFields;
        public int dmPositionX;
        public int dmPositionY;
        public int dmDisplayOrientation;
        public int dmDisplayFixedOutput;
        public short dmColor;
        public short dmDuplex;
        public short dmYResolution;
        public short dmTTOption;
        public short dmCollate;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmFormName;
        public short dmLogPixels;
        public int dmBitsPerPel;
        public int dmPelsWidth;
        public int dmPelsHeight;
        public int dmDisplayFlags;
        public int dmDisplayFrequency;
        public int dmICMMethod;
        public int dmICMIntent;
        public int dmMediaType;
        public int dmDitherType;
        public int dmReserved1;
        public int dmReserved2;
        public int dmPanningWidth;
        public int dmPanningHeight;
    }
}
"@

Write-Host "Detecting maximum refresh rate..." -ForegroundColor Cyan

$devmode = New-Object DisplayConfig+DEVMODE
$devmode.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($devmode)

# Get current display settings
$result = [DisplayConfig]::EnumDisplaySettings($null, [DisplayConfig]::ENUM_CURRENT_SETTINGS, [ref]$devmode)

if ($result -eq 0) {
    Write-Host "Failed to get display settings" -ForegroundColor Red
    exit 1
}

$currentHz = $devmode.dmDisplayFrequency
Write-Host "Current refresh rate: $currentHz Hz" -ForegroundColor Yellow

# Find maximum supported refresh rate
$maxHz = $currentHz
$maxMode = $devmode

$modeNum = 0
while ($true) {
    $testMode = New-Object DisplayConfig+DEVMODE
    $testMode.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($testMode)
    
    $result = [DisplayConfig]::EnumDisplaySettings($null, $modeNum, [ref]$testMode)
    if ($result -eq 0) { break }
    
    # Check if same resolution as current
    if ($testMode.dmPelsWidth -eq $devmode.dmPelsWidth -and 
        $testMode.dmPelsHeight -eq $devmode.dmPelsHeight -and
        $testMode.dmBitsPerPel -eq $devmode.dmBitsPerPel) {
        
        if ($testMode.dmDisplayFrequency -gt $maxHz) {
            $maxHz = $testMode.dmDisplayFrequency
            $maxMode = $testMode
        }
    }
    
    $modeNum++
}

if ($maxHz -eq $currentHz) {
    Write-Host "Monitor already at maximum refresh rate ($maxHz Hz)" -ForegroundColor Green
} else {
    Write-Host "Setting refresh rate to $maxHz Hz..." -ForegroundColor Cyan
    
    $maxMode.dmFields = 0x00400000  # DM_DISPLAYFREQUENCY
    $changeResult = [DisplayConfig]::ChangeDisplaySettings([ref]$maxMode, [DisplayConfig]::CDS_UPDATEREGISTRY)
    
    if ($changeResult -eq [DisplayConfig]::DISP_CHANGE_SUCCESSFUL) {
        Write-Host "Refresh rate set to $maxHz Hz successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to change refresh rate (Error code: $changeResult)" -ForegroundColor Red
    }
}
        "#.to_string(),
        }],
    }
}

fn tweak_dpi_100() -> Tweak {
    Tweak {
        id: "display_dpi_100".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Set DPI Scaling to 100%".to_string(),
        description: "Forces DPI scaling to 100% to avoid blur and rendering overhead in games. May make UI smaller on high-resolution displays.".to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: true,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
            Write-Host "Reverting to Windows recommended DPI scaling..." -ForegroundColor Cyan

            $desktopPath = "HKCU:\Control Panel\Desktop"
            Remove-ItemProperty -Path $desktopPath -Name "LogPixels" -Force -EA 0
            Remove-ItemProperty -Path $desktopPath -Name "Win8DpiScaling" -Force -EA 0

            $path = "HKCU:\Control Panel\Desktop\WindowMetrics"
            Remove-ItemProperty -Path $path -Name "AppliedDPI" -Force -EA 0

            Write-Host "DPI scaling reset to Windows recommended" -ForegroundColor Green
            Write-Host "Sign out and sign in for changes to take effect" -ForegroundColor Yellow
        "#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Registry {
            root_key: "HKCU".to_string(),
            path: "Control Panel\\Desktop".to_string(),
            key: "LogPixels".to_string(),
            expected_value: RegistryValue::DWord(96),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Setting DPI scaling to 100%..." -ForegroundColor Cyan

# Set global DPI to 96 (100%)
$desktopPath = "HKCU:\Control Panel\Desktop"
Set-ItemProperty -Path $desktopPath -Name "LogPixels" -Value 96 -Type DWord -Force
Set-ItemProperty -Path $desktopPath -Name "Win8DpiScaling" -Value 0 -Type DWord -Force

# Disable per-monitor DPI v2
$path = "HKCU:\Control Panel\Desktop\WindowMetrics"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "AppliedDPI" -Value 96 -Type DWord -Force

# Disable DPI awareness for better compatibility
Set-ItemProperty -Path $desktopPath -Name "EnablePerProcessSystemDPI" -Value 0 -Type DWord -Force -EA 0

Write-Host "DPI scaling set to 100%" -ForegroundColor Green
Write-Host "IMPORTANT: Sign out and sign in for changes to take effect" -ForegroundColor Yellow
        "#.to_string(),
        }],
    }
}

fn tweak_8bit_color() -> Tweak {
    Tweak {
        id: "display_8bit_color".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Force 8-bit Color Depth".to_string(),
        description: "Sets display to 8-bit (32bpp) instead of 10-bit for better compatibility with high refresh rates (144Hz+).".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
            Write-Host "To revert to 10-bit color (if supported):" -ForegroundColor Yellow
            Write-Host "Settings > Display > Advanced display > Choose a bit depth > 10-bit" -ForegroundColor Cyan
        "#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
if ((Get-CimInstance -ClassName Win32_VideoController -ErrorAction SilentlyContinue).CurrentBitsPerPixel -eq 32) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Setting color depth to 8-bit (32bpp)..." -ForegroundColor Cyan

Add-Type @"
using System;
using System.Runtime.InteropServices;
public class DisplayConfig {
    [DllImport("user32.dll")]
    public static extern int EnumDisplaySettings(string deviceName, int modeNum, ref DEVMODE devMode);
    
    [DllImport("user32.dll")]
    public static extern int ChangeDisplaySettings(ref DEVMODE devMode, int flags);
    
    public const int ENUM_CURRENT_SETTINGS = -1;
    public const int CDS_UPDATEREGISTRY = 0x01;
    
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct DEVMODE {
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmDeviceName;
        public short dmSpecVersion;
        public short dmDriverVersion;
        public short dmSize;
        public short dmDriverExtra;
        public int dmFields;
        public int dmPositionX;
        public int dmPositionY;
        public int dmDisplayOrientation;
        public int dmDisplayFixedOutput;
        public short dmColor;
        public short dmDuplex;
        public short dmYResolution;
        public short dmTTOption;
        public short dmCollate;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
        public string dmFormName;
        public short dmLogPixels;
        public int dmBitsPerPel;
        public int dmPelsWidth;
        public int dmPelsHeight;
        public int dmDisplayFlags;
        public int dmDisplayFrequency;
        public int dmICMMethod;
        public int dmICMIntent;
        public int dmMediaType;
        public int dmDitherType;
        public int dmReserved1;
        public int dmReserved2;
        public int dmPanningWidth;
        public int dmPanningHeight;
    }
}
"@

$devmode = New-Object DisplayConfig+DEVMODE
$devmode.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($devmode)

[DisplayConfig]::EnumDisplaySettings($null, [DisplayConfig]::ENUM_CURRENT_SETTINGS, [ref]$devmode)

$currentBpp = $devmode.dmBitsPerPel
Write-Host "Current color depth: $currentBpp bpp" -ForegroundColor Yellow

if ($currentBpp -ne 32) {
    // 32 bpp = 8 bit per channel (RGBA)
    $devmode.dmBitsPerPel = 32  
    $devmode.dmFields = 0x00080000  # DM_BITSPERPEL
    
    $result = [DisplayConfig]::ChangeDisplaySettings([ref]$devmode, [DisplayConfig]::CDS_UPDATEREGISTRY)
    
    if ($result -eq 0) {
        Write-Host "Color depth set to 8-bit (32bpp)" -ForegroundColor Green
    } else {
        Write-Host "Note: Some monitors may not support manual color depth changes via Win32" -ForegroundColor Yellow
        Write-Host "To set manually: Settings > Display > Advanced display > Choose a bit depth > 8-bit" -ForegroundColor Cyan
    }
} else {
    Write-Host "Already using 8-bit color depth" -ForegroundColor Green
}
        "#.to_string(),
        }],
    }
}
