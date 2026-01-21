<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        RefreshCw,
        List,
        Key,
        Calendar,
        Settings,
        Folder,
        Globe,
        Rocket,
        MapPin,
        Building,
        AlertTriangle,
        Circle,
    } from "lucide-svelte";

    type StartupCategory = string; // Backend uses strings now (Logon, Service etc)

    interface StartupItem {
        id: string;
        name: string;
        category: string;
        subcategory: string;
        location: string;
        command: string;
        enabled: boolean;
        publisher?: string;
        description?: string;
        source: string;
        safety_rating: "Safe" | "Careful" | "Dangerous" | "Unknown";
        file_exists: boolean;
    }

    let items: StartupItem[] = [];
    let filteredItems: StartupItem[] = [];
    let loading = true;
    let searchTerm = "";
    let selectedCategory = "all";

    // Category definitions with counts and icon components
    const categoryIcons: Record<string, typeof List> = {
        all: List,
        Logon: Key,
        ScheduledTask: Calendar,
        Service: Settings,
        Explorer: Folder,
        Browser: Globe,
        Boot: Rocket,
    };

    let categories = [
        { id: "all", label: "All Items", count: 0 },
        { id: "Logon", label: "Logon", count: 0 },
        { id: "ScheduledTask", label: "Tasks", count: 0 },
        { id: "Service", label: "Services", count: 0 },
        { id: "Explorer", label: "Explorer", count: 0 },
        { id: "Browser", label: "Browser", count: 0 },
        { id: "Boot", label: "Boot", count: 0 },
    ];

    onMount(async () => {
        await refresh();
    });

    async function refresh() {
        try {
            loading = true;
            items = await invoke<StartupItem[]>("scan_startup");
            console.log("Items loaded:", items);

            categories = categories.map((cat) => ({
                ...cat,
                count: items.filter(
                    (item) => cat.id === "all" || item.category === cat.id,
                ).length,
            }));

            filterItems();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    function filterItems() {
        filteredItems = items.filter((item) => {
            const matchesCategory =
                selectedCategory === "all" ||
                item.category === selectedCategory;
            const q = searchTerm.toLowerCase();
            const matchesSearch =
                !searchTerm ||
                item.name.toLowerCase().includes(q) ||
                item.command.toLowerCase().includes(q) ||
                (item.publisher && item.publisher.toLowerCase().includes(q));
            return matchesCategory && matchesSearch;
        });
    }

    async function toggleItem(item: StartupItem) {
        const wasEnabled = item.enabled;
        const newValue = !wasEnabled;
        item.enabled = newValue;
        filteredItems = [...filteredItems];

        try {
            await invoke("set_startup_item_enabled", {
                id: item.id,
                enable: newValue,
            });
        } catch (error) {
            item.enabled = wasEnabled;
            filteredItems = [...filteredItems];
            console.error("Toggle failed:", error);
            alert(`Toggle failed: ${error}`);
        }
    }

    $: {
        searchTerm;
        selectedCategory;
        if (items.length) filterItems();
    }

    // Truncate helper
    function truncate(str: string, max: number): string {
        return str.length > max ? str.slice(0, max) + "..." : str;
    }
</script>

<div class="startup-manager">
    <div class="header">
        <h1>Startup Manager</h1>
        <div class="header-actions">
            <button class="btn-primary" on:click={refresh}>
                <RefreshCw size={14} />
                Refresh
            </button>
        </div>
    </div>

    <div class="controls">
        <input
            class="search-input"
            type="text"
            placeholder="Search by name, path, or publisher..."
            bind:value={searchTerm}
        />

        <div class="category-tabs">
            {#each categories as cat}
                <button
                    class:active={selectedCategory === cat.id}
                    class:warning={cat.id !== "all" && cat.count === 0}
                    on:click={() => (selectedCategory = cat.id)}
                >
                    <span class="icon">
                        <svelte:component
                            this={categoryIcons[cat.id]}
                            size={14}
                        />
                    </span>
                    {cat.label}
                    <span class="count">{cat.count}</span>
                </button>
            {/each}
        </div>
    </div>

    {#if loading}
        <div class="loading-state">
            <div class="spinner"></div>
            <p>Scanning startup entries...</p>
        </div>
    {:else if filteredItems.length === 0}
        <div class="empty-state">
            <p>No startup items found for this filter.</p>
            <button on:click={refresh}>
                <RefreshCw size={14} />
                Refresh
            </button>
        </div>
    {:else}
        <div class="items-grid">
            {#each filteredItems as item}
                <div
                    class="startup-item"
                    class:safe={item.safety_rating === "Safe"}
                    class:careful={item.safety_rating === "Careful"}
                    class:dangerous={item.safety_rating === "Dangerous"}
                    class:unknown={item.safety_rating === "Unknown"}
                    class:disabled={!item.enabled}
                >
                    <div class="toggle-container">
                        <label class="toggle-switch">
                            <input
                                type="checkbox"
                                checked={item.enabled}
                                on:change={() => toggleItem(item)}
                            />
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="content">
                        <div class="header-line">
                            <span class="name" title={item.name}
                                >{item.name}</span
                            >
                            <!-- Safety Badge -->
                            <div
                                class="safety-badge {item.safety_rating.toLowerCase()}"
                                title="Safety Rating"
                            >
                                <Circle size={8} />
                                {item.safety_rating}
                            </div>
                        </div>

                        <div class="details">
                            <div class="subcategory">
                                <MapPin size={12} />
                                {item.subcategory}
                            </div>

                            <div class="command" title={item.command}>
                                {truncate(item.command, 60)}
                            </div>

                            {#if item.publisher}
                                <div class="publisher">
                                    <Building size={12} />
                                    {truncate(item.publisher, 40)}
                                </div>
                            {/if}

                            <!-- File Missing Warning -->
                            {#if !item.file_exists && item.source !== "Service"}
                                <div class="warning-badge">
                                    <AlertTriangle size={12} />
                                    File not found
                                </div>
                            {/if}
                        </div>
                    </div>

                    <div class="category-badge">{item.category}</div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .startup-manager {
        padding: 24px;
        height: 100%;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        color: var(--text-color);
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 24px;
    }
    h1 {
        margin: 0;
        font-size: 24px;
        font-weight: 600;
    }

    .btn-primary {
        background: var(--accent-color);
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: 8px;
        cursor: pointer;
    }
    .btn-primary:hover {
        opacity: 0.9;
    }

    .controls {
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin-bottom: 24px;
    }

    .search-input {
        padding: 10px;
        border: 1px solid var(--border-color);
        background: var(--bg-input, #222);
        color: var(--text-color);
        border-radius: 8px;
    }

    .category-tabs {
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
    }

    .category-tabs button {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 6px 12px;
        border: 1px solid var(--border-color);
        background: var(--bg-card);
        color: var(--text-muted);
        border-radius: 6px;
        cursor: pointer;
        transition: 0.2s;
    }
    .category-tabs button:hover {
        background: var(--bg-hover);
        color: var(--text-color);
    }
    .category-tabs button.active {
        background: var(--accent-color);
        color: white;
        border-color: var(--accent-color);
    }
    .count {
        background: rgba(0, 0, 0, 0.2);
        border-radius: 10px;
        padding: 1px 5px;
        font-size: 11px;
    }

    .items-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 20px;
        overflow-y: auto;
        padding-bottom: 20px;
    }

    .startup-item {
        display: flex;
        gap: 12px;
        padding: 16px;
        background: var(--bg-card);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        position: relative;
        border-left: 4px solid var(--text-muted); /* Default unknown */
    }

    .startup-item.safe {
        border-left-color: var(--success-color, #2ecc71);
    }
    .startup-item.careful {
        border-left-color: var(--warning-color, #f1c40f);
    }
    .startup-item.dangerous {
        border-left-color: var(--danger-color, #e74c3c);
    }
    .startup-item.disabled {
        opacity: 0.6;
        filter: grayscale(0.5);
    }

    .toggle-container {
        padding-top: 4px;
    }
    .toggle-switch {
        position: relative;
        display: inline-block;
        width: 40px;
        height: 22px;
    }
    .toggle-switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }
    .slider {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: var(--bg-secondary);
        border-radius: 22px;
        transition: 0.2s;
    }
    .slider:before {
        content: "";
        position: absolute;
        height: 16px;
        width: 16px;
        left: 3px;
        bottom: 3px;
        background: white;
        border-radius: 50%;
        transition: 0.2s;
    }
    input:checked + .slider {
        background: var(--accent-color);
    }
    input:checked + .slider:before {
        transform: translateX(18px);
    }

    .content {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .header-line {
        display: flex;
        justify-content: space-between;
        align-items: start;
        gap: 8px;
    }
    .name {
        font-weight: 600;
        font-size: 15px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .safety-badge {
        font-size: 11px;
        white-space: nowrap;
        display: flex;
        align-items: center;
        gap: 4px;
    }
    .safety-badge.safe :global(svg) {
        color: #22c55e;
        fill: #22c55e;
    }
    .safety-badge.careful :global(svg) {
        color: #facc15;
        fill: #facc15;
    }
    .safety-badge.dangerous :global(svg) {
        color: #f97316;
        fill: #f97316;
    }
    .safety-badge.unknown :global(svg) {
        color: rgba(255, 255, 255, 0.5);
        fill: rgba(255, 255, 255, 0.5);
    }

    .details {
        display: flex;
        flex-direction: column;
        gap: 2px;
        font-size: 12px;
        color: var(--text-muted);
    }

    .subcategory {
        font-weight: 500;
        color: var(--text-secondary);
    }
    .command {
        font-family: monospace;
        background: var(--bg-secondary);
        padding: 2px 4px;
        border-radius: 4px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
    .publisher {
        font-style: italic;
    }

    .warning-badge {
        color: var(--danger-color);
        font-weight: 600;
        margin-top: 4px;
    }

    .category-badge {
        position: absolute;
        bottom: 12px;
        right: 12px;
        font-size: 10px;
        background: var(--bg-secondary);
        padding: 2px 6px;
        border-radius: 10px;
        opacity: 0.7;
    }

    .loading-state,
    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 16px;
        color: var(--text-muted);
    }
    .spinner {
        width: 32px;
        height: 32px;
        border: 3px solid var(--border-color);
        border-top-color: var(--accent-color);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
