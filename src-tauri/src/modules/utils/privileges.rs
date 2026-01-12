#[allow(unused_imports)]
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn is_admin() -> bool {
    // fast and simple way to check admin rights on windows
    // by trying to open the physical drive or using `net session`
    // `net session` requires admin, returns 0 if admin, >0 if not.
    let output = Command::new("net").arg("session").output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

// Function to relaunch app as admin?
// Tauri normally handles this via manifest, but if we need to request it explicitly:
// We can use the 'runas' verb with ShellExecute, but mostly we rely on the user running the app as admin.
// This function is mostly for UI feedback ("Please restart as Admin").
