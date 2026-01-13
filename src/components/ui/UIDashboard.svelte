<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter UI tweaks (both legacy ui. and new interface_ prefixes)
    $: uiTweaks = allTweaks.filter((t) => t.category === "InterfaceUx");

    // Organize by section
    $: taskbarTweaks = uiTweaks.filter(
        (t) => t.id.includes("taskbar") || t.id.includes("TaskbarGlomLevel"),
    );
    $: visualTweaks = uiTweaks.filter(
        (t) => t.id.includes("context") || t.id.includes("dark"),
    );
    $: explorerTweaks = uiTweaks.filter(
        (t) =>
            t.id.includes("extension") ||
            t.id.includes("file") ||
            t.id.includes("compact") ||
            t.id.includes("hidden"),
    );

    // Fallback: Tweaks not in above sections
    $: otherTweaks = uiTweaks.filter(
        (t) =>
            !taskbarTweaks.includes(t) &&
            !visualTweaks.includes(t) &&
            !explorerTweaks.includes(t),
    );

    interface Section {
        title: string;
        icon: string;
        color: string;
        tweaks: Tweak[];
        description: string;
    }

    $: sections = [
        {
            title: "Taskbar",
            icon: "🎯",
            color: "#3b82f6",
            tweaks: taskbarTweaks,
            description: "Taskbar layout & behavior",
        },
        {
            title: "Appearance",
            icon: "🌙",
            color: "#8b5cf6",
            tweaks: visualTweaks,
            description: "Dark mode & context menus",
        },
        {
            title: "Explorer",
            icon: "📁",
            color: "#10b981",
            tweaks: explorerTweaks,
            description: "File Explorer tweaks",
        },
        {
            title: "Other",
            icon: "⚙️",
            color: "#6b7280",
            tweaks: otherTweaks,
            description: "Additional tweaks",
        },
    ].filter((s) => s.tweaks.length > 0) as Section[];

    type View = "dashboard" | "detail";
    let currentView: View = "dashboard";
    let selectedSection = "";
    let selectedTweaks: Tweak[] = [];

    function openSection(section: Section) {
        selectedSection = section.title;
        selectedTweaks = section.tweaks;
        currentView = "detail";
    }

    function goBack() {
        currentView = "dashboard";
    }

    async function applyTweak(tweak: Tweak) {
        try {
            await invoke("apply_tweak", { id: tweak.id });
            tweak.enabled = true;
            allTweaks = [...allTweaks];
        } catch (e) {
            console.error("Failed to apply tweak:", e);
        }
    }

    async function revertTweak(tweak: Tweak) {
        try {
            await invoke("undo_tweak", { id: tweak.id });
            tweak.enabled = false;
            allTweaks = [...allTweaks];
        } catch (e) {
            console.error("Failed to revert tweak:", e);
        }
    }
</script>

<div class="ui-dashboard" in:fade>
    {#if currentView === "dashboard"}
        <div class="header">
            <h2>🎨 Interface Tweaks</h2>
            <p class="subtitle">Customize Windows appearance and behavior</p>
        </div>

        <div class="dashboard-grid">
            {#each sections as section}
                <button
                    class="section-card"
                    style="--accent: {section.color}"
                    on:click={() => openSection(section)}
                >
                    <div class="card-header">
                        <span class="card-icon">{section.icon}</span>
                        <h3>{section.title}</h3>
                    </div>
                    <p class="card-desc">{section.description}</p>
                    <span class="count">{section.tweaks.length} options</span>
                </button>
            {/each}
        </div>
    {:else}
        <div class="detail-view">
            <button class="back-btn" on:click={goBack}>← Back</button>
            <h2>{selectedSection}</h2>
            <div class="tweak-list">
                {#each selectedTweaks as tweak}
                    <TweakCard
                        {tweak}
                        on:apply={() => applyTweak(tweak)}
                        on:revert={() => revertTweak(tweak)}
                    />
                {/each}
            </div>
        </div>
    {/if}
</div>

<style>
    .ui-dashboard {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        padding: 24px;
        gap: 20px;
    }

    .header {
        text-align: center;
    }

    .header h2 {
        margin: 0;
        font-size: 28px;
        color: var(--text-color);
    }

    .subtitle {
        color: var(--text-muted);
        margin: 8px 0 0;
        font-size: 14px;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 16px;
        align-content: start;
    }

    .section-card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 20px;
        text-align: left;
        cursor: pointer;
        transition: all 0.2s;
    }

    .section-card:hover {
        background: rgba(255, 255, 255, 0.06);
        border-color: var(--accent);
        transform: translateY(-2px);
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 10px;
        margin-bottom: 8px;
    }

    .card-icon {
        font-size: 24px;
    }

    .section-card h3 {
        margin: 0;
        font-size: 16px;
        color: var(--text-color);
    }

    .card-desc {
        margin: 0;
        font-size: 12px;
        color: var(--text-muted);
    }

    .count {
        display: inline-block;
        margin-top: 8px;
        font-size: 11px;
        color: var(--accent);
        background: rgba(59, 130, 246, 0.1);
        padding: 2px 8px;
        border-radius: 10px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 16px;
        overflow: hidden;
    }

    .back-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-muted);
        padding: 8px 16px;
        border-radius: var(--radius-md);
        cursor: pointer;
        align-self: flex-start;
        transition: all 0.2s;
    }

    .back-btn:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
    }

    .detail-view h2 {
        margin: 0;
        font-size: 20px;
        color: var(--text-color);
    }

    .tweak-list {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
</style>
