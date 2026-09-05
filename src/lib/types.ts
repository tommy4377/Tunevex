export type WarningLevel = 'Safe' | 'Careful' | 'Dangerous';

export type TweakCategory =
    | 'Network'
    | 'CpuPerformance'
    | 'GpuOptimization'
    | 'MouseInput'
    | 'Input'
    | 'DisplayMonitor'
    | 'DebloatTelemetry'
    | 'StartupServices'
    | 'FileSystem'
    | 'SecurityPrivacy'
    | 'InterfaceUx'
    | 'System'
    | 'GameOptimizations'
    | 'Hardware'
    | 'Advanced'
    | 'Monitoring'
    | 'BackupRestore'
    | 'Privacy'
    | 'Programs'
    | 'Restore'
    | 'Activation'
    | 'Home'
    | 'AiAdvisor'
    | 'AllTweaks';

export type TweakType = 'Toggle' | 'Action';

export interface Tweak {
    id: string;
    category: TweakCategory;
    name: string;
    description: string;
    warning_level: WarningLevel;
    tweak_type: TweakType;
    requires_restart: boolean;
    enabled?: boolean | null; // null/undefined = not yet detected
    check?: TweakCheck;
    revert_warning?: string; // Shown when reverting a Dangerous tweak
}

export type ProfileOperation = 'enable' | 'disable' | 'run' | 'skip';

export interface ProfilePreviewEntry {
    id: string;
    name: string;
    category: string;
    tweak_type: TweakType;
    operation: ProfileOperation;
    current_enabled: boolean | null;
    can_revert: boolean;
    queryable: boolean;
    warning_level: WarningLevel;
    requires_restart: boolean;
}

export interface ProfileIssue {
    id?: string;
    reason: string;
}

export interface ProfileImportPreview {
    name: string;
    entries: ProfilePreviewEntry[];
    issues: ProfileIssue[];
}

export type TweakCheck =
    | { Registry: { root_key: string; path: string; key: string; value: any } }
    | { Powershell: { script: string; expected_output: string } };

export type TweakOperation =
    | { RegistrySet: { root_key: string; path: string; key: string; value: any } }
    | { Command: { cmd: string; args: string[] } }
    | { Powershell: { script: string } };

export interface DnsBenchmarkResult {
    provider: string;
    primary: string;
    secondary: string;
    description: string;
    primary_latency_ms: number | null;
    secondary_latency_ms: number | null;
    avg_latency_ms: number | null;
}
