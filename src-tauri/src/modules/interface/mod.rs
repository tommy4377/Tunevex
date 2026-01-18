//! Interface & UX Tweaks Module

use crate::modules::types::Tweak;

pub mod appearance;
pub mod context_menu;
pub mod explorer;
pub mod taskbar;

pub fn get_interface_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(appearance::get_appearance_tweaks());
    tweaks.extend(context_menu::get_context_menu_tweaks());
    tweaks.extend(explorer::get_explorer_tweaks());
    tweaks.extend(taskbar::get_taskbar_tweaks());
    tweaks
}
