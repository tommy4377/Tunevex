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
            if k.get_value::<String, _>("DriverDesc").is_ok()
                && k.get_value::<u32, _>("Characteristics").is_ok_and(|flags| flags & 4 != 0) {
                paths.push(sub_path);
            }
        }
    }
    paths
}

pub fn set_nic_property(owner: &str, property: &str, value: &str) -> Result<(), String> {
    use winreg::{enums::*, RegKey, types::ToRegValue};
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut values = Vec::new();
    for path in nic_subkey_paths() {
        let key = root.open_subkey(&path).map_err(|e| e.to_string())?;
        match key.get_value::<String, _>(property) {
            Ok(_) => values.push((path, property.to_string(), value.to_string().to_reg_value())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    super::device_backup::set_values(owner, values)
}

pub fn check_nic_property(property: &str, expected: &str) -> bool {
    query_nic_property(property, expected) == Some(true)
}

pub fn query_nic_property(property: &str, expected: &str) -> Option<bool> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut found_any = false;
    for path in nic_subkey_paths() {
        let k = hklm.open_subkey_with_flags(&path, KEY_READ).ok()?;
        match k.get_value::<String, _>(property) {
            Ok(val) => {
                found_any = true;
                if val.to_lowercase() != expected.to_lowercase() {
                    return Some(false);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {},
            Err(_) => return None,
        }
    }
    found_any.then_some(true)
}
