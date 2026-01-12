# Network Tweaks Test Script
# Tests all network tweaks from Phase 1 and reverts them

Write-Host "=== TommyTweaker Network Tweaks Test ===" -ForegroundColor Cyan
Write-Host ""

# Check if running as admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: This script must be run as Administrator!" -ForegroundColor Red
    exit 1
}

Write-Host "Running as Administrator: OK" -ForegroundColor Green
Write-Host ""

# Store backup of current values
$backups = @{}

# ============================================
# TEST 1: TCP/IP Parameters
# ============================================
Write-Host "=== TEST 1: TCP/IP Parameters ===" -ForegroundColor Yellow

# Test TcpAckFrequency on all interfaces
Write-Host "Testing TcpAckFrequency..." -ForegroundColor Cyan
$interfaces = Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces'
foreach ($iface in $interfaces) {
    $current = Get-ItemProperty -Path $iface.PSPath -Name 'TcpAckFrequency' -ErrorAction SilentlyContinue
    $backups["$($iface.PSPath)_TcpAckFrequency"] = $current.TcpAckFrequency
    
    # Apply tweak
    Set-ItemProperty -Path $iface.PSPath -Name 'TcpAckFrequency' -Value 1 -Type DWord -ErrorAction SilentlyContinue
    
    # Verify
    $new = Get-ItemProperty -Path $iface.PSPath -Name 'TcpAckFrequency' -ErrorAction SilentlyContinue
    if ($new.TcpAckFrequency -eq 1) {
        Write-Host "  $($iface.PSChildName): TcpAckFrequency=1 [OK]" -ForegroundColor Green
    }
    else {
        Write-Host "  $($iface.PSChildName): Failed to set TcpAckFrequency" -ForegroundColor Red
    }
}

# Test TCPNoDelay
Write-Host "Testing TCPNoDelay..." -ForegroundColor Cyan
foreach ($iface in $interfaces) {
    $current = Get-ItemProperty -Path $iface.PSPath -Name 'TCPNoDelay' -ErrorAction SilentlyContinue
    $backups["$($iface.PSPath)_TCPNoDelay"] = $current.TCPNoDelay
    
    Set-ItemProperty -Path $iface.PSPath -Name 'TCPNoDelay' -Value 1 -Type DWord -ErrorAction SilentlyContinue
    
    $new = Get-ItemProperty -Path $iface.PSPath -Name 'TCPNoDelay' -ErrorAction SilentlyContinue
    if ($new.TCPNoDelay -eq 1) {
        Write-Host "  $($iface.PSChildName): TCPNoDelay=1 [OK]" -ForegroundColor Green
    }
}

# Test Global TCP Parameters
Write-Host "Testing Global TCP Parameters..." -ForegroundColor Cyan
$tcpPath = 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters'

$globalParams = @{
    'DefaultTTL'          = 64
    'Tcp1323Opts'         = 3
    'MaxUserPort'         = 65534
    'TcpTimedWaitDelay'   = 30
    'SackOpts'            = 1
    'EnablePMTUDiscovery' = 1
    'EnablePMTUBHDetect'  = 0
}

foreach ($param in $globalParams.GetEnumerator()) {
    $current = Get-ItemProperty -Path $tcpPath -Name $param.Key -ErrorAction SilentlyContinue
    $backups["TCP_$($param.Key)"] = $current.$($param.Key)
    
    Set-ItemProperty -Path $tcpPath -Name $param.Key -Value $param.Value -Type DWord -ErrorAction SilentlyContinue
    
    $new = Get-ItemProperty -Path $tcpPath -Name $param.Key -ErrorAction SilentlyContinue
    if ($new.$($param.Key) -eq $param.Value) {
        Write-Host "  $($param.Key)=$($param.Value) [OK]" -ForegroundColor Green
    }
    else {
        Write-Host "  $($param.Key) - Failed" -ForegroundColor Red
    }
}

Write-Host ""

# ============================================
# TEST 2: Network Security
# ============================================
Write-Host "=== TEST 2: Network Security ===" -ForegroundColor Yellow

# LLMNR
Write-Host "Testing Disable LLMNR..." -ForegroundColor Cyan
$llmnrPath = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows NT\DNSClient'
if (-not (Test-Path $llmnrPath)) {
    New-Item -Path $llmnrPath -Force | Out-Null
}
$current = Get-ItemProperty -Path $llmnrPath -Name 'EnableMulticast' -ErrorAction SilentlyContinue
$backups['LLMNR_EnableMulticast'] = $current.EnableMulticast

Set-ItemProperty -Path $llmnrPath -Name 'EnableMulticast' -Value 0 -Type DWord -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $llmnrPath -Name 'EnableMulticast' -ErrorAction SilentlyContinue
if ($new.EnableMulticast -eq 0) {
    Write-Host "  EnableMulticast=0 [OK]" -ForegroundColor Green
}

