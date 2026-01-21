<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { compactorStore, addLog } from "$lib/compactorStore";

    // Use reactive store values
    $: state = $compactorStore;

    // Local UI state (transient)
    let isDropdownOpen = false;
    let compressedFolders: string[] = [];
    let folderStatsMap: Record<string, FolderStats> = {};

    // Listeners
    let unlistenProgress: () => void;
    let unlistenStatus: () => void;
    let unlistenFile: () => void;
    let unlistenBytes: () => void;

    // Dropdown items
    const algorithms = [
        { id: 0, label: "XPRESS 4K — Fastest, Safest ✓" },
        { id: 1, label: "XPRESS 8K — Balanced" },
        { id: 2, label: "XPRESS 16K — Better Compression" },
        { id: 3, label: "LZX — Maximum (High CPU ⚠️)" },
    ];

    $: selectedAlgoLabel =
        algorithms.find((a) => a.id === state.compressionAlgo)?.label ||
        "Select Algorithm";

    interface FolderStats {
        path: string;
        total_size: number;
        compressed_size: number;
        file_count: number;
    }

    function handleClickOutside(event: MouseEvent) {
        const target = event.target as HTMLElement;
        if (isDropdownOpen && !target.closest(".custom-select-container")) {
            isDropdownOpen = false;
        }
    }

    onMount(async () => {
        document.addEventListener("click", handleClickOutside);
        await refreshFolders();

        // Listeners update the STORE
        // @ts-ignore
        unlistenProgress = await listen<number>(
            "compactor-progress",
            (event) => {
                compactorStore.update((s) => {
                    let total = s.progressTotal || 0;
                    if (event.payload > total && total > 0)
                        total = event.payload + 10;
                    return {
                        ...s,
                        progressCurrent: event.payload,
                        progressTotal: total,
                    };
                });
            },
        );

        // @ts-ignore
        unlistenStatus = await listen<string>("compactor-status", (event) => {
            compactorStore.update((s) => ({ ...s, statusMsg: event.payload }));
            addLog(event.payload);
        });

        // @ts-ignore
        unlistenFile = await listen<string>("compactor-file", (event) => {
            compactorStore.update((s) => ({
                ...s,
                currentFile: event.payload,
            }));
        });

        // @ts-ignore
        unlistenBytes = await listen<number>("compactor-bytes", (event) => {
            compactorStore.update((s) => ({
                ...s,
                bytesAnalyzed: event.payload,
            }));
        });
    });

    onDestroy(() => {
        document.removeEventListener("click", handleClickOutside);
        if (unlistenProgress) unlistenProgress();
        if (unlistenStatus) unlistenStatus();
        if (unlistenFile) unlistenFile();
        if (unlistenBytes) unlistenBytes();
    });

    async function refreshFolders() {
        try {
            compressedFolders = await invoke("get_compressed_folders");
            for (const f of compressedFolders) {
                if (!folderStatsMap[f]) {
                    fetchStats(f);
                }
            }
        } catch (e) {
            console.error(e);
        }
    }

    async function fetchStats(folderPath: string) {
        try {
            const stats: FolderStats = await invoke("get_folder_stats", {
                path: folderPath,
            });
            folderStatsMap[folderPath] = stats;
        } catch (e) {
            console.error(e);
        }
    }

    async function selectFolder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                defaultPath: state.path || undefined,
            });
            if (selected && typeof selected === "string") {
                compactorStore.update((s) => ({ ...s, path: selected }));
            }
        } catch (e) {
            console.error("Dialog error:", e);
        }
    }

    async function cancelOperation() {
        try {
            await invoke("cancel_compactor");
            compactorStore.update((s) => ({
                ...s,
                isScanning: false,
                isCompressing: false,
                statusMsg: "Operation cancelled by user",
                statusType: "info",
            }));
            addLog("Operation cancelled by user");
        } catch (e) {
            console.error("Failed to cancel:", e);
        }
    }

    async function scan() {
        if (!state.path) return;
        compactorStore.update((s) => ({
            ...s,
            isScanning: true,
            statusMsg: "Scanning directory structure...",
            statusType: "info",
            scanResult: null,
            progressCurrent: 0,
            progressTotal: 0,
            bytesAnalyzed: 0,
        }));

        try {
            const res: any = await invoke("scan_storage", { path: state.path });
            compactorStore.update((s) => ({
                ...s,
                scanResult: res,
                progressTotal: res.file_count,
                statusMsg: `Scan Complete: Found ${res.file_count} files (${formatBytes(res.total_size)})`,
                statusType: "success",
            }));
        } catch (e: unknown) {
            if (String(e).includes("Cancelled")) {
                compactorStore.update((s) => ({
                    ...s,
                    statusMsg: "Scan Cancelled",
                }));
            } else {
                compactorStore.update((s) => ({
                    ...s,
                    statusMsg: "Scan Error: " + e,
                    statusType: "error",
                }));
            }
        } finally {
            compactorStore.update((s) => ({ ...s, isScanning: false }));
        }
    }

    async function compress() {
        if (!state.path) return;
        compactorStore.update((s) => ({
            ...s,
            isCompressing: true,
            statusMsg: "Compressing files... (This may take a while)",
            statusType: "info",
        }));

        try {
            const res = await invoke("compress_folder", {
                path: state.path,
                algoIdx: state.compressionAlgo,
            });
            if (res === "Cancelled") {
                compactorStore.update((s) => ({
                    ...s,
                    statusMsg: "Compression Cancelled",
                    statusType: "info",
                }));
            } else {
                compactorStore.update((s) => ({
                    ...s,
                    statusMsg: "Compression Complete: " + res,
                    statusType: "success",
                }));
                await refreshFolders();
            }
        } catch (e) {
            compactorStore.update((s) => ({
                ...s,
                statusMsg: "Compression Error: " + e,
                statusType: "error",
            }));
        } finally {
            compactorStore.update((s) => ({ ...s, isCompressing: false }));
        }
    }

    async function decompress(target: string) {
        if (state.isCompressing) return;
        compactorStore.update((s) => ({
            ...s,
            isCompressing: true,
            statusMsg: `Decompressing ${target}...`,
            statusType: "info",
        }));

        try {
            const res = await invoke("decompress_folder", { path: target });
            compactorStore.update((s) => ({
                ...s,
                statusMsg: "Decompression Complete: " + res,
                statusType: "success",
            }));
            await refreshFolders();
            delete folderStatsMap[target];
            folderStatsMap = folderStatsMap;
        } catch (e) {
            compactorStore.update((s) => ({
                ...s,
                statusMsg: "Decompression Error: " + e,
                statusType: "error",
            }));
        } finally {
            compactorStore.update((s) => ({ ...s, isCompressing: false }));
        }
    }

    function formatBytes(bytes: number, decimals = 2) {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return (
            parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i]
        );
    }

    function getCompressionRatio(original: number, compressed: number) {
        if (original === 0) return 0;
        return Math.max(0, Math.min(100, (compressed / original) * 100));
    }

    function getSavedPercentage(original: number, compressed: number) {
        if (original === 0) return 0;
        const saved = original - compressed;
        return Math.max(0, (saved / original) * 100).toFixed(1);
    }
