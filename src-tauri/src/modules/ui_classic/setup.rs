use rust_embed::RustEmbed;
use std::fs;
use std::path::PathBuf;
use tauri::App;

#[derive(RustEmbed)]
#[folder = "resources/startallback"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "resources/secureuxtheme"]
struct SecureUxAssets;

pub fn deploy_assets(_app: &App) -> Result<(), String> {
    // 1. Deploy StartAllBack
    let sab_target = PathBuf::from("C:\\ProgramData\\TommyTweaker\\startallback");
    deploy_embed::<Asset>(sab_target)?;

    // 2. Deploy SecureUxTheme
    let sux_target = PathBuf::from("C:\\ProgramData\\TommyTweaker\\SecureUxTheme");
    deploy_embed::<SecureUxAssets>(sux_target)?;

    Ok(())
}

fn deploy_embed<A: RustEmbed>(target_dir: PathBuf) -> Result<(), String> {
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }

    for file in A::iter() {
        let file_path = file.as_ref();

        if let Some(content) = A::get(file_path) {
            let path_on_disk = target_dir.join(file_path);

            if let Some(parent) = path_on_disk.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            // Overwrite to ensure we have the correct version
            fs::write(&path_on_disk, content.data.as_ref()).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
