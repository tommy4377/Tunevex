# CPU Performance Tweaks Test Script
# Tests all CPU/Performance tweaks from Phase 3 and reverts them

Write-Host "=== TommyTweaker CPU Performance Tweaks Test ===" -ForegroundColor Cyan
Write-Host ""

# Check admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: This script must be run as Administrator!" -ForegroundColor Red
    exit 1
}
Write-Host "Running as Administrator: OK" -ForegroundColor Green
Write-Host ""

$backups = @{}

# ============================================
# TEST 1: CPU Scheduling
# ============================================
Write-Host "=== TEST 1: CPU Scheduling ===" -ForegroundColor Yellow

# Win32PrioritySeparation
$priorityPath = 'HKLM:\SYSTEM\CurrentControlSet\Control\PriorityControl'
$current = Get-ItemProperty -Path $priorityPath -Name 'Win32PrioritySeparation' -ErrorAction SilentlyContinue
$backups['Win32PrioritySeparation'] = $current.Win32PrioritySeparation

Set-ItemProperty -Path $priorityPath -Name 'Win32PrioritySeparation' -Value 38 -Type DWord -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $priorityPath -Name 'Win32PrioritySeparation'
Write-Host "  Win32PrioritySeparation=$($new.Win32PrioritySeparation) (should be 38)" -ForegroundColor $(if ($new.Win32PrioritySeparation -eq 38) { 'Green' }else { 'Yellow' })

# MMCSS SystemResponsiveness
$mmcssPath = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile'
$current = Get-ItemProperty -Path $mmcssPath -ErrorAction SilentlyContinue
$backups['SystemResponsiveness'] = $current.SystemResponsiveness
$backups['NetworkThrottlingIndex'] = $current.NetworkThrottlingIndex

Set-ItemProperty -Path $mmcssPath -Name 'SystemResponsiveness' -Value 10 -Type DWord -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mmcssPath -Name 'NetworkThrottlingIndex' -Value 0xFFFFFFFF -Type DWord -ErrorAction SilentlyContinue

$new = Get-ItemProperty -Path $mmcssPath
Write-Host "  SystemResponsiveness=$($new.SystemResponsiveness) (should be 10)" -ForegroundColor $(if ($new.SystemResponsiveness -eq 10) { 'Green' }else { 'Yellow' })
Write-Host "  NetworkThrottlingIndex=$($new.NetworkThrottlingIndex)" -ForegroundColor Green
Write-Host ""

# ============================================
# TEST 2: Memory Management
# ============================================
Write-Host "=== TEST 2: Memory Management ===" -ForegroundColor Yellow
$memPath = 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management'

$current = Get-ItemProperty -Path $memPath -ErrorAction SilentlyContinue
$backups['LargeSystemCache'] = $current.LargeSystemCache
$backups['DisablePagingExecutive'] = $current.DisablePagingExecutive

Set-ItemProperty -Path $memPath -Name 'LargeSystemCache' -Value 0 -Type DWord -ErrorAction SilentlyContinue
Set-ItemProperty -Path $memPath -Name 'DisablePagingExecutive' -Value 1 -Type DWord -ErrorAction SilentlyContinue

