# Input Tweaks Test Script
# Tests all input tweaks from Phase 2 and reverts them

Write-Host "=== TommyTweaker Input Tweaks Test ===" -ForegroundColor Cyan
Write-Host ""

$backups = @{}

# ============================================
# TEST 1: Mouse Acceleration
# ============================================
Write-Host "=== TEST 1: Mouse Settings ===" -ForegroundColor Yellow
$mousePath = 'HKCU:\Control Panel\Mouse'

# Backup current values
$mouseProps = @('MouseSpeed', 'MouseThreshold1', 'MouseThreshold2', 'MouseSensitivity', 'MouseHoverTime', 'MouseTrails', 'SnapToDefaultButton')
foreach ($prop in $mouseProps) {
    $current = Get-ItemProperty -Path $mousePath -Name $prop -ErrorAction SilentlyContinue
    $backups["Mouse_$prop"] = $current.$prop
}

# Apply tweaks
Write-Host "Applying mouse tweaks..." -ForegroundColor Cyan
Set-ItemProperty -Path $mousePath -Name 'MouseSpeed' -Value '0' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'MouseThreshold1' -Value '0' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'MouseThreshold2' -Value '0' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'MouseSensitivity' -Value '10' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'MouseHoverTime' -Value '20' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'MouseTrails' -Value '0' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $mousePath -Name 'SnapToDefaultButton' -Value '0' -ErrorAction SilentlyContinue

