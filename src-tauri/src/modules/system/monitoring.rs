use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::command;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub struct SystemMonitor {
    sys: System,
    disks: Disks,
    gpu_usage: Arc<Mutex<f32>>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let gpu_usage = Arc::new(Mutex::new(0.0));
        let gpu_usage_clone = gpu_usage.clone();

        // Spawn background thread for GPU monitoring
        thread::spawn(move || {
            loop {
                #[cfg(target_os = "windows")]
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
                            let sum: f32 = csv_line.split(',')
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

        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            disks: Disks::new_with_refreshed_list(),
            gpu_usage,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.disks.refresh(true); 
    }
}

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

use tauri::Manager; // Import Manager trait for .state()

fn get_gpu_name() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        // Use wmic to get GPU name
        let output = Command::new("wmic")
            .args(&["path", "win32_videocontroller", "get", "name"])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .ok()?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Collect all valid names
        let names: Vec<String> = stdout.lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && l.to_lowercase() != "name")
            .collect();

        if names.is_empty() {
             return Some("Unknown GPU".to_string());
        }

        // Prioritize discrete GPUs
        let keywords = ["nvidia", "amd", "radeon", "geforce", "rtx", "gtx", "arc"];
        
        // Try to find a match for keywords
        let best_match = names.iter()
            .find(|name| {
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
    #[cfg(not(target_os = "windows"))]
    {
        Some("Generic GPU".to_string())
    }
}

#[command]
pub async fn get_system_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    // Run in blocking thread to avoid blocking async runtime with std::Mutex and I/O
    let stats = tokio::task::spawn_blocking(move || {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().unwrap();
        monitor.refresh();

        let disks: Vec<DiskStats> = monitor.disks.iter().map(|disk| {
            DiskStats {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                is_removable: disk.is_removable(),
            }
        }).collect();

        // Get Username
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());

        SystemStats {
            cpu_usage: monitor.sys.global_cpu_usage(),
            ram_usage: monitor.sys.used_memory(),
            ram_total: monitor.sys.total_memory(),
            uptime: sysinfo::System::uptime(),
            username,
            disks,
            gpu: get_gpu_name().map(|name| {
                let usage = *monitor.gpu_usage.lock().unwrap();
                GpuStats { name, usage }
            }),
        }
    }).await.map_err(|e| e.to_string())?;

    Ok(stats)
}

