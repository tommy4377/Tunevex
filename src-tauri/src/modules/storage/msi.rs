use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns NVMe MSI Mode tweaks
pub fn get_storage_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NVMe MSI - High Priority
        // ============================================
        Tweak {
            id: "storage_msi_nvme_high".to_string(), // Renamed from hw_
            category: TweakCategory::FileSystem,
            name: "Enable MSI Mode on NVMe (High Priority)".to_string(),
            description: "Enables MSI with Priority 3 on NVMe drives. Reduces storage latency.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$devices = Get-PnpDevice -Class SCSIAdapter -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'NVMe' }
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
Write-Host "Reverting NVMe MSI Mode..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class SCSIAdapter -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'NVMe' }
foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "NVMe MSI Mode reverted!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on NVMe (High Priority)..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class SCSIAdapter -Status OK -EA SilentlyContinue | Where-Object { $_.FriendlyName -match 'NVMe' }
if (-not $devices) { Write-Host "No NVMe drives found" -ForegroundColor Red; exit 1 }
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
Write-Host "MSI enabled on $count NVMe drive(s) with High Priority!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
