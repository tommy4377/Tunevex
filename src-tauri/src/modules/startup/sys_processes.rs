use crate::modules::startup::types::StartupItem;
use std::collections::HashSet;

pub const WINDOWS_SYSTEM_PROCESS_NAMES: &[&str] = &[
    "smss.exe",
    "csrss.exe",
    "wininit.exe",
    "services.exe",
    "lsass.exe",
    "winlogon.exe",
    "System",
    "svchost.exe",
    "dwm.exe",
    "explorer.exe",
    "conhost.exe",
    "taskhostw.exe",
    "RuntimeBroker.exe",
    "ShellExperienceHost.exe",
    "ctfmon.exe",
    "fontdrvhost.exe",
    "sihost.exe",
    "vasatcservice.exe",
    "searchindexer.exe",
    "searchhost.exe",
    "SecurityHealthService.exe",
    "MsMpEng.exe",
    "NisSrv.exe",
    "MpDefenderCoreService.exe",
    "spoolsv.exe",
    "WmiPrvSE.exe",
    "dllhost.exe",
    "ft.exe",
    "sedsvc.exe",
    "Registry",
    "Memory Compression",
    "SecurityHealthService",
    "MsMpEng",
    "NisSrv",
    "MpDefenderCoreService",
];

pub const WINDOWS_CRITICAL_SERVICE_NAMES: &[&str] = &[
    "RpcSs",
    "WinDefend",
    "WdBoot",
    "WdFilter",
    "WdNisSvc",
    "Dnscache",
    "Dhcp",
    "TermService",
    "Spooler",
    "AudioSrv",
    "AudioEndpointBuilder",
    "lanmanworkstation",
    "lanmanserver",
    "EventLog",
    "Schedule",
    "SENS",
    "ProfSvc",
    "Themes",
    "W32Time",
    "Browser",
    "BITS",
    "Winmgmt",
    "iphlpsvc",
    "Netlogon",
    "NlaSvc",
    "SamSs",
    "Kdc",
    "RpcEptMapper",
    "DcomSvc",
    "PlugPlay",
    "Power",
    "Wcmsvc",
    "WdiServiceHost",
    "WdiSystemHost",
    "SecurityHealthService",
    "TrustedInstaller",
    "wuauserv",
    "msiserver",
    "UdkUserSvc",
    "UserManager",
    "ShavlikProviderSvc",
    "wscsvc",
    "WinDefend",
    "wscsvc",
    "SecurityCenter",
    "IKEEXT",
    "KeyIso",
    "Netlogon",
    "Ntmssvc",
    "Power",
    "ProfSvc",
    "RpcSs",
    "SamSs",
    "Schedule",
    "SENS",
    "SessionEnv",
    "ShellHWDetection",
    " themes",
    "Themes",
    "WinMgmt",
    "wuauserv",
    "wudfsvc",
    "WwanSvc",
];

