//! Debloat Module - App Removal Only
//!
//! Organized into submodules:
//! - apps: AppX package removal (Microsoft, third-party, OEM)
//! - features: Windows optional features
//! - edge: Microsoft Edge debloat
//! - services: Misc, Bluetooth, Printer, Edge services
//! - tasks: Misc tasks

pub mod apps;
pub mod edge;
pub mod features;
pub mod services; // New
pub mod tasks; // New

use crate::modules::types::Tweak;

/// Returns all debloat-related tweaks from all submodules
pub fn get_debloat_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // App removal (Microsoft, third-party, OEM bloatware)
    tweaks.extend(apps::get_tweaks());

    // Windows optional features
    tweaks.extend(features::get_tweaks());

    // Edge debloat
    tweaks.extend(edge::get_tweaks());

    // Services
    tweaks.extend(services::get_service_tweaks());

    // Tasks
    tweaks.extend(tasks::get_task_tweaks());

    tweaks
}
