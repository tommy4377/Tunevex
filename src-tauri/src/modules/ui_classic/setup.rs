use std::fs;
use std::path::{Path, PathBuf};
use tauri::{App, Manager};

pub fn deploy_assets(app: &App) -> Result<(), String> {
    // Target directory in ProgramData
    let target_dir = PathBuf::from("C:\\ProgramData\\TommyTweaker\\startallback");

    // Create if missing
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }

    // Resolve resource directory
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;

    // Try structured path first (common in dev/some builds)
    let mut resource_path = resource_dir.join("resources").join("startallback");

    // Fallback to flattened path (common in prod/msi)
    if !resource_path.exists() {
        resource_path = resource_dir.join("startallback");
    }

    if resource_path.exists() {
        copy_dir_recursive(&resource_path, &target_dir).map_err(|e| e.to_string())?;
    } else {
        println!(
            "Warning: UI Classic assets not found at {:?} or flattened path",
            resource_path
        );
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if ft.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}