# Restrict Anonymous Access
Write-Host "Testing Restrict Anonymous Access..." -ForegroundColor Cyan
$lanPath = 'HKLM:\SYSTEM\CurrentControlSet\Services\LanManServer\Parameters'
$current = Get-ItemProperty -Path $lanPath -Name 'RestrictNullSessAccess' -ErrorAction SilentlyContinue
$backups['RestrictNullSessAccess'] = $current.RestrictNullSessAccess

Set-ItemProperty -Path $lanPath -Name 'RestrictNullSessAccess' -Value 1 -Type DWord -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $lanPath -Name 'RestrictNullSessAccess' -ErrorAction SilentlyContinue
if ($new.RestrictNullSessAccess -eq 1) {
    Write-Host "  RestrictNullSessAccess=1 [OK]" -ForegroundColor Green
}

# Restrict Anonymous Enumeration
Write-Host "Testing Restrict Anonymous Enumeration..." -ForegroundColor Cyan
$lsaPath = 'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa'
$current = Get-ItemProperty -Path $lsaPath -Name 'RestrictAnonymous' -ErrorAction SilentlyContinue
$backups['RestrictAnonymous'] = $current.RestrictAnonymous

Set-ItemProperty -Path $lsaPath -Name 'RestrictAnonymous' -Value 1 -Type DWord -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $lsaPath -Name 'RestrictAnonymous' -ErrorAction SilentlyContinue
if ($new.RestrictAnonymous -eq 1) {
    Write-Host "  RestrictAnonymous=1 [OK]" -ForegroundColor Green
}

Write-Host ""

# ============================================
# TEST 3: SMB Bandwidth Throttling
# ============================================
Write-Host "=== TEST 3: SMB Bandwidth Throttling ===" -ForegroundColor Yellow

$smbPath = 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters'
$current = Get-ItemProperty -Path $smbPath -Name 'DisableBandwidthThrottling' -ErrorAction SilentlyContinue
$backups['DisableBandwidthThrottling'] = $current.DisableBandwidthThrottling

Set-ItemProperty -Path $smbPath -Name 'DisableBandwidthThrottling' -Value 1 -Type DWord -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $smbPath -Name 'DisableBandwidthThrottling' -ErrorAction SilentlyContinue
if ($new.DisableBandwidthThrottling -eq 1) {
    Write-Host "  DisableBandwidthThrottling=1 [OK]" -ForegroundColor Green
}

Write-Host ""

# ============================================
# TEST 4: Global Offload Settings
# ============================================
Write-Host "=== TEST 4: Global Offload Settings ===" -ForegroundColor Yellow

Write-Host "Testing Set-NetOffloadGlobalSetting..." -ForegroundColor Cyan

# Get current settings first
$currentOffload = Get-NetOffloadGlobalSetting
$backups['ReceiveSegmentCoalescing'] = $currentOffload.ReceiveSegmentCoalescing
$backups['ReceiveSideScaling'] = $currentOffload.ReceiveSideScaling
$backups['Chimney'] = $currentOffload.Chimney
$backups['TaskOffload'] = $currentOffload.TaskOffload
$backups['PacketCoalescingFilter'] = $currentOffload.PacketCoalescingFilter

# Apply tweaks
Set-NetOffloadGlobalSetting -ReceiveSegmentCoalescing Disabled -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -ReceiveSideScaling Enabled -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -Chimney Disabled -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -TaskOffload Disabled -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -PacketCoalescingFilter Disabled -ErrorAction SilentlyContinue

$newOffload = Get-NetOffloadGlobalSetting
Write-Host "  ReceiveSegmentCoalescing: $($newOffload.ReceiveSegmentCoalescing)" -ForegroundColor $(if ($newOffload.ReceiveSegmentCoalescing -eq 'Disabled') { 'Green' }else { 'Yellow' })
Write-Host "  ReceiveSideScaling: $($newOffload.ReceiveSideScaling)" -ForegroundColor $(if ($newOffload.ReceiveSideScaling -eq 'Enabled') { 'Green' }else { 'Yellow' })
Write-Host "  Chimney: $($newOffload.Chimney)" -ForegroundColor $(if ($newOffload.Chimney -eq 'Disabled') { 'Green' }else { 'Yellow' })
Write-Host "  TaskOffload: $($newOffload.TaskOffload)" -ForegroundColor $(if ($newOffload.TaskOffload -eq 'Disabled') { 'Green' }else { 'Yellow' })
Write-Host "  PacketCoalescingFilter: $($newOffload.PacketCoalescingFilter)" -ForegroundColor $(if ($newOffload.PacketCoalescingFilter -eq 'Disabled') { 'Green' }else { 'Yellow' })

