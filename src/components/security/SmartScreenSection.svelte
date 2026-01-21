<script lang="ts">
    import { Search } from "lucide-svelte";
    import { SectionHeader, InfoBanner } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";

    export let tweaks: Tweak[] = [];

    async function applySafe() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(e);
                }
            }
        }
        tweaks = tweaks;
    }
</script>

<div class="section-container">
    <SectionHeader
        icon={Search}
        title="SmartScreen Filter"
        description="Control SmartScreen protection for apps, Microsoft Edge, and Store apps."
        actionLabel="Apply Safe Tweaks"
        onAction={applySafe}
    />

    <InfoBanner
        variant="warning"
        message="Disabling SmartScreen reduces protection against malware and phishing."
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
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
