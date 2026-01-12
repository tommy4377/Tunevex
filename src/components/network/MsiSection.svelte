<script lang="ts">
    import TweakCard from "../TweakCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    import { fade } from "svelte/transition";

    export let tweaks: Tweak[] = [];

    async function toggleTweak(tweak: Tweak) {
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
                tweak.enabled = false;
            } else {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
            tweaks = tweaks;
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
        }
    }
</script>

<div class="msi-section" in:fade>
    <div class="header">
        <h2>⚡ Network MSI Mode</h2>
        <p>
            Enable Message Signaled Interrupts (MSI) for lower latency and
            better stability.
        </p>
    </div>

    <div class="info-banner">
        <span class="icon">ℹ️</span>
        <p>
            MSI Mode reduces CPU overhead for network interrupts.
            <strong>High Priority</strong> is recommended for gaming.
            <strong>Normal Priority</strong> is safer for general use.
        </p>
    </div>

    <div class="grid">
        {#each tweaks as tweak (tweak.id)}
            <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
        {/each}
        {#if tweaks.length === 0}
            <div class="empty">No MSI tweaks available</div>
        {/if}
    </div>
</div>

<style>
    .msi-section {
        padding-bottom: 24px;
    }

    .header {
        margin-bottom: 24px;
    }

    h2 {
        font-size: 24px;
        font-weight: 600;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }

    p {
        color: var(--text-muted);
        margin: 0;
        font-size: 14px;
    }

    .info-banner {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: var(--radius-sm);
        padding: 12px 16px;
        display: flex;
        gap: 12px;
        align-items: flex-start;
        margin-bottom: 24px;
        color: var(--text-color);
        font-size: 14px;
        line-height: 1.5;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 16px;
    }

    .empty {
        grid-column: 1 / -1;
        text-align: center;
        padding: 40px;
        color: var(--text-muted);
        font-style: italic;
    }
</style>