</script>

<div class="compactor-section">
    <div class="section-header">
        <div class="title-group">
            <span class="icon">🗜️</span>
            <h3>Compactor</h3>
        </div>
    </div>

    <p class="description">
        Use Windows transparent compression (CompactOS/WOF) to reduce folder
        size without affecting functionality. Games and apps run normally.
    </p>

    <div class="controls-grid">
        <!-- Path Input -->
        <div class="input-group">
            <label for="target-path">Target Folder</label>
            <div class="row">
                <div class="path-container">
                    <input
                        id="target-path"
                        type="text"
                        value={state.path}
                        on:input={(e) =>
                            compactorStore.update((s) => ({
                                ...s,
                                path: e.currentTarget.value,
                            }))}
                        placeholder="e.g. C:\Games"
                        disabled={state.isScanning || state.isCompressing}
                    />
                    <button
                        class="btn-icon"
                        on:click={selectFolder}
                        disabled={state.isScanning || state.isCompressing}
                        title="Browse Folder"
                        aria-label="Browse Folder"
                    >
                        📂
                    </button>
                </div>
                <button
                    class="btn-secondary"
                    on:click={state.isScanning ? cancelOperation : scan}
                    disabled={state.isCompressing && !state.isScanning}
                >
                    {state.isScanning ? "🛑 Stop" : "Scan"}
                </button>
            </div>
        </div>

        <!-- Algorithm Selection -->
        <div class="input-group">
            <label for="compression-trigger">Compression Algorithm</label>
            <div class="row">
                <!-- Custom Dropdown -->
                <div class="custom-select-container">
                    <button
                        id="compression-trigger"
                        class="select-trigger"
                        on:click={() => {
                            if (!state.isScanning && !state.isCompressing)
                                isDropdownOpen = !isDropdownOpen;
                        }}
                        disabled={state.isScanning || state.isCompressing}
                        class:active={isDropdownOpen}
                    >
                        <span>{selectedAlgoLabel}</span>
                        <span class="arrow">{isDropdownOpen ? "▲" : "▼"}</span>
                    </button>

                    {#if isDropdownOpen}
                        <div class="select-dropdown">
                            {#each algorithms as algo}
                                <div
                                    class="select-option"
                                    class:selected={state.compressionAlgo ===
                                        algo.id}
                                    on:click={() => {
                                        compactorStore.update((s) => ({
                                            ...s,
                                            compressionAlgo: algo.id,
                                        }));
                                        isDropdownOpen = false;
                                    }}
                                    on:keydown={(e) =>
                                        e.key === "Enter" &&
                                        (compactorStore.update((s) => ({
                                            ...s,
                                            compressionAlgo: algo.id,
                                        })),
                                        (isDropdownOpen = false))}
                                    role="button"
                                    tabindex="0"
                                >
                                    {algo.label}
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <button
                    class="btn-primary"
                    on:click={state.isCompressing ? cancelOperation : compress}
                    disabled={!state.path ||
                        (state.isScanning && !state.isCompressing)}
                >
                    {state.isCompressing ? "🛑 Stop" : "Compress Now"}
                </button>
            </div>
        </div>
    </div>

    {#if state.statusMsg}
        <div
            class="status-bar"
            class:error={state.statusType === "error"}
            class:success={state.statusType === "success"}
        >
            {state.statusMsg}
        </div>
    {/if}

    {#if state.isScanning || state.isCompressing || (state.scanResult && !state.isScanning)}
        <div class="live-progress">
            <!-- Status Header -->
            <div class="progress-header">
                <span class="progress-label">
                    {#if state.isScanning}
                        🔍 Scanning files...
                    {:else if state.isCompressing}
                        🗜️ Compressing files...
                    {:else if state.scanResult}
                        ✅ Scan complete
                    {/if}
                </span>
                {#if state.progressTotal > 0}
                    <span class="progress-count">
                        {state.progressCurrent} / {state.progressTotal} files
                    </span>
                {:else if state.isScanning || state.isCompressing}
                    <span class="progress-count">
                        {state.progressCurrent} files
                        {#if state.bytesAnalyzed > 0}
                            ({formatBytes(state.bytesAnalyzed)})
                        {/if}
                    </span>
                {/if}
            </div>

            <!-- Main Progress Bar -->
            {#if state.isScanning || state.isCompressing}
                <div class="progress-bar-track main-bar">
                    <div
                        class="progress-bar-fill"
                        class:indeterminate={state.progressTotal === 0}
                        style="width: {state.progressTotal > 0
                            ? Math.min(
                                  100,
                                  (state.progressCurrent /
                                      state.progressTotal) *
                                      100,
                              )
                            : 100}%"
                    ></div>
                </div>
            {/if}

            <!-- Current File Being Processed -->
            {#if (state.isScanning || state.isCompressing) && state.currentFile}
                <div class="current-file-display">
                    <span class="file-icon">📄</span>
                    <span class="file-name" title={state.currentFile}>
                        {state.currentFile.length > 60
                            ? "..." + state.currentFile.slice(-57)
                            : state.currentFile}
                    </span>
                </div>
            {/if}

            <!-- Detailed Stats Panel -->
            {#if state.scanResult}
                <div class="detailed-stats">
                    <!-- Total Saved -->
                    <div class="stat-summary">
                        {#if state.scanResult.compressed_size && state.scanResult.compressed_size < state.scanResult.total_size}
                            <span class="saved-info">
                                💾 {formatBytes(
                                    state.scanResult.total_size -
                                        state.scanResult.compressed_size,
                                )} of {formatBytes(state.scanResult.total_size)}
                                saved ({(
                                    ((state.scanResult.total_size -
                                        state.scanResult.compressed_size) /
                                        state.scanResult.total_size) *
                                    100
                                ).toFixed(1)}%)
                            </span>
                        {:else}
                            <span class="saved-info">
                                📊 {formatBytes(state.scanResult.total_size)} total
                                in
                                {state.scanResult.file_count} files
                            </span>
                        {/if}
                    </div>

                    <!-- Colored Stats Bars -->
                    <div class="stats-legend">
                        <div class="legend-item">
                            <span class="legend-color compressed"></span>
                            <span class="legend-text">
                                {formatBytes(
                                    state.scanResult.compressed_size || 0,
                                )} compressed
                            </span>
                        </div>
                        <div class="legend-item">
                            <span class="legend-color compressible"></span>
                            <span class="legend-text">
                                {formatBytes(
                                    state.scanResult.compressible_size ||
                                        state.scanResult.total_size -
                                            (state.scanResult.compressed_size ||
                                                0),
                                )} compressible
                            </span>
                        </div>
                        {#if state.scanResult.excluded_size}
                            <div class="legend-item">
                                <span class="legend-color excluded"></span>
                                <span class="legend-text">
                                    {formatBytes(
                                        state.scanResult.excluded_size,
                                    )} excluded
                                </span>
                            </div>
                        {/if}
                    </div>

                    <!-- Stacked Bar Chart -->
                    <div class="stacked-bar">
                        {#if state.scanResult.total_size > 0}
                            <div
                                class="bar-segment compressed"
                                style="width: {((state.scanResult
                                    .compressed_size || 0) /
                                    state.scanResult.total_size) *
                                    100}%"
                            ></div>
                            <div
                                class="bar-segment compressible"
                                style="width: {((state.scanResult
                                    .compressible_size ||
                                    state.scanResult.total_size -
                                        (state.scanResult.compressed_size ||
                                            0) -
                                        (state.scanResult.excluded_size || 0)) /
                                    state.scanResult.total_size) *
                                    100}%"
                            ></div>
                            {#if state.scanResult.excluded_size}
                                <div
                                    class="bar-segment excluded"
                                    style="width: {(state.scanResult
                                        .excluded_size /
                                        state.scanResult.total_size) *
                                        100}%"
                                ></div>
                            {/if}
                        {/if}
                    </div>
                </div>
            {/if}

            <!-- Bytes Analyzed (during scan) -->
            {#if (state.isScanning || state.isCompressing) && state.bytesAnalyzed > 0}
                <div class="bytes-analyzed">
                    ⚡ {formatBytes(state.bytesAnalyzed)} processed
                </div>
            {/if}
        </div>
    {/if}

    {#if state.scanResult}
        <div class="stats-card">
            <div class="stat">
                <span class="label">Files Found</span>
                <span class="value">{state.scanResult.file_count}</span>
            </div>
            <div class="stat">
                <span class="label">Total Size</span>
                <span class="value"
                    >{formatBytes(state.scanResult.total_size)}</span
                >
            </div>
        </div>
    {/if}

    <!-- Managed Section -->
    <div class="managed-section">
        <div class="managed-header">
            <h4>Managed Folders</h4>
            <span class="badge">{compressedFolders.length}</span>
        </div>

        {#if compressedFolders.length === 0}
            <div class="empty-list">No folders have been compressed yet.</div>
        {:else}
            <div class="folders-grid">
                {#each compressedFolders as folder}
                    <div class="folder-card">
                        <div class="card-header">
                            <span class="folder-icon">📁</span>
                            <div class="folder-info">
                                <div class="folder-path" title={folder}>
                                    {folder}
                                </div>
                                {#if folderStatsMap[folder]}
                                    <div class="folder-sub">
                                        Disk Usage: {formatBytes(
                                            folderStatsMap[folder]
                                                .compressed_size,
                                        )} / {formatBytes(
                                            folderStatsMap[folder].total_size,
                                        )}
                                    </div>
                                {:else}
                                    <div class="folder-sub">
                                        Calculating stats...
                                    </div>
                                {/if}
                            </div>
                        </div>

                        {#if folderStatsMap[folder]}
                            <div class="compression-bar-container">
                                <div class="bar-labels">
                                    <span>Ratio</span>
                                    <span class="highlight"
                                        >{getSavedPercentage(
                                            folderStatsMap[folder].total_size,
                                            folderStatsMap[folder]
                                                .compressed_size,
                                        )}% Saved</span
                                    >
                                </div>
                                <div class="progress-track">
                                    <div
                                        class="progress-fill"
                                        style="width: {getCompressionRatio(
                                            folderStatsMap[folder].total_size,
                                            folderStatsMap[folder]
                                                .compressed_size,
                                        )}%"
                                    ></div>
                                </div>
                            </div>
                        {/if}

                        <button
                            class="revert-btn"
                            title="Decompress and revert to normal"
                            on:click={() => decompress(folder)}
                            disabled={state.isCompressing}
                        >
                            ↩ Unpack / Cancel
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .compactor-section {
        background: var(--layer-card);
        border: var(--border-glass);
        border-radius: 12px;
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .title-group {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .icon {
        font-size: 24px;
    }

    h3 {
        margin: 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--text-primary);
    }

    .description {
        margin: 0;
        color: var(--text-secondary);
        font-size: 14px;
        line-height: 1.5;
    }

    .controls-grid {
        display: grid;
        gap: 16px;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .input-group label {
        font-size: 13px;
        font-weight: 500;
        color: var(--text-secondary);
    }

    .row {
        display: flex;
        gap: 10px;
        width: 100%;
        position: relative; /* Context for dropdown */
    }

    .path-container {
        flex: 1;
        display: flex;
        gap: 8px;
        min-width: 0;
    }

    input[type="text"] {
        flex: 1;
        background: var(--bg-input, rgba(0, 0, 0, 0.2));
        border: var(--border-glass);
        color: var(--text-primary);
        padding: 10px 12px;
        border-radius: 8px;
        font-size: 14px;
        min-width: 0;
    }

    input[type="text"]:focus {
        outline: none;
        border-color: var(--accent);
    }

    .btn-icon {
        background: var(--bg-secondary);
        border: var(--border-glass);
        color: var(--text-primary);
        padding: 0 12px;
        border-radius: 8px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
        flex-shrink: 0;
    }

    .btn-icon:hover {
        background: var(--layer-hover);
    }

    /* --- Custom Select CSS --- */
    .custom-select-container {
        position: relative;
        flex: 1;
        min-width: 0;
    }

    .select-trigger {
        width: 100%;
        background: var(--bg-input, rgba(0, 0, 0, 0.2));
        border: var(--border-glass);
        color: var(--text-primary);
        padding: 10px 12px;
        border-radius: 8px;
        font-size: 14px;
        cursor: pointer;
        display: flex;
        justify-content: space-between;
        align-items: center;
        text-align: left;
        transition: border-color 0.2s;
    }

    .select-trigger:focus,
    .select-trigger.active {
        border-color: var(--accent);
        outline: none;
    }

    .select-trigger:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .arrow {
        font-size: 10px;
        color: var(--text-muted);
        margin-left: 8px;
    }

    .select-dropdown {
        position: absolute;
        top: calc(100% + 4px);
        left: 0;
        right: 0;
        background: var(--layer-card);
        backdrop-filter: blur(12px);
        border: var(--border-glass);
        border-radius: 8px;
        overflow: hidden;
        z-index: 100;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        animation: fadeIn 0.15s ease-out;
    }

    .select-option {
        padding: 10px 12px;
        font-size: 14px;
        color: var(--text-color);
        cursor: pointer;
        transition: background 0.1s;
    }

    .select-option:hover,
    .select-option:focus {
        background: var(--accent-color);
        color: white;
        outline: none;
    }

    .select-option.selected {
        background: rgba(59, 130, 246, 0.2);
        color: var(--accent-color);
    }

    button {
        padding: 10px 20px;
        border-radius: 8px;
        font-weight: 500;
        font-size: 14px;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
        white-space: nowrap;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn-primary {
        background: var(--accent-color);
        color: white;
    }
    .btn-primary:hover {
        opacity: 0.9;
    }

    .btn-secondary {
        background: var(--bg-secondary);
        color: var(--text-color);
        border: 1px solid var(--border-color);
    }
    .btn-secondary:hover {
        background: var(--bg-hover);
    }

    .status-bar {
        padding: 12px;
        border-radius: 8px;
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
        font-size: 13px;
        border-left: 4px solid var(--text-muted);
        animation: fadeIn 0.3s ease;
    }

    .status-bar.success {
        border-left-color: var(--success-color, #2ecc71);
        background: rgba(46, 204, 113, 0.1);
    }

    .status-bar.error {
        border-left-color: var(--danger-color, #e74c3c);
        background: rgba(231, 76, 60, 0.1);
    }

    .stats-card {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 16px;
        padding: 16px;
        background: var(--bg-secondary);
        border-radius: 8px;
    }

    .stat {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .stat .label {
        font-size: 12px;
        color: var(--text-muted);
    }

    .stat .value {
        font-size: 18px;
        font-weight: 600;
        color: var(--text-color);
    }

    /* Managed Section */
    .managed-section {
        margin-top: 10px;
        border-top: 1px solid var(--border-color);
        padding-top: 20px;
    }

    .managed-header {
        display: flex;
        align-items: center;
        gap: 10px;
        margin-bottom: 16px;
    }

    .managed-header h4 {
        margin: 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--text-color);
    }

    .badge {
        background: var(--bg-secondary);
        font-size: 11px;
        padding: 2px 8px;
        border-radius: 10px;
        color: var(--text-muted);
        border: 1px solid var(--border-color);
    }

    .empty-list {
        color: var(--text-muted);
        font-size: 13px;
        font-style: italic;
        padding: 16px;
        background: rgba(0, 0, 0, 0.1);
        border-radius: 8px;
        text-align: center;
        border: 1px dashed var(--border-color);
    }

    /* Grid Layout */
    .folders-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 16px;
    }

    .folder-card {
        background: var(--bg-secondary);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 16px;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        gap: 14px;
        position: relative;
    }

    .folder-card:hover {
        background: var(--bg-hover, rgba(255, 255, 255, 0.05));
        border-color: var(--accent-color);
        transform: translateY(-2px);
    }

    .card-header {
        display: flex;
        gap: 12px;
        align-items: flex-start;
    }

    .folder-icon {
        font-size: 24px;
        line-height: 1;
    }

    .folder-info {
        flex: 1;
        min-width: 0;
    }

    .folder-path {
        font-family: monospace;
        font-size: 13px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        color: var(--text-color);
        margin-bottom: 4px;
        font-weight: 600;
    }

    .folder-sub {
        font-size: 12px;
        color: var(--text-muted);
    }

    /* Progress Bar */
    .compression-bar-container {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .bar-labels {
        display: flex;
        justify-content: space-between;
        font-size: 11px;
        color: var(--text-muted);
    }

    .highlight {
        color: var(--success-color, #2ecc71);
        font-weight: 600;
    }

    .progress-track {
        height: 6px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        background: var(--accent-color);
        border-radius: 10px;
        transition: width 0.6s cubic-bezier(0.16, 1, 0.3, 1);
    }

    /* Revert Button */
    .revert-btn {
        background: transparent;
        border: 1px solid var(--danger-color, #ef4444);
        color: var(--danger-color, #ef4444);
        padding: 8px;
        font-size: 13px;
        border-radius: 8px;
        margin-top: auto;
        transition: all 0.2s;
    }

    .revert-btn:hover {
        background: rgba(239, 68, 68, 0.1);
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(-4px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    /* Live Progress Section */
    .live-progress {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: 12px;
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 10px;
        animation: fadeIn 0.3s ease;
    }

    .progress-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .progress-label {
        font-weight: 600;
        font-size: 14px;
        color: var(--accent-color);
    }

    .progress-count {
        font-size: 13px;
        color: var(--text-muted);
        font-family: monospace;
    }

    .progress-bar-track {
        height: 8px;
        background: rgba(0, 0, 0, 0.3);
        border-radius: 10px;
        overflow: hidden;
    }

    .progress-bar-fill {
        height: 100%;
        background: linear-gradient(90deg, var(--accent-color), #60a5fa);
        border-radius: 10px;
        transition: width 0.3s ease;
    }

    .progress-bar-fill.indeterminate {
        width: 30% !important;
        animation: indeterminate 2s infinite linear;
        background: linear-gradient(90deg, var(--accent-color), #ffffff);
    }

    @keyframes indeterminate {
        0% {
            transform: translateX(-150%);
        }
        100% {
            transform: translateX(350%);
        }
    }

    .bytes-analyzed {
        font-size: 12px;
        color: var(--accent-color);
        font-family: monospace;
        font-weight: 500;
        white-space: nowrap;
    }

    /* Current File Display */
    .current-file-display {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 12px;
        background: rgba(0, 0, 0, 0.2);
        border-radius: 6px;
        overflow: hidden;
    }

    .file-icon {
        flex-shrink: 0;
    }

    .file-name {
        font-size: 12px;
        color: var(--text-muted);
        font-family: monospace;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    /* Detailed Stats Panel */
    .detailed-stats {
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding: 12px;
        background: rgba(0, 0, 0, 0.15);
        border-radius: 8px;
    }

    .stat-summary {
        font-size: 14px;
        font-weight: 500;
        color: var(--text-color);
    }

    .saved-info {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    /* Stats Legend */
    .stats-legend {
        display: flex;
        flex-wrap: wrap;
        gap: 16px;
    }

    .legend-item {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .legend-color {
        width: 14px;
        height: 14px;
        border-radius: 3px;
        flex-shrink: 0;
    }

    .legend-color.compressed {
        background: #10b981; /* Green */
    }

    .legend-color.compressible {
        background: #3b82f6; /* Blue */
    }

    .legend-color.excluded {
        background: #f59e0b; /* Orange */
    }

    .legend-text {
        font-size: 13px;
        color: var(--text-muted);
    }

    /* Stacked Bar Chart */
    .stacked-bar {
        display: flex;
        height: 12px;
        border-radius: 6px;
        overflow: hidden;
        background: rgba(0, 0, 0, 0.3);
    }

    .bar-segment {
        height: 100%;
        transition: width 0.3s ease;
    }

    .bar-segment.compressed {
        background: #10b981;
    }

    .bar-segment.compressible {
        background: #3b82f6;
    }

    .bar-segment.excluded {
        background: #f59e0b;
    }

    .main-bar {
        height: 10px;
    }
</style>
