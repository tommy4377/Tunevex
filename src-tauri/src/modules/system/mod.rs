//! System & Visuals Tweaks Module
pub mod maintenance;
pub mod msi;
pub mod restore;
pub mod services;

use crate::modules::types::Tweak;

pub fn get_system_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    // Start with new submodules
    tweaks.extend(services::get_service_tweaks());
    tweaks.extend(maintenance::get_maintenance_tweaks());
    tweaks.extend(msi::get_system_msi_tweaks());
    // (Previous system tweaks would go here if they existed, or we assume they were moved/refactored)
    tweaks
}
