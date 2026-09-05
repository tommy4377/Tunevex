use winreg::{enums::*, RegKey, RegValue};

const CLASSES: &[&str] = &["Display", "SCSIAdapter", "Net", "USB", "HDC"];
const MSI: &str = r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties";
const AFFINITY: &str = r"Device Parameters\Interrupt Management\Affinity Policy";

fn class_matches(class: &str, guid: &str, service: &str) -> bool {
    let expected = match class {
        "Display" => "{4d36e968-e325-11ce-bfc1-08002be10318}",
        "SCSIAdapter" | "NVMe" => "{4d36e97b-e325-11ce-bfc1-08002be10318}",
        "Net" => "{4d36e972-e325-11ce-bfc1-08002be10318}",
        "USB" => "{36fc9e60-c465-11cf-8056-444553540000}",
        "HDC" => "{4d36e96a-e325-11ce-bfc1-08002be10318}",
        _ => return false,
    };
    guid.eq_ignore_ascii_case(expected)
        && (class != "NVMe" || service.eq_ignore_ascii_case("stornvme"))
}

fn device_paths(class: &str) -> Result<Vec<String>, String> {
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let pci = root
        .open_subkey(r"SYSTEM\CurrentControlSet\Enum\PCI")
        .map_err(|e| e.to_string())?;
    let mut paths = Vec::new();
    for hardware in pci.enum_keys() {
        let hardware = hardware.map_err(|e| e.to_string())?;
        let key = pci.open_subkey(&hardware).map_err(|e| e.to_string())?;
        for instance in key.enum_keys() {
            let instance = instance.map_err(|e| e.to_string())?;
            let device = key.open_subkey(&instance).map_err(|e| e.to_string())?;
            let guid: String = device.get_value("ClassGUID").unwrap_or_default();
            let service: String = device.get_value("Service").unwrap_or_default();
            let matches = class_matches(class, &guid, &service);
            // Only touch devices whose drivers expose an MSI setting.
            if matches
                && device
                    .open_subkey(MSI)
                    .ok()
                    .and_then(|k| k.get_value::<u32, _>("MSISupported").ok())
                    .is_some()
            {
                paths.push(format!(
                    r"SYSTEM\CurrentControlSet\Enum\PCI\{hardware}\{instance}"
                ));
            }
        }
    }
    Ok(paths)
}

fn values_match(msi: u32, priority: Option<u32>, expected: u32) -> bool {
    msi == 1 && priority.unwrap_or(0) == expected
}

pub fn check_msi_enabled_for_class(class: &str, priority: u32) -> bool {
    let Ok(paths) = device_paths(class) else {
        return false;
    };
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    !paths.is_empty()
        && paths.iter().all(|path| {
            let msi = root
                .open_subkey(format!(r"{path}\{MSI}"))
                .ok()
                .and_then(|k| k.get_value::<u32, _>("MSISupported").ok())
                .unwrap_or(0);
            let actual = root
                .open_subkey(format!(r"{path}\{AFFINITY}"))
                .ok()
                .and_then(|k| k.get_value::<u32, _>("DevicePriority").ok());
            values_match(msi, actual, priority)
        })
}

pub fn query_msi_state(class: &str, priority: u32) -> Option<bool> {
    if device_paths(class).ok()?.is_empty() {
        return None;
    }
    Some(check_msi_enabled_for_class(class, priority))
}

pub fn check_msi_enabled_globally(priority: u32) -> bool {
    let mut found = false;
    for class in CLASSES {
        let Ok(paths) = device_paths(class) else {
            return false;
        };
        if !paths.is_empty() {
            found = true;
            if !check_msi_enabled_for_class(class, priority) {
                return false;
            }
        }
    }
    found
}

pub fn check_msi_enabled_on_net(priority: u32) -> bool {
    check_msi_enabled_for_class("Net", priority)
}

pub fn apply_msi_set(owner: &str, class: &str, priority: u32) -> Result<(), String> {
    if owner == "system_msi_global_safe"
        && class == "Display"
        && !CLASSES
            .iter()
            .any(|c| device_paths(c).is_ok_and(|p| !p.is_empty()))
    {
        return Err("No supported PCI devices expose MSI settings".into());
    }
    let paths = device_paths(class)?;
    if paths.is_empty() && owner == "system_msi_global_safe" {
        return Ok(());
    }
    let values = paths
        .into_iter()
        .flat_map(|path| {
            [
                (
                    format!(r"{path}\{MSI}"),
                    "MSISupported".into(),
                    RegValue {
                        bytes: 1u32.to_le_bytes().to_vec(),
                        vtype: REG_DWORD,
                    },
                ),
                (
                    format!(r"{path}\{AFFINITY}"),
                    "DevicePriority".into(),
                    RegValue {
                        bytes: priority.to_le_bytes().to_vec(),
                        vtype: REG_DWORD,
                    },
                ),
            ]
        })
        .collect();
    // Keep the driver's MessageNumberLimit intact.
    super::device_backup::set_values(owner, values)
}

pub fn apply_msi_remove(owner: &str) -> Result<(), String> {
    super::device_backup::restore(owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn msi_detection_checks_value_and_priority() {
        assert!(!values_match(0, Some(3), 3));
        assert!(!values_match(1, Some(1), 3));
        assert!(values_match(1, Some(3), 3));
        assert!(values_match(1, None, 0));
        assert!(!values_match(1, None, 3));
    }
    #[test]
    fn device_classes_use_guids_and_nvme_does_not_include_sata() {
        assert!(class_matches(
            "Display",
            "{4D36E968-E325-11CE-BFC1-08002BE10318}",
            "gpu"
        ));
        assert!(!class_matches(
            "USB",
            "{4d36e968-e325-11ce-bfc1-08002be10318}",
            "gpu"
        ));
        assert!(class_matches(
            "NVMe",
            "{4d36e97b-e325-11ce-bfc1-08002be10318}",
            "stornvme"
        ));
        assert!(!class_matches(
            "NVMe",
            "{4d36e97b-e325-11ce-bfc1-08002be10318}",
            "storahci"
        ));
    }
}
