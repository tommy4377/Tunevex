//! Security & Protection Tweaks Module
//!
//! Refactored into submodules:
//! - defender: Windows Defender
//! - firewall: Windows Firewall
//! - uac: User Account Control
//! - exploit: Exploit mitigations
//! - authentication: Login settings
//! - hardening: General hardening
//! - smartscreen: SmartScreen
//! - error_reporting: WER
//! - services: Remote services
//! - updates: Windows Update control

use crate::modules::types::Tweak;

pub mod authentication;
pub mod defender;
pub mod error_reporting;
pub mod exploit;
pub mod firewall;
pub mod hardening;
pub mod services; // New
pub mod smartscreen;
pub mod uac;
pub mod updates; // New

pub fn get_security_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    tweaks.extend(defender::get_defender_tweaks());
    tweaks.extend(firewall::get_firewall_tweaks());
    tweaks.extend(uac::get_uac_tweaks());
    tweaks.extend(exploit::get_exploit_tweaks());
    tweaks.extend(authentication::get_authentication_tweaks());
    tweaks.extend(hardening::get_hardening_tweaks());
    tweaks.extend(smartscreen::get_smartscreen_tweaks());
    tweaks.extend(error_reporting::get_error_reporting_tweaks());
    tweaks.extend(services::get_service_tweaks()); // New
    tweaks.extend(updates::get_update_tweaks()); // New

    tweaks
}
