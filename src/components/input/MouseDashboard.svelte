<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    $: mouseTweaks = allTweaks.filter((t) => t.category === "MouseInput");

    async function applySafeTweaks(tweaks: Tweak[]) {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        allTweaks = allTweaks;
    }
</script>

<div class="section-container">
    <div class="section-header">
        <div class="header-text">
            <h2>🖱️ Mouse Optimization</h2>
            <p>Reduce input lag, disable acceleration, and adjust scaling.</p>
        </div>
        <button
            class="optimize-btn safe"
            on:click={() => applySafeTweaks(mouseTweaks)}
        >
            ✅ Apply Safe Tweaks
        </button>
    </div>

    <div class="tweaks-wrapper">
        <TweakList tweaks={mouseTweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .tweaks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, 320px);
        gap: 16px;
    }

    .section-header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .header-text h2 {
        font-size: 20px;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }
    .header-text p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 14px;
    }

    .optimize-btn {
        border: none;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        font-weight: 500;
        cursor: pointer;
        color: white;
        white-space: nowrap;
    }
    .optimize-btn.safe {
        background: #10b981;
    }
    .optimize-btn.safe:hover {
        background: #059669;
    }

    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
    }
</style>
