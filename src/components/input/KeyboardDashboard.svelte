<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    $: keyboardTweaks = allTweaks.filter(
        (t) => t.category === "MouseInput" && !t.id.includes("mouse"),
    );

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
</script>

<div class="keyboard-dashboard">
    <div class="header">
        <h2>⌨️ Keyboard Optimization</h2>
        <p class="subtitle">
            Minimize latency, increase repeat rate, and disable annoying sticky
            keys.
        </p>
    </div>

    <div class="tweaks-grid">
        {#each keyboardTweaks as tweak}
            <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
        {/each}
        {#if keyboardTweaks.length === 0}
            <p class="empty">No keyboard tweaks found.</p>
        {/if}
    </div>
</div>

<style>
    .keyboard-dashboard {
        padding: 0 0 24px 0;
        animation: fadeIn 0.3s ease-out;
    }

    .header {
        margin-bottom: 24px;
    }

    h2 {
        font-size: 20px;
        font-weight: 600;
        margin-bottom: 8px;
        color: var(--text-color);
    }

    .subtitle {
        color: var(--text-muted);
        font-size: 14px;
    }

    .tweaks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 12px;
    }

    .empty {
        color: var(--text-muted);
        grid-column: 1 / -1;
        padding: 24px;
        text-align: center;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
