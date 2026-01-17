<script lang="ts">
    import { Mouse, Check } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { SectionHeader } from "../ui";
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
    <SectionHeader
        icon={Mouse}
        title="Mouse Optimization"
        description="Reduce input lag, disable acceleration, and adjust scaling."
        actionLabel="Apply Safe Tweaks"
        onAction={() => applySafeTweaks(mouseTweaks)}
    />

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

    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
    }
</style>
