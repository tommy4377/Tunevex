<script lang="ts">
    import { Shield } from "lucide-svelte";
    import { SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";

    export let tweaks: Tweak[] = [];

    async function applyAllSafe() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
        }
        tweaks = tweaks; // Trigger reactivity
    }
</script>

<div class="section-container">
    <SectionHeader
        icon={Shield}
        title="Windows Defender"
        description="Configure Windows Defender antivirus protection, cloud features, and scan exclusions."
        actionLabel="Apply Safe Tweaks"
        onAction={applyAllSafe}
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
