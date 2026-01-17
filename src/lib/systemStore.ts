import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Types
export interface DiskStats {
    name: string;
    mount_point: string;
    total_space: number;
    available_space: number;
    is_removable: boolean;
}

export interface GpuStats {
    name: string;
    usage: number;
}

export interface SystemStats {
    cpu_usage: number;
    ram_usage: number;
    ram_total: number;
    uptime: number;
    username: string;
    disks: DiskStats[];
    gpu: GpuStats | null;
}

// Stores
export const systemStats = writable<SystemStats>({
    cpu_usage: 0,
    ram_usage: 0,
    ram_total: 1,
    uptime: 0,
    username: "User",
    disks: [],
    gpu: null,
});

export const lastFetch = writable<number>(0);

// Helper to fetch only if stale (e.g. older than 2s to allow some freshness but prevent tab-switch spam)
// For "instant" load we rely on the existing store value
export async function refreshStatsIfNeeded() {
    // Always return current value immediately for UI responsiveness
    // Background update logic:
    try {
        const quick = await invoke<SystemStats>('get_quick_stats');

        systemStats.update(s => ({
            ...s,
            ...quick,
            // Keep old disks if new fetch hasn't happened yet
            disks: s.disks
        }));

        // Fetch disks separately without blocking
        invoke<DiskStats[]>('get_disk_stats').then(disks => {
            systemStats.update(s => ({ ...s, disks }));
        }).catch(console.error);

        lastFetch.set(Date.now());
    } catch (e) {
        console.error("Store fetch failed", e);
    }
}
