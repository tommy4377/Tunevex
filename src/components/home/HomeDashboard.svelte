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
    } from "lucide-svelte";

    import type { SystemStats } from "$lib/systemStore";

    import { systemStats, refreshStatsIfNeeded } from "$lib/systemStore";

    let stats: SystemStats;
    // Subscribe to store
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
        // Initial fetch if needed, otherwise uses cached value immediately
        refreshStatsIfNeeded();
        // Poll every 2s
        interval = setInterval(refreshStatsIfNeeded, 2000);
    });

    onDestroy(() => {
        if (interval) clearInterval(interval);
        unsubscribe(); // Unsubscribe to prevent memory leaks
    });

    function formatBytes(bytes: number) {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
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
        tempMsg = "Scanning & Cleaning...";
        try {
            const res = await invoke("clear_temp_files");
            tempMsg = res as string;
            setTimeout(() => (tempMsg = ""), 5000);
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
        netMsg = "Resetting IP/Winsock...";
        try {
            const res = await invoke("reset_network");
            netMsg = res as string;
            setTimeout(() => (netMsg = ""), 5000);
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
            }, 5000);
        } catch (e) {
            restoreMsg = `Error: ${e}`;
            restoreSuccess = false;
        } finally {
            isCreatingRestore = false;
        }
    }
</script>

