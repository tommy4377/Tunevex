use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_tcp_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_tcp_ack_freq".to_string(),
            category: TweakCategory::Network,
            name: "Optimize TCP ACK Frequency".to_string(),
            description: "Sets TcpAckFrequency=1 to reduce ACK delay via registry. Improves responsiveness.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,

            check: Some(TweakCheck::Powershell {
                script: r#"
$k = Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' -EA 0
$e = $true
foreach ($i in $k) {
  $v = Get-ItemProperty -Path $i.PSPath -Name 'TcpAckFrequency' -EA 0
  if ($v.TcpAckFrequency -ne 1) { $e = $false; break }
}
$e
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Remove-ItemProperty -Path $_.PSPath -Name 'TcpAckFrequency' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Set-ItemProperty -Path $_.PSPath -Name 'TcpAckFrequency' -Value 1 -Type DWord -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_tcp_no_delay".to_string(),
            category: TweakCategory::Network,
            name: "Disable Nagle's Algorithm".to_string(),
            description: "Sets TCPNoDelay=1 via registry. disables packet buffering for lower latency.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$k = Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' -EA 0
$e = $true
foreach ($i in $k) {
  $v = Get-ItemProperty -Path $i.PSPath -Name 'TCPNoDelay' -EA 0
  if ($v.TCPNoDelay -ne 1) { $e = $false; break }
}
$e
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Remove-ItemProperty -Path $_.PSPath -Name 'TCPNoDelay' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Set-ItemProperty -Path $_.PSPath -Name 'TCPNoDelay' -Value 1 -Type DWord -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_default_ttl".to_string(),
            category: TweakCategory::Network,
            name: "Set Default TTL".to_string(),
            description: "Sets DefaultTTL to 64.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "DefaultTTL".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "DefaultTTL".to_string(),
                expected_value: RegistryValue::DWord(64),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "DefaultTTL".to_string(),
                    value: RegistryValue::DWord(64),
                }
            ]
        },
        Tweak {
            id: "net_tcp_1323_opts".to_string(),
            category: TweakCategory::Network,
            name: "Enable TCP Timestamps & Window Scaling".to_string(),
            description: "Enables RFC 1323 options (Tcp1323Opts=3).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "Tcp1323Opts".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "Tcp1323Opts".to_string(),
                expected_value: RegistryValue::DWord(3),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "Tcp1323Opts".to_string(),
                    value: RegistryValue::DWord(3),
                }
            ]
        },
        Tweak {
            id: "net_max_user_port".to_string(),
            category: TweakCategory::Network,
            name: "Increase Max User Port".to_string(),
            description: "Sets MaxUserPort to 65534.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "MaxUserPort".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "MaxUserPort".to_string(),
                expected_value: RegistryValue::DWord(65534),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "MaxUserPort".to_string(),
                    value: RegistryValue::DWord(65534),
                }
            ]
        },
        Tweak {
            id: "net_tcp_timed_wait".to_string(),
            category: TweakCategory::Network,
            name: "Reduce TCP Timed Wait Delay".to_string(),
            description: "Sets TcpTimedWaitDelay to 30.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "TcpTimedWaitDelay".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "TcpTimedWaitDelay".to_string(),
                expected_value: RegistryValue::DWord(30),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "TcpTimedWaitDelay".to_string(),
                    value: RegistryValue::DWord(30),
                }
            ]
        },
        // Auto-Tuning Check requires PowerShell or netsh check
        Tweak {
            id: "net_tcp_autotuning".to_string(),
            category: TweakCategory::Network,
            name: "Set TCP Auto-Tuning Level: Normal".to_string(),
            description: "Ensures TCP Auto-Tuning is set to 'Normal'.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "autotuninglevel=normal".into()], // Re-apply normal as revert (idempotent, but safe)
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName Internet).AutoTuningLevelLocal".to_string(),
                expected_output: "Normal".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "autotuninglevel=normal".into()],
                }
            ]
        },
        Tweak {
            id: "net_tcp_ctcp".to_string(),
            category: TweakCategory::Network,
            name: "Set Congestion Provider to CUBIC".to_string(),
            description: "Sets TCP Congestion Provider to 'CUBIC'.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "supplemental".into(), "template=internet".into(), "congestionprovider=cubic".into()],
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName Internet).CongestionProvider".to_string(),
                expected_output: "CUBIC".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "supplemental".into(), "template=internet".into(), "congestionprovider=cubic".into()],
                }
            ]
        },
        Tweak {
            id: "net_tcp_ecn".to_string(),
            category: TweakCategory::Network,
            name: "Enable TCP ECN Capability".to_string(),
            description: "Enables Explicit Congestion Notification (ECN). Can improve performance on modern networks, but some routers/ISPs may not handle ECN correctly.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "ecncapability=disabled".into()],
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
             check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName Internet).EcnCapability".to_string(),
                expected_output: "Enabled".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "ecncapability=enabled".into()],
                }
            ]
        },
        Tweak {
            id: "net_global_max_tcp_window".to_string(),
            category: TweakCategory::Network,
            name: "Set GlobalMaxTcpWindowSize".to_string(),
            description: "Sets GlobalMaxTcpWindowSize to 65535 for better throughput on most broadband connections.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "GlobalMaxTcpWindowSize".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "GlobalMaxTcpWindowSize".to_string(),
                expected_value: RegistryValue::DWord(65535),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "GlobalMaxTcpWindowSize".to_string(),
                    value: RegistryValue::DWord(65535),
                }
            ]
        },
        Tweak {
            id: "net_enable_pmtu_discovery".to_string(),
            category: TweakCategory::Network,
            name: "Enable Path MTU Discovery".to_string(),
            description: "Enables Path MTU Discovery (EnablePMTUDiscovery=1) so Windows can discover optimal packet size.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnablePMTUDiscovery".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "EnablePMTUDiscovery".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnablePMTUDiscovery".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        Tweak {
            id: "net_disable_pmtu_bh_detect".to_string(),
            category: TweakCategory::Network,
            name: "Disable PMTU Black Hole Detection".to_string(),
            description: "Disables PMTU black hole detection (EnablePMTUBHDetect=0) to reduce overhead.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnablePMTUBHDetect".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "EnablePMTUBHDetect".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnablePMTUBHDetect".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]
        },
        Tweak {
            id: "net_enable_sack".to_string(),
            category: TweakCategory::Network,
            name: "Enable Selective Acknowledgment (SACK)".to_string(),
            description: "Enables TCP Selective Acknowledgment (SackOpts=1) to improve performance when packet loss occurs.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "SackOpts".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "SackOpts".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "SackOpts".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        Tweak {
            id: "net_tcp_delack_ticks".to_string(),
            category: TweakCategory::Network,
            name: "Set TCP Delayed ACK Ticks to 0".to_string(),
            description: "Sets TcpDelAckTicks=0 to send acknowledgements immediately.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$k = Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' -EA 0
$e = $true
foreach ($i in $k) {
  $v = Get-ItemProperty -Path $i.PSPath -Name 'TcpDelAckTicks' -EA 0
  if ($v.TcpDelAckTicks -ne 0) { $e = $false; break }
}
$e
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Remove-ItemProperty -Path $_.PSPath -Name 'TcpDelAckTicks' -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
             operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces' | ForEach-Object {
    Set-ItemProperty -Path $_.PSPath -Name 'TcpDelAckTicks' -Value 0 -Type DWord -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_tcp_bbr".to_string(),
            category: TweakCategory::Network,
            name: "Set Congestion Provider to BBR (Windows 11+)".to_string(),
            description: "Sets TCP congestion provider to BBR. Only works on Windows 11 22H2+.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "supplemental".into(), "template=internet".into(), "congestionprovider=cubic".into()],
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName Internet).CongestionProvider".to_string(),
                expected_output: "BBR".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "supplemental".into(), "template=internet".into(), "congestionprovider=bbr".into()],
                }
            ]
        },
         Tweak {
            id: "net_network_throttling".to_string(),
            category: TweakCategory::Network,
            name: "Disable Network Throttling Index".to_string(),
            description: "Sets NetworkThrottlingIndex to FFFFFFFF (Disabled). Crucial for gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "NetworkThrottlingIndex".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "NetworkThrottlingIndex".to_string(),
                expected_value: RegistryValue::DWord(0xFFFFFFFF),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "NetworkThrottlingIndex".to_string(),
                    value: RegistryValue::DWord(0xFFFFFFFF),
                }
            ]
        },
        Tweak {
            id: "net_system_responsiveness".to_string(),
            category: TweakCategory::Network,
            name: "Optimize System Responsiveness".to_string(),
            description: "Sets SystemResponsiveness to 0.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "SystemResponsiveness".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "SystemResponsiveness".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "SystemResponsiveness".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]
        },

        // ============================================
        // C.16: TCP Initial RTO
        // ============================================
        Tweak {
            id: "net_tcp_initial_rto".to_string(),
            category: TweakCategory::Network,
            name: "Reduce TCP Initial Retransmission Timeout".to_string(),
            description: "Reduces TCP Initial RTO to 2 seconds (minimum) for faster connection retries.

Default is 3 seconds. Reducing to 2 seconds means faster recovery from initial connection failures.

Useful for gaming and real-time applications.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-NetTCPSetting -SettingName Internet).InitialRtoMs -eq 2000) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "initialRto=3000".into()],
                }
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "initialRto=2000".into()],
                }
            ]
        },
    ]
}
