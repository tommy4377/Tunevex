<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        Cpu,
        HardDrive,
        Clock,
        Trash2,
        Files,
        Wifi,
        Router,
        Palette,
        Shield,
        Save,
        MonitorUp,
        MemoryStick,
        Gauge,
    } from "lucide-svelte";

    import type { SystemStats } from "$lib/systemStore";
    import { systemStats, refreshStatsIfNeeded } from "$lib/systemStore";

    let stats: SystemStats;
    const unsubscribe = systemStats.subscribe((value) => {
        stats = value;
    });

    let interval: any;

    // Status messages for quick actions
    let trashMsg = "";
    let isCleaningTrash = false;
    let tempMsg = "";
    let isCleaningTemp = false;
    let dnsMsg = "";
    let isFlushingDns = false;
    let netMsg = "";
    let isResettingNet = false;

    // Restore Point Logic
    let restoreDesc = "Manual Backup";
    let isCreatingRestore = false;
    let restoreMsg = "";
    let restoreSuccess = false;

    onMount(() => {
        refreshStatsIfNeeded();
        interval = setInterval(refreshStatsIfNeeded, 2000);
    });

    onDestroy(() => {
        if (interval) clearInterval(interval);
        unsubscribe();
    });

    function formatBytes(bytes: number) {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
    }

    function formatUptime(seconds: number) {
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        return `${h}h ${m}m`;
    }

    async function cleanTrash() {
        if (isCleaningTrash) return;
        isCleaningTrash = true;
        trashMsg = "Cleaning...";
        try {
            const res = await invoke("empty_recycle_bin");
            trashMsg = res as string;
            setTimeout(() => (trashMsg = ""), 3000);
        } catch (e) {
            trashMsg = "Failed";
        } finally {
            isCleaningTrash = false;
        }
    }

    async function cleanTemp() {
        if (isCleaningTemp) return;
        isCleaningTemp = true;
        tempMsg = "Cleaning...";
        try {
            const res = await invoke("clear_temp_files");
            tempMsg = res as string;
            setTimeout(() => (tempMsg = ""), 4000);
        } catch (e) {
            tempMsg = "Failed";
        } finally {
            isCleaningTemp = false;
        }
    }

    async function flushDns() {
        if (isFlushingDns) return;
        isFlushingDns = true;
        dnsMsg = "Flushing...";
        try {
            const res = await invoke("flush_dns_cache");
            dnsMsg = res as string;
            setTimeout(() => (dnsMsg = ""), 3000);
        } catch (e) {
            dnsMsg = "Failed";
        } finally {
            isFlushingDns = false;
        }
    }

    async function resetNetwork() {
        if (isResettingNet) return;
        isResettingNet = true;
        netMsg = "Resetting...";
        try {
            const res = await invoke("reset_network");
            netMsg = res as string;
            setTimeout(() => (netMsg = ""), 4000);
        } catch (e) {
            netMsg = "Failed";
        } finally {
            isResettingNet = false;
        }
    }

    async function createRestorePoint() {
        if (!restoreDesc.trim()) return;
        isCreatingRestore = true;
        restoreMsg = "Creating...";
        restoreSuccess = false;
        try {
            const res = await invoke("create_restore_point", {
                description: restoreDesc,
            });
            restoreMsg = res as string;
            restoreSuccess = true;
            setTimeout(() => {
                if (restoreSuccess) restoreMsg = "";
            }, 4000);
        } catch (e) {
            restoreMsg = `Error: ${e}`;
            restoreSuccess = false;
        } finally {
            isCreatingRestore = false;
        }
    }
</script>

