<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";

    interface Program {
        id: string;
        name: string;
        description: string;
        category: string;
        installed?: boolean; // We might verify this
    }

    let allPrograms: Program[] = [];
    let isLoadingCatalog = true;

    import { onMount } from "svelte";

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

    // Reactive categories
    $: {
        if (allPrograms.length > 0) {
            const cats = new Set(allPrograms.map((p) => p.category));
            categories = Array.from(cats).sort();
            if (!activeTab && categories.length > 0) {
                activeTab = categories[0];
            }
        }
    }

    let processingMap: Record<string, string> = {}; // id -> status string
    let selectedIds = new Set<string>(); // IDs selected for bulk actions

    let searchQuery = "";
    let searchResults: Program[] = [];
    let isSearching = false;

    $: displayPrograms =
        searchQuery.length > 0
            ? searchResults
            : allPrograms.filter((p) => p.category === activeTab);

    async function handleSearch(e: KeyboardEvent) {
        if (e.key === "Enter" && searchQuery.trim().length > 0) {
            isSearching = true;
            searchResults = [];
            try {
                searchResults = await invoke<Program[]>("search_packages", {
                    query: searchQuery,
                });
                // Map search results to have "Search Result" category
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

    function toggleSelection(id: string) {
        if (selectedIds.has(id)) {
            selectedIds.delete(id);
        } else {
            selectedIds.add(id);
        }
        selectedIds = selectedIds; // Trigger reactivity
    }

    async function install(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "Installing...";
        try {
            await invoke("install_package", { id });
            processingMap[id] = "Installed ✅";
        } catch (e) {
            console.error(e);
            processingMap[id] = "Error ❌";
        } finally {
            // keep status for a bit
            setTimeout(() => {
                if (processingMap[id] === "Installed ✅")
                    delete processingMap[id];
            }, 3000);
        }
    }

    async function installSelected() {
        const ids = Array.from(selectedIds);
        if (ids.length === 0) return;

        ids.forEach((id) => (processingMap[id] = "Queued..."));

        try {
            await invoke("install_packages_bulk", { ids });
            ids.forEach((id) => {
                processingMap[id] = "Installed ✅";
                selectedIds.delete(id);
            });
            selectedIds = selectedIds;
        } catch (e) {
            console.error(e);
            ids.forEach((id) => (processingMap[id] = "Error ❌"));
        } finally {
            ids.forEach((id) => {
                setTimeout(() => {
                    if (processingMap[id] === "Installed ✅")
                        delete processingMap[id];
                }, 3000);
            });
        }
    }

    async function uninstall(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "Uninstalling...";
        try {
            await invoke("uninstall_package", { id });
            processingMap[id] = "Uninstalled 🗑️";
        } catch (e) {
            console.error(e);
            processingMap[id] = "Error ❌";
        } finally {
            setTimeout(() => {
                if (processingMap[id] === "Uninstalled 🗑️")
                    delete processingMap[id];
            }, 3000);
        }
    }
</script>

<div class="programs-container" in:fade>
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

        <div class="search-wrapper">
            <input
                type="text"
                placeholder="Search packages (Enter)..."
                bind:value={searchQuery}
                on:keydown={handleSearch}
                class="search-input"
            />
            {#if isSearching}
                <span class="spinner">⌛</span>
            {/if}
        </div>

        {#if selectedIds.size > 0}
            <button class="bulk-install-btn" on:click={installSelected} in:fade>
                Install Selected ({selectedIds.size})
            </button>
        {/if}
    </div>

    <div class="program-grid">
        {#if isLoadingCatalog}
            <div class="loading-state">
                <span class="spinner">⌛</span> Loading catalog...
            </div>
        {:else}
            {#each displayPrograms as prog}
                <div
                    class="card"
                    class:selected={selectedIds.has(prog.id)}
                    role="button"
                    tabindex="0"
                    on:click={() => toggleSelection(prog.id)}
                    on:keydown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                            e.preventDefault();
                            toggleSelection(prog.id);
                        }
                    }}
                >
                    <div class="card-content">
                        <div class="card-header-row">
                            <h3>{prog.name}</h3>
                            <input
                                type="checkbox"
                                checked={selectedIds.has(prog.id)}
                                tabindex="-1"
                            />
                        </div>
                        <p class="desc">{prog.description}</p>
                        <code class="pkg-id">{prog.id}</code>
                    </div>
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <div
                        class="card-actions"
                        on:click|stopPropagation
                        on:keydown|stopPropagation
                    >
                        {#if processingMap[prog.id]}
                            <div class="status">{processingMap[prog.id]}</div>
                        {:else}
                            <button
                                class="action-btn install"
                                on:click={() => install(prog.id)}
                                >Install</button
                            >
                            <button
                                class="action-btn uninstall"
                                on:click={() => uninstall(prog.id)}
                                >Uninstall</button
                            >
                        {/if}
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>

<style>
    .programs-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        position: relative;
    }

    .bulk-install-btn {
        background: #10b981;
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: 20px;
        cursor: pointer;
        font-weight: 600;
        animation: pulse 2s infinite;
        white-space: nowrap;
    }

    .search-wrapper {
        margin-left: auto;
        display: flex;
        align-items: center;
        gap: 8px;
        position: relative;
    }

    .search-input {
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        color: var(--text-color);
        padding: 8px 12px;
        border-radius: 20px;
        font-size: 13px;
        outline: none;
        width: 200px;
        transition: all 0.2s;
    }

    .search-input:focus {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
        width: 240px;
    }

    .tabs {
        display: flex;
        padding: 16px 24px;
        gap: 8px;
        border-bottom: 1px solid var(--border-color);
        overflow-x: auto;
        align-items: center;
        flex-shrink: 0;
        min-height: 56px;
    }

    .tabs::-webkit-scrollbar {
        height: 4px;
    }

    .tabs::-webkit-scrollbar-track {
        background: transparent;
    }

    .tabs::-webkit-scrollbar-thumb {
        background: var(--border-color);
        border-radius: 4px;
    }

    .tabs button {
        background: transparent;
        border: 1px solid transparent;
        color: var(--text-muted);
        padding: 8px 16px;
        border-radius: 20px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        transition: all 0.2s;
    }

    .tabs button:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
    }

    .tabs button.active {
        background: var(--accent-color);
        color: white;
    }

    .program-grid {
        flex: 1;
        overflow-y: auto;
        padding: 24px;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 20px;
        min-height: 0;
    }

    .card {
        background: var(--surface-1);
        backdrop-filter: blur(12px);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 16px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        gap: 12px;
        cursor: pointer;
        transition: all 0.2s;
        overflow: hidden;
        min-height: 160px;
    }

    .card.selected {
        border-color: var(--accent-color);
        background: rgba(59, 130, 246, 0.05);
    }

    .card:hover {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.06);
    }

    .card-content {
        flex: 1;
        overflow: hidden;
    }

    .card-header-row {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 8px;
        margin-bottom: 8px;
    }

    .card-header-row input[type="checkbox"] {
        accent-color: var(--accent-color);
        cursor: pointer;
        flex-shrink: 0;
    }

    .loading-state {
        grid-column: 1 / -1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
        padding: 48px;
        color: var(--text-muted);
        font-size: 16px;
    }

    .card-content h3 {
        margin: 0;
        font-size: 15px;
        color: var(--text-color);
        font-weight: 600;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        flex: 1;
    }

    .desc {
        font-size: 13px;
        color: var(--text-muted);
        margin: 0 0 8px 0;
        line-height: 1.4;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }

    .pkg-id {
        font-size: 11px;
        color: var(--text-muted);
        background: rgba(0, 0, 0, 0.2);
        padding: 2px 6px;
        border-radius: 4px;
        font-family: monospace;
        display: inline-block;
        max-width: 100%;
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
        border: none;
        padding: 6px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
    }
    .action-btn.install {
        background: #10b981;
        color: white;
    }
    .action-btn.uninstall {
        background: rgba(255, 255, 255, 0.1);
        color: var(--text-color);
    }
    .action-btn:hover {
        opacity: 0.9;
    }

    .status {
        width: 100%;
        text-align: center;
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        padding: 6px;
    }
</style>
