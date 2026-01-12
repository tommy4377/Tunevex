<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak, TweakCategory } from "$lib/types";
    import { activeCategory } from "$lib/stores";

    export let tweaks: Tweak[] = [];
    export let showHeader = true;

    // Filter displayed tweaks based on active category
    let displayedTweaks: Tweak[] = [];
    let currentCategory: TweakCategory | null = null;
    let categoryTitle = "";

    // Subscribe to store
    activeCategory.subscribe((cat) => {
        currentCategory = cat;
        if (cat) {
            displayedTweaks = tweaks.filter((t) => t.category === cat);
            const rawTitle =
                displayedTweaks.length > 0 ? getCategoryName(cat) : "Tweaks";
            // Strip emoji
            categoryTitle = rawTitle.replace(/^[^\w\s]+/, "").trim();
        } else {
            displayedTweaks = [];
            categoryTitle = "";
        }
    });

    $: if (showHeader && currentCategory) {
        // Reactive update if list changes but category stays same
        displayedTweaks = tweaks.filter((t) => t.category === currentCategory);
    } else if (!showHeader) {
        // If header is hidden, we assume the parent is controlling the list filter (like TcpSection)
        // So we just show all passed 'tweaks'
        displayedTweaks = tweaks;
    }

    function getCategoryName(cat: TweakCategory): string {
        switch (cat) {
            case "Network":
                return "🌐 Network Optimization";
            case "CpuPerformance":
                return "🚀 CPU & Performance";
            case "Privacy":
                return "🛡️ Privacy & Telemetry";
            case "DebloatTelemetry":
                return "🧹 Debloat & Apps";
            case "StartupServices":
                return "⚡ Startup & Services";
            case "System":
                return "🎨 System & Visuals";
            case "GameOptimizations":
                return "🎮 Gaming Optimization";
            case "Input":
                return "🖱️ Mouse & Input";
            default:
                return "Tweaks";
        }
    }

    async function toggleTweak(tweak: Tweak) {
        // In a real app we'd fetch current state.
        // For MVP, we just toggle local state and call backend.
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
                tweak.enabled = false;
            } else {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
            tweaks = tweaks; // Trigger reactivity
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
            // Optionally revert the optimistic UI or show a toast here
            // For now, we just ensure it doesn't crash the console
        }
    }
</script>

<div class="list-container">
    {#if showHeader && currentCategory}
        <div class="header">
            <h1>{categoryTitle}</h1>
            <p class="subtitle">
                {displayedTweaks.length} optimizations available
            </p>
        </div>
    {/if}

    <div class="scroll-area">
        {#each displayedTweaks as tweak (tweak.id)}
            <div class="tweak-item">
                <div class="info">
                    <div class="top-row">
                        <span class="name">{tweak.name}</span>
                        {#if tweak.warning_level === "Dangerous"}
                            <span class="badge danger">Dangerous</span>
                        {:else if tweak.warning_level === "Careful"}
                            <span class="badge warning">Careful</span>
                        {/if}
                    </div>
                    <p class="description">{tweak.description}</p>
                </div>

                <button
                    class="toggle-btn"
                    class:on={tweak.enabled}
                    on:click={() => toggleTweak(tweak)}
                >
                    {tweak.enabled ? "Enabled" : "Disabled"}
                </button>
            </div>
        {/each}

        {#if displayedTweaks.length === 0}
            <div class="empty-state">
                <p>No tweaks available in this category yet.</p>
            </div>
        {/if}
        <!-- Spacer to ensure last item is never covered by anything -->
        <div style="height: 48px; width: 100%; flex-shrink: 0;"></div>
    </div>
</div>

<style>
    .list-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: hidden;
        padding: 0 24px;
    }

    .header {
        margin-bottom: 24px;
        flex-shrink: 0;
    }

    h1 {
        font-size: 24px;
        font-weight: 600;
        margin-bottom: 8px;
    }

    .subtitle {
        margin: 0;
        color: var(--text-muted);
        font-size: 13px;
    }

    .scroll-area {
        flex: 1;
        overflow-y: auto;
        padding-right: 8px;
        /* Padding bottom is handled by spacer div now for better cross-browser reliability */
    }

    .tweak-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: var(--radius-md);
        margin-bottom: 12px;
        transition: background 0.2s;
    }

    /* ... skipped ... */

    .badge {
        font-size: 10px;
        padding: 2px 6px;
        border-radius: var(--radius-sm);
        font-weight: 600;
        text-transform: uppercase;
    }

    /* ... skipped ... */

    /* Toggle Button */
    .toggle-btn {
        min-width: 80px;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        border: 1px solid var(--border-color);
        background: transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.2s;
    }

    .toggle-btn:hover {
        border-color: var(--text-muted);
        color: var(--text-color);
    }

    .toggle-btn.on {
        background: var(--accent-color);
        border-color: var(--accent-color);
        color: white;
    }

    .toggle-btn.on:hover {
        background: var(--accent-hover);
    }

    .empty-state {
        padding: 40px;
        text-align: center;
        color: var(--text-muted);
        font-style: italic;
    }
</style>
