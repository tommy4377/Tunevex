// src-tauri/src/modules/system/monitoring.rs

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::{command, Manager};
use winreg::{enums::*, RegKey};
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(serde::Serialize, Clone)]
pub struct DiskStats {
    pub name: String,
    pub mountpoint: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

#[derive(serde::Serialize, Clone)]
pub struct GpuStats {
    pub name: String,
    pub usage: f32,
}

#[derive(serde::Serialize, Clone)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub ram_usage: u64,
    pub ram_total: u64,
    pub uptime: u64,
    pub username: String,
    pub disks: Vec<DiskStats>,
    pub gpu: Option<GpuStats>,
}

// ─── SystemMonitor ────────────────────────────────────────────────────────────

pub struct SystemMonitor {
    pub sys: System,
    pub disks: Disks,
    pub gpu_usage: Arc<Mutex<f32>>,
    pub gpu_name: String,
    gpu_thread_handle: Option<thread::JoinHandle<()>>,
    gpu_cancel: Arc<AtomicBool>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let gpu_usage = Arc::new(Mutex::new(0.0f32));
        let gpu_cancel = Arc::new(AtomicBool::new(false));
        let gpu_usage_clone = gpu_usage.clone();
        let gpu_cancel_clone = gpu_cancel.clone();

        // GPU monitoring via typeperf — nativo Windows, no PDH bindings necessari
        let gpu_thread = thread::spawn(move || {
            loop {
                if gpu_cancel_clone.load(Ordering::SeqCst) {
                    break;
                }

                let output = std::process::Command::new("typeperf")
                    .args(["-sc", "1", r"\GPU Engine(*engtype_3D)\Utilization Percentage"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();

                if let Ok(out) = output {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    // typeperf restituisce CSV: prima riga header, seconda riga dati
                    // Sommiamo tutti i valori delle istanze GPU
                    let mut total = 0.0f32;
                    let mut count = 0;
                    for line in stdout.lines().skip(1) {
                        // Ogni campo dopo il timestamp è un valore GPU
                        for field in line.split(',').skip(1) {
                            let trimmed = field.trim().trim_matches('"');
                            if let Ok(v) = trimmed.parse::<f32>() {
                                if v >= 0.0 {
                                    total += v;
                                    count += 1;
                                }
                            }
                        }
                    }
                    if count > 0 {
                        if let Ok(mut g) = gpu_usage_clone.lock() {
                            *g = total.clamp(0.0, 100.0);
                        }
                    }
                }

                // Sleep in slice da 100ms per rispondere al cancel entro ~100ms
                for _ in 0..20 {
                    if gpu_cancel_clone.load(Ordering::SeqCst) {
                        return;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        let gpu_name = get_gpu_name().unwrap_or_else(|| "Unknown GPU".to_string());

        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            disks: Disks::new_with_refreshed_list(),
            gpu_usage,
            gpu_name,
            gpu_thread_handle: Some(gpu_thread),
            gpu_cancel,
        }
    }
}

impl Drop for SystemMonitor {
    fn drop(&mut self) {
        self.gpu_cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self.gpu_thread_handle.take() {
            let _ = handle.join();
        }
    }
}

// ─── GPU name da registry ─────────────────────────────────────────────────────

fn get_gpu_name() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = hklm
        .open_subkey(
            r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
        )
        .ok()?;

    let mut best: Option<(u32, String)> = None;

    for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
        let Ok(sub) = class_key.open_subkey(&subkey_name) else {
            continue;
        };
        let Ok(driver_desc) = sub.get_value::<String, _>("DriverDesc") else {
            continue;
        };
        let vram: u32 = sub
            .get_value("HardwareInformation.qwMemorySize")
            .unwrap_or(0u32);
        if best.as_ref().map_or(true, |(bv, _)| vram > *bv) {
            best = Some((vram, driver_desc));
        }
    }

    best.map(|(_, name)| name)
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

#[command]
pub async fn get_quick_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    tokio::task::spawn_blocking(move || -> Result<SystemStats, String> {
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
            uptime: System::uptime(),
            username,
            disks: Vec::new(),
            gpu: Some(GpuStats {
                name: monitor.gpu_name.clone(),
                usage: gpu_usage_val,
            }),
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_disk_stats(app: tauri::AppHandle) -> Result<Vec<DiskStats>, String> {
    tokio::task::spawn_blocking(move || -> Result<Vec<DiskStats>, String> {
        let state = app.state::<Mutex<SystemMonitor>>();
        let mut monitor = state.lock().map_err(|e| e.to_string())?;

        // true = rimuovi dischi stale, aggiorna la lista
        monitor.disks.refresh(true);

        Ok(monitor
            .disks
            .iter()
            .map(|disk| DiskStats {
                name: disk.name().to_string_lossy().into_owned(),
                mountpoint: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                is_removable: disk.is_removable(),
            })
            .collect())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_system_stats(app: tauri::AppHandle) -> Result<SystemStats, String> {
    let mut stats = get_quick_stats(app.clone()).await?;
    stats.disks = get_disk_stats(app).await?;
    Ok(stats)
}