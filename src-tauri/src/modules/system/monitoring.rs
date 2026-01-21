use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::command;

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

        // Spawn background thread for GPU monitoring via PowerShell Get-Counter
        thread::spawn(move || {
            loop {
                {
                    // Use PowerShell Get-Counter which is more reliable than typeperf
                    let output = Command::new("powershell")
                        .args(&[
                            "-NoProfile",
                            "-Command",
                            r#"
$counters = Get-Counter '\GPU Engine(*engtype_3D)\Utilization Percentage' -EA SilentlyContinue
if ($counters) {
    $total = ($counters.CounterSamples | Measure-Object -Property CookedValue -Sum).Sum
    [math]::Min($total, 100)
} else { 0 }
"#,
                        ])
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .output();

                    match output {
                        Ok(out) => {
                            let stdout = String::from_utf8_lossy(&out.stdout);
                            if let Ok(usage) = stdout.trim().parse::<f32>() {
                                let clamped = usage.min(100.0).max(0.0);
                                if let Ok(mut g) = gpu_usage_clone.lock() {
                                    *g = clamped;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                thread::sleep(Duration::from_millis(2000));
            }
        });

        // Fetch GPU name via wmic (no COM conflict)
        let gpu_name = get_gpu_name().unwrap_or_else(|| "Unknown GPU".to_string());
        println!("[GPU Monitor] Detected GPU: {}", gpu_name);

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

use tauri::Manager;

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

/// Get GPU name using PowerShell CIM (more robust than wmic)
fn get_gpu_name() -> Option<String> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_VideoController | Sort-Object -Property AdapterRAM -Descending | Select-Object -ExpandProperty Name | Select-Object -First 1",
        ])
        .creation_flags(0x08000000)
        .output()
        .ok()?;

    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if name.is_empty() {
        return None;
    }

    Some(name)
}

#[command]
pub async fn get_quick_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    let stats = tokio::task::spawn_blocking(move || {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().unwrap();
        monitor.sys.refresh_cpu_all();
        monitor.sys.refresh_memory();

        let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());

        SystemStats {
            cpu_usage: monitor.sys.global_cpu_usage(),
            ram_usage: monitor.sys.used_memory(),
            ram_total: monitor.sys.total_memory(),
            uptime: sysinfo::System::uptime(),
            username,
            disks: Vec::new(),
            gpu: Some({
                let usage = *monitor.gpu_usage.lock().unwrap();
                GpuStats {
                    name: monitor.gpu_name.clone(),
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
        monitor.disks.refresh(true);

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
    let mut stats = get_quick_stats(app.clone()).await?;
    stats.disks = get_disk_stats(app).await?;
    Ok(stats)
}
