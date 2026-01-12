//! UI Classic Module
//! StartAllBack equivalent for Windows UI customization

pub mod tweaks;

use crate::modules::types::Tweak;

/// Returns all UI Classic tweaks for Windows interface customization
pub fn get_ui_classic_tweaks() -> Vec<Tweak> {
    tweaks::get_tweaks()
}
