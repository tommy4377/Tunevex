use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

/// Returns System MSI tweaks
pub fn get_system_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Global Safe - All Supported Devices (Priority 0)
        // ============================================
        Tweak {
            id: "system_msi_global_safe".to_string(),
            category: TweakCategory::System,
            name: "⚡ Enable MSI Mode Globally (Safe Priority)".to_string(),
            description: "Enables MSI with Priority 0 on all supported PCI devices. Safest option for broad compatibility.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            check: None, // Complex to check all devices
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting Global MSI Mode..." -ForegroundColor Yellow
$classes = @('Display', 'SCSIAdapter', 'Net', 'USB', 'HDC')
$count = 0
foreach ($class in $classes) {
    $devices = Get-PnpDevice -Class $class -Status OK -EA SilentlyContinue
    foreach ($dev in $devices) {
        $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
        if (Test-Path $msiPath) {
            Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
            Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
            Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
            $count++
        }
    }
}
Write-Host "Reverted MSI on $count device(s)!" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI Mode Globally (Safe Priority 0)..." -ForegroundColor Yellow
$classes = @('Display', 'SCSIAdapter', 'Net', 'USB', 'HDC')
$count = 0
foreach ($class in $classes) {
    $devices = Get-PnpDevice -Class $class -Status OK -EA SilentlyContinue
    foreach ($dev in $devices) {
        $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
        $msiPath = "$basePath\MessageSignaledInterruptProperties"
        if (-not (Test-Path $basePath)) { New-Item -Path $basePath -Force | Out-Null }
        if (-not (Test-Path $msiPath)) { New-Item -Path $msiPath -Force | Out-Null }
        Set-ItemProperty -Path $msiPath -Name 'MSISupported' -Value 1 -Type DWord -Force
        Set-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -Value 1 -Type DWord -Force
        Set-ItemProperty -Path $msiPath -Name 'Priority' -Value 0 -Type DWord -Force
        $count++
    }
}
Write-Host "MSI enabled on $count device(s) with Safe Priority!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
