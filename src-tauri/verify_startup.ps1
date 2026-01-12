$ErrorActionPreference = "Stop"
$logPath = "C:\TommyTweaker_Logs"
if (!(Test-Path $logPath)) { New-Item -ItemType Directory -Path $logPath -Force | Out-Null }
$logFile = "$logPath\startup_verification_$(Get-Date -Format 'yyyyMMdd_HHmmss').txt"

function Log {
    param([string]$message)
    $timestamp = Get-Date -Format "HH:mm:ss"
    "$timestamp $message" | Out-File -FilePath $logFile -Append
    Write-Host "$timestamp $message"
}

function Verify-ServiceTweak {
    param(
        [string]$TweakName,
        [string[]]$ServiceNames,
        [scriptblock]$ApplyBlock,
        [scriptblock]$RevertBlock,
        [string]$ExpectedState = "Disabled"
    )

    Log "=================================================="
    Log "TESTING TWEAK: $TweakName"
    Log "=================================================="
    
    # 1. APPLY
    Log "ACTION: APPLY"
    try {
        & $ApplyBlock
        Log "Apply script executed."
        
        foreach ($svc in $ServiceNames) {
            $s = Get-Service -Name $svc -ErrorAction SilentlyContinue
            if ($s) {
                if ($s.StartType -eq $ExpectedState -or ($ExpectedState -eq "Manual" -and $s.StartType -eq "Manual")) {
                    Log "VERIFICATION: SUCCESS - Service '$svc' is $ExpectedState."
                }
                else {
                    Log "VERIFICATION: FAILED - Service '$svc' is $($s.StartType), expected $ExpectedState."
                }
            }
            else {
                Log "VERIFICATION: SKIP - Service '$svc' not found on this system."
            }
        }
    }
    catch {
        Log "ERROR during Apply: $_"
    }

    # 2. REVERT
    Log "ACTION: REVERT"
    try {
        & $RevertBlock
        Log "Revert script executed."
        
        foreach ($svc in $ServiceNames) {
            $s = Get-Service -Name $svc -ErrorAction SilentlyContinue
            if ($s) {
                # Just check it's NOT disabled (usually Manual or Automatic)
                if ($s.StartType -ne "Disabled") {
                    Log "VERIFICATION: SUCCESS - Service '$svc' RESTORED to $($s.StartType)."
                }
                else {
                    Log "VERIFICATION: FAILED - Service '$svc' remains Disabled."
                }
            }
        }
    }
    catch {
        Log "ERROR during Revert: $_"
    }
}

Log "Starting Startup/Services Verification..."

# 1. Windows Search
Verify-ServiceTweak "Disable Windows Search" @("WSearch") {
    Stop-Service -Name "WSearch" -Force -EA 0
    Set-Service -Name "WSearch" -StartupType Disabled -EA 0
} {
    Set-Service -Name "WSearch" -StartupType Automatic -EA 0
    Start-Service -Name "WSearch" -EA 0
}

# 2. BITS
Verify-ServiceTweak "Set BITS to Manual" @("BITS") {
    Set-Service -Name "BITS" -StartupType Manual -EA 0
} {
    Set-Service -Name "BITS" -StartupType Automatic -EA 0
} "Manual"

# 3. Misc Services
$miscServices = @("WMPNetworkSvc", "MapsBroker", "Fax", "RetailDemo", "WalletService", "PhoneSvc", "TapiSrv", "WpcMonSvc", "lfsvc", "SharedAccess", "TabletInputService", "SEMgrSvc", "WbioSrvc", "icssvc", "MixedRealityOpenXRSvc")
Verify-ServiceTweak "Disable Misc Services" $miscServices {
    foreach ($svc in $miscServices) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
} {
    foreach ($svc in $miscServices) { Set-Service -Name $svc -StartupType Manual -EA 0 }
}

# 4. Telemetry Services
$telemetryServices = @("DiagTrack", "dmwappushservice", "diagnosticshub.standardcollector.service", "WerSvc", "wercplsupport", "PcaSvc")
Verify-ServiceTweak "Disable Telemetry Services" $telemetryServices {
    foreach ($svc in $telemetryServices) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
} {
    foreach ($svc in $telemetryServices) { Set-Service -Name $svc -StartupType Automatic -EA 0; Start-Service -Name $svc -EA 0 }
}

# 5. Remote Services
$remoteServices = @("RemoteRegistry", "RemoteAccess", "WinRM", "TermService", "SessionEnv")
Verify-ServiceTweak "Disable Remote Services" $remoteServices {
    foreach ($svc in $remoteServices) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
} {
    foreach ($svc in $remoteServices) { Set-Service -Name $svc -StartupType Manual -EA 0 }
}

# 6. Xbox Services
$xboxServices = @("XboxGipSvc", "XblAuthManager", "XboxNetApiSvc", "XblGameSave")
Verify-ServiceTweak "Disable Xbox Services" $xboxServices {
    foreach ($svc in $xboxServices) { Stop-Service -Name $svc -Force -EA 0; Set-Service -Name $svc -StartupType Disabled -EA 0 }
} {
    foreach ($svc in $xboxServices) { Set-Service -Name $svc -StartupType Manual -EA 0 }
}

Log "Verification Complete."
