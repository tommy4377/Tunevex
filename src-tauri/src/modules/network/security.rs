use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_security_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_disable_llmnr".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable LLMNR Protocol".to_string(),
            description: "Disable Link-Local Multicast Name Resolution (LLMNR).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows NT\\DNSClient".to_string(),
                    key: "EnableMulticast".to_string(), // LLMNR
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows NT\\DNSClient".to_string(),
                key: "EnableMulticast".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows NT\\DNSClient".to_string(),
                key: "EnableMulticast".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        Tweak {
            id: "net_restrict_anonymous_access".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Restrict Anonymous Access".to_string(),
            description: "Restricts anonymous access to named pipes and shares.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanManServer\\Parameters".to_string(),
                    key: "RestrictNullSessAccess".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\LanManServer\\Parameters".to_string(),
                key: "RestrictNullSessAccess".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\LanManServer\\Parameters".to_string(),
                key: "RestrictNullSessAccess".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "net_restrict_anonymous_enum".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Restrict Anonymous Enumeration".to_string(),
            description: "Restricts anonymous enumeration of shares and SAM accounts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                    key: "RestrictAnonymous".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                key: "RestrictAnonymous".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Lsa".to_string(),
                key: "RestrictAnonymous".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "net_disable_smb_throttling".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable SMB Bandwidth Throttling".to_string(),
            description: "Disables SMB bandwidth throttling.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters"
                        .to_string(),
                    key: "DisableBandwidthThrottling".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters"
                    .to_string(),
                key: "DisableBandwidthThrottling".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters"
                    .to_string(),
                key: "DisableBandwidthThrottling".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "net_disable_netbios".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable NetBIOS over TCP/IP".to_string(),
            description: "Disables NetBIOS on all adapters. Reduces legacy broadcast noise. Avoid on LANs using NetBIOS names.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$adapters = Get-CimInstance -ClassName Win32_NetworkAdapterConfiguration | Where-Object { $_.TcpipNetbiosOptions -ne $null }
$allDisabled = $true
foreach ($a in $adapters) {
    if ($a.TcpipNetbiosOptions -ne 2) { $allDisabled = $false; break }
}
$allDisabled
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-CimInstance -ClassName 'Win32_NetworkAdapterConfiguration' | Where-Object { $_.TcpipNetbiosOptions -ne $null } | Invoke-CimMethod -MethodName 'SetTcpipNetbios' -Arguments @{ 'TcpipNetbiosOptions' = [UInt32]0 }
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-CimInstance -ClassName 'Win32_NetworkAdapterConfiguration' | Where-Object { $_.TcpipNetbiosOptions -ne $null } | Invoke-CimMethod -MethodName 'SetTcpipNetbios' -Arguments @{ 'TcpipNetbiosOptions' = [UInt32]2 }
"#.to_string(),
                }
            ]
        },
    ]
}
