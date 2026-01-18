<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";
    import { onMount } from "svelte";
    import {
        Loader,
        CheckCircle,
        XCircle,
        Download,
        Trash2,
        X,
        Package,
        Search as SearchIcon,
    } from "lucide-svelte";

    interface Program {
        id: string;
        name: string;
        description: string;
        category: string;
        installed?: boolean;
    }

    type ProcessingStatus =
        | "queued"
        | "installing"
        | "installed"
        | "uninstalling"
        | "removed"
        | "error"
        | "cancelled";

    let allPrograms: Program[] = [];
    let isLoadingCatalog = true;
    let isCancelled = false;

    onMount(async () => {
        try {
            allPrograms = await invoke("get_popular_packages");
        } catch (e) {
            console.error("Failed to load catalog", e);
        } finally {
            isLoadingCatalog = false;
        }
    });

    let activeTab = "";
    let categories: string[] = [];

    $: {
        if (allPrograms.length > 0) {
            const cats = new Set(allPrograms.map((p) => p.category));
            categories = Array.from(cats).sort();
            if (!activeTab && categories.length > 0) {
                activeTab = categories[0];
            }
        }
    }

    let processingMap: Record<string, ProcessingStatus> = {};
    let selectedIds = new Set<string>();

    let searchQuery = "";
    let searchResults: Program[] = [];
    let isSearching = false;

    $: displayPrograms =
        searchQuery.length > 0
            ? searchResults
            : allPrograms.filter((p) => p.category === activeTab);

    $: isProcessing = Object.values(processingMap).some(
        (s) => s === "installing" || s === "uninstalling" || s === "queued",
    );

    async function handleSearch(e: KeyboardEvent) {
        if (e.key === "Enter" && searchQuery.trim().length > 0) {
            isSearching = true;
            searchResults = [];
            try {
                searchResults = await invoke<Program[]>("search_packages", {
                    query: searchQuery,
                });
                searchResults = searchResults.map((p) => ({
                    ...p,
                    category: "Search Result",
                }));
            } catch (err) {
                console.error("Search failed", err);
            } finally {
                isSearching = false;
            }
        } else if (searchQuery.length === 0) {
            searchResults = [];
        }
    }

    function toggleSelection(id: string, event: Event) {
        event.stopPropagation();
        if (selectedIds.has(id)) {
            selectedIds.delete(id);
        } else {
            selectedIds.add(id);
        }
        selectedIds = selectedIds;
    }

    function clearSelection() {
        selectedIds.clear();
        selectedIds = selectedIds;
    }

    async function install(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "installing";
        processingMap = processingMap;
        try {
            await invoke("install_package", { id });
            processingMap[id] = "installed";
        } catch (e) {
            console.error(e);
            processingMap[id] = "error";
        }
        processingMap = processingMap;
        setTimeout(() => {
            if (
                processingMap[id] === "installed" ||
                processingMap[id] === "error"
            ) {
                delete processingMap[id];
                processingMap = processingMap;
            }
        }, 4000);
    }

    async function installSelected() {
        const ids = Array.from(selectedIds);
        if (ids.length === 0) return;

        isCancelled = false;
        ids.forEach((id) => (processingMap[id] = "queued"));
        processingMap = processingMap;

        for (const id of ids) {
            if (isCancelled) {
                processingMap[id] = "cancelled";
                continue;
            }
            processingMap[id] = "installing";
            processingMap = processingMap;
            try {
                await invoke("install_package", { id });
                processingMap[id] = "installed";
                selectedIds.delete(id);
            } catch (e) {
                console.error(e);
                processingMap[id] = "error";
            }
            processingMap = processingMap;
        }
        selectedIds = selectedIds;

        // Clear statuses after delay
        setTimeout(() => {
            ids.forEach((id) => {
                if (
                    processingMap[id] === "installed" ||
                    processingMap[id] === "cancelled"
                ) {
                    delete processingMap[id];
                }
            });
            processingMap = processingMap;
        }, 4000);
    }

    function cancelQueue() {
        isCancelled = true;
        // Update any queued items to cancelled
        for (const id in processingMap) {
            if (processingMap[id] === "queued") {
                processingMap[id] = "cancelled";
            }
        }
        processingMap = processingMap;
    }

    async function uninstall(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "uninstalling";
        processingMap = processingMap;
        try {
            await invoke("uninstall_package", { id });
            processingMap[id] = "removed";
        } catch (e) {
            console.error(e);
            processingMap[id] = "error";
        }
        processingMap = processingMap;
        setTimeout(() => {
            if (
                processingMap[id] === "removed" ||
                processingMap[id] === "error"
            ) {
                delete processingMap[id];
                processingMap = processingMap;
            }
        }, 4000);
    }

    function getStatusIcon(status: ProcessingStatus) {
        switch (status) {
            case "queued":
            case "installing":
            case "uninstalling":
                return Loader;
            case "installed":
            case "removed":
                return CheckCircle;
            case "error":
                return XCircle;
            case "cancelled":
                return X;
            default:
                return null;
        }
    }

    function getStatusClass(status: ProcessingStatus) {
        switch (status) {
            case "queued":
            case "installing":
            case "uninstalling":
                return "processing";
            case "installed":
            case "removed":
                return "success";
            case "error":
                return "error";
            case "cancelled":
                return "cancelled";
            default:
                return "";
        }
    }

    function getStatusText(status: ProcessingStatus) {
        switch (status) {
            case "queued":
                return "Queued";
            case "installing":
                return "Installing...";
            case "uninstalling":
                return "Removing...";
            case "installed":
                return "Installed";
            case "removed":
                return "Removed";
            case "error":
                return "Failed";
            case "cancelled":
                return "Cancelled";
            default:
                return "";
        }
    }