<div class="home-dashboard">
    <div class="welcome-section">
        <h1>Welcome back, {stats.username}</h1>
        <p>System status and quick maintenance actions.</p>
    </div>

    <!-- Storage & Restore (Moved Up as Requested) -->
    <div class="split-section">
        <!-- Storage Section -->
        <div class="section-container storage-section">
            <h2>Storage Drives</h2>
            <div class="drives-grid">
                {#each stats.disks.sort( (a, b) => a.mount_point.localeCompare(b.mount_point), ) as disk}
                    <div class="drive-card">
                        <div class="drive-icon"><HardDrive size={20} /></div>
                        <div class="drive-info">
                            <div class="drive-header">
                                <span class="drive-name"
                                    >{disk.name || "Local Disk"} ({disk.mount_point})</span
                                >
                                <span class="drive-usage"
                                    >{formatBytes(
                                        disk.total_space - disk.available_space,
                                    )} / {formatBytes(disk.total_space)}</span
                                >
                            </div>
                            <div class="bar-bg">
                                <div
                                    class="bar-fill"
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
                    </div>
                {/each}
                {#if stats.disks.length === 0}
                    <p class="empty-msg">No drives detected</p>
                {/if}
            </div>
        </div>

        <!-- Quick Restore Point -->
        <div class="section-container restore-section">
            <h2>
                <Shield
                    size={18}
                    style="margin-right: 8px; vertical-align: text-bottom;"
                />Quick Restore Point
            </h2>
            <div class="restore-layout">
                <div class="input-row">
                    <input
                        type="text"
                        bind:value={restoreDesc}
                        placeholder="Restore Point Description"
                        disabled={isCreatingRestore}
                    />
                    <button
                        class="restore-btn"
                        on:click={createRestorePoint}
                        disabled={isCreatingRestore}
                    >
                        {#if isCreatingRestore}
                            <div class="spinner-sm"></div>
                        {:else}
                            <Save size={18} />
                        {/if}
                    </button>
                </div>
                {#if restoreMsg}
                    <div
                        class="restore-status"
                        class:success={restoreSuccess}
                        class:error={!restoreSuccess}
                    >
                        {restoreMsg}
                    </div>
                {/if}
            </div>
        </div>
    </div>

    <!-- Main Stats Grid (CPU, RAM, GPU) -->
    <div class="stats-grid">
        <!-- CPU CArd -->
        <div class="stat-card">
            <div class="icon-circle"><Cpu /></div>
            <div class="stat-info">
                <span class="label">CPU Usage</span>
                <span class="value">{stats.cpu_usage.toFixed(1)}%</span>
                <div class="bar-bg">
                    <div
                        class="bar-fill"
                        style="width: {stats.cpu_usage}%"
                    ></div>
                </div>
            </div>
        </div>

        <!-- RAM Card -->
        <div class="stat-card">
            <div class="icon-circle"><HardDrive /></div>
            <div class="stat-info">
                <span class="label">RAM Usage</span>
                <span class="value"
                    >{Math.round(
                        (stats.ram_usage / stats.ram_total) * 100,
                    )}%</span
                >
                <div class="sub-text">
                    {formatBytes(stats.ram_usage)} / {formatBytes(
                        stats.ram_total,
                    )}
                </div>
                <div class="bar-bg">
                    <div
                        class="bar-fill"
                        style="width: {(stats.ram_usage / stats.ram_total) *
                            100}%"
                    ></div>
                </div>
            </div>
        </div>

        <!-- GPU Card -->
        <div class="stat-card">
            <div class="icon-circle"><Palette /></div>
            <div class="stat-info">
                <span class="label">GPU Usage</span>
                {#if stats.gpu}
                    <span class="value">{stats.gpu.usage.toFixed(1)}%</span>
                    <div class="sub-text" title={stats.gpu.name}>
                        {stats.gpu.name}
                    </div>
                    <div class="bar-bg">
                        <div
                            class="bar-fill"
                            style="width: {stats.gpu.usage}%"
                        ></div>
                    </div>
                {:else}
                    <span class="value">--</span>
                    <div class="sub-text">Detecting...</div>
                {/if}
            </div>
        </div>
    </div>

    <!-- Quick Actions Section -->
    <div class="actions-section">
        <h2>Quick Maintenance</h2>
        <div class="quick-actions-grid">
            <button
                class="action-card"
                on:click={cleanTrash}
                disabled={isCleaningTrash}
                class:busy={isCleaningTrash}
            >
                <div class="icon-box warning">
                    <Trash2 size={24} />
                </div>
                <div class="action-details">
                    <h3>Empty Recycle Bin</h3>
                    <p>Permanently delete files in trash</p>
                </div>
                {#if trashMsg}
                    <span class="status-msg">{trashMsg}</span>
                {/if}
            </button>

            <button
                class="action-card"
                on:click={cleanTemp}
                disabled={isCleaningTemp}
                class:busy={isCleaningTemp}
            >
                <div class="icon-box info">
                    <Files size={24} />
                </div>
                <div class="action-details">
                    <h3>Clear Temp Files</h3>
                    <p>Free up space by removing temp data</p>
                </div>
                {#if tempMsg}
                    <span class="status-msg">{tempMsg}</span>
                {/if}
            </button>

            <button
                class="action-card"
                on:click={flushDns}
                disabled={isFlushingDns}
                class:busy={isFlushingDns}
            >
                <div class="icon-box success">
                    <Wifi size={24} />
                </div>
                <div class="action-details">
                    <h3>Flush DNS</h3>
                    <p>Reset network cache connectivity</p>
                </div>
                {#if dnsMsg}
                    <span class="status-msg">{dnsMsg}</span>
                {/if}
            </button>

            <button
                class="action-card"
                on:click={resetNetwork}
                disabled={isResettingNet}
                class:busy={isResettingNet}
            >
                <div class="icon-box warning">
                    <Router size={24} />
                </div>
                <div class="action-details">
                    <h3>Network Reset</h3>
                    <p>Full reset (Winsock/IP)</p>
                </div>
                {#if netMsg}
                    <span class="status-msg">{netMsg}</span>
                {/if}
            </button>
        </div>
    </div>
</div>

<style>
    .home-dashboard {
        padding: 0 32px 32px;
        height: 100%;
        overflow-y: auto;
    }

    .welcome-section {
        margin-bottom: 32px;
    }

    .welcome-section h1 {
        font-size: 28px;
        font-weight: 700;
        margin-bottom: 8px;
        background: linear-gradient(to right, #fff, #aaa);
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .welcome-section p {
        color: var(--text-muted);
        font-size: 16px;
    }

    /* Stats Grid */
    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 24px;
        margin-bottom: 32px;
    }

    .stat-card {
        background: var(--layer-card);
        backdrop-filter: blur(20px);
        border: var(--border-glass);
        border-radius: var(--radius-card);
        padding: 24px;
        display: flex;
        align-items: center;
        gap: 20px;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .stat-card:hover {
        transform: translateY(-2px);
        background: var(--layer-hover);
    }

    .icon-circle {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.05);
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--accent);
    }

    .stat-info {
        flex: 1;
        overflow: hidden; /* For GPU long name text overflow */
    }

    .label {
        display: block;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: var(--text-secondary);
        margin-bottom: 4px;
        font-weight: 600;
    }

    .value {
        display: block;
        font-size: 24px;
        font-weight: 700;
        margin-bottom: 8px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .sub-text {
        font-size: 12px;
        color: var(--text-secondary);
        margin-bottom: 8px;
    }

    .bar-bg {
        height: 6px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 3px;
        overflow: hidden;
    }

    .bar-fill {
        height: 100%;
        background: var(--accent);
        transition: width 0.5s ease;
        border-radius: 3px;
    }

    /* Split Section (Storage + Restore) */
    .split-section {
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: 24px;
        margin-bottom: 32px;
    }
    @media (max-width: 900px) {
        .split-section {
            grid-template-columns: 1fr;
        }
    }

    .section-container {
        background: var(--layer-card);
        backdrop-filter: blur(20px);
        border: var(--border-glass);
        border-radius: var(--radius-card);
        padding: 24px;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    /* Added hover effect for storage/restore containers */
    .section-container:hover {
        background: var(--layer-hover);
        transform: translateY(-2px);
    }

    .section-container h2 {
        font-size: 18px;
        font-weight: 600;
        margin-bottom: 16px;
        color: var(--text-primary);
    }

    /* Storage Section */
    .drives-grid {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .drive-card {
        display: flex;
        align-items: center;
        gap: 16px;
        padding: 12px 12px; /* Increased padding slightly */
        border-radius: 12px; /* Added radius for hover effect */
        transition: background 0.2s; /* Added transition */
    }

    /* Added hover for individual drives too */
    .drive-card:hover {
        background: rgba(255, 255, 255, 0.05);
    }

    .drive-icon {
        color: var(--text-secondary);
    }

    .drive-info {
        flex: 1;
    }

    .drive-header {
        display: flex;
        justify-content: space-between;
        margin-bottom: 6px;
        font-size: 13px;
    }

    .drive-name {
        font-weight: 500;
        color: var(--text-primary);
    }
    .drive-usage {
        color: var(--text-secondary);
    }

    /* Restore Section */
    .restore-layout {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .input-row {
        display: flex;
        gap: 12px;
    }

    .input-row input {
        flex: 1;
        background: rgba(0, 0, 0, 0.2);
        border: var(--border-glass);
        border-radius: 6px;
        padding: 10px 14px;
        color: var(--text-primary);
        font-size: 13px;
        outline: none;
    }
    .input-row input:focus {
        border-color: var(--accent);
    }

    .restore-btn {
        background: var(--accent);
        color: white;
        border: none;
        width: 42px;
        border-radius: 6px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: opacity 0.2s;
    }
    .restore-btn:hover:not(:disabled) {
        opacity: 0.9;
    }
    .restore-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .restore-status {
        font-size: 12px;
        padding: 8px;
        border-radius: 6px;
        text-align: center;
    }
    .restore-status.success {
        background: rgba(16, 185, 129, 0.15);
        color: #34d399;
    }
    .restore-status.error {
        background: rgba(239, 68, 68, 0.15);
        color: #f87171;
    }

    /* Actions Section (Existing) */
    .actions-section h2 {
        font-size: 20px;
        font-weight: 600;
        margin-bottom: 20px;
    }

    .quick-actions-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 24px;
    }

    .action-card {
        background: var(--layer-card);
        backdrop-filter: blur(20px);
        border: var(--border-glass);
        border-radius: var(--radius-card);
        padding: 24px;
        display: flex;
        align-items: flex-start;
        gap: 16px;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        position: relative;
        overflow: hidden;
        /* Reset button styles */
        width: 100%;
        text-align: left;
        color: inherit;
        font: inherit;
        outline: none;
    }

    .action-card:hover:not(:disabled) {
        background: var(--layer-hover);
        border-color: var(--accent);
        transform: translateY(-2px);
    }

    .action-card:active:not(:disabled) {
        transform: scale(0.98);
    }

    .action-card:disabled {
        cursor: wait;
        opacity: 0.7;
        background: rgba(255, 255, 255, 0.01);
    }

    .action-card.busy {
        border-color: var(--accent);
    }

    .spinner-sm {
        width: 18px;
        height: 18px;
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .icon-box {
        width: 48px;
        height: 48px;
        border-radius: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .icon-box.warning {
        background: rgba(239, 68, 68, 0.15);
        color: #ef4444;
    }
    .icon-box.info {
        background: rgba(59, 130, 246, 0.15);
        color: #3b82f6;
    }
    .icon-box.success {
        background: rgba(34, 197, 94, 0.15);
        color: #22c55e;
    }

    .action-details h3 {
        font-size: 16px;
        font-weight: 600;
        margin: 0 0 4px 0;
        color: var(--text-primary);
    }

    .action-details p {
        font-size: 13px;
        color: var(--text-secondary);
        margin: 0;
        line-height: 1.4;
    }

    .status-msg {
        position: absolute;
        top: 6px;
        right: 6px;
        font-size: 10px;
        font-weight: 600;
        color: var(--accent);
        background: rgba(0, 0, 0, 0.75);
        padding: 3px 7px;
        border-radius: 6px;
        z-index: 10;
        max-width: 120px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>
