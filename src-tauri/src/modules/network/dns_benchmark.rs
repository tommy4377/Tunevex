use hickory_resolver::config::*;
use hickory_resolver::Resolver;
use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone)]
pub struct DnsProvider {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub description: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct DnsBenchmarkResult {
    pub provider: String,
    pub primary: String,
    pub secondary: String,
    pub description: String,
    pub primary_latency_ms: Option<u64>,
    pub secondary_latency_ms: Option<u64>,
    pub avg_latency_ms: Option<u64>,
}

const TIMEOUT: Duration = Duration::from_millis(800);
const PASSES: u32 = 3;

fn measure_latency(ip: &str) -> Option<u64> {
    let mut total = 0;
    let mut successes = 0;

    // Parse IP address
    let ip_addr: IpAddr = ip.parse().ok()?;

    // Create resolver config with the specific DNS server
    let mut config = ResolverConfig::new();
    config.add_name_server(NameServerConfig {
        socket_addr: SocketAddr::new(ip_addr, 53),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    });

    let mut opts = ResolverOpts::default();
    opts.timeout = TIMEOUT;
    opts.attempts = 1;

    // Create resolver
    let resolver = Resolver::new(config, opts).ok()?;

    for _ in 0..PASSES {
        let start = Instant::now();

        // Perform actual DNS query for google.com (A record)
        match resolver.lookup_ip("google.com") {
            Ok(_) => {
                total += start.elapsed().as_millis() as u64;
                successes += 1;
            }
            Err(_) => {}
        }

        // Small delay between queries
        thread::sleep(Duration::from_millis(100));
    }

    if successes > 0 {
        Some(total / successes as u64)
    } else {
        None
    }
}

pub fn get_providers() -> Vec<DnsProvider> {
    vec![
        DnsProvider {
            name: "Google".into(),
            primary: "8.8.8.8".into(),
            secondary: "8.8.4.4".into(),
            description:
                "World's most popular public DNS. Fast, reliable, but collects analytical data."
                    .into(),
        },
        DnsProvider {
            name: "Cloudflare".into(),
            primary: "1.1.1.1".into(),
            secondary: "1.0.0.1".into(),
            description: "Focuses on speed and privacy. Claims not to log IP addresses.".into(),
        },
        DnsProvider {
            name: "Cloudflare Security".into(),
            primary: "1.1.1.2".into(),
            secondary: "1.0.0.2".into(),
            description: "Cloudflare DNS with added malware blocking.".into(),
        },
        DnsProvider {
            name: "Quad9".into(),
            primary: "9.9.9.9".into(),
            secondary: "149.112.112.112".into(),
            description: "Blocks malicious domains using threat intelligence. Privacy focused."
                .into(),
        },
        DnsProvider {
            name: "Quad9 Unsecured".into(),
            primary: "9.9.9.10".into(),
            secondary: "149.112.112.10".into(),
            description: "Quad9 without malware blocking (no validation).".into(),
        },
        DnsProvider {
            name: "OpenDNS".into(),
            primary: "208.67.222.222".into(),
            secondary: "208.67.220.220".into(),
            description: "Veteran provider (Cisco). Reliable, phishing protection.".into(),
        },
        DnsProvider {
            name: "OpenDNS Family".into(),
            primary: "208.67.222.123".into(),
            secondary: "208.67.220.123".into(),
            description: "OpenDNS with Adult Content blocking.".into(),
        },
        DnsProvider {
            name: "AdGuard".into(),
            primary: "94.140.14.14".into(),
            secondary: "94.140.15.15".into(),
            description: "Blocks ads, trackers, and phishing.".into(),
        },
        DnsProvider {
            name: "AdGuard Family".into(),
            primary: "94.140.14.15".into(),
            secondary: "94.140.15.16".into(),
            description: "Blocks ads, trackers, and adult content.".into(),
        },
        DnsProvider {
            name: "CleanBrowsing".into(),
            primary: "185.228.168.9".into(),
            secondary: "185.228.169.9".into(),
            description: "Security filter. Blocks phishing, malware, and malicious domains.".into(),
        },
        DnsProvider {
            name: "Level3".into(),
            primary: "4.2.2.1".into(),
            secondary: "4.2.2.2".into(),
            description: "Enterprise-grade carrier DNS. Very fast in US.".into(),
        },
        DnsProvider {
            name: "Verisign".into(),
            primary: "64.6.64.6".into(),
            secondary: "64.6.65.6".into(),
            description: "Stable and secure, emphasizes zero data selling.".into(),
        },
        DnsProvider {
            name: "Comodo Secure".into(),
            primary: "8.26.56.26".into(),
            secondary: "8.20.247.20".into(),
            description: "Blocks malware and spyware sites.".into(),
        },
        DnsProvider {
            name: "Neustar".into(),
            primary: "156.154.70.1".into(),
            secondary: "156.154.71.1".into(),
            description: "Enterprise performance and reliability. 'UltraDNS'.".into(),
        },
        DnsProvider {
            name: "ControlD".into(),
            primary: "76.76.2.1".into(),
            secondary: "76.76.10.1".into(),
            description: "Highly customizable DNS with ad-blocking options.".into(),
        },
        DnsProvider {
            name: "Alternate DNS".into(),
            primary: "76.76.19.19".into(),
            secondary: "76.223.122.150".into(),
            description: "Ad-blocking DNS provider.".into(),
        },
        DnsProvider {
            name: "Yandex".into(),
            primary: "77.88.8.8".into(),
            secondary: "77.88.8.1".into(),
            description: "Basic Russian DNS. Fast in CIS approach.".into(),
        },
        DnsProvider {
            name: "SafeDNS".into(),
            primary: "195.46.39.39".into(),
            secondary: "195.46.39.40".into(),
            description: "Cloud-based web filtering and parental control.".into(),
        },
        DnsProvider {
            name: "Mullvad".into(),
            primary: "194.242.2.2".into(),
            secondary: "194.242.2.3".into(),
            description: "Privacy-first DNS from the VPN provider. No logging.".into(),
        },
        DnsProvider {
            name: "DNS.Watch".into(),
            primary: "84.200.69.80".into(),
            secondary: "84.200.70.40".into(),
            description: "German provider stating no censorship and no logging.".into(),
        },
    ]
}

pub fn run_benchmark() -> Vec<DnsBenchmarkResult> {
    let providers = get_providers();
    let mut results = Vec::new();

    // Spawn threads for each provider to run in parallel
    let handles: Vec<_> = providers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                let p1 = measure_latency(&p.primary);
                let p2 = measure_latency(&p.secondary);

                let avg = match (p1, p2) {
                    (Some(a), Some(b)) => Some((a + b) / 2),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                };

                DnsBenchmarkResult {
                    provider: p.name,
                    primary: p.primary,
                    secondary: p.secondary,
                    description: p.description,
                    primary_latency_ms: p1,
                    secondary_latency_ms: p2,
                    avg_latency_ms: avg,
                }
            })
        })
        .collect();

    for handle in handles {
        if let Ok(res) = handle.join() {
            results.push(res);
        }
    }

    results.sort_by(|a, b| {
        let lat_a = a.avg_latency_ms.unwrap_or(u64::MAX);
        let lat_b = b.avg_latency_ms.unwrap_or(u64::MAX);
        lat_a.cmp(&lat_b)
    });

    results
}

pub fn apply_dns(primary: String, secondary: String) -> Result<(), String> {
    let script = format!(
        "Get-NetAdapter | Where-Object {{ $_.Status -eq 'Up' }} | ForEach-Object {{ Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('{}','{}') -ErrorAction Stop }}",
        primary, secondary
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    // Clear cache
    let _ = Command::new("powershell")
        .args(["-Command", "Clear-DnsClientCache"])
        .output();

    Ok(())
}
