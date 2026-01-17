<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    export let tweaks: Tweak[] = [];
</script>

<div class="section-container">
    <div class="header">
        <h2>User Account Control</h2>
        <p>Configure UAC prompts, secure desktop, and admin approval mode.</p>
        <div class="warning-banner">
            ⚠️ Disabling UAC significantly reduces system security
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
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
    h2 {
        font-size: 20px;
        margin-bottom: 8px;
    }
    p {
        color: var(--text-muted);
        font-size: 14px;
        margin: 0 0 16px 0;
    }
    .warning-banner {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
        padding: 12px;
        border-radius: 8px;
        font-size: 13px;
        margin-bottom: 16px;
        align-self: flex-start;
    }
    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
