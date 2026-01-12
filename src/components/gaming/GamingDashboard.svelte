<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter gaming-related tweaks
    $: gamingTweaks = allTweaks.filter(
        (t) => t.category === "GameOptimizations",
    );

    $: gameBarTweaks = gamingTweaks.filter(
        (t) =>
            t.id.includes("gamebar") ||
            t.id.includes("game_bar") ||
            t.id.includes("gamedvr") ||
            t.id.includes("game_dvr"),
    );

    $: fsoTweaks = gamingTweaks.filter(
        (t) => t.id.includes("fso") || t.id.includes("fullscreen"),
    );

    $: gameModeTweaks = gamingTweaks.filter(
        (t) =>
            t.id.includes("gamemode") ||
            t.id.includes("game_mode") ||
            t.id.includes("priority"),
    );

    $: xboxTweaks = gamingTweaks.filter(
        (t) => t.id.includes("xbox") && !gameBarTweaks.includes(t),
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

<div class="gaming-dashboard">
    <h2>🎮 Gaming Optimizations</h2>
    <p class="subtitle">
        Disable overlays, optimize fullscreen, and prioritize games
    </p>

    <div class="sections">
        <!-- Game Bar & DVR Section -->
        {#if gameBarTweaks.length > 0}
            <section class="tweak-section">
                <h3>📹 Game Bar & DVR</h3>
                <div class="tweaks-grid">
                    {#each gameBarTweaks as tweak}
                        <TweakCard
                            {tweak}
                            on:toggle={() => toggleTweak(tweak)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        <!-- FSO Section -->
        {#if fsoTweaks.length > 0}
            <section class="tweak-section">
                <h3>🖥️ Fullscreen Optimizations</h3>
                <div class="tweaks-grid">
                    {#each fsoTweaks as tweak}
                        <TweakCard
                            {tweak}
                            on:toggle={() => toggleTweak(tweak)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        <!-- Game Mode & Priority Section -->
        {#if gameModeTweaks.length > 0}
            <section class="tweak-section">
                <h3>⚡ Game Mode & Priority</h3>
                <div class="tweaks-grid">
                    {#each gameModeTweaks as tweak}
                        <TweakCard
                            {tweak}
                            on:toggle={() => toggleTweak(tweak)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        <!-- Xbox Services Section -->
        {#if xboxTweaks.length > 0}
            <section class="tweak-section">
                <h3>🎮 Xbox Services</h3>
                <div class="tweaks-grid">
                    {#each xboxTweaks as tweak}
                        <TweakCard
                            {tweak}
                            on:toggle={() => toggleTweak(tweak)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        <!-- All Gaming Tweaks (fallback) -->
        {#if gamingTweaks.length === 0}
            <div class="empty-state">
                <p>No gaming tweaks available</p>
            </div>
        {:else if gameBarTweaks.length === 0 && fsoTweaks.length === 0 && gameModeTweaks.length === 0}
            <section class="tweak-section">
                <h3>🎮 All Gaming Tweaks</h3>
                <div class="tweaks-grid">
                    {#each gamingTweaks as tweak}
                        <TweakCard
                            {tweak}
                            on:toggle={() => toggleTweak(tweak)}
                        />
                    {/each}
                </div>
            </section>
        {/if}
    </div>
</div>

<style>
    .gaming-dashboard {
        height: 100%;
        overflow-y: auto;
        padding: 24px;
        color: var(--text-color);
    }

    h2 {
        margin: 0 0 8px 0;
        font-size: 24px;
        font-weight: 600;
    }

    .subtitle {
        color: var(--text-muted);
        margin: 0 0 24px 0;
        font-size: 14px;
    }

    .sections {
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .tweak-section {
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 20px;
    }

    .tweak-section h3 {
        margin: 0 0 16px 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--accent-color);
    }

    .tweaks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 12px;
    }

    .empty-state {
        text-align: center;
        padding: 48px;
        color: var(--text-muted);
    }
</style>
