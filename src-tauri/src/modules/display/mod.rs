//! Display/Monitor Tweaks Module
pub mod gpu;
pub mod monitor;
pub mod system;

use crate::modules::types::Tweak;

pub fn get_display_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(monitor::get_monitor_tweaks());
    tweaks.extend(gpu::get_gpu_tweaks());
    tweaks.extend(system::get_system_tweaks());
    tweaks
}
