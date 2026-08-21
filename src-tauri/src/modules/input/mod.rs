//! Input (Mouse/Keyboard) Tweaks Module
//! Trigger refresh
//!
//! Based on:
//! - Atlas playbook: qol/disable-mouse-accel.yml
//! - Atlas playbook: qol/explorer/minimize-mouse-hover-time.yml
//! - Atlas playbook: qol/disable-touch-keyboard-features.yml
//! - prompt.md CATEGORY 4: MOUSE & INPUT

mod keyboard;
mod mouse;
mod usb;

use crate::modules::types::Tweak;

/// Returns all input-related tweaks (mouse, keyboard)
pub fn get_input_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    // ==== Mouse Tweaks ====
    tweaks.extend(mouse::get_mouse_tweaks());

    // ==== Keyboard Tweaks ====
    tweaks.extend(keyboard::get_keyboard_tweaks());

    // ==== USB Tweaks ====
    tweaks.extend(usb::get_usb_msi_tweaks());

    tweaks
}
