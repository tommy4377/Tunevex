//! GPU Optimization Tweaks Module
//!
//! Hardware GPU Scheduling and other GPU-related tweaks
//! Includes vendor-specific tweaks (Nvidia/AMD) with auto-detection

pub mod msi;
pub mod scheduling;
pub mod vendor;

use crate::modules::types::Tweak;

pub fn get_gpu_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(scheduling::get_scheduling_tweaks());
    tweaks.extend(msi::get_gpu_msi_tweaks());
    // Hardware-specific (Nvidia/AMD) - auto-detected
    tweaks.extend(vendor::get_vendor_gpu_tweaks());
    tweaks
}
