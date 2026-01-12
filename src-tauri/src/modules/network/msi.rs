use crate::modules::types::{Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

/// Returns Network Adapter MSI tweaks
pub fn get_network_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NIC MSI - High Priority (Realtek/Intel)
        // ============================================
        Tweak {
            id: "net_msi_nic_high".to_string(), // Renamed
            category: TweakCategory::Network,
            name: "🌐 Enable MSI Mode on NIC (High Priority)".to_string(),
            description: "Enables MSI with Priority 3 on Realtek/Intel network adapters. Reduces network latency.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$vendors = @('VEN_10EC', 'VEN_8086')  # Realtek, Intel
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
if (-not $devices) { return 'NoDevice' }
$allEnabled = $true
foreach ($dev in $devices) {
    $path = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    $msi = Get-ItemProperty -Path $path -EA SilentlyContinue
    if (-not $msi -or $msi.MSISupported -ne 1 -or $msi.Priority -ne 3) { $allEnabled = $false; break }
}
$allEnabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting NIC MSI Mode..." -ForegroundColor Yellow
$vendors = @('VEN_10EC', 'VEN_8086')
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "NIC MSI Mode reverted!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on NIC (High Priority)..." -ForegroundColor Yellow
$vendors = @('VEN_10EC', 'VEN_8086')  # Realtek, Intel
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
if (-not $devices) { Write-Host "No supported NIC found" -ForegroundColor Red; exit 1 }
$count = 0
foreach ($dev in $devices) {
    $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
    $msiPath = "$basePath\MessageSignaledInterruptProperties"
    if (-not (Test-Path $basePath)) { New-Item -Path $basePath -Force | Out-Null }
    if (-not (Test-Path $msiPath)) { New-Item -Path $msiPath -Force | Out-Null }
    Set-ItemProperty -Path $msiPath -Name 'MSISupported' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'Priority' -Value 3 -Type DWord -Force
    Write-Host "  Enabled: $($dev.FriendlyName)" -ForegroundColor Green
    $count++
}
Write-Host "MSI enabled on $count NIC(s) with High Priority!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },

        // ============================================
        // NIC MSI - Normal Priority (Safe)
        // ============================================
        Tweak {
            id: "net_msi_nic_normal".to_string(), // Renamed
            category: TweakCategory::Network,
            name: "🌐 Enable MSI Mode on NIC (Normal Priority)".to_string(),
            description: "Enables MSI with Priority 1 on network adapters. Safer option for compatibility.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$vendors = @('VEN_10EC', 'VEN_8086')
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
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
Write-Host "Reverting NIC MSI Mode..." -ForegroundColor Yellow
$vendors = @('VEN_10EC', 'VEN_8086')
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "NIC MSI Mode reverted!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on NIC (Normal Priority)..." -ForegroundColor Yellow
$vendors = @('VEN_10EC', 'VEN_8086')
$devices = Get-PnpDevice -Class Net -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
if (-not $devices) { Write-Host "No supported NIC found" -ForegroundColor Red; exit 1 }
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
Write-Host "MSI enabled on $count NIC(s) with Normal Priority!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
