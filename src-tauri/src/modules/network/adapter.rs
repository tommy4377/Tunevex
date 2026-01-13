use crate::modules::types::{TweakType, RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

pub fn get_adapter_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_disable_flow_control".to_string(),
            category: TweakCategory::Network,
            name: "Disable Flow Control".to_string(),
            description: "Prevents network adapter from pausing transmission. Reduces latency jitter.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $false
foreach ($a in $adapters) {
    $val = Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName '*FlowControl' -ErrorAction SilentlyContinue
    if ($val.DisplayValue -ne 'Disabled') { $enabled = $false; break } else { $enabled = $true }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*FlowControl' -DisplayValue 'Rx & Tx Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Flow Control' -DisplayValue 'Rx & Tx Enabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*FlowControl' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Flow Control' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_lso".to_string(),
            category: TweakCategory::Network,
            name: "Disable Large Send Offload".to_string(),
            description: "Disables hardware packet segmentation. Can fix stuttering.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    $v4 = Get-NetAdapterLso -Name $a.Name -ErrorAction SilentlyContinue
    if ($v4.IPv4Enabled -eq $true -or $v4.IPv6Enabled -eq $true) { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*LsoV2IPv4' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*LsoV2IPv6' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Large Send Offload V2 (IPv4)' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Large Send Offload V2 (IPv6)' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*LsoV2IPv4' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*LsoV2IPv6' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Large Send Offload V2 (IPv4)' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Large Send Offload V2 (IPv6)' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_checksum_offload".to_string(),
            category: TweakCategory::Network,
            name: "Disable Checksum Offload".to_string(),
            description: "Forces CPU to handle checksums. Can reduce micro-stuttering.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    if ((Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName '*ChecksumOffload*' -ErrorAction SilentlyContinue).DisplayValue -match 'Enabled') { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*IPChecksumOffloadIPv4' -DisplayValue 'Rx & Tx Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*TCPChecksumOffloadIPv4' -DisplayValue 'Rx & Tx Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*UDPChecksumOffloadIPv4' -DisplayValue 'Rx & Tx Enabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*IPChecksumOffloadIPv4' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*TCPChecksumOffloadIPv4' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*UDPChecksumOffloadIPv4' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_configure_rss".to_string(),
            category: TweakCategory::Network,
            name: "Configure RSS for Gaming".to_string(),
            description: "Sets RSS profile to ClosestProcessor and enables RSS.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterRss -Name $_.Name -Profile NUMAStatic -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    $rss = Get-NetAdapterRss -Name $a.Name -ErrorAction SilentlyContinue
    if ($rss.Enabled -eq $false -or $rss.Profile -notmatch 'Closest') { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Enable-NetAdapterRss -Name $_.Name -ErrorAction SilentlyContinue
    Set-NetAdapterRss -Name $_.Name -Profile Closest -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_rsc".to_string(),
            category: TweakCategory::Network,
            name: "Disable Receive Segment Coalescing (RSC)".to_string(),
            description: "Disables RSC on all adapters.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    if ((Get-NetAdapterRsc -Name $a.Name -ErrorAction SilentlyContinue).IPv4Enabled -eq $true) { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Enable-NetAdapterRsc -Name $_.Name -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Disable-NetAdapterRsc -Name $_.Name -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_jumbo_packet".to_string(),
            category: TweakCategory::Network,
            name: "Disable Jumbo Packet".to_string(),
            description: "Ensures Jumbo Packet is disabled (1514 bytes).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Jumbo Packet' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Jumbo Packet' -DisplayValue '1514' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    $val = Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName 'Jumbo Packet' -ErrorAction SilentlyContinue
    if ($val.DisplayValue -ne 'Disabled' -and $val.DisplayValue -ne '1514') { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }), 
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Jumbo Packet' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Jumbo Packet' -DisplayValue '1514' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        
         Tweak {
            id: "net_disable_chimney".to_string(),
            category: TweakCategory::Network,
            name: "Disable Chimney Offload".to_string(),
            description: "Disables TCP Chimney Offload.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -Chimney Enabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-NetOffloadGlobalSetting).Chimney -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -Chimney Disabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_task_offload".to_string(),
            category: TweakCategory::Network,
            name: "Disable Task Offload".to_string(),
            description: "Forces CPU to handle network processing.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-NetOffloadGlobalSetting).TaskOffload -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -TaskOffload Enabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -TaskOffload Disabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_packet_coalescing".to_string(),
            category: TweakCategory::Network,
            name: "Disable Packet Coalescing Filter".to_string(),
            description: "Prevents grouping of packets for reduced latency.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-NetOffloadGlobalSetting).PacketCoalescingFilter -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -PacketCoalescingFilter Enabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: "Set-NetOffloadGlobalSetting -PacketCoalescingFilter Disabled -ErrorAction SilentlyContinue".to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_power_mgmt".to_string(),
            category: TweakCategory::Network,
            name: "Disable Network Adapter Power Management".to_string(),
            description: "Prevents network adapter from entering low-power states.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    if ((Get-NetAdapterPowerManagement -Name $a.Name).WakeOnMagicPacket -eq 'Enabled') { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapterPowerManagement | ForEach-Object {
    $_ | Set-NetAdapterPowerManagement -WakeOnMagicPacket Enabled -WakeOnPattern Enabled -ErrorAction SilentlyContinue
}
Get-NetAdapter | ForEach-Object {
    $pnpDeviceId = (Get-CimInstance Win32_NetworkAdapter | Where-Object { $_.NetConnectionID -eq $_.Name }).PNPDeviceID
    if ($pnpDeviceId) {
        $instancePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$pnpDeviceId\Device Parameters"
        Set-ItemProperty -Path $instancePath -Name 'PnPCapabilities' -Value 0 -ErrorAction SilentlyContinue
    }
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapterPowerManagement | ForEach-Object {
    $_ | Set-NetAdapterPowerManagement -WakeOnMagicPacket Disabled -WakeOnPattern Disabled -ErrorAction SilentlyContinue
}
Get-NetAdapter | ForEach-Object {
    $pnpDeviceId = (Get-CimInstance Win32_NetworkAdapter | Where-Object { $_.NetConnectionID -eq $_.Name }).PNPDeviceID
    if ($pnpDeviceId) {
        $instancePath = "HKLM:\SYSTEM\CurrentControlSet\Enum\$pnpDeviceId\Device Parameters"
        Set-ItemProperty -Path $instancePath -Name 'PnPCapabilities' -Value 24 -ErrorAction SilentlyContinue
    }
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_ipv6".to_string(),
            category: TweakCategory::Network,
            name: "Disable IPv6".to_string(),
            description: "Disables IPv6 to reduce network overhead if not used.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                    key: "DisabledComponents".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                expected_value: RegistryValue::DWord(0xFFFFFFFF),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                    key: "DisabledComponents".to_string(),
                    value: RegistryValue::DWord(0xFFFFFFFF),
                }
            ]
        },
        Tweak {
            id: "net_optimize_speed".to_string(),
            category: TweakCategory::Network,
            name: "Optimize Link Speed (Auto-Negotiate)".to_string(),
            description: "Ensures network adapter negotiates the highest possible speed (e.g. 1Gbps/10Gbps).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*SpeedDuplex' -DisplayValue '0' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Speed & Duplex' -DisplayValue 'Auto Negotiation' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    $val = Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName '*SpeedDuplex' -ErrorAction SilentlyContinue
    if ($val.DisplayValue -notmatch 'Auto') { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*SpeedDuplex' -DisplayValue '0' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Speed & Duplex' -DisplayValue 'Auto Negotiation' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_disable_eee".to_string(),
            category: TweakCategory::Network,
            name: "Disable Energy Efficient Ethernet (EEE)".to_string(),
            description: "Disables EEE on all active adapters to avoid power-saving induced latency spikes.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*EEE*' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Energy Efficient Ethernet' -DisplayValue 'Enabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-NetAdapter
$enabled = $true
foreach ($a in $adapters) {
    $val = Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName '*EEE*' -ErrorAction SilentlyContinue
    if ($val -eq $null) {
         $val = Get-NetAdapterAdvancedProperty -Name $a.Name -DisplayName 'Energy Efficient Ethernet' -ErrorAction SilentlyContinue
    }
    if ($val.DisplayValue -ne 'Disabled' -and $val -ne $null) { $enabled = $false; break }
}
$enabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-NetAdapter | ForEach-Object {
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName '*EEE*' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
    Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Energy Efficient Ethernet' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_qos_limit_reservable_bandwidth".to_string(),
            category: TweakCategory::Network,
            name: "Limit Reservable Bandwidth to 0%".to_string(),
            description: "Sets NonBestEffortLimit=0 so QoS does not reserve bandwidth.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                    key: "NonBestEffortLimit".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                key: "NonBestEffortLimit".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                    key: "NonBestEffortLimit".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]
        },
    ]
}
