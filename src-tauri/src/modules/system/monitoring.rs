use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

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

        // Spawn background thread for GPU monitoring via typeperf (native Windows)
        let gpu_thread = thread::spawn(move || {
            loop {
                if gpu_cancel_clone.load(Ordering::SeqCst) {
                    break;
                }

                {
                    // Use typeperf to get GPU engine utilization - native Windows tool
                    let output = Command::new("typeperf")
                        .args([
                            "-sc",
                            "1",
                            "\"\\GPU Engine(*engtype_3D)\\Utilization Percentage\"",
                        ])
                        .creation_flags(CREATE_NO_WINDOW)
                        .output();

                    if let Ok(out) = output {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        // Parse output - typeperf returns CSV with value in last column
                        let lines: Vec<&str> = stdout.lines().collect();
                        if let Some(last_line) = lines.last() {
                            let parts: Vec<&str> = last_line.split(',').collect();
                            if let Some(value) = parts.last() {
                                let trimmed = value.trim().trim_matches('"');
                                if let Ok(usage) = trimmed.parse::<f32>() {
                                    let clamped = usage.clamp(0.0, 100.0);
                                    if let Ok(mut g) = gpu_usage_clone.lock() {
                                        *g = clamped;
                                    }
                                }
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

use winreg::enums::*;
use winreg::RegKey;

/// Get GPU name from registry (native Windows)
fn get_gpu_name() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let video_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";
    
    let mut best_gpu: Option<(u32, String)> = None;
    
    if let Ok(class_key) = hklm.open_subkey(video_path) {
        for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
            if let Ok(subkey) = class_key.open_subkey(&subkey_name) {
                // Get adapter RAM to find the primary GPU
                let adapter_ram: u32 = subkey.get_value("HardwareInformation.qwMemorySize").unwrap_or(0);
                if let Ok(driver_desc) = subkey.get_value::<String, _>("DriverDesc") {
                    let current_best = best_gpu.take();
                    match current_best {
                        Some((ram, _)) if ram >= adapter_ram => {
                            best_gpu = Some((ram, driver_desc));
                        }
                        None => {
                            best_gpu = Some((adapter_ram, driver_desc));
                        }
                        _ => {
                            best_gpu = current_best;
                        }
                    }
                }
            }
        }
    }
    
    best_gpu.map(|(_, name)| name)
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
