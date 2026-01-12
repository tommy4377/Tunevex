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
    Privacy, // Phase 4: Privacy & Telemetry
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    FileOperation(FileOp),
    Command {
        cmd: String,
        args: Vec<String>,
    },
    Powershell {
        script: String,
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
        expected_output: String, // "True", "Enabled", "1", etc.
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tweak {
    pub id: String,
    pub category: TweakCategory,
    pub name: String,
    pub description: String,
    pub warning_level: WarningLevel,
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
            operations,
            revert_operations: None,
            check,
            requires_restart,
            enabled: false,
        }
    }
}
