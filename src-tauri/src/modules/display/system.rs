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
        name: "Request 0.5ms Timer Resolution".to_string(),
        description: "Runs a persistent SYSTEM task that requests a 0.5ms Windows timer resolution. This can help a narrow set of latency-sensitive applications, but increases timer wakeups and power use; measure the target workload before and after.".to_string(),
        warning_level: WarningLevel::Careful,
        requires_restart: false,
        revert_operations: Some(vec![TweakOperation::Powershell {
            script: r#"
$taskName = "TommyTweaker.SetTimerResolution"
$scriptPath = Join-Path $env:ProgramData "TommyTweaker\SetTimerResolution.ps1"
Stop-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction SilentlyContinue
Remove-Item -LiteralPath $scriptPath -Force -ErrorAction SilentlyContinue
Write-Host "Stopped the timer-resolution holder and restored Windows timer selection." -ForegroundColor Green
"#
            .to_string(),
        }]),
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::Powershell {
            script: r#"
$task = Get-ScheduledTask -TaskName "TommyTweaker.SetTimerResolution" -ErrorAction SilentlyContinue
if ($task -and $task.State -ne "Disabled") { "True" } else { "False" }
"#
            .to_string(),
            expected_output: "True".to_string(),
        }),
        operations: vec![TweakOperation::Powershell {
            script: r#"
$taskName = "TommyTweaker.SetTimerResolution"
$directory = Join-Path $env:ProgramData "TommyTweaker"
$scriptPath = Join-Path $directory "SetTimerResolution.ps1"
New-Item -ItemType Directory -Path $directory -Force | Out-Null

$timerHolder = @'
Add-Type @"
using System;
using System.Runtime.InteropServices;
public static class TommyTweakerTimerResolution {
    [DllImport("ntdll.dll")]
    public static extern int NtSetTimerResolution(uint desired, bool set, out uint current);
}
"@
$current = [uint32]0
$result = [TommyTweakerTimerResolution]::NtSetTimerResolution(5000, $true, [ref]$current)
if ($result -ne 0) { throw "NtSetTimerResolution failed with NTSTATUS $result" }
try {
    while ($true) { Start-Sleep -Seconds 3600 }
} finally {
    [void][TommyTweakerTimerResolution]::NtSetTimerResolution(5000, $false, [ref]$current)
}
'@
Set-Content -LiteralPath $scriptPath -Value $timerHolder -Encoding UTF8 -Force

Stop-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction SilentlyContinue
$action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument "-NoProfile -NonInteractive -WindowStyle Hidden -ExecutionPolicy Bypass -File `"$scriptPath`""
$trigger = New-ScheduledTaskTrigger -AtStartup
$principal = New-ScheduledTaskPrincipal -UserId "SYSTEM" -LogonType ServiceAccount -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -ExecutionTimeLimit ([TimeSpan]::Zero)
Register-ScheduledTask -TaskName $taskName -Action $action -Trigger $trigger -Principal $principal -Settings $settings -Force | Out-Null
Start-ScheduledTask -TaskName $taskName
Start-Sleep -Milliseconds 500
$task = Get-ScheduledTask -TaskName $taskName -ErrorAction Stop
if ($task.State -eq "Disabled") { throw "Timer-resolution task is disabled" }
Write-Host "0.5ms timer-resolution holder started. Task: $taskName" -ForegroundColor Green
"#
            .to_string(),
        }],
    }
}
