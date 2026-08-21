//! CPU & Performance Tweaks Module
//!
//! Refactored into submodules:
//! - scheduling.rs: Process Priority & MMCSS
//! - power.rs: Power Plans & States
//! - memory.rs: Memory & NTFS
//! - timer.rs: Timer Resolution & TSC
//! - vendor.rs: Intel/AMD specific tweaks (auto-detected)

use crate::modules::types::Tweak;

pub mod memory;
pub mod power;
pub mod scheduling;
pub mod timer;
pub mod vendor;

/// Returns all CPU and performance-related tweaks
pub fn get_cpu_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // CPU Scheduling & Priority
    tweaks.extend(scheduling::get_scheduling_tweaks());

    // Power Management
    tweaks.extend(power::get_power_tweaks());

    // Memory & Storage
    tweaks.extend(memory::get_memory_tweaks());

    // Timer & Boot Config
    tweaks.extend(timer::get_timer_tweaks());

    // Hardware-specific (Intel/AMD) - auto-detected
    tweaks.extend(vendor::get_hardware_specific_tweaks());

    tweaks
}
