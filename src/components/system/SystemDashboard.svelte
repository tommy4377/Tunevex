<script lang="ts">
    import { fade } from "svelte/transition";
    import TweakCard from "../TweakCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter System tweaks
    // Category is "System"
    // We can also split by ID prefix or analysis
    $: systemTweaks = allTweaks.filter((t) => t.category === "System");
    $: hardwareTweaks = allTweaks.filter(
        (t) => t.category === "Hardware" || t.id.includes("msi_global"),
    ); // Including global msi if mapped to Hardware or System

    // Grouping
    // Services, Maintenance, MSI (Hardware)
    // Actually backend System module: maintenance, services, msi.
    // msi.rs tweak category is "Hardware" in my previous edit?
    // Let's check msi.rs content again or just assume catch-all.
    // Wait, I set category to "Hardware" in system/msi.rs but user said "Hardware shouldn't exist".
    // I should probably map "Hardware" category in sidebar to SystemDashboard if I keep it, or I should have changed category to "System".
    // I entered "Hardware" in msi.rs.
    // Let's filter for both "System" and "Hardware" categories here to be safe and show them.

    $: combinedTweaks = allTweaks.filter(
        (t) => t.category === "System" || t.category === "Hardware",
    );

    $: maintenanceTweaks = combinedTweaks.filter(
        (t) => t.id.includes("maintenance") || t.id.includes("cleanup"),
    );
    $: serviceTweaks = combinedTweaks.filter(
        (t) => t.id.includes("service") || t.id.includes("svc"),
    );
    $: msiTweaks = combinedTweaks.filter((t) => t.id.includes("msi"));

    async function toggleTweak(tweak: Tweak) {
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
                tweak.enabled = false;
            } else {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
            allTweaks = allTweaks;
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
        }
    }

    let activeTab: "general" | "services" | "hardware" = "general";
</script>

<div class="system-dashboard">
    <div class="header-section">
        <h1>System Optimization</h1>
        <p>
            Manage system services, maintenance tasks, and hardware interrupts.
        </p>
    </div>

    <div class="tabs">
        <button
            class:active={activeTab === "general"}
            on:click={() => (activeTab = "general")}
        >
            <span class="icon">🛠️</span>
            <span>Maintenance</span>
        </button>
        <button
            class:active={activeTab === "services"}
            on:click={() => (activeTab = "services")}
        >
            <span class="icon">⚙️</span>
            <span>Services</span>
        </button>
        <button
            class:active={activeTab === "hardware"}
            on:click={() => (activeTab = "hardware")}
        >
            <span class="icon">⚡</span>
            <span>Hardware / MSI</span>
        </button>
    </div>

    <div class="content">
        {#if activeTab === "general"}
            <div class="grid" in:fade>
                {#each maintenanceTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
                {#if maintenanceTweaks.length === 0}
                    <div class="empty">No maintenance tweaks found.</div>
                {/if}
            </div>
        {:else if activeTab === "services"}
            <div class="grid" in:fade>
                {#each serviceTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
                {#if serviceTweaks.length === 0}
                    <div class="empty">No service tweaks found.</div>
                {/if}
            </div>
        {:else if activeTab === "hardware"}
            <div class="grid" in:fade>
                <div class="info-banner">
                    <span class="icon">ℹ️</span>
                    <p>
                        Global MSI Mode attempts to enable Message Signaled
                        Interrupts for all supported devices. Use with caution.
                    </p>
                </div>
                {#each msiTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
                {#if msiTweaks.length === 0}
                    <div class="empty">No hardware/MSI tweaks found.</div>
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
    .system-dashboard {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        color: var(--text-color);
        box-sizing: border-box;
        overflow: hidden;
    }

    .header-section {
        margin-bottom: 24px;
        flex-shrink: 0;
    }

    h1 {
        font-size: 24px;
        font-weight: 700;
        margin: 0 0 8px 0;
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

    .content {
        flex: 1;
        overflow-y: auto;
        padding-bottom: 20px;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 16px;
    }

    .info-banner {
        grid-column: 1 / -1;
        background: rgba(245, 158, 11, 0.1);
        border: 1px solid rgba(245, 158, 11, 0.2);
        border-radius: var(--radius-sm);
        padding: 12px 16px;
        display: flex;
        gap: 12px;
        align-items: flex-start;
        color: var(--text-color);
        font-size: 14px;
        line-height: 1.5;
        margin-bottom: 8px;
    }

    .empty {
        text-align: center;
        padding: 40px;
        color: var(--text-muted);
        font-style: italic;
        grid-column: 1 / -1;
    }
</style>
