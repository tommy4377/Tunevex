pub const NIC_CLASS_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";

pub fn nic_subkey_paths() -> Vec<String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm.open_subkey(NIC_CLASS_PATH) {
        Ok(k) => k,
        Err(_) => return Vec::new(),
    };
    let mut paths = Vec::new();
    for sub in class_key.enum_keys().flatten() {
        if sub.len() != 4 || !sub.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let sub_path = format!("{}\\{}", NIC_CLASS_PATH, sub);
        if let Ok(k) = hklm.open_subkey(&sub_path) {
            if k.get_value::<String, _>("DriverDesc").is_ok() {
                paths.push(sub_path);
            }
        }
    }
    paths
}

pub fn set_nic_property(property: &str, value: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for path in nic_subkey_paths() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE | KEY_QUERY_VALUE) {
            Ok(k) => {
                let already: Result<String, _> = k.get_value(property);
                if already.is_ok() {
                    if let Err(e) = k.set_value(property, &value.to_string()) {
                        eprintln!(
                            "[NicProp] Warning: set {} on {} failed: {}",
                            property, path, e
                        );
                    }
                }
            }
            Err(e) => eprintln!("[NicProp] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

pub fn check_nic_property(property: &str, expected: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut found_any = false;
    for path in nic_subkey_paths() {
        if let Ok(k) = hklm.open_subkey_with_flags(&path, KEY_READ) {
            if let Ok(val) = k.get_value::<String, _>(property) {
                found_any = true;
                if val.to_lowercase() != expected.to_lowercase() {
                    return false;
                }
            }
        }
    }
    found_any
}
