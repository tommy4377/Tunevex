use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns Network Adapter MSI tweaks
pub fn get_network_msi_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NIC MSI - High Priority (Realtek/Intel)
        // ============================================
        Tweak {
            id: "net_msi_nic_high".to_string(), // Renamed
            category: TweakCategory::Network,
            name: "Enable MSI Mode on NIC (High Priority)".to_string(),
            description: "Enables MSI with Priority 3 on Realtek/Intel network adapters. Reduces network latency.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
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
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
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
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
}

if (-not $devices) { Write-Host "No supported PCI NIC found" -ForegroundColor Red; exit 1 }
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
            name: "Enable MSI Mode on NIC (Normal Priority)".to_string(),
            description: "Enables MSI with Priority 1 on network adapters. Safer option for compatibility.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
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
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
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
$devices = @()
# Method 1: Get-NetAdapter
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    foreach ($a in $adapters) {
        $dev = Get-PnpDevice -InstanceId $a.PnPDeviceID -ErrorAction SilentlyContinue
        if ($dev -and $dev.InstanceId -match "PCI") { $devices += $dev }
    }
}
# Method 2: Fallback WMI
if ($devices.Count -eq 0) {
    $wmi = Get-WmiObject Win32_NetworkAdapter -ErrorAction SilentlyContinue | Where-Object { $_.PhysicalAdapter -eq $true -and $_.PNPDeviceID -match "PCI" }
    if ($wmi) {
        foreach ($w in $wmi) {
            $dev = Get-PnpDevice -InstanceId $w.PNPDeviceID -ErrorAction SilentlyContinue
            if ($dev) { $devices += $dev }
        }
    }
}

if (-not $devices) { Write-Host "No supported PCI NIC found" -ForegroundColor Red; exit 1 }
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

        // ============================================
        // B.16: Additional Vendor Support (Qualcomm/Broadcom/Marvell/Killer)
        // ============================================
        Tweak {
            id: "net_msi_additional_vendors".to_string(),
            category: TweakCategory::Network,
            name: "Enable MSI on Additional NICs".to_string(),
            description: "Enables MSI mode for Qualcomm, Broadcom, Marvell, Killer, and MediaTek network adapters with Priority 2.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$vendors = @("VEN_168C", "VEN_14E4", "VEN_11AB", "VEN_1969", "VEN_14C3")
$devices = Get-PnpDevice -Class Net -ErrorAction SilentlyContinue | Where-Object { 
    $id = $_.InstanceId; ($vendors | Where-Object { $id -match $_ }) 
}
if ($devices) {
    $allEnabled = $true
    foreach ($dev in $devices) {
        $val = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties" -Name "MSISupported" -ErrorAction SilentlyContinue
        if (!$val -or $val.MSISupported -ne 1) { $allEnabled = $false }
    }
    if ($allEnabled) { "True" } else { "False" }
} else {
    "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting additional NIC MSI settings..." -ForegroundColor Yellow
$vendorIds = @("VEN_168C", "VEN_14E4", "VEN_11AB", "VEN_1969", "VEN_14C3")

$devices = Get-PnpDevice -Class Net -ErrorAction SilentlyContinue | Where-Object {
    $_.InstanceId -match "PCI" -and ($vendorIds | Where-Object { $_.InstanceId -match $_ })
}

foreach ($dev in $devices) {
    $msiPath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
    Remove-ItemProperty -Path $msiPath -Name 'MSISupported' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -EA SilentlyContinue
    Remove-ItemProperty -Path $msiPath -Name 'Priority' -EA SilentlyContinue
    Write-Host "  Reverted: $($dev.FriendlyName)" -ForegroundColor Green
}
Write-Host "Additional NIC MSI reverted" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling MSI on additional network adapters..." -ForegroundColor Yellow

# Vendor IDs: Qualcomm Atheros, Broadcom, Marvell, Killer, MediaTek
$vendorIds = @("VEN_168C", "VEN_14E4", "VEN_11AB", "VEN_1969", "VEN_14C3")

$devices = Get-PnpDevice -Class Net -ErrorAction SilentlyContinue | Where-Object {
    $instanceId = $_.InstanceId
    $_.Status -eq 'OK' -and $instanceId -match "PCI" -and ($vendorIds | ForEach-Object { $instanceId -match $_ }) -contains $true
}

if (-not $devices) {
    Write-Host "No additional vendor NICs found (Qualcomm/Broadcom/Marvell/Killer/MediaTek)" -ForegroundColor Yellow
    Write-Host "This is normal if you have Intel/Realtek NICs" -ForegroundColor Cyan
    exit 0
}

$count = 0
foreach ($dev in $devices) {
    $basePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($dev.InstanceId)\Device Parameters\Interrupt Management"
    $msiPath = "$basePath\MessageSignaledInterruptProperties"
    
    if (-not (Test-Path $basePath)) { New-Item -Path $basePath -Force | Out-Null }
    if (-not (Test-Path $msiPath)) { New-Item -Path $msiPath -Force | Out-Null }
    
    Set-ItemProperty -Path $msiPath -Name 'MSISupported' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'MessageNumberLimit' -Value 1 -Type DWord -Force
    Set-ItemProperty -Path $msiPath -Name 'Priority' -Value 2 -Type DWord -Force  # Priority 2 for balanced approach
    
    Write-Host "  Enabled: $($dev.FriendlyName)" -ForegroundColor Green
    $count++
}
Write-Host "MSI enabled on $count additional NIC(s)" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