pub const WINDOWS_SYSTEM_TASK_PREFIXES: &[&str] = &[
    r"\Microsoft\Windows\TaskScheduler",
    r"\Microsoft\Windows\SystemRestore",
    r"\Microsoft\Windows\WindowsUpdate",
    r"\Microsoft\Windows\Security",
    r"\Microsoft\Windows\DiskDiagnostic",
    r"\Microsoft\Windows\MemoryDiagnostic",
    r"\Microsoft\Windows\DataDiscovery",
    r"\Microsoft\Windows\Defrag",
    r"\Microsoft\Windows\PI",
    r"\Microsoft\Windows\Time Synchronization",
    r"\Microsoft\Windows\Windows Filtering Platform",
    r"\Microsoft\Windows\Autochk",
    r"\Microsoft\Windows\Backup",
    r"\Microsoft\Windows\Boot",
    r"\Microsoft\Windows\CertificateServicesClient",
    r"\Microsoft\Windows\Chkdsk",
    r"\Microsoft\Windows\Clip",
    r"\Microsoft\Windows\CloudExperienceHost",
    r"\Microsoft\Windows\Customer Experience Improvement Program",
    r"\Microsoft\Windows\Data Integrity Scan",
    r"\Microsoft\Windows\Diagnosis",
    r"\Microsoft\Windows\DiskFootprint",
    r"\Microsoft\Windows\DiskCleanup",
    r"\Microsoft\Windows\ESENT",
    r"\Microsoft\Windows\FileClassification",
    r"\Microsoft\Windows\FileHistory",
    r"\Microsoft\Windows\Help",
    r"\Microsoft\Windows\Input",
    r"\Microsoft\Windows\International",
    r"\Microsoft\Windows\Location",
    r"\Microsoft\Windows\Management",
    r"\Microsoft\Windows\Maps",
    r"\Microsoft\Windows\Mobile Broadband",
    r"\Microsoft\Windows\Multimedia",
    r"\Microsoft\Windows\NetTrace",
    r"\Microsoft\Windows\Network",
    r"\Microsoft\Windows\Not",
    r"\Microsoft\Windows\Oobe",
    r"\Microsoft\Windows\Performance",
    r"\Microsoft\Windows\Print",
    r"\Microsoft\Windows\Printing",
    r"\Microsoft\Windows\Proximity",
    r"\Microsoft\Windows\Ras",
    r"\Microsoft\Windows\Rdx",
    r"\Microsoft\Windows\Recovery",
    r"\Microsoft\Windows\RemoteAccess",
    r"\Microsoft\Windows\RemoteBox",
    r"\Microsoft\Windows\Reset",
    r"\Microsoft\Windows\ResourceManager",
    r"\Microsoft\Windows\Server",
    r"\Microsoft\Windows\Setup",
    r"\Microsoft\Windows\Sharing",
    r"\Microsoft\Windows\Shell",
    r"\Microsoft\Windows\SoftwareProtectionPlatform",
    r"\Microsoft\Windows\StateRepository",
    r"\Microsoft\Windows\Storage",
    r"\Microsoft\Windows\Sysprep",
    r"\Microsoft\Windows\System",
    r"\Microsoft\Windows\Taxonomy",
    r"\Microsoft\Windows\Tcpip",
    r"\Microsoft\Windows\Telemetry",
    r"\Microsoft\Windows\Trustedinstaller",
    r"\Microsoft\Windows\UPnP",
    r"\Microsoft\Windows\Update",
    r"\Microsoft\Windows\WaaS",
    r"\Microsoft\Windows\WDI",
    r"\Microsoft\Windows\Web",
    r"\Microsoft\Windows\Windows",
    r"\Microsoft\Windows\WindowsErrorReporting",
    r"\Microsoft\Windows\WindowsMedia",
    r"\Microsoft\Windows\Wininet",
    r"\Microsoft\Windows\Wired",
    r"\Microsoft\Windows\WLAN",
    r"\Microsoft\Windows\Work Folders",
    r"\Microsoft\Windows\WwanSvc",
];

fn process_names_set() -> HashSet<&'static str> {
    WINDOWS_SYSTEM_PROCESS_NAMES.iter().copied().collect()
}

fn service_names_set() -> HashSet<&'static str> {
    WINDOWS_CRITICAL_SERVICE_NAMES.iter().copied().collect()
}

pub fn is_windows_system_process(name_or_path: &str) -> bool {
    let lower = name_or_path.to_lowercase();
    let proc_set = process_names_set();
    proc_set.contains(lower.as_str())
        || lower.ends_with("\\system32\\svchost.exe")
        || lower.contains("\\services.exe")
        || lower.contains("\\lsass.exe")
        || lower.contains("\\csrss.exe")
        || lower.contains("\\smss.exe")
        || lower.contains("\\wininit.exe")
        || lower.contains("\\winlogon.exe")
        || lower.contains("\\dwm.exe")
        || lower.contains("\\conhost.exe")
}

pub fn is_critical_service_name(service_name: &str) -> bool {
    let lower = service_name.to_lowercase();
    let svc_set = service_names_set();
    svc_set.contains(lower.as_str())
        || lower == "rpcss"
        || lower == "lsass"
        || lower == "services"
        || lower == "smss"
        || lower == "wininit"
        || lower == "csrss"
        || lower == "winlogon"
        || lower == "dcomsvc"
        || lower == "plugplay"
}

pub fn is_windows_system_task(task_path: &str) -> bool {
    let lower = task_path.to_lowercase();
    for prefix in WINDOWS_SYSTEM_TASK_PREFIXES {
        if lower.starts_with(&prefix.to_lowercase()) {
            return true;
        }
    }
    false
}

pub fn is_windows_system_item(item: &StartupItem) -> bool {
    let name_lower = item.name.to_lowercase();
    let cmd_lower = item.command.to_lowercase();

    match item.source {
        crate::modules::startup::types::AutostartSource::Service => {
            let svc_name = item.id.strip_prefix("SVC:").unwrap_or(&item.id);
            is_critical_service_name(svc_name)
                || is_critical_service_name(&item.name)
                || is_critical_service_name(&item.subcategory)
        }
        crate::modules::startup::types::AutostartSource::TaskScheduler => {
            is_windows_system_task(&item.id.strip_prefix("TASK|").unwrap_or(&item.id))
                || is_windows_system_task(&item.name)
        }
        _ => {
            if is_windows_system_process(&name_lower) {
                return true;
            }
            if is_windows_system_process(&cmd_lower) {
                return true;
            }
            let exe_name = cmd_lower
                .split_whitespace()
                .next()
                .unwrap_or(&cmd_lower)
                .split('\\')
                .last()
                .unwrap_or(&cmd_lower);
            is_windows_system_process(exe_name) || is_windows_system_process(&name_lower)
        }
    }
}