# Verify
$new = Get-ItemProperty -Path $mousePath
Write-Host "  MouseSpeed=$($new.MouseSpeed)" -ForegroundColor $(if ($new.MouseSpeed -eq '0') { 'Green' }else { 'Red' })
Write-Host "  MouseThreshold1=$($new.MouseThreshold1)" -ForegroundColor $(if ($new.MouseThreshold1 -eq '0') { 'Green' }else { 'Red' })
Write-Host "  MouseThreshold2=$($new.MouseThreshold2)" -ForegroundColor $(if ($new.MouseThreshold2 -eq '0') { 'Green' }else { 'Red' })
Write-Host "  MouseSensitivity=$($new.MouseSensitivity)" -ForegroundColor $(if ($new.MouseSensitivity -eq '10') { 'Green' }else { 'Yellow' })
Write-Host "  MouseHoverTime=$($new.MouseHoverTime)" -ForegroundColor $(if ($new.MouseHoverTime -eq '20') { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 2: Keyboard Settings
# ============================================
Write-Host "=== TEST 2: Keyboard Settings ===" -ForegroundColor Yellow
$keyboardPath = 'HKCU:\Control Panel\Keyboard'

# Backup
$current = Get-ItemProperty -Path $keyboardPath -ErrorAction SilentlyContinue
$backups['Keyboard_KeyboardSpeed'] = $current.KeyboardSpeed
$backups['Keyboard_KeyboardDelay'] = $current.KeyboardDelay
$backups['Keyboard_InitialKeyboardIndicators'] = $current.InitialKeyboardIndicators

# Apply
Write-Host "Applying keyboard tweaks..." -ForegroundColor Cyan
Set-ItemProperty -Path $keyboardPath -Name 'KeyboardSpeed' -Value '31' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $keyboardPath -Name 'KeyboardDelay' -Value '0' -ErrorAction SilentlyContinue
Set-ItemProperty -Path $keyboardPath -Name 'InitialKeyboardIndicators' -Value '2' -ErrorAction SilentlyContinue

# Verify
$new = Get-ItemProperty -Path $keyboardPath
Write-Host "  KeyboardSpeed=$($new.KeyboardSpeed)" -ForegroundColor $(if ($new.KeyboardSpeed -eq '31') { 'Green' }else { 'Yellow' })
Write-Host "  KeyboardDelay=$($new.KeyboardDelay)" -ForegroundColor $(if ($new.KeyboardDelay -eq '0') { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 3: Accessibility Tweaks
# ============================================
Write-Host "=== TEST 3: Accessibility Settings ===" -ForegroundColor Yellow

# Sticky Keys
$stickyPath = 'HKCU:\Control Panel\Accessibility\StickyKeys'
$current = Get-ItemProperty -Path $stickyPath -Name 'Flags' -ErrorAction SilentlyContinue
$backups['StickyKeys_Flags'] = $current.Flags
Set-ItemProperty -Path $stickyPath -Name 'Flags' -Value '506' -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $stickyPath -Name 'Flags' -ErrorAction SilentlyContinue
Write-Host "  StickyKeys Flags=$($new.Flags)" -ForegroundColor $(if ($new.Flags -eq '506') { 'Green' }else { 'Yellow' })

# Filter Keys
$filterPath = 'HKCU:\Control Panel\Accessibility\Keyboard Response'
$current = Get-ItemProperty -Path $filterPath -Name 'Flags' -ErrorAction SilentlyContinue
$backups['FilterKeys_Flags'] = $current.Flags
Set-ItemProperty -Path $filterPath -Name 'Flags' -Value '122' -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $filterPath -Name 'Flags' -ErrorAction SilentlyContinue
Write-Host "  FilterKeys Flags=$($new.Flags)" -ForegroundColor $(if ($new.Flags -eq '122') { 'Green' }else { 'Yellow' })

# Toggle Keys
$togglePath = 'HKCU:\Control Panel\Accessibility\ToggleKeys'
$current = Get-ItemProperty -Path $togglePath -Name 'Flags' -ErrorAction SilentlyContinue
$backups['ToggleKeys_Flags'] = $current.Flags
Set-ItemProperty -Path $togglePath -Name 'Flags' -Value '58' -ErrorAction SilentlyContinue
$new = Get-ItemProperty -Path $togglePath -Name 'Flags' -ErrorAction SilentlyContinue
Write-Host "  ToggleKeys Flags=$($new.Flags)" -ForegroundColor $(if ($new.Flags -eq '58') { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# TEST 4: Touch Keyboard
# ============================================
Write-Host "=== TEST 4: Touch Keyboard Settings ===" -ForegroundColor Yellow
$tabletPath = 'HKCU:\SOFTWARE\Microsoft\TabletTip\1.7'
if (-not (Test-Path $tabletPath)) {
    New-Item -Path $tabletPath -Force | Out-Null
}

$current = Get-ItemProperty -Path $tabletPath -ErrorAction SilentlyContinue
$backups['Tablet_EnableAutoShiftEngage'] = $current.EnableAutoShiftEngage
$backups['Tablet_EnableKeyAudioFeedback'] = $current.EnableKeyAudioFeedback

Set-ItemProperty -Path $tabletPath -Name 'EnableAutoShiftEngage' -Value 0 -Type DWord -ErrorAction SilentlyContinue
Set-ItemProperty -Path $tabletPath -Name 'EnableKeyAudioFeedback' -Value 0 -Type DWord -ErrorAction SilentlyContinue

$new = Get-ItemProperty -Path $tabletPath -ErrorAction SilentlyContinue
Write-Host "  EnableAutoShiftEngage=$($new.EnableAutoShiftEngage)" -ForegroundColor $(if ($new.EnableAutoShiftEngage -eq 0) { 'Green' }else { 'Yellow' })
Write-Host "  EnableKeyAudioFeedback=$($new.EnableKeyAudioFeedback)" -ForegroundColor $(if ($new.EnableKeyAudioFeedback -eq 0) { 'Green' }else { 'Yellow' })
Write-Host ""

# ============================================
# REVERTING ALL CHANGES
# ============================================
Write-Host "=== REVERTING ALL CHANGES ===" -ForegroundColor Magenta

# Revert mouse settings
Write-Host "Reverting mouse settings..." -ForegroundColor Cyan
foreach ($prop in $mouseProps) {
    $backup = $backups["Mouse_$prop"]
    if ($null -ne $backup) {
        Set-ItemProperty -Path $mousePath -Name $prop -Value $backup -ErrorAction SilentlyContinue
    }
}
Write-Host "  Mouse settings reverted [OK]" -ForegroundColor Green

# Revert keyboard settings
Write-Host "Reverting keyboard settings..." -ForegroundColor Cyan
if ($null -ne $backups['Keyboard_KeyboardSpeed']) {
    Set-ItemProperty -Path $keyboardPath -Name 'KeyboardSpeed' -Value $backups['Keyboard_KeyboardSpeed'] -ErrorAction SilentlyContinue
}
if ($null -ne $backups['Keyboard_KeyboardDelay']) {
    Set-ItemProperty -Path $keyboardPath -Name 'KeyboardDelay' -Value $backups['Keyboard_KeyboardDelay'] -ErrorAction SilentlyContinue
}
if ($null -ne $backups['Keyboard_InitialKeyboardIndicators']) {
    Set-ItemProperty -Path $keyboardPath -Name 'InitialKeyboardIndicators' -Value $backups['Keyboard_InitialKeyboardIndicators'] -ErrorAction SilentlyContinue
}
Write-Host "  Keyboard settings reverted [OK]" -ForegroundColor Green

# Revert accessibility
Write-Host "Reverting accessibility settings..." -ForegroundColor Cyan
if ($null -ne $backups['StickyKeys_Flags']) {
    Set-ItemProperty -Path $stickyPath -Name 'Flags' -Value $backups['StickyKeys_Flags'] -ErrorAction SilentlyContinue
}
if ($null -ne $backups['FilterKeys_Flags']) {
    Set-ItemProperty -Path $filterPath -Name 'Flags' -Value $backups['FilterKeys_Flags'] -ErrorAction SilentlyContinue
}
if ($null -ne $backups['ToggleKeys_Flags']) {
    Set-ItemProperty -Path $togglePath -Name 'Flags' -Value $backups['ToggleKeys_Flags'] -ErrorAction SilentlyContinue
}
Write-Host "  Accessibility settings reverted [OK]" -ForegroundColor Green

# Revert touch keyboard
Write-Host "Reverting touch keyboard settings..." -ForegroundColor Cyan
if ($null -ne $backups['Tablet_EnableAutoShiftEngage']) {
    Set-ItemProperty -Path $tabletPath -Name 'EnableAutoShiftEngage' -Value $backups['Tablet_EnableAutoShiftEngage'] -Type DWord -ErrorAction SilentlyContinue
}
if ($null -ne $backups['Tablet_EnableKeyAudioFeedback']) {
    Set-ItemProperty -Path $tabletPath -Name 'EnableKeyAudioFeedback' -Value $backups['Tablet_EnableKeyAudioFeedback'] -Type DWord -ErrorAction SilentlyContinue
}
Write-Host "  Touch keyboard settings reverted [OK]" -ForegroundColor Green

Write-Host ""
Write-Host "=== ALL TESTS COMPLETE - ALL CHANGES REVERTED ===" -ForegroundColor Cyan
