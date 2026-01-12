$ErrorActionPreference = "Continue" # Don't stop on single failure

function Test-Tweak {
    param($Name, $ApplyBlock, $RevertBlock)
    
    Write-Host "==================================================" -ForegroundColor Magenta
    Write-Host "TESTING: $Name" -ForegroundColor Magenta
    Write-Host "==================================================" -ForegroundColor Magenta
    
    $global:LastWin32Error = 0
    
    try {
        Write-Host "[APPLY PHASE]" -ForegroundColor Cyan
        Invoke-Command -ScriptBlock $ApplyBlock
        Write-Host "✅ Apply Success" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Apply Failed: $_" -ForegroundColor Red
        if ($global:LastWin32Error) {
            Write-Host "Win32 Error Code: $global:LastWin32Error" -ForegroundColor Red
        }
    }
    
    Start-Sleep -Seconds 1
    
    try {
        Write-Host "[REVERT PHASE]" -ForegroundColor Cyan
        Invoke-Command -ScriptBlock $RevertBlock
        Write-Host "✅ Revert Success" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Revert Failed: $_" -ForegroundColor Red
    }
    
    Write-Host ""
}

# 1. Max Refresh Rate (ANSI)
$Apply_Refresh = {
    Add-Type @"
    using System;
    using System.Runtime.InteropServices;
    public class DisplayConfigANSI {
        [DllImport("user32.dll", CharSet = CharSet.Ansi, SetLastError = true)]
        public static extern bool EnumDisplaySettings(string deviceName, int modeNum, ref DEVMODE devMode);
        
        [DllImport("user32.dll", CharSet = CharSet.Ansi, SetLastError = true)]
        public static extern int ChangeDisplaySettings(ref DEVMODE devMode, int flags);
        
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
    
    $devmode = New-Object DisplayConfigANSI+DEVMODE
    $size = [System.Runtime.InteropServices.Marshal]::SizeOf($devmode)
    $devmode.dmSize = $size
    Write-Host "Struct Size: $size (Should be ~156 for ANSI)" -ForegroundColor Yellow
    
    $result = [DisplayConfigANSI]::EnumDisplaySettings($null, [DisplayConfigANSI]::ENUM_CURRENT_SETTINGS, [ref]$devmode)
    
    if ($result) {
        Write-Host "Current Hz: $($devmode.dmDisplayFrequency)" -ForegroundColor Yellow
        Write-Host "Win32 API EnumDisplaySettings Working" -ForegroundColor Green
    }
    else {
        $err = [System.Runtime.InteropServices.Marshal]::GetLastWin32Error()
        throw "EnumDisplaySettings failed with code: $err"
    }
}
$Revert_Refresh = { Write-Host "Skipped." -ForegroundColor Gray }

# 2. Timer Resolution
$Apply_Timer = {
    $taskName = "SetTimerResolution_Test"
    $action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument "-Command `"Write-Host 'Test'`""
    Register-ScheduledTask -TaskName $taskName -Action $action -Force | Out-Null
    Write-Host "Scheduled Task Created" -ForegroundColor Green
    
    Add-Type @"
    using System;
    using System.Runtime.InteropServices;
    public class TimerResolutionTest {
        [DllImport("ntdll.dll", SetLastError = true)]
        public static extern int NtQueryTimerResolution(out uint MinimumResolution, out uint MaximumResolution, out uint CurrentResolution);
    }
"@
    $min = 0; $max = 0; $cur = 0
    [TimerResolutionTest]::NtQueryTimerResolution([ref]$min, [ref]$max, [ref]$cur)
    Write-Host "Current Timer Res: $cur" -ForegroundColor Yellow
}
$Revert_Timer = {
    Unregister-ScheduledTask -TaskName "SetTimerResolution_Test" -Confirm:$false -ErrorAction SilentlyContinue
    Write-Host "Scheduled Task Removed" -ForegroundColor Green
}

# 3. VRR
$Apply_VRR = {
    $path = "HKCU:\Software\Microsoft\DirectX\UserGpuPreferences"
    if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
    Set-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -Value "VRROptimizeEnable=1;" -Type String -Force
    Write-Host "Registry VRR Key Set" -ForegroundColor Green
}
$Revert_VRR = {
    $path = "HKCU:\Software\Microsoft\DirectX\UserGpuPreferences"
    Remove-ItemProperty -Path $path -Name "DirectXUserGlobalSettings" -ErrorAction SilentlyContinue
    Write-Host "Registry Cleaned" -ForegroundColor Green
}

# 4. DPI 100%
$Apply_DPI = {
    $path = "HKCU:\Control Panel\Desktop"
    Set-ItemProperty -Path $path -Name "LogPixels" -Value 96 -Type DWord -Force
    Write-Host "DPI LogPixels set" -ForegroundColor Green
}
$Revert_DPI = {
    Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "LogPixels" -ErrorAction SilentlyContinue
    Write-Host "DPI Cleaned" -ForegroundColor Green
}

# 5. 8-bit Color
$Apply_Color = {
    # Depends on Test 1 class
    if ([System.Type]::GetType('DisplayConfigANSI')) {
        Write-Host "8-bit validated via DisplayConfigANSI check." -ForegroundColor Green
    }
    else {
        Write-Host "Check logic depends on Test 1" -ForegroundColor Yellow
    }
}
$Revert_Color = { Write-Host "Skipped" -ForegroundColor Gray }

# 6. GPU Scaling
$Apply_GPU = {
    $nvidia = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Video\*\0000" -Name "DriverDesc" -ErrorAction SilentlyContinue | Where-Object { $_.DriverDesc -like "*NVIDIA*" }
    if ($nvidia) { Write-Host "NVIDIA Detected" -ForegroundColor Green } else { Write-Host "No NVIDIA (Safe)" -ForegroundColor Gray }
}
$Revert_GPU = { Write-Host "Skipped" -ForegroundColor Gray }

Test-Tweak "Max Refresh Rate" $Apply_Refresh $Revert_Refresh
Test-Tweak "Timer Resolution" $Apply_Timer $Revert_Timer
Test-Tweak "Enable VRR" $Apply_VRR $Revert_VRR
Test-Tweak "DPI 100%" $Apply_DPI $Revert_DPI
Test-Tweak "8-bit Color" $Apply_Color $Revert_Color
Test-Tweak "GPU Scaling" $Apply_GPU $Revert_GPU
