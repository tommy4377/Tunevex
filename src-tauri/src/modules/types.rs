use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WarningLevel {
    #[default]
    Safe, // Green - No risks
    Careful,   // Yellow - Minor compatibility risks
    Dangerous, // Red - Security/stability risks
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TweakCategory {
    #[default]
    Network,
    CpuPerformance,
    GpuOptimization,
    MouseInput,
    DisplayMonitor,
    DebloatTelemetry,
    StartupServices,
    FileSystem,
    SecurityPrivacy,
    InterfaceUx,
    System,
    GameOptimizations,
    Hardware,
    Advanced,
    Monitoring,
    BackupRestore,
    Privacy,
    Activation, // Windows/Office activation
    Home,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistryValue {
    String(String),
    DWord(u32),
    QWord(u64),
    Binary(Vec<u8>),
    MultiString(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOp {
    Delete { path: String },
    Copy { src: String, dest: String },
    Move { src: String, dest: String },
    Write { path: String, content: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TweakOperation {
    RegistrySet {
        root_key: String, // HKLM, HKCU, etc.
        path: String,
        key: String,
        value: RegistryValue,
    },
    RegistryDelete {
        root_key: String,
        path: String,
        key: String,
    },
    ServiceDisable {
        name: String,
    },
    ServiceSetMode {
        name: String,
        mode: String, // "Auto", "Manual", "Disabled"
    },
    ScheduledTaskDisable {
        path: String, // Task path usually
        name: String,
    },
    ScheduledTaskEnable {
        path: String,
        name: String,
    },
    FileOperation(FileOp),
    Command {
        cmd: String,
        args: Vec<String>,
    },
    Powershell {
        script: String,
    },
    /// Write a named property (as a DWORD string value) into every physical
    /// NIC subkey under
    ///   HKLM\\SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}\\XXXX
    /// Subkeys that don't have a \"DriverDesc\" value (i.e. are not real adapters)
    /// are skipped automatically.
    NetAdapterProperty {
        /// The registry value name, e.g. \"*FlowControl\", \"*LsoV2IPv4\"
        property: String,
        /// The value to write as a REG_SZ (NIC advanced properties use strings)
        value: String,
    },
    /// Set static DNS servers on every active network interface by writing to
    ///   HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}\NameServer
    /// Uses pure winreg — no PowerShell or netsh required.
    SetDnsServers {
        primary: String,
        secondary: String,
    },
    /// Clear static DNS on every active network interface (restores DHCP DNS).
    /// Sets NameServer = "" on all Tcpip\Parameters\Interfaces\{GUID} subkeys.
    ResetDnsServers,
    /// Stop or start + set startup-type of one or more Defender-related services
    /// using sc.exe (no PowerShell).
    /// action: "disable" → sc stop + sc config start= disabled
    /// action: "enable"  → sc config start= auto + sc start
    DefenderServiceControl {
        services: Vec<String>,
        action: String, // "enable" | "disable"
    },
    /// Add or remove a path from the Defender exclusion list by writing to
    ///   HKLM\\SOFTWARE\\Microsoft\\Windows Defender\\Exclusions\\Paths
    /// action: "add" → creates REG_DWORD entry with value 0
    /// action: "remove" → deletes the entry
    DefenderExclusion {
        paths: Vec<String>,
        action: String, // "add" | "remove"
    },
    /// Enumerate every subkey under HKLM\SYSTEM\CurrentControlSet\Services that
    /// does NOT match "Xbl|Xbox", and either:
    ///   enable_split = false → set SvcHostSplitDisable = REG_DWORD 1
    ///   enable_split = true  → delete SvcHostSplitDisable (restores default)
    /// Only touches subkeys that already have a "Start" value (real services).
    SvcHostSplitAll {
        /// false = disable splitting (set value 1); true = restore splitting (delete value)
        enable_split: bool,
    },
    /// Enable MSI mode on all devices of a given PCI class (Display, SCSIAdapter, Net, USB, HDC).
    /// Writes MSISupported=1, MessageNumberLimit=1, Priority=priority to registry.
    MsiSet {
        class: String,
        priority: u32,
    },
    /// Remove MSI settings from all devices of a given PCI class.
    /// Deletes MSISupported, MessageNumberLimit, Priority registry values.
    MsiRemove {
        class: String,
    },
    /// Enable MSI mode on all network adapters (Net class) at specified priority.
    /// Used by network/msi.rs - writes to PCI\Net subkeys.
    MsiSetNet {
        priority: u32,
    },
    /// Remove MSI settings from all network adapters.
    MsiRemoveNet,
    /// Set a registry value on all TCP/IP network interfaces.
    /// Iterates HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces
    /// and sets the specified key/value on each interface.
    NetworkInterfacesSet {
        key: String,
        value: RegistryValue,
    },
    /// Delete a registry value from all TCP/IP network interfaces.
    NetworkInterfacesDelete {
        key: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TweakCheck {
    Registry {
        root_key: String,
        path: String,
        key: String,
        #[serde(rename = "value")]
        expected_value: RegistryValue,
    },
    Powershell {
        script: String,
        expected_output: String, // \"True\", \"Enabled\", \"1\", etc.
    },
    /// Check that every physical NIC subkey has the given property set to the
    /// given value.  Returns true only when ALL adapter subkeys agree.
    NetAdapterProperty {
        property: String,
        expected_value: String,
    },
    /// Returns true when at least one network interface's static NameServer
    /// registry value contains the specified IP address.
    /// Path: HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}\NameServer
    DnsServersContain { ip: String },
    /// Returns true only when ALL of the listed registry key/value pairs match.
    /// Each entry is (root_key, path, key, expected_value).
    MultiRegistry { checks: Vec<RegistryCheck> },
    /// Reads Windows Defender status natively from the registry (no PowerShell).
    ///
    /// Checks performed (all must be satisfied for `enabled = true`):
    ///  1. Tamper Protection is OFF:
    ///       HKLM\SOFTWARE\Microsoft\Windows Defender\Features\TamperProtection != 5
    ///  2. The specified `check` variant:
    ///     - "realtime_disabled" → DisableRealtimeMonitoring == 1 in policy key
    ///     - "av_disabled"       → DisableAntiSpyware == 1 AND DisableAntiVirus == 1 in policy key
    MpComputerStatus {
        /// "realtime_disabled" | "av_disabled"
        check: String,
    },
    /// Returns true if ALL paths in `paths` appear as value names under
    ///   HKLM\\SOFTWARE\\Microsoft\\Windows Defender\\Exclusions\\Paths
    DefenderExclusionPath { paths: Vec<String> },
    /// Run an arbitrary binary and return true if its combined stdout+stderr
    /// output contains the given substring (case-insensitive).
    /// Used to check bcdedit /enum, powercfg /q, and similar CLI tools
    /// without spawning PowerShell.
    CommandOutputContains {
        cmd: String,
        args: Vec<String>,
        contains: String,
    },
    /// Returns true when the specified registry key path does NOT exist at all.
    /// Useful for checking that a device/setting has been removed.
    RegistryKeyAbsent { root_key: String, path: String },
    /// Returns true if the specified scheduled task is disabled
    ScheduledTaskDisabled { name: String },
    /// Returns true if the specified service is disabled
    ServiceDisabled { name: String },
    /// Returns true if the specified service is set to the expected startup mode
    /// (auto, manual, demand, disabled, etc.)
    ServiceMode { name: String, mode: String },
    /// Returns true if all specified services are disabled
    MultiServiceDisabled { names: Vec<String> },
    /// Check if MSI is enabled globally for all PCI device classes at the specified priority.
    MsiEnabledGlobally { priority: u32 },
    /// Check if MSI is enabled on all network adapters at the specified priority.
    MsiEnabledOnNet { priority: u32 },
    /// Check if a registry value exists on ALL network interfaces with the expected value.
    /// Returns true only when ALL interfaces have the value.
    NetworkInterfacesCheck {
        key: String,
        expected_value: RegistryValue,
    },
}

/// A single registry check used inside TweakCheck::MultiRegistry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryCheck {
    pub root_key: String,
    pub path: String,
    pub key: String,
    pub expected_value: RegistryValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum TweakType {
    #[default]
    Toggle, // Switch (On/Off)
    Action, // Button (Run Immediately)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tweak {
    pub id: String,
    pub category: TweakCategory,
    pub name: String,
    pub description: String,
    pub warning_level: WarningLevel,
    #[serde(default)]
    pub tweak_type: TweakType, // Defaults to Toggle
    pub operations: Vec<TweakOperation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_operations: Option<Vec<TweakOperation>>, // Operations to undo the tweak (optional)
    pub check: Option<TweakCheck>, // Defines how to check if enabled
    pub requires_restart: bool,
    #[serde(default)]
    pub enabled: bool,
}

impl Tweak {
    /// Create a new Tweak with default revert_operations (None)
    pub fn new(
        id: impl Into<String>,
        category: TweakCategory,
        name: impl Into<String>,
        description: impl Into<String>,
        warning_level: WarningLevel,
        requires_restart: bool,
        check: Option<TweakCheck>,
        operations: Vec<TweakOperation>,
    ) -> Self {
        Self {
            id: id.into(),
            category,
            name: name.into(),
            description: description.into(),
            warning_level,
            tweak_type: TweakType::Toggle,
            operations,
            revert_operations: None,
            check,
            requires_restart,
            enabled: false,
        }
    }
}
