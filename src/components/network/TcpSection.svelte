<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";

    export let tweaks: Tweak[] = [];

    async function autoOptimize() {
        // Logic to apply all "Safe" tcp tweaks
        // For MVP, apply specifically `net_tcp_ack_freq`, `net_global_default_ttl`, etc.
        // We'll iterate the safe ones and apply them.
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true; // Optimistic update
            }
        }
    }
</script>

<div class="section-container">
    <div class="header">
        <h2>TCP/IP Optimization</h2>
        <p>
            Optimize packet handling, timestamps, and request queues for lower
            latency.
        </p>
        <button class="optimize-btn" on:click={autoOptimize}>
            ⚡ One-Click Optimize (Safe)
        </button>
    </div>

    <!-- Re-use the TweakList logic but filter passed tweaks -->
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
        border-bottom: 1px solid var(--border-color);
    }

    h2 {
        font-size: 20px;
        margin-bottom: 8px;
    }
    p {
        color: var(--text-muted);
        font-size: 14px;
        margin-bottom: 16px;
    }

    .optimize-btn {
        background: #3b82f6;
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
    }

    .optimize-btn:hover {
        background: #2563eb;
    }

    .tweaks-wrapper {
        flex: 1;
        overflow: hidden; /* Logic handled by TweakList scroll */
        display: flex; /* Fix TweakList fill */
        flex-direction: column;
        flex-direction: column;
        /* margin: 0 -24px; Removed as parent now handles padding */
    }

    /* Hack: TweakList has its own padding/header. We might need to hide its header via prop */
</style>
