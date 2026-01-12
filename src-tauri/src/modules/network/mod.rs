//! Network Tweaks Module
//!
//! Refactored into submodules:
//! - tcp.rs
//! - adapter.rs
//! - security.rs
//! - dns.rs

use crate::modules::types::Tweak;

pub mod adapter;
pub mod dns;
pub mod dns_benchmark;
pub mod maintenance;
pub mod msi;
pub mod security;
pub mod tcp;

pub fn get_network_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    tweaks.extend(tcp::get_tcp_tweaks());
    tweaks.extend(adapter::get_adapter_tweaks());
    tweaks.extend(security::get_security_tweaks());
    tweaks.extend(dns::get_dns_tweaks());
    tweaks.extend(maintenance::get_maintenance_tweaks());
    tweaks.extend(msi::get_network_msi_tweaks());

    tweaks
}
