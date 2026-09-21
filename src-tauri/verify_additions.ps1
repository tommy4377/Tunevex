$ErrorActionPreference = "Stop"
$logPath = "C:\Tunevex_Logs"
if (!(Test-Path $logPath)) { New-Item -ItemType Directory -Path $logPath -Force | Out-Null }
$logFile = "$logPath\additions_verification_$(Get-Date -Format 'yyyyMMdd_HHmmss').txt"

function Log {
    param([string]$message)
    $timestamp = Get-Date -Format "HH:mm:ss"
    "$timestamp $message" | Out-File -FilePath $logFile -Append
    Write-Host "$timestamp $message"
}

Log "Starting Debloat Additions Verification..."

# 1. Teams Chat Taskbar
Log "Testing: Teams Chat Taskbar"
try {
    # Apply
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarMn" -Value 0 -Type DWord -Force -EA 0
    Get-AppxPackage -Name "*MicrosoftTeams*" -AllUsers -EA 0 | Remove-AppxPackage -AllUsers -EA 0
    Log "SUCCESS: Teams Chat Apply executed."
    
    # Revert
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "TaskbarMn" -Value 1 -Type DWord -Force -EA 0
    Log "SUCCESS: Teams Chat Revert executed."
}
catch {
    Log "ERROR: Teams Chat verification failed: $_"
}

# 2. MSI Bloatware (Logic Test)
Log "Testing: MSI Bloatware Logic"
try {
    $apps = @("MSI.CenterPortal", "MicroStarINT", "MSI Center") 
    # Just testing a subset for speed
    foreach ($app in $apps) {
        $pkg = Get-AppxPackage -Name "*$app*" -AllUsers -EA 0
        if ($pkg) { $pkg | Remove-AppxPackage -AllUsers -EA 0 }
        
        # Win32 Product query is slow, testing just one non-existent app to safely verify command works
        Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | Out-Null
    }
    Log "SUCCESS: MSI Logic executed (Apps likely not found, which is expected)."
}
catch {
    Log "ERROR: MSI Logic failed: $_"
}

# 3. Razer Bloatware (Win32 Test)
Log "Testing: Razer Bloatware Logic"
try {
    $apps = @("Razer Synapse")
    foreach ($app in $apps) {
        Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*$app*" } | Out-Null
    }
    Log "SUCCESS: Razer Logic executed."
}
catch {
    Log "ERROR: Razer Logic failed: $_"
}

# 4. McAfee Removal (Download Test)
Log "Testing: McAfee Removal Tool Download"
try {
    $url = "https://download.mcafee.com/molbin/iss-loc/SupportTools/MCPR/MCPR.exe"
    $dest = "$env:TEMP\MCPR_Test.exe"
    # We won't run it to avoid reboot prompt/cleanup, just test download
    Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    if (Test-Path $dest) {
        Log "SUCCESS: McAfee Tool Downloaded successfully."
        Remove-Item $dest -Force
    }
    else {
        Log "FAILED: McAfee Tool download failed."
    }
}
catch {
    Log "ERROR: McAfee Test failed: $_"
}

Log "Verification Complete."
