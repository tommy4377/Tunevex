<script lang="ts">
    import { Plug } from "lucide-svelte";
    import { SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";

    export let tweaks: Tweak[] = [];

    async function autoOptimize() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                tweak.enabled = await invoke<boolean | null>("apply_tweak", { id: tweak.id });
            }
        }
        tweaks = tweaks; // Trigger reactivity
    }
</script>

<div class="section-container">
    <SectionHeader
        icon={Plug}
        title="Network Adapter Settings"
        description="Configure offloading, flow control, and interrupt moderation for your NIC."
        actionLabel="One-Click Optimize"
        onAction={autoOptimize}
    />

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
    .tweaks-wrapper {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        display: flex;
        flex-direction: column;
        padding-bottom: 24px;
    }
</style>
