use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_system_tweaks() -> Vec<Tweak> {
    vec![tweak_timer_resolution()]
}

fn tweak_timer_resolution() -> Tweak {
    Tweak {
        id: "display_timer_resolution".to_string(),
        category: TweakCategory::DisplayMonitor,
        name: "Set Timer Resolution to 0.5ms".to_string(),
        description: "Reduces Windows timer interval from 15.6ms to 0.5ms for lower input lag. May slightly increase power consumption.".to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
            Write-Host "Reverting timer resolution to Windows default..." -ForegroundColor Cyan

            # Remove scheduled task
            $taskName = "SetTimerResolution"
            Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -EA 0

            # Reset timer resolution
            Add-Type @"
            using System;
            using System.Runtime.InteropServices;
            public class TimerResolution {
                [DllImport("ntdll.dll")]
                public static extern int NtSetTimerResolution(uint DesiredResolution, bool SetResolution, out uint CurrentResolution);
            }
"@

            $res = [uint32]0
            [TimerResolution]::NtSetTimerResolution(156250, $false, [ref]$res)  # Reset to default 15.625ms

            Write-Host "Timer resolution reset to default" -ForegroundColor Green
        "#.to_string(),
        }]),
        tweak_type: TweakType::Toggle, enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
if (Get-ScheduledTask -TaskName "SetTimerResolution" -ErrorAction SilentlyContinue) { "True" } else { "False" }
"#.to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
Write-Host "Setting Windows Timer Resolution to 0.5ms..." -ForegroundColor Cyan

# Use Windows Multimedia Timer API via PowerShell
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class TimerResolution {
    [DllImport("ntdll.dll", SetLastError = true)]
    public static extern int NtSetTimerResolution(uint DesiredResolution, bool SetResolution, out uint CurrentResolution);
    
    [DllImport("ntdll.dll", SetLastError = true)]
    public static extern int NtQueryTimerResolution(out uint MinimumResolution, out uint MaximumResolution, out uint CurrentResolution);
}
"@

# Query current timer resolution
$minRes = [uint32]0
$maxRes = [uint32]0
$curRes = [uint32]0

[TimerResolution]::NtQueryTimerResolution([ref]$minRes, [ref]$maxRes, [ref]$curRes)

$currentMs = [math]::Round($curRes / 10000.0, 2)
Write-Host "Current timer resolution: $currentMs ms" -ForegroundColor Yellow

# Set to 0.5ms (5000 * 100ns = 0.5ms)
$desiredResolution = 5000  # 0.5ms in 100-nanosecond units
$newRes = [uint32]0

$result = [TimerResolution]::NtSetTimerResolution($desiredResolution, $true, [ref]$newRes)

if ($result -eq 0) {
    $newMs = [math]::Round($newRes / 10000.0, 2)
    Write-Host "Timer resolution set to $newMs ms successfully!" -ForegroundColor Green
} else {
    Write-Host "Failed to set timer resolution (Error code: $result)" -ForegroundColor Red
}

# Create scheduled task to set timer resolution on boot
$taskName = "SetTimerResolution"
$taskExists = Get-ScheduledTask -TaskName $taskName -EA 0

if (!$taskExists) {
    Write-Host "Creating scheduled task for timer resolution on startup..." -ForegroundColor Cyan
    
    $action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument "-NoProfile -WindowStyle Hidden -Command `"[void][System.Reflection.Assembly]::LoadWithPartialName('System.Runtime.InteropServices'); Add-Type @'using System; using System.Runtime.InteropServices; public class TimerResolution { [DllImport(`"ntdll.dll`")] public static extern int NtSetTimerResolution(uint DesiredResolution, bool SetResolution, out uint CurrentResolution); }'@; `$res = 0; [TimerResolution]::NtSetTimerResolution(5000, `$true, [ref]`$res)`""
    
    $trigger = New-ScheduledTaskTrigger -AtStartup
    $principal = New-ScheduledTaskPrincipal -UserId "SYSTEM" -LogonType ServiceAccount -RunLevel Highest
    $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable
    
    Register-ScheduledTask -TaskName $taskName -Action $action -Trigger $trigger -Principal $principal -Settings $settings -EA 0 | Out-Null
    
    Write-Host "Scheduled task created" -ForegroundColor Green
}
        "#.to_string(),
        }],
    }
}
