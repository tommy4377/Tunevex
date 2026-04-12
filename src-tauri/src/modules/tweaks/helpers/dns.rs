pub const TCPIP_INTERFACES_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";

pub fn tcpip_interface_subkeys() -> Vec<String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let parent = match hklm.open_subkey(TCPIP_INTERFACES_PATH) {
        Ok(k) => k,
        Err(_) => return Vec::new(),
    };
    parent
        .enum_keys()
        .flatten()
        .map(|guid| format!("{}\\{}", TCPIP_INTERFACES_PATH, guid))
        .collect()
}

pub fn set_dns_servers(primary: &str, secondary: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let value = format!("{},{}", primary, secondary);
    for path in tcpip_interface_subkeys() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE) {
            Ok(k) => {
                if let Err(e) = k.set_value("NameServer", &value) {
                    eprintln!("[DNS] Warning: set NameServer on {} failed: {}", path, e);
                }
            }
            Err(e) => eprintln!("[DNS] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

pub fn reset_dns_servers() -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let empty = String::new();
    for path in tcpip_interface_subkeys() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE) {
            Ok(k) => {
                if let Err(e) = k.set_value("NameServer", &empty) {
                    eprintln!("[DNS] Warning: clear NameServer on {} failed: {}", path, e);
                }
            }
            Err(e) => eprintln!("[DNS] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

pub fn check_dns_servers_contain(ip: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for path in tcpip_interface_subkeys() {
        if let Ok(k) = hklm.open_subkey_with_flags(&path, KEY_READ) {
            if let Ok(val) = k.get_value::<String, _>("NameServer") {
                if val.split(',').any(|s| s.trim() == ip) {
                    return true;
                }
            }
        }
    }
    false
}
