//! Storage Module
//!
//! Storage compression, scanning, and power management

pub mod commands;
pub mod compression;
pub mod msi;
pub mod ntfs;
pub mod power;
pub mod scanner;
pub mod state;

pub use compression::Algorithm;

use crate::modules::types::Tweak;

/// Returns all storage-related tweaks
pub fn get_storage_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(power::get_storage_power_tweaks());
    tweaks.extend(ntfs::get_ntfs_tweaks());
    tweaks.extend(msi::get_storage_msi_tweaks());
    tweaks
}
