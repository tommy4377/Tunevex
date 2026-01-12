use crate::modules::types::{Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

/// Returns USB MSI Mode tweaks
pub fn get_usb_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // USB MSI - Normal Priority
        // ============================================
        Tweak {
            id: "input_msi_usb_normal".to_string(), // Renamed
            category: TweakCategory::MouseInput, // Changed category to Input
            name: "🔌 Enable MSI Mode on USB Controllers (Normal)".to_string(),
            description: "Enables MSI with Priority 1 on USB host controllers. Reduces USB latency.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$devices = Get-PnpDevice -Class USB -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'Host Controller|xHCI' }
if (-not $devices) { return 'NoDevice' }
$allEnabled = $true
foreach ($dev in $devices) {
    $path = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    $msi = Get-ItemProperty -Path $path -EA SilentlyContinue
    if (-not $msi -or $msi.MSISupported -ne 1 -or $msi.Priority -ne 1) { $allEnabled = $false; break }
}
$allEnabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting USB MSI Mode..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class USB -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'Host Controller|xHCI' }
foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "USB MSI Mode reverted!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on USB Controllers (Normal Priority)..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class USB -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'Host Controller|xHCI' }
if (-not $devices) { Write-Host "No USB host controllers found" -ForegroundColor Red; exit 1 }
$count = 0
foreach ($dev in $devices) {
    $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
    $msiPath = "$basePath\MessageSignaledInterruptProperties"
    if (-not (Test-Path $basePath)) { New-Item -Path $basePath -Force | Out-Null }
    if (-not (Test-Path $msiPath)) { New-Item -Path $msiPath -Force | Out-Null }
    Set-ItemProperty -Path $msiPath -Name 'MSISupported' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'Priority' -Value 1 -Type DWord -Force
    Write-Host "  Enabled: $($dev.FriendlyName)" -ForegroundColor Green
    $count++
}
Write-Host "MSI enabled on $count USB controller(s)!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
