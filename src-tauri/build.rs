fn main() {
    println!("cargo:rerun-if-env-changed=TOMMYTWEAKER_SKIP_ELEVATION");
    let skip_elevation = std::env::var("PROFILE").as_deref() == Ok("debug")
        && std::env::var_os("TOMMYTWEAKER_SKIP_ELEVATION").is_some();
    if cfg!(target_os = "windows") && !skip_elevation {
        // Request admin privileges via linker flag instead of winres/manifest
        // This avoids conflict with Tauri's own resource generation
        println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator' uiAccess='false'");
    }
    tauri_build::build()
}
