use serde::Serialize;
use std::process::Command;
use winreg::{enums::*, RegKey};

#[derive(Debug, Serialize)]
pub struct SystemProfile {
    pub cpu_name: String,
    pub cpu_vendor: String,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub gpu_name: String,
    pub is_laptop: bool,
    pub system_drive_type: String,
    pub system_drive_free_pct: f32,
    pub connection_type: String,
    pub link_speed_mbps: u32,
    pub windows_version: String,
    pub power_plan: String,
    pub vbs_enabled: bool,
    pub hpet_enabled: bool,
    pub page_file_auto: bool,
    pub defender_realtime: bool,
    pub ram_usage_pct: f32,
    pub cpu_usage_pct: f32,
    pub startup_items_count: u32,
    pub running_services_count: u32,
    pub flags: DiagnosticFlags,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticFlags {
    pub vbs_on_gaming_rig: bool,
    pub balanced_power_plan: bool,
    pub hpet_on_gaming: bool,
    pub high_ram_at_idle: bool,
    pub many_startup_items: bool,
    pub hdd_system_drive: bool,
}

pub fn scan_system_profile() -> Result<SystemProfile, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let cpu_key = hklm
        .open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
        .map_err(|e| e.to_string())?;
    let cpu_name: String = cpu_key.get_value("ProcessorNameString").unwrap_or_default();
    let cpu_vendor = if cpu_name.contains("Intel") {
        "Intel".to_string()
    } else if cpu_name.contains("AMD") {
        "AMD".to_string()
    } else {
        "Unknown".to_string()
    };

    let ram_gb = get_ram_gb();
    let ram_usage_pct = get_ram_usage_pct();

    let gpu_name = get_gpu_name(&hklm);

    let is_laptop = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Power")
        .map(|k| k.get_value::<u32, _>("BatteryPresent").unwrap_or(0) == 1)
        .unwrap_or(false);

    let ver_key = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .map_err(|e| e.to_string())?;
    let build: String = ver_key.get_value("CurrentBuildNumber").unwrap_or_default();
    let edition: String = ver_key.get_value("EditionID").unwrap_or_default();
    let windows_version = format!("Windows 11 {} Build {}", edition, build);

    let vbs_enabled = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\DeviceGuard")
        .map(|k| {
            k.get_value::<u32, _>("EnableVirtualizationBasedSecurity")
                .unwrap_or(0)
                == 1
        })
        .unwrap_or(false);

    let power_plan = get_active_power_plan();

    let page_file_auto = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management")
        .map(|k| k.get_value::<String, _>("PagingFiles").is_ok())
        .unwrap_or(true);

    let defender_realtime = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows Defender\\Real-Time Protection")
        .map(|k| {
            k.get_value::<u32, _>("DisableRealtimeMonitoring")
                .unwrap_or(0)
                == 0
        })
        .unwrap_or(true);

    let startup_items_count = count_startup_items(&hklm);
    let running_services_count = count_running_services();
    let cpu_usage_pct = get_cpu_usage();

    let hpet_enabled = !hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Enum\\ACPI\\PNP0103\\0\\Device Parameters")
        .map(|k| k.get_value::<u32, _>("ConfigFlags").unwrap_or(0) & 0x1 != 0)
        .unwrap_or(false);

    let (connection_type, link_speed_mbps) = get_network_info();
    let (system_drive_type, system_drive_free_pct) = get_storage_info();

    let flags = DiagnosticFlags {
        vbs_on_gaming_rig: vbs_enabled && !is_laptop,
        balanced_power_plan: power_plan.to_lowercase().contains("balanced"),
        hpet_on_gaming: hpet_enabled && !is_laptop,
        high_ram_at_idle: ram_usage_pct > 65.0,
        many_startup_items: startup_items_count > 15,
        hdd_system_drive: system_drive_type == "HDD",
    };

    Ok(SystemProfile {
        cpu_name,
        cpu_vendor,
        cpu_cores: get_cpu_cores(),
        ram_gb,
        ram_usage_pct,
        gpu_name,
        is_laptop,
        system_drive_type,
        system_drive_free_pct,
        connection_type,
        link_speed_mbps,
        windows_version,
        power_plan,
        vbs_enabled,
        hpet_enabled,
        page_file_auto,
        defender_realtime,
        cpu_usage_pct,
        startup_items_count,
        running_services_count,
        flags,
    })
}

fn get_ram_gb() -> u32 {
    let out = Command::new("wmic")
        .args(["ComputerSystem", "get", "TotalPhysicalMemory"])
        .output()
        .ok();
    out.and_then(|o| {
        let s = String::from_utf8_lossy(&o.stdout);
        s.lines()
            .nth(1)?
            .trim()
            .parse::<u64>()
            .ok()
            .map(|b| (b / (1024 * 1024 * 1024)) as u32)
    })
    .unwrap_or(0)
}

fn get_gpu_name(hklm: &RegKey) -> String {
    hklm.open_subkey(
        "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000",
    )
    .and_then(|k| k.get_value("DriverDesc"))
    .unwrap_or_else(|_| "Unknown GPU".to_string())
}

fn get_active_power_plan() -> String {
    Command::new("powercfg")
        .args(["/getactivescheme"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.split('(').nth(1)?.split(')').next().map(str::to_string)
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_network_info() -> (String, u32) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let base =
        "SYSTEM\\CurrentControlSet\\Control\\Network\\{4D36E972-E325-11CE-BFC1-08002BE10318}";
    if let Ok(net_key) = hklm.open_subkey(base) {
        for name in net_key.enum_keys().flatten() {
            if let Ok(conn_key) = net_key.open_subkey(format!("{}\\Connection", name)) {
                let media_type: String = conn_key.get_value("MediaSubType").unwrap_or_default();
                if media_type.contains("Wireless") || media_type.contains("WiFi") {
                    return ("WiFi".to_string(), 0);
                }
                return ("Ethernet".to_string(), 1000);
            }
        }
    }
    ("Unknown".to_string(), 0)
}

fn get_storage_info() -> (String, f32) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let storage_type = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Services\\stornvme\\Enum")
        .map(|k| k.get_value::<u32, _>("Count").unwrap_or(0))
        .map(|count| if count > 0 { "NVMe" } else { "SSD" })
        .unwrap_or("HDD");

    let free_pct = Command::new("wmic")
        .args([
            "logicaldisk",
            "where",
            "DeviceID='C:'",
            "get",
            "FreeSpace,Size",
        ])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            let nums: Vec<u64> = s
                .lines()
                .nth(1)?
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if nums.len() >= 2 && nums[1] > 0 {
                Some((nums[0] as f32 / nums[1] as f32) * 100.0)
            } else {
                None
            }
        })
        .unwrap_or(50.0);

    (storage_type.to_string(), free_pct)
}

fn count_startup_items(hklm: &RegKey) -> u32 {
    let mut count = 0u32;
    let run_paths = [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run",
    ];
    for path in &run_paths {
        if let Ok(key) = hklm.open_subkey(path) {
            count += key.enum_values().count() as u32;
        }
    }
    count
}

fn count_running_services() -> u32 {
    Command::new("sc")
        .args(["query", "state=", "running"])
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.trim_start().starts_with("SERVICE_NAME"))
                .count() as u32
        })
        .unwrap_or(0)
}

fn get_cpu_cores() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(0)
}

fn get_ram_usage_pct() -> f32 {
    0.0
}

fn get_cpu_usage() -> f32 {
    0.0
}
