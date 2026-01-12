use crate::modules::types::{Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

/// Returns GPU MSI Mode tweaks
pub fn get_gpu_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // GPU MSI - High Priority (NVIDIA/AMD/Intel)
        // ============================================
        Tweak {
            id: "gpu_msi_high".to_string(), // Renamed from hw_
            category: TweakCategory::GpuOptimization,
            name: "🎨 Enable MSI Mode on GPU (High Priority)".to_string(),
            description: "Enables MSI with Priority 3 on NVIDIA/AMD/Intel GPUs. Reduces DPC latency and micro-stuttering.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$vendors = @('VEN_10DE', 'VEN_1002', 'VEN_8086')  # NVIDIA, AMD, Intel
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
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
Write-Host "Reverting GPU MSI Mode..." -ForegroundColor Yellow
$vendors = @('VEN_10DE', 'VEN_1002', 'VEN_8086')
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
foreach ($dev in $devices) {
    $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
    $msiPath = "$basePath\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    # Clean up empty keys
    $props = Get-ItemProperty -Path $msiPath -EA SilentlyContinue
    if ($props -and (Get-Item $msiPath).Property.Count -eq 0) {
        Remove-Item -Path $msiPath -Force -EA SilentlyContinue
    }
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "GPU MSI Mode reverted!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on GPU (High Priority)..." -ForegroundColor Yellow
$vendors = @('VEN_10DE', 'VEN_1002', 'VEN_8086')  # NVIDIA, AMD, Intel
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
    $vendors | ForEach-Object { $_.InstanceId -match $_ } | Where-Object { $_ }
}
if (-not $devices) { Write-Host "No supported GPU found" -ForegroundColor Red; exit 1 }
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
Write-Host "MSI enabled on $count GPU(s) with High Priority!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
