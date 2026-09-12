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
            check: Some(TweakCheck::NetworkInterfacesCheck {
                key: "TcpAckFrequency".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::NetworkInterfacesDelete {
                    key: "TcpAckFrequency".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::NetworkInterfacesSet {
                    key: "TcpAckFrequency".to_string(),
                    value: RegistryValue::DWord(1),
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
            check: Some(TweakCheck::NetworkInterfacesCheck {
                key: "TCPNoDelay".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::NetworkInterfacesDelete {
                    key: "TCPNoDelay".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::NetworkInterfacesSet {
                    key: "TCPNoDelay".to_string(),
                    value: RegistryValue::DWord(1),
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
            name: "Enable TCP Timestamps".to_string(),
            description: "Enables RFC 1323 timestamps through netsh. Revert explicitly disables timestamps; window scaling and RSS are independent.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "Tcp1323Opts".to_string(),
                },
                TweakOperation::Command { cmd: "netsh".into(), args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "timestamps=disabled".into()] }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::TcpGlobal { settings: vec![("timestamps".into(), "enabled".into())] }),
            operations: vec![
                TweakOperation::Command { cmd: "netsh".into(), args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "timestamps=enabled".into()] }
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
        Tweak {
            id: "net_tcp_autotuning".to_string(),
            category: TweakCategory::Network,
            name: "Set TCP Auto-Tuning Level: Normal".to_string(),
            description: "Ensures TCP Auto-Tuning is set to 'Normal'.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'Tunevex\backups\autotuning.txt'
if (!(Test-Path -LiteralPath $file)) { $file = Join-Path $env:ProgramData 'TommyTweaker\backups\autotuning.txt' }
$value = (Get-Content -Raw -LiteralPath $file -ErrorAction Stop).Trim()
if ($value -notin @('disabled', 'highlyrestricted', 'restricted', 'normal', 'experimental')) { throw 'Invalid saved TCP Auto-Tuning level' }
netsh int tcp set global "autotuninglevel=$value"
if ($LASTEXITCODE -ne 0) { throw 'Failed to restore TCP Auto-Tuning' }
Remove-Item -LiteralPath $file
"#.into(),
            }]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::TcpGlobal { settings: vec![("autotuninglevel".into(), "normal".into())] }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'Tunevex\backups\autotuning.txt'
if (!(Test-Path -LiteralPath $file)) {
    $dump = netsh int tcp dump
    if ($LASTEXITCODE -ne 0) { throw 'Cannot query TCP Auto-Tuning' }
    $match = [regex]::Match(($dump -join ' '), '\bautotuninglevel=(disabled|highlyrestricted|restricted|normal|experimental)\b')
    if (!$match.Success) { throw 'Cannot read the current TCP Auto-Tuning level' }
    New-Item -ItemType Directory -Force -Path (Split-Path $file) | Out-Null
    $match.Groups[1].Value | Set-Content -LiteralPath $file -ErrorAction Stop
}
netsh int tcp set global autotuninglevel=normal
if ($LASTEXITCODE -ne 0) { throw 'Failed to set TCP Auto-Tuning' }
"#.into(),
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
                    args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "global".to_string(), "ecncapability=disabled".to_string()],
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::TcpGlobal { settings: vec![("ecncapability".into(), "enabled".into())] }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".to_string(), "set".into(), "global".to_string(), "ecncapability=enabled".to_string()],
                }
            ]
        },
        // REMOVED: net_global_max_tcp_window - conflicts with TCP window scaling (RFC 1323), limits throughput
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
            check: Some(TweakCheck::NetworkInterfacesCheck {
                key: "TcpDelAckTicks".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::NetworkInterfacesDelete {
                    key: "TcpDelAckTicks".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::NetworkInterfacesSet {
                    key: "TcpDelAckTicks".to_string(),
                    value: RegistryValue::DWord(0),
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
                script: "(Get-NetTCPSetting -SettingName Internet -ErrorAction Stop).CongestionProvider -in @('BBR', 'BBR2')".into(),
                expected_output: "True".into(),
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
            id: "net_tcp_initial_rto".to_string(),
            category: TweakCategory::Network,
            name: "Reduce TCP Initial Retransmission Timeout".to_string(),
            description: "Reduces TCP Initial RTO to 2 seconds (minimum) for faster connection retries. Default is 3 seconds.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::TcpGlobal { settings: vec![("initialrto".into(), "2000".into())] }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "initialRto=3000".to_string()],
                }
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "tcp".into(), "set".into(), "global".into(), "initialRto=2000".to_string()],
                }
            ]
        },
        // ============================================
        // NEW PART 2 NETWORK TWEAKS
        // ============================================
        Tweak {
            id: "net_ipv4_preference".to_string(),
            category: TweakCategory::Network,
            name: "Prefer IPv4 over IPv6".to_string(),
            description: "Forces Windows to prefer IPv4 routing without fully disabling IPv6. Safer than a full IPv6 disable - keeps Microsoft Store and Xbox login working while reducing routing overhead.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                expected_value: RegistryValue::DWord(32),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                value: RegistryValue::DWord(32),
            }],
        },
        Tweak {
            id: "net_disable_teredo".to_string(),
            category: TweakCategory::Network,
            name: "Disable Teredo Tunneling".to_string(),
            description: "Disables the Teredo IPv6 tunneling adapter that adds latency overhead even on pure IPv4 connections.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTeredoConfiguration -PolicyStore PersistentStore -ErrorAction Stop).Type -eq 'Disabled'".into(),
                expected_output: "True".into(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["interface".to_string(), "teredo".to_string(), "set".to_string(), "state".to_string(), "default".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["interface".to_string(), "teredo".to_string(), "set".to_string(), "state".to_string(), "disabled".to_string()],
            }],
        },
        Tweak {
            id: "net_ntp_pool".to_string(),
            category: TweakCategory::Network,
            name: "Use Global NTP Pool".to_string(),
            description: "Replaces time.windows.com with the global NTP pool for more accurate time sync. Prevents timestamp desync in multiplayer games.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters".to_string(),
                key: "NtpServer".to_string(),
                expected_value: RegistryValue::String("0.pool.ntp.org,0x9 1.pool.ntp.org,0x9 2.pool.ntp.org,0x9 3.pool.ntp.org,0x9".to_string()),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters".to_string(),
                    key: "NtpServer".to_string(),
                    value: RegistryValue::String("time.windows.com,0x9".to_string()),
                },
                TweakOperation::Command {
                    cmd: "w32tm".to_string(),
                    args: vec!["/config".to_string(), "/update".to_string()],
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters".to_string(),
                    key: "NtpServer".to_string(),
                    value: RegistryValue::String("0.pool.ntp.org,0x9 1.pool.ntp.org,0x9 2.pool.ntp.org,0x9 3.pool.ntp.org,0x9".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters".to_string(),
                    key: "Type".to_string(),
                    value: RegistryValue::String("NTP".to_string()),
                },
                TweakOperation::Command {
                    cmd: "w32tm".to_string(),
                    args: vec!["/config".to_string(), "/update".to_string()],
                },
                TweakOperation::Command {
                    cmd: "net".to_string(),
                    args: vec!["stop".to_string(), "w32time".to_string()],
                },
                TweakOperation::Command {
                    cmd: "net".to_string(),
                    args: vec!["start".to_string(), "w32time".to_string()],
                },
            ],
        },
        Tweak {
            id: "net_tcp_heuristics_disable".to_string(),
            category: TweakCategory::Network,
            name: "Disable TCP Window Scaling Heuristics".to_string(),
            description: "Disables Windows TCP heuristics that can interfere with throughput on modern high-speed connections.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName Internet -ErrorAction Stop).ScalingHeuristics -eq 'Disabled'".into(),
                expected_output: "True".into(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "heuristics".to_string(), "enabled".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "heuristics".to_string(), "disabled".to_string()],
            }],
        },
        Tweak {
            id: "net_tcp_fast_open".to_string(),
            category: TweakCategory::Network,
            name: "Enable TCP Fast Open".to_string(),
            description: "Enables TCP Fast Open and Fast Open Fallback, reducing round-trips for new connections. Noticeable on frequently-visited sites.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::TcpGlobal { settings: vec![("fastopen".into(), "enabled".into()), ("fastopenfallback".into(), "enabled".into())] }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "global".to_string(), "fastopen=disabled".to_string()],
                },
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "global".to_string(), "fastopenfallback=disabled".to_string()],
                },
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "global".to_string(), "fastopen=enabled".to_string()],
                },
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "global".to_string(), "fastopenfallback=enabled".to_string()],
                },
            ],
        },
        Tweak {
            id: "net_tcp_icw10".to_string(),
            category: TweakCategory::Network,
            name: "TCP Initial Congestion Window = 10".to_string(),
            description: "Sets the Initial Congestion Window to 10 segments (default 4). Maximizes the data burst on new connections, speeding up page loads and download starts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "(Get-NetTCPSetting -SettingName InternetCustom -ErrorAction Stop).InitialCongestionWindowMss -eq 10".into(),
                expected_output: "True".into(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "supplemental".to_string(), "template=custom".to_string(), "icw=4".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "netsh".to_string(),
                args: vec!["int".to_string(), "tcp".to_string(), "set".to_string(), "supplemental".to_string(), "template=custom".to_string(), "icw=10".to_string()],
            }],
        },
        Tweak {
            id: "net_irp_stack_size".to_string(),
            category: TweakCategory::Network,
            name: "Increase Network Buffer Stack Size".to_string(),
            description: "Increases the LanmanServer IRP stack size to 30, improving LAN file transfer reliability and speed.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters".to_string(),
                key: "IRPStackSize".to_string(),
                expected_value: RegistryValue::DWord(30),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters".to_string(),
                key: "IRPStackSize".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters".to_string(),
                key: "IRPStackSize".to_string(),
                value: RegistryValue::DWord(30),
            }],
        },
    ]
}
