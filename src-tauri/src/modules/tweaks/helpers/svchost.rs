pub fn apply_svc_host_split_all(enable_split: bool) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const SERVICES_PATH: &str = "SYSTEM\\CurrentControlSet\\Services";
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services_key = hklm
        .open_subkey_with_flags(SERVICES_PATH, KEY_READ)
        .map_err(|e| format!("Failed to open Services key: {}", e))?;

    for sub_name in services_key.enum_keys().flatten() {
        let lower = sub_name.to_lowercase();
        if lower.contains("xbl") || lower.contains("xbox") {
            continue;
        }

        let sub_path = format!("{}\\{}", SERVICES_PATH, sub_name);

        let sub_key = match hklm.open_subkey_with_flags(&sub_path, KEY_READ | KEY_SET_VALUE) {
            Ok(k) => k,
            Err(_) => continue,
        };

        if sub_key.get_value::<u32, _>("Start").is_err() {
            continue;
        }

        if enable_split {
            let _ = sub_key.delete_value("SvcHostSplitDisable");
        } else if let Err(e) = sub_key.set_value("SvcHostSplitDisable", &1u32) {
            eprintln!("[SvcHostSplit] Warning: set on {} failed: {}", sub_name, e);
        }
    }
    Ok(())
}
