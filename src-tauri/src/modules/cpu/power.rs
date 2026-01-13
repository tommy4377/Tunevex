//! CPU Power Management Tweaks
//!
//! Based on:
//! - DisablePowerSaving.ps1 from AtlasOS
//! - CPU Idle scripts
//! - Various power optimization sources

use crate::modules::types::{TweakType, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

/// Returns all CPU power management tweaks
pub fn get_power_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Ultimate Performance Power Plan
        // ============================================
        Tweak {
            id: "cpu_ultimate_performance".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Ultimate Performance Power Plan".to_string(),
            description: "Enables Windows' hidden Ultimate Performance power plan for maximum performance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "powercfg -setactive 381b4222-f694-41f0-9685-ff5bb260df2e".to_string(), // Restore Balanced
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$scheme = powercfg /getactivescheme
if ($scheme -match "e9a42b02-d5df-448d-aa00-03f14749eb61") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Duplicate Ultimate Performance power plan
$guid = powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null
if ($LASTEXITCODE -eq 0) {
    # Extract GUID and set as active
    $newGuid = ($guid -split ' ')[-1]
    powercfg -setactive $newGuid
    Write-Host "Ultimate Performance power plan enabled"
} else {
    # If already exists, just activate it
    powercfg -setactive e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null
}
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Disable USB Selective Suspend
        // ============================================
        Tweak {
            id: "cpu_disable_usb_suspend".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable USB Selective Suspend".to_string(),
            description: "Prevents USB devices from powering down. Fixes mouse/keyboard lag issues.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
powercfg -setacvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 1
powercfg -setdcvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 1
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = powercfg /q SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226
if ($res -match "Current AC Power Setting Index: 0x00000000") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Disable USB selective suspend for all power schemes
powercfg -setacvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg -setdcvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Disable PCIe Link State Power Management
        // ============================================
        Tweak {
            id: "cpu_disable_pcie_lpm".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable PCIe Link State Power Management".to_string(),
            description: "Prevents PCIe devices (GPU, NVMe) from entering low-power states.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
powercfg -setacvalueindex SCHEME_CURRENT 501a4d13-42af-4429-9fd1-a8218c268e20 ee12f906-d277-404b-b6da-e5fa1a576df5 2
powercfg -setdcvalueindex SCHEME_CURRENT 501a4d13-42af-4429-9fd1-a8218c268e20 ee12f906-d277-404b-b6da-e5fa1a576df5 2
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = powercfg /q SCHEME_CURRENT 501a4d13-42af-4429-9fd1-a8218c268e20 ee12f906-d277-404b-b6da-e5fa1a576df5
if ($res -match "Current AC Power Setting Index: 0x00000000") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Disable ASPM (Active State Power Management) for PCIe
powercfg -setacvalueindex SCHEME_CURRENT 501a4d13-42af-4429-9fd1-a8218c268e20 ee12f906-d277-404b-b6da-e5fa1a576df5 0
powercfg -setdcvalueindex SCHEME_CURRENT 501a4d13-42af-4429-9fd1-a8218c268e20 ee12f906-d277-404b-b6da-e5fa1a576df5 0
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Disable CPU Core Parking
        // ============================================
        Tweak {
            id: "cpu_disable_core_parking".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Core Parking".to_string(),
            description: "Prevents Windows from parking CPU cores. Removes latency from unparking cores during gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$processorSettings = '54533251-82be-4824-96c1-47b60b740d00'
$coreParking = '0cc5b647-c1df-4637-891a-dec35c318583'
powercfg -setacvalueindex SCHEME_CURRENT $processorSettings $coreParking 5
powercfg -setdcvalueindex SCHEME_CURRENT $processorSettings $coreParking 5
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = powercfg /q SCHEME_CURRENT 54533251-82be-4824-96c1-47b60b740d00 0cc5b647-c1df-4637-891a-dec35c318583
if ($res -match "Current AC Power Setting Index: 0x00000064") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Unhide Core Parking settings in Power Options and set to 100%
$coreParking = '0cc5b647-c1df-4637-891a-dec35c318583'
$processorSettings = '54533251-82be-4824-96c1-47b60b740d00'

# Make core parking visible in power options
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Power\PowerSettings\$processorSettings\$coreParking" -Name 'Attributes' -Value 0 -ErrorAction SilentlyContinue

# Set min cores to 100% (no parking)
powercfg -setacvalueindex SCHEME_CURRENT $processorSettings $coreParking 100
powercfg -setdcvalueindex SCHEME_CURRENT $processorSettings $coreParking 100
powercfg -setactive SCHEME_CURRENT
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // CPU Idle Disable (from Disable Idle.cmd)
        // ============================================
        Tweak {
            id: "cpu_disable_idle".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Idle States".to_string(),
            description: "Forces CPU to maximum speed always. NOT recommended with HyperThreading/SMT. Ensure good cooling!".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
powercfg /setacvalueindex scheme_current sub_processor 5d76a2ca-e8c0-402f-a133-2158492d58ad 0
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = powercfg /q SCHEME_CURRENT sub_processor 5d76a2ca-e8c0-402f-a133-2158492d58ad
if ($res -match "Current AC Power Setting Index: 0x00000001") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Disable CPU Idle States
powercfg /setacvalueindex scheme_current sub_processor 5d76a2ca-e8c0-402f-a133-2158492d58ad 1
powercfg /setactive scheme_current
Write-Host "CPU Idle disabled - Task Manager will show 100% usage" -ForegroundColor Yellow
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // NEW: Atlas Power Scheme (comprehensive)
        // ============================================
        Tweak {
            id: "cpu_atlas_power_scheme".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Create Ultimate Power Scheme".to_string(),
            description: "Creates custom Ultimate Power Scheme based on ultimate performance with all power-saving disabled.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "powercfg -setactive 381b4222-f694-41f0-9685-ff5bb260df2e".to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$scheme = powercfg /getactivescheme
if ($scheme -match "11111111-1111-1111-1111-111111111111") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Creating Ultimate Power Scheme..." -ForegroundColor Yellow

# Create/activate Ultimate Performance based scheme
if (!(powercfg /l | Select-String "11111111-1111-1111-1111-111111111111" -Quiet)) {
    powercfg /duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 11111111-1111-1111-1111-111111111111 2>$null
}
powercfg /setactive 11111111-1111-1111-1111-111111111111
powercfg /changename scheme_current "Ultimate Power Scheme" "Optimized for latency and performance"

Write-Host "Ultimate Power Scheme created and activated!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // NEW: USB 3 Link Power Management
        // ============================================
        Tweak {
            id: "cpu_usb3_link_power".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable USB 3 Link Power Management".to_string(),
            description: "Sets USB 3 Link Power Management to maximum performance. Prevents USB device latency spikes and disconnections.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$result = powercfg /q scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009
if ($result -match "Current AC Power Setting Index: 0x00000003") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Restore default (Moderate Power Savings = 1)
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 1
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 1
powercfg /setactive scheme_current
Write-Host "USB 3 Link Power restored to moderate" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# USB 3 Link Power Management values:
# 0 = Maximum Power Savings (WRONG for performance!)
# 1 = Moderate Power Savings (default)
# 2 = Minimum Power Savings
# 3 = Maximum Performance (CORRECT!)

# Set USB 3 Link Power to Maximum Performance (3)
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 3
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 3

# Also disable USB Selective Suspend
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0

# USB Hub Selective Suspend Timeout - 0ms
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 0853a681-27c8-4100-a2fd-82013e970683 0
powercfg /setdcvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 0853a681-27c8-4100-a2fd-82013e970683 0

powercfg /setactive scheme_current
Write-Host "USB 3 Link Power set to Maximum Performance" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // NEW: Disable Throttle States
        // ============================================
        Tweak {
            id: "cpu_disable_throttle_states".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable CPU Throttle States".to_string(),
            description: "Disables CPU throttle states (T-states) for consistent performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Allow Throttle States - On (1)
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb 1
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = powercfg /q SCHEME_CURRENT 54533251-82be-4824-96c1-47b60b740d00 3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb
if ($res -match "Current AC Power Setting Index: 0x00000000") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Allow Throttle States - Off
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb 0
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // NEW: Disable ACPI Power-Saving Devices
        // ============================================
        Tweak {
            id: "cpu_disable_acpi_devices".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Power-Saving ACPI Devices".to_string(),
            description: "Disables ACPI Processor Aggregator and other power-saving system devices.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$devices = Get-PnpDevice -Class System | Where-Object { $_.FriendlyName -match "ACPI Processor Aggregator" }
foreach ($device in $devices) {
    Enable-PnpDevice -InstanceId $device.InstanceId -Confirm:$false -EA 0
}
"#.to_string(),
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$dev = Get-PnpDevice -Class System -FriendlyName "*ACPI Processor Aggregator*" -EA 0
if ($dev.Status -ne "OK") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Disable ACPI Processor Aggregator (causes core parking)
$devices = Get-PnpDevice | Where-Object { $_.FriendlyName -match "ACPI Processor Aggregator" }
foreach ($device in $devices) {
    Disable-PnpDevice -InstanceId $device.InstanceId -Confirm:$false -EA 0
}
"#.to_string(),
                }
            ]
        },
        
        // ============================================
        // Comprehensive Power-Saving Disable
        // ============================================
        Tweak {
            id: "cpu_disable_all_power_saving".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable All Power-Saving Features".to_string(),
            description: "Comprehensive power-saving disable: NVMe idle, USB3 link power, throttle states, device D3, EEE.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "powercfg -setactive 381b4222-f694-41f0-9685-ff5bb260df2e".to_string(), // Best effort: restore balanced
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$storage = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Storage" -Name "StorageD3InModernStandby" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty StorageD3InModernStandby
$adapterCheck = $true
$adapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue
if ($adapters) {
    # Check if any adapter has EnableGreenEthernet enabled (value 1). If key missing, assume disabled/safe.
    # We only flag as False if we explicitly find it enabled.
    $green = Get-NetAdapterAdvancedProperty -Name "*" -RegistryKeyword "EnableGreenEthernet" -ErrorAction SilentlyContinue
    if ($green -and ($green | Where-Object { $_.RegistryValue -ne "0" })) {
        $adapterCheck = $false
    }
}

if (($storage -eq 0) -and $adapterCheck) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Configuring Ultimate Power Scheme..." -ForegroundColor Yellow

# Create/activate Ultimate Performance based scheme
if (!(powercfg /l | Select-String "11111111-1111-1111-1111-111111111111" -Quiet)) {
    powercfg /duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 11111111-1111-1111-1111-111111111111 2>$null
}
powercfg /setactive 11111111-1111-1111-1111-111111111111
powercfg /changename scheme_current "Ultimate Power Scheme" "Optimized for latency and performance"

# NVMe Idle Timeout - 0ms
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d3d55efd-c1ff-424e-9dc3-441be7833010 0
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 d639518a-e56d-4345-8af2-b9f32fb26109 0
# NVME NOPPME - Off
powercfg /setacvalueindex scheme_current 0012ee47-9041-4b5d-9b77-535fba8b1442 fc7372b6-ab2d-43ee-8797-15e9841f2cca 0
# USB Hub Selective Suspend Timeout - 0ms
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 0853a681-27c8-4100-a2fd-82013e970683 0
# USB selective suspend - Disabled
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
# USB 3 Link Power Management - Off
powercfg /setacvalueindex scheme_current 2a737441-1930-4402-8d77-b2bebba308a3 d4e98f31-5ffe-4ce1-be31-1b38b384c009 0
# Allow Throttle States - Off
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb 0
# Processor time check interval - 200ms (reduces DPCs)
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 200
powercfg /setactive scheme_current

Write-Host "Disabling network adapter power-saving..." -ForegroundColor Yellow
$props = Get-NetAdapter -Physical | Get-NetAdapterAdvancedProperty -EA 0
foreach ($s in @("ULPMode","EEE","EEELinkAdvertisement","AdvancedEEE","EnableGreenEthernet","EeePhyEnable",
    "uAPSDSupport","EnablePowerManagement","EnableSavePowerNow","bLowPowerEnable","PowerSaveMode",
    "PowerSavingMode","SavePowerNowEnabled","AutoPowerSaveModeEnabled","SelectiveSuspend")) {
    $props | Where-Object { $_.RegistryKeyword -match $s } | Set-NetAdapterAdvancedProperty -RegistryValue 0 -EA 0
}

Write-Host "Disabling storage power-saving..." -ForegroundColor Yellow
# D3 support for NVMe
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Storage" -Name "StorageD3InModernStandby" -Value 0 -Type DWord -Force -EA 0
# NVMe idle power mode
$nvmePath = "HKLM:\SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device"
if (!(Test-Path $nvmePath)) { New-Item -Path $nvmePath -Force | Out-Null }
Set-ItemProperty -Path $nvmePath -Name "IdlePowerMode" -Value 0 -Type DWord -Force -EA 0

Write-Host "Power-saving disabled!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
