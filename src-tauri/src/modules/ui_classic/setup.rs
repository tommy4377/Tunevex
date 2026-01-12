use rust_embed::RustEmbed;
use std::fs;
use std::path::PathBuf;
use tauri::App;

#[derive(RustEmbed)]
#[folder = "resources/startallback"]
struct Asset;

pub fn deploy_assets(_app: &App) -> Result<(), String> {
    // Target directory in ProgramData
    let target_dir = PathBuf::from("C:\\ProgramData\\TommyTweaker\\startallback");

    // Create if missing
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }

    for file in Asset::iter() {
        let file_path = file.as_ref();

        if let Some(content) = Asset::get(file_path) {
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
