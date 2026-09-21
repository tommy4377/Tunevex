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
        Ok(out) if out.status.success() => {
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
            .to_lowercase();
            combined.contains(&contains.to_lowercase())
        }
        _ => false,
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
    matches!(RegKey::predef(hkey).open_subkey(path), Err(e) if e.kind() == std::io::ErrorKind::NotFound)
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

pub fn query_registry_value(
    root: &str,
    path: &str,
    name: &str,
    expected: &RegistryValue,
) -> Option<bool> {
    let key = crate::modules::registry::operations::get_root_key(root);
    match key.open_subkey(path) {
        Ok(key) => match key.get_raw_value(name) {
            Ok(_) => Some(check_registry_value(root, path, name, expected)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Some(false),
            Err(_) => None,
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Some(false),
        Err(_) => None,
    }
}

pub fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    query_powershell(script)
        .is_some_and(|output| output.eq_ignore_ascii_case(expected_output.trim()))
}

pub fn query_powershell(script: &str) -> Option<String> {
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
        Err(_) => return None,
    };
    if let Some(mut stdin) = child.stdin.take() {
        if writeln!(
            stdin,
            "& {{ $ErrorActionPreference = 'Stop';\n{}\n}}\n",
            script
        )
        .is_err()
        {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
    }
    match child.wait_with_output() {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            Some(stdout)
        }
        _ => None,
    }
}

pub fn tcp_global_matches(output: &str, settings: &[(String, String)]) -> bool {
    !settings.is_empty()
        && settings.iter().all(|(key, value)| {
            output
                .lines()
                .filter(|line| line.trim_start().starts_with("set global "))
                .flat_map(str::split_whitespace)
                .any(|token| {
                    token.split_once('=').is_some_and(|(k, v)| {
                        k.eq_ignore_ascii_case(key) && v.eq_ignore_ascii_case(value)
                    })
                })
        })
}

pub fn check_tcp_global(settings: &[(String, String)]) -> bool {
    Command::new("netsh")
        .args(["interface", "tcp", "dump"])
        .creation_flags(0x08000000)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .is_some_and(|out| tcp_global_matches(&String::from_utf8_lossy(&out.stdout), settings))
}

pub fn check_scheduled_task_disabled(name: &str) -> bool {
    query_scheduled_task_disabled(name) == Some(true)
}

pub fn query_scheduled_task_disabled(name: &str) -> Option<bool> {
    // Task Scheduler's COM Enabled property is independent of output language,
    // task readiness, and unrelated settings such as idle or battery conditions.
    let name = name.replace('\'', "''");
    query_powershell(&format!(
        "$s = New-Object -ComObject 'Schedule.Service'; $s.Connect(); -not $s.GetFolder('\\').GetTask('{name}').Enabled"
    )).and_then(|s| match s.as_str() { "true" => Some(true), "false" => Some(false), _ => None })
}

#[cfg(test)]
mod detector_tests {
    use super::*;

    #[test]
    fn tcp_checks_match_only_the_requested_setting() {
        let dump = "# enabled\nset global rss=enabled ecncapability=disabled timestamps=allowed fastopen=enabled fastopenfallback=disabled";
        assert!(!tcp_global_matches(
            dump,
            &[("ecncapability".into(), "enabled".into())]
        ));
        assert!(!tcp_global_matches(
            dump,
            &[("timestamps".into(), "enabled".into())]
        ));
        assert!(tcp_global_matches(
            dump,
            &[("rss".into(), "enabled".into())]
        ));
        assert!(!tcp_global_matches(
            dump,
            &[
                ("fastopen".into(), "enabled".into()),
                ("fastopenfallback".into(), "enabled".into())
            ]
        ));
        assert!(!tcp_global_matches("", &[("rss".into(), "enabled".into())]));
    }
    #[test]
    fn powershell_requires_success_and_exact_output() {
        assert!(check_powershell_output("'True'", "True"));
        assert!(!check_powershell_output("'False'", "True"));
        assert!(!check_powershell_output("'True'; throw 'failed'", "True"));
        assert!(!check_powershell_output("'not True'", "True"));
    }
    #[test]
    fn missing_task_is_unknown_not_disabled() {
        assert_eq!(
            query_scheduled_task_disabled(r"\Tunevex-Nonexistent-Detector-Test"),
            None
        );
    }
}