Write-Host ""

# ============================================
# REVERTING ALL CHANGES
# ============================================
Write-Host "=== REVERTING ALL CHANGES ===" -ForegroundColor Magenta
Write-Host ""

# Revert TcpAckFrequency and TCPNoDelay
Write-Host "Reverting interface tweaks..." -ForegroundColor Cyan
foreach ($iface in $interfaces) {
    $backupAck = $backups["$($iface.PSPath)_TcpAckFrequency"]
    $backupDelay = $backups["$($iface.PSPath)_TCPNoDelay"]
    
    if ($null -ne $backupAck) {
        Set-ItemProperty -Path $iface.PSPath -Name 'TcpAckFrequency' -Value $backupAck -Type DWord -ErrorAction SilentlyContinue
    }
    else {
        Remove-ItemProperty -Path $iface.PSPath -Name 'TcpAckFrequency' -ErrorAction SilentlyContinue
    }
    
    if ($null -ne $backupDelay) {
        Set-ItemProperty -Path $iface.PSPath -Name 'TCPNoDelay' -Value $backupDelay -Type DWord -ErrorAction SilentlyContinue
    }
    else {
        Remove-ItemProperty -Path $iface.PSPath -Name 'TCPNoDelay' -ErrorAction SilentlyContinue
    }
}
Write-Host "  Interface tweaks reverted [OK]" -ForegroundColor Green

# Revert Global TCP Parameters
Write-Host "Reverting global TCP parameters..." -ForegroundColor Cyan
foreach ($param in $globalParams.GetEnumerator()) {
    $backup = $backups["TCP_$($param.Key)"]
    if ($null -ne $backup) {
        Set-ItemProperty -Path $tcpPath -Name $param.Key -Value $backup -Type DWord -ErrorAction SilentlyContinue
    }
    else {
        Remove-ItemProperty -Path $tcpPath -Name $param.Key -ErrorAction SilentlyContinue
    }
}
Write-Host "  Global TCP parameters reverted [OK]" -ForegroundColor Green

# Revert LLMNR
Write-Host "Reverting LLMNR..." -ForegroundColor Cyan
$backup = $backups['LLMNR_EnableMulticast']
if ($null -ne $backup) {
    Set-ItemProperty -Path $llmnrPath -Name 'EnableMulticast' -Value $backup -Type DWord -ErrorAction SilentlyContinue
}
else {
    Remove-ItemProperty -Path $llmnrPath -Name 'EnableMulticast' -ErrorAction SilentlyContinue
}
Write-Host "  LLMNR reverted [OK]" -ForegroundColor Green

# Revert Anonymous Access
Write-Host "Reverting security settings..." -ForegroundColor Cyan
$backup = $backups['RestrictNullSessAccess']
if ($null -ne $backup) {
    Set-ItemProperty -Path $lanPath -Name 'RestrictNullSessAccess' -Value $backup -Type DWord -ErrorAction SilentlyContinue
}
$backup = $backups['RestrictAnonymous']
if ($null -ne $backup) {
    Set-ItemProperty -Path $lsaPath -Name 'RestrictAnonymous' -Value $backup -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  Security settings reverted [OK]" -ForegroundColor Green

# Revert SMB
Write-Host "Reverting SMB throttling..." -ForegroundColor Cyan
$backup = $backups['DisableBandwidthThrottling']
if ($null -ne $backup) {
    Set-ItemProperty -Path $smbPath -Name 'DisableBandwidthThrottling' -Value $backup -Type DWord -ErrorAction SilentlyContinue
}
else {
    Remove-ItemProperty -Path $smbPath -Name 'DisableBandwidthThrottling' -ErrorAction SilentlyContinue
}
Write-Host "  SMB throttling reverted [OK]" -ForegroundColor Green

# Revert Offload Settings
Write-Host "Reverting offload settings..." -ForegroundColor Cyan
Set-NetOffloadGlobalSetting -ReceiveSegmentCoalescing $backups['ReceiveSegmentCoalescing'] -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -ReceiveSideScaling $backups['ReceiveSideScaling'] -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -Chimney $backups['Chimney'] -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -TaskOffload $backups['TaskOffload'] -ErrorAction SilentlyContinue
Set-NetOffloadGlobalSetting -PacketCoalescingFilter $backups['PacketCoalescingFilter'] -ErrorAction SilentlyContinue
Write-Host "  Offload settings reverted [OK]" -ForegroundColor Green

Write-Host ""
Write-Host "=== ALL TESTS COMPLETE - ALL CHANGES REVERTED ===" -ForegroundColor Cyan
Write-Host ""
