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
        Sparkles,
        Check,
        Shield,
    } from "lucide-svelte";
    import StartupScanReport from "./StartupScanReport.svelte";

    type StartupCategory = string;

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
        safety_rating: "Safe" | "Careful" | "Dangerous" | "Unknown" | "Critical";
        file_exists: boolean;
    }

    let items: StartupItem[] = [];
    let filteredItems: StartupItem[] = [];
    let loading = true;
    let searchTerm = "";
    let selectedCategory = "all";

    let scanResult: any = null;
    let scanning = false;
    let applying = false;
    let applyResults: string[] = [];
    let scanError = "";
    let selectedRecs: Set<string> = new Set();
    let selectedRec: any = null;
    let showRecDetail = false;
    let view: "items" | "scan" = "items";

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

    async function runAiScan() {
        scanning = true;
        scanError = "";
        scanResult = null;
        try {
            scanResult = await invoke<any>("ai_scan_startup", { items });
            selectedRecs = new Set(
                scanResult.recommendations
                    .filter((r: any) => r.action === "disable" && r.priority !== "low")
                    .map((r: any) => r.item_id)
            );
            view = "scan";
        } catch (e) {
            scanError = e as string;
        } finally {
            scanning = false;
        }
    }

    async function applySelected() {
        applying = true;
        const toApply = scanResult.recommendations.filter(
            (r: any) => r.action === "disable" && selectedRecs.has(r.item_id)
        );
        try {
            applyResults = await invoke<string[]>("ai_apply_startup_recommendations", {
                recommendationsJson: JSON.stringify(toApply),
            });
            for (const rec of toApply) {
                await invoke("record_ai_memory", {
                    kindJson: JSON.stringify({
                        type: "StartupAction",
                        item_id: rec.item_id,
                        item_name: rec.item_id,
                        action: "disabled",
                        reason: rec.reason,
                    }),
                });
            }
            await refresh();
        } catch (e) {
            scanError = e as string;
        } finally {
            applying = false;
        }
    }

    function priorityColor(p: string) {
        return p === "high" ? "#f87171" : p === "medium" ? "#fbbf24" : "#34d399";
    }

    function openRecDetail(rec: any) {
        selectedRec = rec;
        showRecDetail = true;
    }

    function closeRecDetail() {
        showRecDetail = false;
        selectedRec = null;
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

    {#if view === "scan" && scanResult}
        <StartupScanReport
            {scanResult}
            allItems={items}
            onBack={() => view = "items"}
            onRefresh={runAiScan}
        />
    {:else}
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
                        class:critical={item.safety_rating === "Critical"}
                        class:disabled={!item.enabled}
                    >
                        <div class="toggle-container">
                            <label class="toggle-switch" class:disabled={item.safety_rating === "Critical"}>
                                <input
                                    type="checkbox"
                                    checked={item.enabled}
                                    disabled={item.safety_rating === "Critical"}
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
                                    {#if item.safety_rating === "Critical"}
                                        <Shield size={8} />
                                    {:else}
                                        <Circle size={8} />
                                    {/if}
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

        <div class="ai-scan-section">
            <div class="ai-scan-header">
                <Sparkles size={14} />
                <span>AI Startup Scan</span>
                <button class="scan-btn" on:click={runAiScan} disabled={scanning || applying}>
                    {#if scanning}
                        Analyzing…
                    {:else}
                        Scan &amp; Recommend
                    {/if}
                </button>
            </div>

            {#if scanError}
                <div class="scan-error">{scanError}</div>
            {/if}

            {#if scanResult}
                <p class="scan-summary">{scanResult.summary}</p>
                <p class="scan-hint">Click "View Full Report" below to see all findings.</p>
                <div class="rec-list">
                    {#each scanResult.recommendations.filter((r: any) => r.action !== "keep") as rec}
                        <div
                            class="rec-row"
                            class:selected={selectedRecs.has(rec.item_id)}
                            class:investigate={rec.action === "investigate"}
                        >
                            <div class="rec-check" class:checked={selectedRecs.has(rec.item_id)}
                                on:click={() => {
                                    if (selectedRecs.has(rec.item_id)) selectedRecs.delete(rec.item_id);
                                    else selectedRecs.add(rec.item_id);
                                    selectedRecs = selectedRecs;
                                }}
                                on:keydown={(e) => {
                                    if (e.key === " " || e.key === "Enter") {
                                        e.preventDefault();
                                        if (selectedRecs.has(rec.item_id)) selectedRecs.delete(rec.item_id);
                                        else selectedRecs.add(rec.item_id);
                                        selectedRecs = selectedRecs;
                                    }
                                }}
                                role="checkbox" aria-checked={selectedRecs.has(rec.item_id)} tabindex="0">
                                {#if selectedRecs.has(rec.item_id)}<Check size={9} />{/if}
                            </div>
                            <div class="rec-body" on:click={() => openRecDetail(rec)} role="button" tabindex="0" on:keydown={(e) => e.key === "Enter" && openRecDetail(rec)}>
                                <div class="rec-header">
                                    <span class="rec-id">{rec.item_id.split("\\").pop() ?? rec.item_id}</span>
                                    <span class="rec-name">{rec.item_name || rec.item_id.split("\\").pop()}</span>
                                    <span class="rec-action" style="color: {rec.action === 'investigate' ? '#fbbf24' : '#f87171'}">
                                        {rec.action}
                                    </span>
                                    <span class="rec-prio" style="color: {priorityColor(rec.priority)}">
                                        {rec.priority}
                                    </span>
                                </div>
                                <p class="rec-reason">{rec.reason}</p>
                            </div>
                            <button class="rec-info-btn" on:click={() => openRecDetail(rec)} title="View details">
                                <Sparkles size={12} />
                            </button>
                        </div>
                    {/each}
                </div>

                {#if scanResult.recommendations.some((r: any) => r.action === "disable")}
                    <div class="apply-bar">
                        <button class="report-link-btn" on:click={() => view = "scan"}>
                            View Full Report ({scanResult.recommendations.length} items)
                        </button>
                        <span class="sel-count">{selectedRecs.size} selected</span>
                        <button class="apply-btn" on:click={applySelected}
                                disabled={applying || selectedRecs.size === 0}>
                            {#if applying}
                                Applying…
                            {:else}
                                Apply Selected
                            {/if}
                        </button>
                    </div>
                {/if}

                {#if applyResults.length > 0}
                    <div class="apply-results">
                        {#each applyResults as r}<div class="result-line">{r}</div>{/each}
                    </div>
                {/if}
            {/if}
        </div>
    {/if}

    {#if showRecDetail && selectedRec}
        <div class="rec-modal-backdrop" on:click={closeRecDetail} role="button" tabindex="-1" on:keydown={(e) => e.key === "Escape" && closeRecDetail()}>
            <div class="rec-modal" on:click|stopPropagation on:keydown={(e) => e.key === "Escape" && closeRecDetail()} role="dialog" tabindex="-1">
                <div class="rec-modal-header">
                    <div class="rec-modal-title">
                        <span class="modal-name">{selectedRec.item_name || selectedRec.item_id.split("\\").pop()}</span>
                        <span class="modal-id">{selectedRec.item_id}</span>
                    </div>
                    <button class="modal-close" on:click={closeRecDetail}>×</button>
                </div>
                <div class="rec-modal-body">
                    <div class="modal-row">
                        <span class="modal-label">Action</span>
                        <span class="modal-value rec-action" style="color: {selectedRec.action === 'investigate' ? '#fbbf24' : '#f87171'}">{selectedRec.action}</span>
                    </div>
                    <div class="modal-row">
                        <span class="modal-label">Priority</span>
                        <span class="modal-value" style="color: {priorityColor(selectedRec.priority)}">{selectedRec.priority}</span>
                    </div>
                    <div class="modal-row">
                        <span class="modal-label">Reason</span>
                        <p class="modal-reason">{selectedRec.reason}</p>
                    </div>
                    <div class="modal-row">
                        <span class="modal-label">Startup Item ID</span>
                        <code class="modal-code">{selectedRec.item_id}</code>
                    </div>
                    {#if selectedRec.item_name}
                        <div class="modal-row">
                            <span class="modal-label">Display Name</span>
                            <span class="modal-value">{selectedRec.item_name}</span>
                        </div>
                    {/if}
                </div>
                <div class="rec-modal-footer">
                    <button class="modal-back-btn" on:click={closeRecDetail}>
                        ← Back to list
                    </button>
                    {#if selectedRec.action !== "keep"}
                        <button
                            class="modal-toggle-btn"
                            class:selected={selectedRecs.has(selectedRec.item_id)}
                            on:click={() => {
                                if (selectedRecs.has(selectedRec.item_id)) {
                                    selectedRecs.delete(selectedRec.item_id);
                                } else {
                                    selectedRecs.add(selectedRec.item_id);
                                }
                                selectedRecs = selectedRecs;
                            }}
                        >
                            {selectedRecs.has(selectedRec.item_id) ? "✓ Selected" : "Select for apply"}
                        </button>
                    {/if}
                </div>
            </div>
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
    .startup-item.critical {
        border-left-color: #a855f7;
        opacity: 0.75;
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
    .safety-badge.critical :global(svg) {
        color: #a855f7;
        fill: #a855f7;
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

    .ai-scan-section {
        margin-top: 24px;
        background: var(--bg-card);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 16px 20px;
        display: flex; flex-direction: column; gap: 12px;
    }
    .ai-scan-header {
        display: flex; align-items: center; gap: 8px;
        color: var(--accent-color); font-size: 13px; font-weight: 600;
    }
    .ai-scan-header span { flex: 1; }
    .scan-btn {
        display: flex; align-items: center; gap: 6px;
        background: rgba(129,140,248,0.12);
        border: 1px solid rgba(129,140,248,0.2);
        border-radius: 6px; padding: 5px 12px;
        color: var(--accent-color); font-size: 12px; cursor: pointer;
        transition: background 0.15s;
    }
    .scan-btn:hover:not(:disabled) { background: rgba(129,140,248,0.2); }
    .scan-btn:disabled { opacity: 0.5; cursor: not-allowed; }
    .scan-summary { font-size: 12px; color: var(--text-secondary); margin: 0; line-height: 1.5; }
    .rec-list { display: flex; flex-direction: column; gap: 6px; max-height: 280px; overflow-y: auto; padding-right: 4px; }
    .rec-list::-webkit-scrollbar { width: 4px; }
    .rec-list::-webkit-scrollbar-track { background: transparent; }
    .rec-list::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.1); border-radius: 2px; }
    .rec-row {
        display: flex; align-items: flex-start; gap: 10px;
        padding: 9px 12px; background: rgba(255,255,255,0.02);
        border: 1px solid rgba(255,255,255,0.06); border-radius: 8px;
        cursor: pointer; user-select: none; transition: border-color 0.15s, background 0.15s;
    }
    .rec-row:hover      { background: var(--bg-hover); }
    .rec-row.selected   { border-color: rgba(248,113,113,0.25); background: rgba(248,113,113,0.04); }
    .rec-row.investigate { border-left: 2px solid #fbbf24; }
    .rec-check {
        width: 15px; height: 15px; flex-shrink: 0; margin-top: 1px;
        border-radius: 3px; border: 1px solid rgba(255,255,255,0.18);
        background: rgba(255,255,255,0.04);
        display: flex; align-items: center; justify-content: center;
        transition: all 0.12s; color: white;
    }
    .rec-check.checked  { background: #f87171; border-color: #f87171; }
    .rec-body           { flex: 1; min-width: 0; }
    .rec-header         { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; margin-bottom: 3px; }
    .rec-id             { font-size: 11px; font-family: monospace; color: var(--text-secondary); }
    .rec-action         { font-size: 10px; font-weight: 700; text-transform: uppercase; }
    .rec-prio           { font-size: 10px; font-weight: 600; }
    .rec-name           { font-size: 11px; color: var(--text-color); font-weight: 500; }
    .rec-info-btn       {
        background: none; border: none; color: var(--text-muted);
        cursor: pointer; padding: 4px; border-radius: 4px;
        display: flex; align-items: center; opacity: 0;
        transition: opacity 0.15s, color 0.15s;
    }
    .rec-row:hover .rec-info-btn { opacity: 1; }
    .rec-info-btn:hover { color: var(--accent-color); }
    .rec-reason         { margin: 0; font-size: 11px; color: var(--text-muted); line-height: 1.4; }
    .apply-bar          {
        display: flex; align-items: center; justify-content: flex-end; gap: 12px;
        border-top: 1px solid var(--border-color); padding-top: 10px;
    }
    .sel-count          { font-size: 11px; color: var(--text-muted); }
    .apply-btn          {
        display: flex; align-items: center; gap: 6px;
        background: rgba(248,113,113,0.12); border: 1px solid rgba(248,113,113,0.25);
        border-radius: 6px; padding: 5px 14px;
        color: #f87171; font-size: 12px; cursor: pointer;
    }
    .apply-btn:hover:not(:disabled) { background: rgba(248,113,113,0.22); }
    .apply-btn:disabled { opacity: 0.5; cursor: not-allowed; }
    .apply-results      {
        font-size: 11px; font-family: monospace;
        color: var(--text-muted); background: rgba(0,0,0,0.2);
        border-radius: 6px; padding: 8px 12px;
        display: flex; flex-direction: column; gap: 2px;
    }
    .result-line::before { content: "→ "; color: var(--accent-color); }
    .scan-error         {
        font-size: 12px; color: #f87171;
        background: rgba(248,113,113,0.08);
        border: 1px solid rgba(248,113,113,0.2);
        border-radius: 6px; padding: 8px 12px;
    }

    .rec-modal-backdrop {
        position: fixed; inset: 0; z-index: 1000;
        background: rgba(0,0,0,0.65);
        display: flex; align-items: center; justify-content: center;
        backdrop-filter: blur(4px);
    }
    .rec-modal {
        background: var(--bg-card);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        width: 520px; max-width: 92vw;
        max-height: 80vh; overflow-y: auto;
        display: flex; flex-direction: column;
        box-shadow: 0 24px 64px rgba(0,0,0,0.5);
    }
    .rec-modal-header {
        display: flex; align-items: flex-start; justify-content: space-between;
        padding: 20px 24px 16px;
        border-bottom: 1px solid var(--border-color);
        gap: 12px;
    }
    .rec-modal-title { display: flex; flex-direction: column; gap: 4px; flex: 1; min-width: 0; }
    .modal-name { font-size: 16px; font-weight: 600; color: var(--text-color); }
    .modal-id { font-size: 11px; font-family: monospace; color: var(--text-muted); word-break: break-all; }
    .modal-close {
        background: none; border: none; color: var(--text-muted);
        font-size: 22px; cursor: pointer; padding: 0 4px; line-height: 1;
        border-radius: 4px;
    }
    .modal-close:hover { color: var(--text-color); }
    .rec-modal-body { padding: 20px 24px; display: flex; flex-direction: column; gap: 14px; }
    .modal-row { display: flex; flex-direction: column; gap: 4px; }
    .modal-label { font-size: 11px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; }
    .modal-value { font-size: 13px; color: var(--text-color); font-weight: 500; }
    .modal-code { font-size: 12px; font-family: monospace; color: var(--text-secondary); background: rgba(255,255,255,0.05); padding: 4px 8px; border-radius: 6px; word-break: break-all; }
    .modal-reason { font-size: 13px; color: var(--text-secondary); line-height: 1.5; margin: 0; }
    .rec-modal-footer {
        display: flex; align-items: center; justify-content: space-between;
        padding: 16px 24px; border-top: 1px solid var(--border-color);
        gap: 12px;
    }
    .modal-back-btn {
        background: none; border: 1px solid var(--border-color);
        border-radius: 8px; padding: 8px 16px; color: var(--text-muted);
        font-size: 13px; cursor: pointer;
    }
    .modal-back-btn:hover { color: var(--text-color); border-color: var(--text-muted); }
    .modal-toggle-btn {
        background: rgba(129,140,248,0.12); border: 1px solid rgba(129,140,248,0.25);
        border-radius: 8px; padding: 8px 16px; color: var(--accent-color);
        font-size: 13px; cursor: pointer;
    }
    .modal-toggle-btn:hover { background: rgba(129,140,248,0.2); }
    .modal-toggle-btn.selected { background: rgba(248,113,113,0.12); border-color: rgba(248,113,113,0.25); color: #f87171; }
    .scan-hint { font-size: 11px; color: var(--text-muted); margin: 0 0 8px 0; font-style: italic; }
    .report-link-btn {
        background: rgba(129,140,248,0.1); border: 1px solid rgba(129,140,248,0.2);
        border-radius: 6px; padding: 5px 12px; color: var(--accent-color);
        font-size: 12px; cursor: pointer; transition: background 0.15s;
    }
    .report-link-btn:hover { background: rgba(129,140,248,0.2); }
</style>
