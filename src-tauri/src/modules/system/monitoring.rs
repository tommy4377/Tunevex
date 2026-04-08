use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
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
    _gpu_thread_handle: Option<thread::JoinHandle<()>>,
    _gpu_cancel: Arc<AtomicBool>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let gpu_usage = Arc::new(Mutex::new(0.0));
        let gpu_cancel = Arc::new(AtomicBool::new(false));

        let gpu_usage_clone = gpu_usage.clone();
        let gpu_cancel_clone = gpu_cancel.clone();

        // Spawn background thread for GPU monitoring via PowerShell Get-Counter
        let gpu_thread = thread::spawn(move || {
            loop {
                if gpu_cancel_clone.load(Ordering::SeqCst) {
                    break;
                }

                {
                    let output = Command::new("powershell")
                        .args([
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
                        .creation_flags(0x08000000)
                        .output();

                    if let Ok(out) = output {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        if let Ok(usage) = stdout.trim().parse::<f32>() {
                            let clamped = usage.clamp(0.0, 100.0);
                            if let Ok(mut g) = gpu_usage_clone.lock() {
                                *g = clamped;
                            }
                        }
                    }
                }

                // Sleep in small increments to allow cancellation
                for _ in 0..20 {
                    if gpu_cancel_clone.load(Ordering::SeqCst) {
                        return;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
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
            _gpu_thread_handle: Some(gpu_thread),
            _gpu_cancel: gpu_cancel,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.disks.refresh(true);
    }
}

impl Drop for SystemMonitor {
    fn drop(&mut self) {
        self._gpu_cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self._gpu_thread_handle.take() {
            let _ = handle.join();
        }
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
    let stats = tokio::task::spawn_blocking(move || -> Result<SystemStats, String> {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().map_err(|e| e.to_string())?;
        monitor.sys.refresh_cpu_all();
        monitor.sys.refresh_memory();

        let username = std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string());

        let gpu_usage_val = *monitor.gpu_usage.lock().map_err(|e| e.to_string())?;

        Ok(SystemStats {
            cpu_usage: monitor.sys.global_cpu_usage(),
            ram_usage: monitor.sys.used_memory(),
            ram_total: monitor.sys.total_memory(),
            uptime: sysinfo::System::uptime(),
            username,
            disks: Vec::new(),
            gpu: Some(GpuStats {
                name: monitor.gpu_name.clone(),
                usage: gpu_usage_val,
            }),
        })
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(stats)
}

#[command]
pub async fn get_disk_stats(app: tauri::AppHandle) -> Result<Vec<DiskStats>, String> {
    let disks = tokio::task::spawn_blocking(move || -> Result<Vec<DiskStats>, String> {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().map_err(|e| e.to_string())?;
        monitor.disks.refresh(true);

        Ok(monitor
            .disks
            .iter()
            .map(|disk| DiskStats {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                is_removable: disk.is_removable(),
            })
            .collect())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(disks)
}

#[command]
pub async fn get_system_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    let mut stats = get_quick_stats(app.clone()).await?;
    stats.disks = get_disk_stats(app).await?;
    Ok(stats)
}
