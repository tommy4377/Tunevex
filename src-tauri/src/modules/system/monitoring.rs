// src-tauri/src/modules/system/monitoring.rs

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::{command, Manager};
use windows::Win32::System::Performance::{
    PdhAddCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE, PDH_HCOUNTER, PDH_HQUERY,
};
use winreg::{enums::*, RegKey};

// ─── Public data structs ─────────────────────────────────────────────────────

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

// ─── SystemMonitor ───────────────────────────────────────────────────────────

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

        let gpu_thread = thread::spawn(move || {
            // PDH_HQUERY / PDH_HCOUNTER wrap *mut c_void which is !Send.
            // We store them as usize (pointer-width integer) and reconstruct
            // the newtypes only inside this thread — safe because:
            //   1. Only this thread reads/writes the handles.
            //   2. Drop signals gpu_cancel and then joins this thread, so the
            //      caller cannot outlive the handles.
            let mut raw_query: usize = 0;
            let mut raw_counter: usize = 0;
            let mut pdh_ok = false;

            unsafe {
                let mut q: PDH_HQUERY = std::mem::zeroed();
                // null PCWSTR = real-time data source
                if PdhOpenQueryW(windows::core::PCWSTR::null(), 0, &mut q) == 0 {
                    let mut c: PDH_HCOUNTER = std::mem::zeroed();
                    let counter_path =
                        windows::core::w!(r"\GPU Engine(*engtype_3D)\Utilization Percentage");
                    if PdhAddCounterW(q, counter_path, 0, &mut c) == 0 {
                        // Warm-up: first sample is always 0, discard it
                        PdhCollectQueryData(q);
                        // Store as usize to satisfy Send bound
                        raw_query = q.0 as usize;
                        raw_counter = c.0 as usize;
                        pdh_ok = true;
                    } else {
                        eprintln!("[GPU Monitor] PdhAddCounterW failed");
                        PdhCloseQuery(q);
                    }
                } else {
                    eprintln!("[GPU Monitor] PdhOpenQueryW failed");
                }
            }

            // ── Polling loop ─────────────────────────────────────────────────
            loop {
                if gpu_cancel_clone.load(Ordering::SeqCst) {
                    break;
                }

                if pdh_ok {
                    unsafe {
                        // Reconstruct newtypes from stored usize values
                        let q = PDH_HQUERY(raw_query as *mut c_void);
                        let c = PDH_HCOUNTER(raw_counter as *mut c_void);

                        PdhCollectQueryData(q);

                        let mut fmt_val: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
                        let status = PdhGetFormattedCounterValue(
                            c,
                            PDH_FMT_DOUBLE,
                            None,
                            &mut fmt_val,
                        );

                        if status == 0 {
                            // SAFETY: PDH_FMT_DOUBLE guarantees doubleValue is valid
                            let usage = fmt_val.Anonymous.doubleValue as f32;
                            if let Ok(mut g) = gpu_usage_clone.lock() {
                                *g = usage.clamp(0.0, 100.0);
                            }
                        }
                    }
                }

                // Sleep in 100ms slices so cancel responds within ~100ms
                for _ in 0..20 {
                    if gpu_cancel_clone.load(Ordering::SeqCst) {
                        if pdh_ok {
                            unsafe {
                                PdhCloseQuery(PDH_HQUERY(raw_query as *mut c_void));
                            }
                        }
                        return;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }

            // Normal exit: close handles
            if pdh_ok {
                unsafe {
                    PdhCloseQuery(PDH_HQUERY(raw_query as *mut c_void));
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
        // 1. Signal thread to stop
        self.gpu_cancel.store(true, Ordering::SeqCst);
        // 2. Wait — thread closes PDH handles on its way out
        if let Some(handle) = self.gpu_thread_handle.take() {
            let _ = handle.join();
        }
    }
}

// ─── Registry-based GPU name detection ──────────────────────────────────────

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
        let update = best.as_ref().map_or(true, |(best_vram, _)| vram > *best_vram);
        if update {
            best = Some((vram, driver_desc));
        }
    }

    best.map(|(_, name)| name)
}

// ─── Tauri commands ──────────────────────────────────────────────────────────

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

        monitor.disks.refresh(false);

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