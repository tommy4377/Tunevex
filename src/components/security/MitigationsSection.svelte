<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    export let tweaks: Tweak[] = [];
</script>

<div class="section-container">
    <div class="header">
        <h2>🚀 Exploit Mitigations</h2>
        <p>
            Control CPU vulnerability mitigations (Spectre/Meltdown), DEP, VBS,
            HVCI, and process protections.
        </p>
        <div class="warning-banner danger">
            ⛔ DANGER: These settings significantly reduce system security. Only
            disable for isolated gaming PCs.
        </div>
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
    .warning-banner {
        padding: 12px;
        border-radius: 8px;
        font-size: 13px;
    }
    .warning-banner.danger {
        background: rgba(239, 68, 68, 0.15);
        color: #ef4444;
        border: 1px solid rgba(239, 68, 68, 0.3);
    }
    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