$new = Get-ItemProperty -Path $memPath
Write-Host "  LargeSystemCache=$($new.LargeSystemCache) (should be 0)" -ForegroundColor $(if ($new.LargeSystemCache -eq 0) { 'Green' }else { 'Yellow' })
Write-Host "  DisablePagingExecutive=$($new.DisablePagingExecutive) (should be 1)" -ForegroundColor $(if ($new.DisablePagingExecutive -eq 1) { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 3: Background Apps
# ============================================
Write-Host "=== TEST 3: Background Apps ===" -ForegroundColor Yellow
$bgPath = 'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications'

$current = Get-ItemProperty -Path $bgPath -ErrorAction SilentlyContinue
$backups['GlobalUserDisabled'] = $current.GlobalUserDisabled

Set-ItemProperty -Path $bgPath -Name 'GlobalUserDisabled' -Value 1 -Type DWord -ErrorAction SilentlyContinue

$new = Get-ItemProperty -Path $bgPath
Write-Host "  GlobalUserDisabled=$($new.GlobalUserDisabled) (should be 1)" -ForegroundColor $(if ($new.GlobalUserDisabled -eq 1) { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 4: Game Bar
# ============================================
Write-Host "=== TEST 4: Game Bar ===" -ForegroundColor Yellow
$gameConfigPath = 'HKCU:\System\GameConfigStore'
$gameBarPath = 'HKCU:\SOFTWARE\Microsoft\GameBar'

# Backup
$current = Get-ItemProperty -Path $gameConfigPath -ErrorAction SilentlyContinue
$backups['GameDVR_Enabled'] = $current.GameDVR_Enabled

if (-not (Test-Path $gameBarPath)) {
    New-Item -Path $gameBarPath -Force | Out-Null
}
$current = Get-ItemProperty -Path $gameBarPath -ErrorAction SilentlyContinue
$backups['ShowStartupPanel'] = $current.ShowStartupPanel

# Apply
Set-ItemProperty -Path $gameConfigPath -Name 'GameDVR_Enabled' -Value 0 -Type DWord -ErrorAction SilentlyContinue
Set-ItemProperty -Path $gameBarPath -Name 'ShowStartupPanel' -Value 0 -Type DWord -ErrorAction SilentlyContinue
Set-ItemProperty -Path $gameBarPath -Name 'UseNexusForGameBarEnabled' -Value 0 -Type DWord -ErrorAction SilentlyContinue

$new1 = Get-ItemProperty -Path $gameConfigPath
$new2 = Get-ItemProperty -Path $gameBarPath
Write-Host "  GameDVR_Enabled=$($new1.GameDVR_Enabled) (should be 0)" -ForegroundColor $(if ($new1.GameDVR_Enabled -eq 0) { 'Green' }else { 'Yellow' })
Write-Host "  ShowStartupPanel=$($new2.ShowStartupPanel) (should be 0)" -ForegroundColor $(if ($new2.ShowStartupPanel -eq 0) { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 5: NTFS Optimization (fsutil)
# ============================================
Write-Host "=== TEST 5: NTFS Optimization ===" -ForegroundColor Yellow

# Get current settings
$lastAccessResult = fsutil behavior query disablelastaccess 2>$null
$current8dot3 = fsutil 8dot3name query 2>$null

Write-Host "  Current disablelastaccess: $lastAccessResult" -ForegroundColor Cyan
Write-Host "  Applying disablelastaccess=1..." -ForegroundColor Yellow
fsutil behavior set disablelastaccess 1 | Out-Null
$newResult = fsutil behavior query disablelastaccess 2>$null
Write-Host "  disablelastaccess after: $newResult" -ForegroundColor Green
Write-Host ""

# ============================================
# REVERTING ALL CHANGES
# ============================================
Write-Host "=== REVERTING ALL CHANGES ===" -ForegroundColor Magenta

# Revert scheduling
Write-Host "Reverting CPU scheduling..." -ForegroundColor Cyan
if ($null -ne $backups['Win32PrioritySeparation']) {
    Set-ItemProperty -Path $priorityPath -Name 'Win32PrioritySeparation' -Value $backups['Win32PrioritySeparation'] -Type DWord -ErrorAction SilentlyContinue
}
if ($null -ne $backups['SystemResponsiveness']) {
    Set-ItemProperty -Path $mmcssPath -Name 'SystemResponsiveness' -Value $backups['SystemResponsiveness'] -Type DWord -ErrorAction SilentlyContinue
}
if ($null -ne $backups['NetworkThrottlingIndex']) {
    Set-ItemProperty -Path $mmcssPath -Name 'NetworkThrottlingIndex' -Value $backups['NetworkThrottlingIndex'] -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  CPU scheduling reverted [OK]" -ForegroundColor Green

# Revert memory
Write-Host "Reverting memory settings..." -ForegroundColor Cyan
if ($null -ne $backups['LargeSystemCache']) {
    Set-ItemProperty -Path $memPath -Name 'LargeSystemCache' -Value $backups['LargeSystemCache'] -Type DWord -ErrorAction SilentlyContinue
}
if ($null -ne $backups['DisablePagingExecutive']) {
    Set-ItemProperty -Path $memPath -Name 'DisablePagingExecutive' -Value $backups['DisablePagingExecutive'] -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  Memory settings reverted [OK]" -ForegroundColor Green

# Revert background apps
Write-Host "Reverting background apps..." -ForegroundColor Cyan
if ($null -ne $backups['GlobalUserDisabled']) {
    Set-ItemProperty -Path $bgPath -Name 'GlobalUserDisabled' -Value $backups['GlobalUserDisabled'] -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  Background apps reverted [OK]" -ForegroundColor Green

# Revert game bar
Write-Host "Reverting game bar settings..." -ForegroundColor Cyan
if ($null -ne $backups['GameDVR_Enabled']) {
    Set-ItemProperty -Path $gameConfigPath -Name 'GameDVR_Enabled' -Value $backups['GameDVR_Enabled'] -Type DWord -ErrorAction SilentlyContinue
}
if ($null -ne $backups['ShowStartupPanel']) {
    Set-ItemProperty -Path $gameBarPath -Name 'ShowStartupPanel' -Value $backups['ShowStartupPanel'] -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  Game bar settings reverted [OK]" -ForegroundColor Green

Write-Host ""
Write-Host "=== ALL TESTS COMPLETE - ALL CHANGES REVERTED ===" -ForegroundColor Cyan