</script>

<div class="programs-container" in:fade>
    <!-- Header with tabs and search -->
    <div class="header">
        <div class="tabs">
            {#each categories as cat}
                <button
                    class:active={activeTab === cat && searchQuery.length === 0}
                    on:click={() => {
                        activeTab = cat;
                        searchQuery = "";
                        searchResults = [];
                    }}
                >
                    {cat}
                </button>
            {/each}
        </div>

        <div class="search-wrapper">
            <SearchIcon size={14} />
            <input
                type="text"
                placeholder="Search packages (Enter)..."
                bind:value={searchQuery}
                on:keydown={handleSearch}
                class="search-input"
            />
            {#if isSearching}
                <span class="spinner"><Loader size={14} /></span>
            {/if}
        </div>
    </div>

    <!-- Program Grid -->
    <div class="program-grid">
        {#if isLoadingCatalog}
            <div class="loading-state">
                <Loader size={24} class="spin" />
                <span>Loading catalog...</span>
            </div>
        {:else if displayPrograms.length === 0}
            <div class="empty-state">
                <Package size={48} />
                <p>No packages found</p>
            </div>
        {:else}
            {#each displayPrograms as prog}
                <div class="card" class:selected={selectedIds.has(prog.id)}>
                    <div class="card-header">
                        <label class="checkbox-wrapper">
                            <input
                                type="checkbox"
                                checked={selectedIds.has(prog.id)}
                                on:change={(e) => toggleSelection(prog.id, e)}
                                disabled={!!processingMap[prog.id]}
                            />
                        </label>
                        <h3 title={prog.name}>{prog.name}</h3>
                    </div>

                    <p class="desc">{prog.description}</p>
                    <code class="pkg-id">{prog.id}</code>

                    <div class="card-actions">
                        {#if processingMap[prog.id]}
                            <div
                                class="status-badge {getStatusClass(
                                    processingMap[prog.id],
                                )}"
                            >
                                <svelte:component
                                    this={getStatusIcon(processingMap[prog.id])}
                                    size={14}
                                />
                                {getStatusText(processingMap[prog.id])}
                            </div>
                        {:else}
                            <button
                                class="action-btn install"
                                on:click={() => install(prog.id)}
                            >
                                <Download size={12} />
                                Install
                            </button>
                            <button
                                class="action-btn uninstall"
                                on:click={() => uninstall(prog.id)}
                            >
                                <Trash2 size={12} />
                                Remove
                            </button>
                        {/if}
                    </div>
                </div>
            {/each}
        {/if}
    </div>

    <!-- Sticky Footer with Bulk Actions -->
    <div class="footer-bar">
        <div class="selection-info">
            <span class="count">{selectedIds.size} selected</span>
            {#if selectedIds.size > 0}
                <button class="clear-btn" on:click={clearSelection}>
                    <X size={12} />
                    Clear
                </button>
            {/if}
        </div>

        <div class="bulk-actions">
            {#if isProcessing}
                <button class="cancel-btn" on:click={cancelQueue}>
                    <X size={14} />
                    Cancel Queue
                </button>
            {/if}
            <button
                class="install-btn"
                on:click={installSelected}
                disabled={selectedIds.size === 0 || isProcessing}
            >
                <Download size={14} />
                Install Selected ({selectedIds.size})
            </button>
        </div>
    </div>
</div>

<style>
    .programs-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    /* Header */
    .header {
        display: flex;
        align-items: center;
        padding: 16px 24px;
        gap: 16px;
        border-bottom: var(--border-glass);
        flex-shrink: 0;
    }

    .tabs {
        display: flex;
        gap: 8px;
        overflow-x: auto;
        flex: 1;
    }

    .tabs::-webkit-scrollbar {
        height: 4px;
    }

    .tabs button {
        background: transparent;
        border: 1px solid transparent;
        color: var(--text-muted);
        padding: 8px 16px;
        border-radius: 20px;
        cursor: pointer;
        font-size: 13px;
        font-weight: 500;
        white-space: nowrap;
        transition: all 0.2s;
    }

    .tabs button:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-primary);
    }

    .tabs button.active {
        background: var(--accent-color);
        color: white;
    }

    .search-wrapper {
        display: flex;
        align-items: center;
        gap: 8px;
        background: rgba(255, 255, 255, 0.05);
        border: var(--border-glass);
        border-radius: 20px;
        padding: 8px 12px;
        color: var(--text-muted);
    }

    .search-input {
        background: transparent;
        border: none;
        color: var(--text-primary);
        font-size: 13px;
        outline: none;
        width: 180px;
    }

    .spinner {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    /* Grid - Fixed card widths prevent stretching */
    .program-grid {
        flex: 1;
        overflow-y: auto;
        padding: 24px;
        padding-bottom: 100px; /* Space for footer */
        display: grid;
        /* Fixed width columns - cards won't stretch */
        grid-template-columns: repeat(auto-fill, 280px);
        /* Left-align grid, leaves empty space on right when few items */
        justify-content: start;
        gap: 16px;
        min-height: 0;
    }

    /* Center the grid content on larger screens */
    @media (min-width: 1200px) {
        .program-grid {
            justify-content: center;
        }
    }

    .loading-state,
    .empty-state {
        grid-column: 1 / -1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;
        padding: 64px;
        color: var(--text-muted);
    }

    /* Cards */
    .card {
        background: var(--layer-card);
        backdrop-filter: blur(12px);
        border: var(--border-glass);
        border-radius: var(--radius-lg, 16px);
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 12px;
        transition: all 0.2s;
    }

    .card:hover {
        border-color: var(--toggle-on-border);
        background: var(--layer-hover);
    }

    .card.selected {
        border-color: var(--toggle-on-border);
        background: var(--toggle-on-bg);
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .checkbox-wrapper input {
        width: 18px;
        height: 18px;
        accent-color: var(--accent-color);
        cursor: pointer;
    }

    .card-header h3 {
        margin: 0;
        font-size: 15px;
        font-weight: 600;
        color: var(--text-primary);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        flex: 1;
    }

    .desc {
        font-size: 13px;
        color: var(--text-muted);
        margin: 0;
        line-height: 1.4;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
        flex: 1;
    }

    .pkg-id {
        font-size: 11px;
        color: var(--text-muted);
        background: rgba(0, 0, 0, 0.2);
        padding: 4px 8px;
        border-radius: 6px;
        font-family: monospace;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .card-actions {
        display: flex;
        gap: 8px;
        margin-top: auto;
    }

    .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        border: none;
        padding: 8px 12px;
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
    }

    .action-btn.install {
        background: var(--btn-safe-bg);
        border: 1px solid var(--btn-safe-border);
        color: var(--btn-safe-color);
    }

    .action-btn.install:hover {
        background: var(--btn-safe-hover-bg);
    }

    .action-btn.uninstall {
        background: var(--toggle-off-bg);
        border: 1px solid var(--toggle-off-border);
        color: var(--text-muted);
    }

    .action-btn.uninstall:hover {
        background: rgba(255, 255, 255, 0.08);
        color: var(--text-primary);
    }

    /* Status Badge */
    .status-badge {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 8px 16px;
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        width: 100%;
    }

    .status-badge.processing {
        background: rgba(96, 205, 255, 0.1);
        color: var(--accent-color);
    }

    .status-badge.processing :global(svg) {
        animation: spin 1s linear infinite;
    }

    .status-badge.success {
        background: rgba(34, 197, 94, 0.1);
        color: #22c55e;
    }

    .status-badge.error {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
    }

    .status-badge.cancelled {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-muted);
    }

    /* Footer Bar */
    .footer-bar {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 16px 24px;
        background: rgba(20, 30, 45, 0.9);
        backdrop-filter: blur(16px);
        border-top: var(--border-glass);
    }

    .selection-info {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .count {
        font-size: 14px;
        color: var(--text-muted);
    }

    .clear-btn {
        display: flex;
        align-items: center;
        gap: 4px;
        background: transparent;
        border: none;
        color: var(--text-muted);
        font-size: 12px;
        cursor: pointer;
        padding: 4px 8px;
        border-radius: 4px;
    }

    .clear-btn:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-primary);
    }

    .bulk-actions {
        display: flex;
        gap: 12px;
    }

    .install-btn {
        display: flex;
        align-items: center;
        gap: 8px;
        background: var(--btn-safe-bg);
        border: 1px solid var(--btn-safe-border);
        color: var(--btn-safe-color);
        padding: 10px 20px;
        border-radius: 10px;
        font-size: 14px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
    }

    .install-btn:hover:not(:disabled) {
        background: var(--btn-safe-hover-bg);
    }

    .install-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .cancel-btn {
        display: flex;
        align-items: center;
        gap: 6px;
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
        color: #ef4444;
        padding: 10px 16px;
        border-radius: 10px;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
    }

    .cancel-btn:hover {
        background: rgba(239, 68, 68, 0.2);
    }
</style>
