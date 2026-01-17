use std::ffi::c_void;
use tauri::Runtime;
use tauri::WebviewWindow;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE};

// Undocumented APIs
use windows::core::{s, PCSTR};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};

#[repr(C)]
struct WindowCompositionAttributeData {
    attribute: u32,
    data: *mut c_void,
    size_of_data: usize,
}

#[repr(C)]
struct AccentPolicy {
    accent_state: u32,
    accent_flags: u32,
    gradient_color: u32,
    animation_id: u32,
}

const WCA_ACCENT_POLICY: u32 = 19;
const ACCENT_ENABLE_ACRYLICBLURBEHIND: u32 = 4;
const ACCENT_ENABLE_BLURBEHIND: u32 = 3;

pub fn apply_legacy_acrylic<R: Runtime>(
    window: &WebviewWindow<R>,
    color: (u8, u8, u8, u8),
) -> Result<(), Box<dyn std::error::Error>> {
    let hwnd = window.hwnd()?.0 as isize;
    let hwnd = HWND(hwnd as _);

    unsafe {
        // Enforce basic DWM Rounding/Dark Mode first just in case
        let use_dark_mode = BOOL::from(true);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &use_dark_mode as *const _ as *const _,
            std::mem::size_of::<BOOL>() as u32,
        );

        // Load user32.dll for SetWindowCompositionAttribute
        let user32 = LoadLibraryA(s!("user32.dll"))?;
        let set_window_composition_attribute: Option<
            unsafe extern "system" fn(HWND, *const WindowCompositionAttributeData) -> BOOL,
        > = std::mem::transmute(GetProcAddress(user32, s!("SetWindowCompositionAttribute")));

        if let Some(set_window_composition_attribute) = set_window_composition_attribute {
            // Calculate ABGR color
            // Windows uses 0xAABBGGRR
            let (r, g, b, a) = color;
            let gradient_color =
                ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);

            let mut policy = AccentPolicy {
                accent_state: ACCENT_ENABLE_ACRYLICBLURBEHIND,
                accent_flags: 2, // 2 = Mixed? usually 0 or 2 used.
                gradient_color: gradient_color,
                animation_id: 0,
            };

            let mut data = WindowCompositionAttributeData {
                attribute: WCA_ACCENT_POLICY,
                data: &mut policy as *mut _ as *mut c_void,
                size_of_data: std::mem::size_of::<AccentPolicy>(),
            };

            set_window_composition_attribute(hwnd, &mut data);
            println!("Legacy Acrylic applied via user32!");
        } else {
            return Err("Could not find SetWindowCompositionAttribute".into());
        }
    }

    Ok(())
}
