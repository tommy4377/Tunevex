use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_dns_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_dns_google".to_string(),
            category: TweakCategory::Network,
            name: "Set DNS: Google".to_string(),
            description: "Use Google Public DNS servers (8.8.8.8 / 8.8.4.4).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,

            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$b = $false; Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object { if ((Get-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex).ServerAddresses -contains '8.8.8.8') { $b = $true } }; $b
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$adapters = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }
foreach ($adapter in $adapters) {
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ResetServerAddresses -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$adapters = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }
foreach ($adapter in $adapters) {
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses @('8.8.8.8','8.8.4.4') -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_dns_cloudflare".to_string(),
            category: TweakCategory::Network,
            name: "Set DNS: Cloudflare".to_string(),
            description: "Use Cloudflare's fast and privacy-focused DNS (1.1.1.1 / 1.0.0.1).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,

            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$b = $false; Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object { if ((Get-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex).ServerAddresses -contains '1.1.1.1') { $b = $true } }; $b
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$adapters = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }
foreach ($adapter in $adapters) {
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ResetServerAddresses -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$adapters = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }
foreach ($adapter in $adapters) {
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses @('1.1.1.1','1.0.0.1') -ErrorAction SilentlyContinue
}
"#.to_string(),
                }
            ]
        },
        Tweak {
            id: "net_dns_benchmark".to_string(),
            category: TweakCategory::Network,
            name: "🔍 DNS Benchmark & Auto-Select Fastest".to_string(),
            description: "Tests 25+ DNS servers and applies the fastest one (Legacy method, use Dashboard instead).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, tweak_type: TweakType::Action, enabled: false,
            check: None,
            operations: vec![] // Legacy Placeholder or Script
        },
        // ... (We can omit other static DNS helpers if the Benchmark UI is the primary way)
        // Keeping only these examples for brevity as user should use the Benchmark UI
        Tweak {
            id: "net_dns_max_cache_ttl".to_string(),
            category: TweakCategory::Network,
            name: "Increase DNS Max Cache TTL".to_string(),
            description: "Increases MaxCacheTtl to 24 hours (86400s). Keeps domains cached longer.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxCacheTtl".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                key: "MaxCacheTtl".to_string(),
                expected_value: RegistryValue::DWord(86400),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxCacheTtl".to_string(),
                    value: RegistryValue::DWord(86400),
                }
            ]
        },
        Tweak {
            id: "net_dns_max_negative_cache_ttl".to_string(),
            category: TweakCategory::Network,
            name: "Lower Negative DNS Cache TTL".to_string(),
            description: "Reduces MaxNegativeCacheTtl to 60s so failed lookups retry faster.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxNegativeCacheTtl".to_string(),
                }
            ]), tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                key: "MaxNegativeCacheTtl".to_string(),
                expected_value: RegistryValue::DWord(60),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxNegativeCacheTtl".to_string(),
                    value: RegistryValue::DWord(60),
                }
            ]
        },

        // ============================================
        // C.15: DNS Cache Optimization
        // ============================================
        Tweak {
            id: "net_dns_cache_optimization".to_string(),
            category: TweakCategory::Network,
            name: "⚡ Optimize DNS Cache".to_string(),
            description: "Optimizes DNS cache for better performance.

Sets:
- CacheHashTableSize = 384 (larger hash table)
- MaxCacheEntryTtlLimit = 64000 (longer cache entries)
- ServiceConnHardTimeout = 30 (faster failover)

Reduces DNS lookup latency and improves browsing speed.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$path = "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\Parameters"
$hash = Get-ItemProperty -Path $path -Name "CacheHashTableSize" -ErrorAction SilentlyContinue
$ttl = Get-ItemProperty -Path $path -Name "MaxCacheEntryTtlLimit" -ErrorAction SilentlyContinue
$timeout = Get-ItemProperty -Path $path -Name "ServiceConnHardTimeout" -ErrorAction SilentlyContinue

if (($hash.CacheHashTableSize -eq 384) -and ($ttl.MaxCacheEntryTtlLimit -eq 64000) -and ($timeout.ServiceConnHardTimeout -eq 30)) {
    "True"
} else {
    "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "CacheHashTableSize".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxCacheEntryTtlLimit".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "ServiceConnHardTimeout".to_string(),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "CacheHashTableSize".to_string(),
                    value: RegistryValue::DWord(384),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "MaxCacheEntryTtlLimit".to_string(),
                    value: RegistryValue::DWord(64000),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Dnscache\\Parameters".to_string(),
                    key: "ServiceConnHardTimeout".to_string(),
                    value: RegistryValue::DWord(30),
                },
            ]
        },
    ]
}
