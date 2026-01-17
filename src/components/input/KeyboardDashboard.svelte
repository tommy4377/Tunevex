<script lang="ts">
    import { Keyboard } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    $: keyboardTweaks = allTweaks.filter(
        (t) =>
            (t.category === "Input" || t.category === "MouseInput") &&
            (t.id.includes("keyboard") || t.id.includes("filter")),
    );

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
        icon={Keyboard}
        title="Keyboard Response"
        description="Optimize keyboard data queue size and repeat rates."
        actionLabel="Apply Safe Tweaks"
        onAction={() => applySafeTweaks(keyboardTweaks)}
    />

    <div class="tweaks-wrapper">
        <TweakList tweaks={keyboardTweaks} showHeader={false} />
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
