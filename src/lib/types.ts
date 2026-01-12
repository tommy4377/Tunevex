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
    | 'Restore';

export type TweakType = 'Toggle' | 'Action';

export interface Tweak {
    id: string;
    category: TweakCategory;
    name: string;
    description: string;
    warning_level: WarningLevel;
    tweak_type: TweakType;
    requires_restart: boolean;
    enabled?: boolean; // Frontend state
    check?: TweakCheck;
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
