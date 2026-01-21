use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns GPU MSI Mode tweaks
pub fn get_gpu_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // GPU MSI - High Priority (NVIDIA/AMD/Intel)
        // ============================================
        Tweak {
            id: "gpu_msi_high".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Enable MSI Mode on GPU (High Priority)".to_string(),
            description: "Enables Message Signaled Interrupts (MSI) mode with High Priority for GPU. Reduces latency and improves performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
    $_.InstanceId -match 'VEN_10DE|VEN_1002|VEN_8086'
}
if (-not $devices -or $devices.Count -eq 0) { 
    Write-Output "False"
    exit 
}
$allEnabled = $true
foreach ($dev in $devices) {
    $path = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    if (-not (Test-Path $path)) { 
        $allEnabled = $false
        break 
    }
    $msi = Get-ItemProperty -Path $path -EA SilentlyContinue
    if (-not $msi -or $msi.MSISupported -ne 1) { 
        $allEnabled = $false
        break 
    }
}
if ($allEnabled) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting GPU MSI Mode..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
    $_.InstanceId -match 'VEN_10DE|VEN_1002|VEN_8086'
}
$count = 0
foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    if (Test-Path $msiPath) {
        Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
        Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
        Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
        Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
        $count++
    }
}
if ($count -eq 0) {
    Write-Host "No GPU MSI settings found to revert" -ForegroundColor Yellow
} else {
    Write-Host "GPU MSI Mode reverted on $count device(s)!" -ForegroundColor Green
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode on GPU (High Priority)..." -ForegroundColor Yellow
$devices = Get-PnpDevice -Class Display -Status OK -EA SilentlyContinue | Where-Object {
    $_.InstanceId -match 'VEN_10DE|VEN_1002|VEN_8086'
}
if (-not $devices -or $devices.Count -eq 0) { 
    Write-Host "No supported GPU found (NVIDIA/AMD/Intel)" -ForegroundColor Red
    Write-Host "Listing all display devices:" -ForegroundColor Yellow
    Get-PnpDevice -Class Display -Status OK | ForEach-Object { Write-Host "  $($_.FriendlyName) - $($_.InstanceId)" }
    exit 1 
}
$count = 0
foreach ($dev in $devices) {
    $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
    $msiPath = "$basePath\MessageSignaledInterruptProperties"
    
    if (-not (Test-Path $basePath)) { 
        New-Item -Path $basePath -Force | Out-Null 
    }
    if (-not (Test-Path $msiPath)) { 
        New-Item -Path $msiPath -Force | Out-Null 
    }
    
    Set-ItemProperty -Path $msiPath -Name 'MSISupported' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'Priority' -Value 3 -Type DWord -Force
    
    Write-Host "  Enabled: $($dev.FriendlyName)" -ForegroundColor Green
    $count++
}
Write-Host "MSI enabled on $count GPU(s) with High Priority!" -ForegroundColor Green
Write-Host "Restart required for changes to take effect." -ForegroundColor Yellow
"#.to_string(),
                }
            ]
        },
    ]
}
