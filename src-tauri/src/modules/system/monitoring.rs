use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::command;

// WMI for robust GPU detection
use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

pub struct SystemMonitor {
    sys: System,
    disks: Disks,
    gpu_usage: Arc<Mutex<f32>>,
    gpu_name: String,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let gpu_usage = Arc::new(Mutex::new(0.0));
        let gpu_usage_clone = gpu_usage.clone();

        // Spawn background thread for GPU monitoring via typeperf
        thread::spawn(move || {
            loop {
                {
                    // Run typeperf for 1 sample (effectively current snapshot)
                    // We sum all engine loads.
                    // Note: This block is synchronous and takes ~1s to execute by typeperf design
                    let output = Command::new("typeperf")
                        .args(&["\\GPU Engine(*)\\Utilization Percentage", "-sc", "1"])
                        .creation_flags(0x08000000)
                        .output();

                    if let Ok(out) = output {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        // Output format:
                        // "Timestamp","value","value"...
                        // "01/16/2026...", "0.00", "5.00"...

                        let lines: Vec<&str> = stdout.trim().lines().collect();
                        if lines.len() >= 2 {
                            let csv_line = lines.last().unwrap(); // Get the data line
                                                                  // Parse CSV line
                                                                  // Split by comma, strip quotes
                            let sum: f32 = csv_line
                                .split(',')
                                .skip(1) // Skip timestamp
                                .filter_map(|s| {
                                    let s = s.trim().trim_matches('"');
                                    // Handle both dot and comma decimals depending on locale
                                    let clean_s = s.replace(',', ".");
                                    clean_s.parse::<f32>().ok()
                                })
                                .sum();

                            // Cap at 100%
                            let usage = if sum > 100.0 { 100.0 } else { sum };

                            if let Ok(mut g) = gpu_usage_clone.lock() {
                                *g = usage;
                            }
                        }
                    }
                }
                // Determine sleep based on overhead. typeperf takes ~1s.
                // We want update every ~2s total.
                thread::sleep(Duration::from_millis(1000));
            }
        });

        // Fetch GPU name via WMI (robust, cross-vendor)
        let gpu_name = get_gpu_name_wmi().unwrap_or_else(|| {
            // Fallback to wmic command if WMI crate fails
            get_gpu_name_wmic().unwrap_or_else(|| "Unknown GPU".to_string())
        });

        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            disks: Disks::new_with_refreshed_list(),
            gpu_usage,
            gpu_name,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.disks.refresh(true);
    }
}

use tauri::Manager; // Import Manager trait for .state()

#[derive(serde::Serialize)]
pub struct DiskStats {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
}

#[derive(serde::Serialize)]
pub struct GpuStats {
    name: String,
    usage: f32,
}

#[derive(serde::Serialize)]
pub struct SystemStats {
    cpu_usage: f32,
    ram_usage: u64,
    ram_total: u64,
    uptime: u64,
    username: String,
    disks: Vec<DiskStats>,
    gpu: Option<GpuStats>,
}

// WMI struct for Win32_VideoController
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32VideoController {
    name: Option<String>,
    adapter_ram: Option<u64>,
}

/// Get GPU name using WMI crate (Best Practice)
fn get_gpu_name_wmi() -> Option<String> {
    // Initialize COM library
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;

    // Query all video controllers
    let results: Vec<Win32VideoController> = wmi_con.query().ok()?;

    if results.is_empty() {
        return None;
    }

    // Prioritize discrete GPUs (NVIDIA, AMD Radeon RX, Intel Arc)
    let discrete_keywords = ["nvidia", "geforce", "rtx", "gtx", "radeon", "rx", "arc"];
    let integrated_keywords = ["intel", "uhd", "iris", "vega", "amd"];

    // First pass: look for discrete
    for gpu in &results {
        if let Some(ref name) = gpu.name {
            let lower = name.to_lowercase();
            if discrete_keywords.iter().any(|k| lower.contains(k)) {
                return Some(name.clone());
            }
        }
    }

    // Second pass: look for integrated
    for gpu in &results {
        if let Some(ref name) = gpu.name {
            let lower = name.to_lowercase();
            if integrated_keywords.iter().any(|k| lower.contains(k)) {
                return Some(name.clone());
            }
        }
    }

    // Fallback: return first found
    results.first().and_then(|g| g.name.clone())
}

/// Fallback: Get GPU name using wmic command (Legacy)
fn get_gpu_name_wmic() -> Option<String> {
    let output = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "name"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Collect all valid names
    let names: Vec<String> = stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && l.to_lowercase() != "name")
        .collect();

    if names.is_empty() {
        return None;
    }

    // Prioritize discrete GPUs
    let keywords = [
        "nvidia", "geforce", "rtx", "gtx", // NVIDIA discrete
        "radeon", "rx",  // AMD discrete
        "arc", // Intel Arc discrete
        "intel", "uhd", "iris", // Intel integrated
        "vega", "amd", // AMD integrated
    ];

    // Try to find a match for keywords
    let best_match = names.iter().find(|name| {
        let lower = name.to_lowercase();
        keywords.iter().any(|&k| lower.contains(k))
    });

    // Return best match or just the first one
    if let Some(name) = best_match {
        Some(name.clone())
    } else {
        Some(names[0].clone())
    }
}

#[command]
pub async fn get_quick_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    let stats = tokio::task::spawn_blocking(move || {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().unwrap();
        // Only refresh CPU and Memory, NOT disks
        monitor.sys.refresh_cpu_all();
        monitor.sys.refresh_memory();

        // Get Username
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());

        SystemStats {
            cpu_usage: monitor.sys.global_cpu_usage(),
            ram_usage: monitor.sys.used_memory(),
            ram_total: monitor.sys.total_memory(),
            uptime: sysinfo::System::uptime(),
            username,
            disks: Vec::new(), // Return empty, fetched separately
            gpu: Some({
                let usage = *monitor.gpu_usage.lock().unwrap();
                GpuStats {
                    name: monitor.gpu_name.clone(), // Use cached name
                    usage,
                }
            }),
        }
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(stats)
}

#[command]
pub async fn get_disk_stats(app: tauri::AppHandle) -> Result<Vec<DiskStats>, String> {
    let disks = tokio::task::spawn_blocking(move || {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().unwrap();
        monitor.disks.refresh(true); // Slow operation

        monitor
            .disks
            .iter()
            .map(|disk| DiskStats {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                is_removable: disk.is_removable(),
            })
            .collect()
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(disks)
}

#[command]
pub async fn get_system_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    // Legacy support or full fetch if needed
    let mut stats = get_quick_stats(app.clone()).await?;
    stats.disks = get_disk_stats(app).await?;
    Ok(stats)
}
