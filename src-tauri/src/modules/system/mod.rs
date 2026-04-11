//! System & Visuals Tweaks Module
pub mod maintenance;
pub mod memory;
pub mod monitoring;
pub mod msi;
pub mod priority;
pub mod restore;
pub mod services;

use crate::modules::types::Tweak;

pub fn get_system_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    // Priority and scheduling tweaks
    tweaks.extend(priority::get_priority_tweaks());
    // Memory management tweaks
    tweaks.extend(memory::get_memory_tweaks());
    // Services
    tweaks.extend(services::get_service_tweaks());
    // Maintenance tweaks
    tweaks.extend(maintenance::get_system_tweaks());
    tweaks.extend(msi::get_system_msi_tweaks());
    tweaks
}
