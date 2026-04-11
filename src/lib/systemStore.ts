// src/lib/systemStore.ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface DiskStats {
  name: string;
  mountpoint: string;
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

const initialStats: SystemStats = {
  cpu_usage: 0,
  ram_usage: 0,
  ram_total: 1,
  uptime: 0,
  username: 'User',
  disks: [],
  gpu: null,
};

export const systemStats = writable<SystemStats>(initialStats);
export const lastFetch = writable<number>(0);

export async function refreshStatsIfNeeded(): Promise<void> {
  try {
    // Quick stats (CPU, RAM, GPU) — non blocca
    const quick = await invoke<SystemStats>('get_quick_stats');
    systemStats.update(s => ({ ...s, ...quick, disks: s.disks }));

    // Disks in background — non blocca la UI
    invoke<DiskStats[]>('get_disk_stats')
      .then(disks => {
        systemStats.update(s => ({ ...s, disks }));
      })
      .catch(e => console.error('Disk fetch failed:', e));

    lastFetch.set(Date.now());
  } catch (e) {
    console.error('Store fetch failed:', e);
  }
}