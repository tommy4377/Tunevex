<script lang="ts">
    import { fade } from "svelte/transition";
    import Compactor from "./Compactor.svelte";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "compactor" = "dashboard";
    let activeTab: "tools" | "tweaks" = "tools";

    $: storageTweaks = allTweaks.filter((t) => t.category === "FileSystem");
</script>

<div class="storage-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-view" in:fade>
            <div class="header-section">
                <h1>Storage Manager</h1>
                <p>Advanced disk optimization tools and filesystem tweaks.</p>
            </div>

            <div class="tabs">
                <button
                    class:active={activeTab === "tools"}
                    on:click={() => (activeTab = "tools")}
                >
                    <span class="icon">🛠️</span>
                    <span>Tools</span>
                </button>
                <button
                    class:active={activeTab === "tweaks"}
                    on:click={() => (activeTab = "tweaks")}
                >
                    <span class="icon">💾</span>
                    <span>Tweaks</span>
                </button>
            </div>

            {#if activeTab === "tools"}
                <div class="dashboard-grid" in:fade>
                    <!-- Compactor Card -->
                    <div
                        class="tool-card"
                        on:click={() => (currentView = "compactor")}
                        on:keydown={(e) =>
                            e.key === "Enter" && (currentView = "compactor")}
                        role="button"
                        tabindex="0"
                    >
                        <div class="card-icon">🗜️</div>
                        <h3>CompactOS</h3>
                        <p>
                            Transparent NTFS compression using XPRESS/LZX
                            algorithms.
                        </p>
                        <div class="status-pill">Ready</div>
                    </div>
                </div>
            {:else if activeTab === "tweaks"}
                <div class="tweaks-section" in:fade>
                    <TweakList tweaks={storageTweaks} showHeader={false} />
                </div>
            {/if}
        </div>
    {:else}
        <div class="detail-view" in:fade>
            <button
                class="back-btn"
                on:click={() => (currentView = "dashboard")}
            >
                ← Return to dashboard
            </button>

            {#if currentView === "compactor"}
                <Compactor />
            {/if}
        </div>
    {/if}
</div>

<style>
    .storage-container {
        height: 100%;
        color: var(--text-color);
        display: flex;
        flex-direction: column;
        padding: 24px;
        box-sizing: border-box;
        overflow: hidden;
    }

    .dashboard-view {
        display: flex;
        flex-direction: column;
        flex: 1;
        overflow: hidden;
    }

    .header-section {
        margin-bottom: 24px;
        flex-shrink: 0;
    }

    h1 {
        font-size: 24px;
        font-weight: 700;
        margin-bottom: 8px;
        color: var(--text-color);
        margin-top: 0;
    }

    p {
        color: var(--text-muted);
        font-size: 14px;
        margin: 0;
    }

    .tabs {
        display: flex;
        gap: 12px;
        margin-bottom: 24px;
        border-bottom: 1px solid var(--border-color);
        padding-bottom: 0;
        flex-shrink: 0;
    }

    .tabs button {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 12px 16px;
        background: transparent;
        border: none;
        border-bottom: 2px solid transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        transition: all 0.2s;
    }

    .tabs button:hover {
        color: var(--text-color);
        background: rgba(255, 255, 255, 0.03);
    }

    .tabs button.active {
        color: var(--accent-color);
        border-bottom-color: var(--accent-color);
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 24px;
        overflow-y: auto;
    }

    .tool-card {
        background: var(--bg-card); /* Fallback or variable */
        background-color: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 24px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        gap: 12px;
        position: relative;
    }

    .tool-card:hover {
        transform: translateY(-2px);
        background-color: rgba(255, 255, 255, 0.06);
        border-color: var(--accent-color);
    }

    .card-icon {
        font-size: 32px;
        margin-bottom: 4px;
    }

    .tool-card h3 {
        margin: 0;
        font-size: 18px;
        font-weight: 600;
    }

    .tool-card p {
        margin: 0;
        font-size: 14px;
        color: var(--text-muted);
        line-height: 1.5;
        flex: 1;
    }

    .status-pill {
        align-self: flex-start;
        font-size: 11px;
        padding: 4px 10px;
        border-radius: 12px;
        background: rgba(59, 130, 246, 0.1);
        color: var(--accent-color);
        font-weight: 500;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 0;
        overflow-y: auto;
    }

    .back-btn {
        align-self: flex-start;
        background: none;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 14px;
        padding: 0;
        margin-bottom: 16px;
        transition: color 0.2s;
    }

    .back-btn:hover {
        color: var(--text-color);
    }

    .tweaks-section {
        flex: 1;
        overflow: hidden; /* TweakList handles its own scroll */
    }
</style>
