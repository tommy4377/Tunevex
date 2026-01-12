//! GPU Optimization Tweaks Module
//!
//! Hardware GPU Scheduling and other GPU-related tweaks

pub mod msi;
pub mod scheduling;

use crate::modules::types::Tweak;

pub fn get_gpu_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(scheduling::get_scheduling_tweaks());
    tweaks.extend(msi::get_gpu_msi_tweaks());
    tweaks
}
