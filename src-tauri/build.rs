fn main() {
    if cfg!(target_os = "windows") {
        // Request admin privileges via linker flag instead of winres/manifest
        // This avoids conflict with Tauri's own resource generation
        println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator' uiAccess='false'");
    }
    tauri_build::build()
}
