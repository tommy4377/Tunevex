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
    // Note: Maintenance actions are commands, not tweaks in the list, but we can expose a getter if needed.
    // However, the user wants them as buttons in Home, so we exported specific commands.
    // tweaks.extend(maintenance::get_maintenance_tweaks());
    tweaks.extend(msi::get_system_msi_tweaks());
    tweaks
}
