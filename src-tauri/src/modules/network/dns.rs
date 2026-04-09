use crate::modules::types::{
    RegistryCheck, RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType,
    WarningLevel,
};

/// Path shared by all DNS-related registry tweaks.
const DNSCACHE_PARAMS: &str = "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters";

pub fn get_dns_tweaks() -> Vec<Tweak> {
    vec![
        // ── DNS Provider: Google ─────────────────────────────────────────────
        Tweak {
            id: "net_dns_google".to_string(),
            category: TweakCategory::Network,
            name: "Set DNS: Google".to_string(),
            description: "Use Google Public DNS servers (8.8.8.8 / 8.8.4.4).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Native check: look for 8.8.8.8 in NameServer of any Tcpip interface subkey
            check: Some(TweakCheck::DnsServersContain {
                ip: "8.8.8.8".to_string(),
            }),
            // Native apply: write NameServer to every Tcpip\Parameters\Interfaces\{GUID}
            operations: vec![TweakOperation::SetDnsServers {
                primary: "8.8.8.8".to_string(),
                secondary: "8.8.4.4".to_string(),
            }],
            // Native revert: clear NameServer → DHCP takes over
            revert_operations: Some(vec![TweakOperation::ResetDnsServers]),
        },
        // ── DNS Provider: Cloudflare ─────────────────────────────────────────
        Tweak {
            id: "net_dns_cloudflare".to_string(),
            category: TweakCategory::Network,
            name: "Set DNS: Cloudflare".to_string(),
            description: "Use Cloudflare's fast and privacy-focused DNS (1.1.1.1 / 1.0.0.1)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::DnsServersContain {
                ip: "1.1.1.1".to_string(),
            }),
            operations: vec![TweakOperation::SetDnsServers {
                primary: "1.1.1.1".to_string(),
                secondary: "1.0.0.1".to_string(),
            }],
            revert_operations: Some(vec![TweakOperation::ResetDnsServers]),
        },
        // ── DNS Benchmark (Action, no PowerShell) ────────────────────────────
        Tweak {
            id: "net_dns_benchmark".to_string(),
            category: TweakCategory::Network,
            name: "DNS Benchmark & Auto-Select Fastest".to_string(),
            description:
                "Tests 25+ DNS servers and applies the fastest one (Legacy method, use Dashboard instead)."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![], // Handled by the benchmark_dns / apply_dns_server Tauri commands
        },
        // ── Increase DNS Max Cache TTL ────────────────────────────────────────
        Tweak {
            id: "net_dns_max_cache_ttl".to_string(),
            category: TweakCategory::Network,
            name: "Increase DNS Max Cache TTL".to_string(),
            description: "Increases MaxCacheTtl to 24 hours (86400 s). Keeps domains cached longer.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxCacheTtl".to_string(),
                expected_value: RegistryValue::DWord(86400),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxCacheTtl".to_string(),
                value: RegistryValue::DWord(86400),
            }],
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxCacheTtl".to_string(),
            }]),
        },
        // ── Lower Negative DNS Cache TTL ─────────────────────────────────────
        Tweak {
            id: "net_dns_max_negative_cache_ttl".to_string(),
            category: TweakCategory::Network,
            name: "Lower Negative DNS Cache TTL".to_string(),
            description: "Reduces MaxNegativeCacheTtl to 60 s so failed lookups retry faster.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxNegativeCacheTtl".to_string(),
                expected_value: RegistryValue::DWord(60),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxNegativeCacheTtl".to_string(),
                value: RegistryValue::DWord(60),
            }],
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: DNSCACHE_PARAMS.to_string(),
                key: "MaxNegativeCacheTtl".to_string(),
            }]),
        },
        // ── Optimize DNS Cache ────────────────────────────────────────────────
        // C.15: DNS Cache Optimization
        Tweak {
            id: "net_dns_cache_optimization".to_string(),
            category: TweakCategory::Network,
            name: "Optimize DNS Cache".to_string(),
            description:
                "Optimizes DNS cache with larger hash table, longer entry TTL, \
                 and faster failover for improved browsing speed."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Native multi-registry check: all three values must match simultaneously.
            check: Some(TweakCheck::MultiRegistry {
                checks: vec![
                    RegistryCheck {
                        root_key: "HKLM".to_string(),
                        path: DNSCACHE_PARAMS.to_string(),
                        key: "CacheHashTableSize".to_string(),
                        expected_value: RegistryValue::DWord(384),
                    },
                    RegistryCheck {
                        root_key: "HKLM".to_string(),
                        path: DNSCACHE_PARAMS.to_string(),
                        key: "MaxCacheEntryTtlLimit".to_string(),
                        expected_value: RegistryValue::DWord(64000),
                    },
                    RegistryCheck {
                        root_key: "HKLM".to_string(),
                        path: DNSCACHE_PARAMS.to_string(),
                        key: "ServiceConnHardTimeout".to_string(),
                        expected_value: RegistryValue::DWord(30),
                    },
                ],
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "CacheHashTableSize".to_string(),
                    value: RegistryValue::DWord(384),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "MaxCacheEntryTtlLimit".to_string(),
                    value: RegistryValue::DWord(64000),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "ServiceConnHardTimeout".to_string(),
                    value: RegistryValue::DWord(30),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "CacheHashTableSize".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "MaxCacheEntryTtlLimit".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: DNSCACHE_PARAMS.to_string(),
                    key: "ServiceConnHardTimeout".to_string(),
                },
            ]),
        },
    ]
}
