use crate::modules::types::RegistryValue;
use crate::modules::utils::security::validate_command;
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn check_command_output_contains(cmd: &str, args: &[String], contains: &str) -> bool {
    if validate_command(cmd).is_err() {
        eprintln!("[check] Blocked non-whitelisted command: '{}'", cmd);
        return false;
    }
    let output = Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .output();
    match output {
        Ok(out) => {
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
            .to_lowercase();
            combined.contains(&contains.to_lowercase())
        }
        Err(_) => false,
    }
}

pub fn check_registry_key_absent(root_key: &str, path: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hkey = match root_key.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        _ => return false,
    };
    RegKey::predef(hkey).open_subkey(path).is_err()
}

pub fn check_registry_value(
    root_key: &str,
    path: &str,
    key: &str,
    expected_value: &RegistryValue,
) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match root_key.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        _ => return false,
    };

    let regkey = match RegKey::predef(hkey).open_subkey(path) {
        Ok(k) => k,
        Err(_) => return false,
    };

    match expected_value {
        RegistryValue::DWord(expected) => regkey
            .get_value::<u32, _>(key)
            .map(|v| v == *expected)
            .unwrap_or(false),
        RegistryValue::QWord(expected) => regkey
            .get_value::<u64, _>(key)
            .map(|v| v == *expected)
            .unwrap_or(false),
        RegistryValue::String(expected) => regkey
            .get_value::<String, _>(key)
            .map(|v| v == *expected)
            .unwrap_or(false),
        RegistryValue::Binary(expected) => regkey
            .get_raw_value(key)
            .map(|v| v.bytes == *expected)
            .unwrap_or(false),
        RegistryValue::MultiString(expected) => regkey
            .get_value::<Vec<String>, _>(key)
            .map(|v| v == *expected)
            .unwrap_or(false),
    }
}

pub fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "RemoteSigned",
            "-Command",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", script);
    }
    match child.wait_with_output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            stdout == expected_output.trim().to_lowercase()
        }
        Err(_) => false,
    }
}
