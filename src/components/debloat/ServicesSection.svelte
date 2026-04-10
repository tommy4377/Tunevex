<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { Server, Check } from "lucide-svelte";

    export let tweaks: Tweak[] = [];

    async function applyAllSafe() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error("Failed to apply tweak:", tweak.id, e);
                    alert(`Failed to apply ${tweak.name}: ${e}`);
                }
            }
        }
        tweaks = tweaks; // Trigger reactivity
    }
</script>

<div class="section-container">
    <div class="header">
        <h2><Server size={20} /> Services & Tasks</h2>
        <p>
            Disable unnecessary Windows services and scheduled tasks.
        </p>
        <button class="optimize-btn safe" on:click={applyAllSafe}>
            <Check size={14} />
            Apply Safe Tweaks
        </button>
    </div>
    <div class="tweaks-wrapper">
        <TweakList {tweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    .header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: var(--border-glass);
    }
    h2 {
        font-size: 20px;
        margin-bottom: 8px;
        display: flex;
        align-items: center;
        gap: 10px;
        color: var(--text-color);
    }
    h2 :global(svg) {
        color: var(--accent-color);
    }
    p {
        color: var(--text-muted);
        font-size: 14px;
        margin-bottom: 16px;
    }
    .optimize-btn {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        border: none;
        padding: 8px 16px;
        border-radius: 8px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
    }
    .optimize-btn.safe {
        background: var(--btn-safe-bg);
        border: 1px solid var(--btn-safe-border);
        color: var(--btn-safe-color);
    }
    .optimize-btn.safe:hover {
        background: var(--btn-safe-hover-bg);
    }
    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>