<div class="dashboard">
    <!-- Header -->
    <div class="header">
        <div class="welcome">
            <h1>Welcome, {stats.username}</h1>
            <span class="uptime"
                ><Clock size={12} /> Uptime: {formatUptime(stats.uptime)}</span
            >
        </div>
    </div>

    <!-- Main Grid Layout -->
    <div class="main-grid">
        <!-- Left Column: System Stats -->
        <div class="column stats-column">
            <h2>System Status</h2>
            <div class="stats-cards">
                <!-- CPU -->
                <div class="mini-stat">
                    <div class="stat-icon"><Cpu size={18} /></div>
                    <div class="stat-data">
                        <span class="stat-label">CPU</span>
                        <span class="stat-value"
                            >{stats.cpu_usage.toFixed(0)}%</span
                        >
                    </div>
                    <div class="mini-bar">
                        <div
                            class="mini-fill"
                            style="width: {stats.cpu_usage}%"
                        ></div>
                    </div>
                </div>

                <!-- RAM -->
                <div class="mini-stat">
                    <div class="stat-icon"><MemoryStick size={18} /></div>
                    <div class="stat-data">
                        <span class="stat-label">RAM</span>
                        <span class="stat-value"
                            >{Math.round(
                                (stats.ram_usage / stats.ram_total) * 100,
                            )}%</span
                        >
                    </div>
                    <div class="mini-bar">
                        <div
                            class="mini-fill"
                            style="width: {(stats.ram_usage / stats.ram_total) *
                                100}%"
                        ></div>
                    </div>
                </div>

                <!-- GPU -->
                <div class="mini-stat">
                    <div class="stat-icon"><Gauge size={18} /></div>
                    <div class="stat-data">
                        <span class="stat-label">GPU</span>
                        <span class="stat-value"
                            >{stats.gpu
                                ? stats.gpu.usage.toFixed(0) + "%"
                                : "--"}</span
                        >
                    </div>
                    <div class="mini-bar">
                        <div
                            class="mini-fill"
                            style="width: {stats.gpu ? stats.gpu.usage : 0}%"
                        ></div>
                    </div>
                </div>
            </div>

            <!-- Storage Drives -->
            <h2>Storage</h2>
            <div class="drives-list">
                {#each stats.disks.sort( (a, b) => a.mount_point.localeCompare(b.mount_point), ) as disk}
                    <div class="drive-row">
                        <HardDrive size={14} />
                        <span class="drive-label"
                            >{disk.name || "Disk"} ({disk.mount_point})</span
                        >
                        <span class="drive-space"
                            >{formatBytes(disk.available_space)} free</span
                        >
                        <div class="drive-bar">
                            <div
                                class="drive-fill"
                                class:warning={(disk.total_space -
                                    disk.available_space) /
                                    disk.total_space >
                                    0.9}
                                style="width: {((disk.total_space -
                                    disk.available_space) /
                                    disk.total_space) *
                                    100}%"
                            ></div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Right Column: Quick Actions & Restore -->
        <div class="column actions-column">
            <h2>Quick Maintenance</h2>
            <div class="actions-grid">
                <button
                    class="action-btn"
                    on:click={cleanTrash}
                    disabled={isCleaningTrash}
                >
                    <Trash2 size={18} />
                    <span>Empty Trash</span>
                    {#if trashMsg}<span class="msg">{trashMsg}</span>{/if}
                </button>

                <button
                    class="action-btn"
                    on:click={cleanTemp}
                    disabled={isCleaningTemp}
                >
                    <Files size={18} />
                    <span>Clear Temp</span>
                    {#if tempMsg}<span class="msg">{tempMsg}</span>{/if}
                </button>

                <button
                    class="action-btn"
                    on:click={flushDns}
                    disabled={isFlushingDns}
                >
                    <Wifi size={18} />
                    <span>Flush DNS</span>
                    {#if dnsMsg}<span class="msg">{dnsMsg}</span>{/if}
                </button>

                <button
                    class="action-btn"
                    on:click={resetNetwork}
                    disabled={isResettingNet}
                >
                    <Router size={18} />
                    <span>Reset Network</span>
                    {#if netMsg}<span class="msg">{netMsg}</span>{/if}
                </button>
            </div>

            <!-- Restore Point -->
            <div class="restore-section">
                <h2><Shield size={16} /> Create Restore Point</h2>
                <div class="restore-row">
                    <input
                        type="text"
                        bind:value={restoreDesc}
                        placeholder="Description..."
                        disabled={isCreatingRestore}
                    />
                    <button
                        class="create-btn"
                        on:click={createRestorePoint}
                        disabled={isCreatingRestore}
                    >
                        {#if isCreatingRestore}
                            <div class="spinner"></div>
                        {:else}
                            <Save size={16} />
                        {/if}
                    </button>
                </div>
                {#if restoreMsg}
                    <div
                        class="restore-msg"
                        class:success={restoreSuccess}
                        class:error={!restoreSuccess}
                    >
                        {restoreMsg}
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>

<style>
    .dashboard {
        padding: 20px 24px;
        height: 100%;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    /* Header */
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .welcome h1 {
        font-size: 22px;
        font-weight: 700;
        margin: 0 0 4px 0;
        background: linear-gradient(90deg, #fff, #999);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .uptime {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 12px;
        color: var(--text-muted);
    }

    /* Main Grid */
    .main-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 20px;
        flex: 1;
        min-height: 0;
    }

    @media (max-width: 900px) {
        .main-grid {
            grid-template-columns: 1fr;
        }
    }

    .column {
        background: var(--layer-card);
        border: var(--border-glass);
        border-radius: var(--radius-lg, 16px);
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 12px;
        overflow: hidden;
    }

    .column h2 {
        font-size: 14px;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin: 0;
        display: flex;
        align-items: center;
        gap: 8px;
    }

    /* Stats Cards */
    .stats-cards {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .mini-stat {
        display: grid;
        grid-template-columns: 44px 1fr 100px;
        align-items: center;
        gap: 16px;
        padding: 14px 16px;
        background: rgba(255, 255, 255, 0.03);
        border-radius: 12px;
    }

    .stat-icon {
        width: 44px;
        height: 44px;
        border-radius: 10px;
        background: rgba(96, 205, 255, 0.1);
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--accent-color);
    }

    .stat-data {
        display: flex;
        flex-direction: column;
    }

    .stat-label {
        font-size: 11px;
        color: var(--text-muted);
        text-transform: uppercase;
    }

    .stat-value {
        font-size: 22px;
        font-weight: 700;
    }

    .mini-bar {
        height: 8px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
        overflow: hidden;
    }

    .mini-fill {
        height: 100%;
        background: var(--accent-color);
        border-radius: 3px;
        transition: width 0.3s ease;
    }

    /* Drives */
    .drives-list {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .drive-row {
        display: grid;
        grid-template-columns: 16px 1fr auto 80px;
        align-items: center;
        gap: 10px;
        padding: 8px 10px;
        background: rgba(255, 255, 255, 0.02);
        border-radius: 8px;
        font-size: 13px;
        color: var(--text-muted);
    }

    .drive-row :global(svg) {
        color: var(--text-muted);
    }

    .drive-label {
        color: var(--text-primary);
        font-weight: 500;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .drive-space {
        font-size: 11px;
    }

    .drive-bar {
        height: 4px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 2px;
        overflow: hidden;
    }

    .drive-fill {
        height: 100%;
        background: var(--accent-color);
        border-radius: 2px;
    }

    .drive-fill.warning {
        background: #ef4444;
    }

    /* Actions Grid */
    .actions-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 10px;
    }

    .action-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 10px;
        padding: 24px 16px;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.06);
        border-radius: 14px;
        color: var(--text-primary);
        cursor: pointer;
        transition: all 0.2s;
        position: relative;
    }

    .action-btn:hover:not(:disabled) {
        background: var(--layer-hover);
        border-color: var(--accent-color);
        transform: translateY(-1px);
    }

    .action-btn:disabled {
        opacity: 0.6;
        cursor: wait;
    }

    .action-btn span {
        font-size: 13px;
        font-weight: 600;
    }

    .action-btn .msg {
        position: absolute;
        top: 4px;
        right: 4px;
        font-size: 9px;
        background: rgba(0, 0, 0, 0.7);
        padding: 2px 6px;
        border-radius: 4px;
        color: var(--accent-color);
    }

    .action-btn :global(svg) {
        color: var(--accent-color);
    }

    /* Restore Section */
    .restore-section {
        padding-top: 16px;
        border-top: var(--border-glass);
    }

    .restore-row {
        display: flex;
        gap: 8px;
        margin-top: 12px;
    }

    .restore-row input {
        flex: 1;
        background: rgba(0, 0, 0, 0.2);
        border: var(--border-glass);
        border-radius: 8px;
        padding: 10px 12px;
        color: var(--text-primary);
        font-size: 13px;
        outline: none;
    }

    .restore-row input:focus {
        border-color: var(--accent-color);
    }

    .create-btn {
        width: 40px;
        background: var(--accent-color);
        border: none;
        border-radius: 8px;
        color: white;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: opacity 0.2s;
    }

    .create-btn:hover:not(:disabled) {
        opacity: 0.85;
    }

    .create-btn:disabled {
        opacity: 0.5;
        cursor: wait;
    }

    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .restore-msg {
        margin-top: 8px;
        padding: 8px;
        border-radius: 6px;
        font-size: 12px;
        text-align: center;
    }

    .restore-msg.success {
        background: rgba(34, 197, 94, 0.1);
        color: #22c55e;
    }

    .restore-msg.error {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
    }
</style>
