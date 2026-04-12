pub fn check_msi_enabled_globally(priority: u32) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    const CLASSES: &[&str] = &["Display", "SCSIAdapter", "Net", "USB", "HDC"];

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let _pci_key = match hklm.open_subkey_with_flags(PCI_PATH, KEY_READ) {
        Ok(k) => k,
        Err(_) => return false,
    };

    for class in CLASSES {
        let class_key_path = format!("{}\\{}", PCI_PATH, class);
        let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
            Ok(k) => k,
            Err(_) => continue,
        };

        for dev_name in class_key.enum_keys().flatten() {
            let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
            let _dev_key = match hklm.open_subkey_with_flags(&dev_path, KEY_READ) {
                Ok(k) => k,
                Err(_) => continue,
            };

            let msi_path = format!(
                "{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
                dev_path
            );
            let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_READ) {
                Ok(k) => k,
                Err(_) => return false,
            };

            let msi_supported: u32 = match msi_key.get_value("MSISupported") {
                Ok(v) => v,
                Err(_) => return false,
            };
            let msg_limit: u32 = match msi_key.get_value("MessageNumberLimit") {
                Ok(v) => v,
                Err(_) => return false,
            };
            let prio: u32 = match msi_key.get_value("Priority") {
                Ok(v) => v,
                Err(_) => return false,
            };

            if msi_supported != 1 || msg_limit != 1 || prio != priority {
                return false;
            }
        }
    }
    true
}

pub fn apply_msi_set(class: &str, priority: u32) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    let class_key_path = format!("{}\\{}", PCI_PATH, class);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => return Ok(()),
    };

    for dev_name in class_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
        let base_path = format!("{}\\Device Parameters\\Interrupt Management", dev_path);
        let msi_path = format!("{}\\{}", base_path, "MessageSignaledInterruptProperties");

        if hklm.create_subkey(&base_path).is_err() {
            continue;
        }
        if hklm.create_subkey(&msi_path).is_err() {
            continue;
        }

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let _ = msi_key.set_value("MSISupported", &1u32);
        let _ = msi_key.set_value("MessageNumberLimit", &1u32);
        let _ = msi_key.set_value("Priority", &priority);
    }
    Ok(())
}

pub fn apply_msi_remove(class: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    let class_key_path = format!("{}\\{}", PCI_PATH, class);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => return Ok(()),
    };

    for dev_name in class_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
        let msi_path = format!(
            "{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
            dev_path
        );

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let _ = msi_key.delete_value("MSISupported");
        let _ = msi_key.delete_value("MessageNumberLimit");
        let _ = msi_key.delete_value("Priority");
    }
    Ok(())
}

pub fn check_msi_enabled_on_net(priority: u32) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI\\Net";

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let net_key = match hklm.open_subkey_with_flags(PCI_PATH, KEY_READ) {
        Ok(k) => k,
        Err(_) => return false,
    };

    for dev_name in net_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}", PCI_PATH, dev_name);
        let msi_path = format!(
            "{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
            dev_path
        );

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_READ) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let msi_supported: u32 = match msi_key.get_value("MSISupported") {
            Ok(v) => v,
            Err(_) => return false,
        };
        let msg_limit: u32 = match msi_key.get_value("MessageNumberLimit") {
            Ok(v) => v,
            Err(_) => return false,
        };
        let prio: u32 = match msi_key.get_value("Priority") {
            Ok(v) => v,
            Err(_) => return false,
        };

        if msi_supported != 1 || msg_limit != 1 || prio != priority {
            return false;
        }
    }
    true
}
