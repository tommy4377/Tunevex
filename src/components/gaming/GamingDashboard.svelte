<script lang="ts">
    import { fade } from "svelte/transition";
    import { Gamepad2, Palette, Zap } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Card, CardGrid, BackButton, SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "general" | "visuals" | "priority" =
        "dashboard";

    // Filters
    $: visualTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            (t.id.includes("dvr") ||
                t.id.includes("gamebar") ||
                t.id.includes("fso") ||
                t.id.includes("transparency") ||
                t.id.includes("visual")),
    );

    $: priorityTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            (t.id.includes("mmcss") ||
                t.id.includes("priority") ||
                t.id.includes("affinity") ||
                t.id.includes("throttling")),
    );

    $: generalTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            !visualTweaks.includes(t) &&
            !priorityTweaks.includes(t),
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

    const sections = [
        {
            id: "general",
            icon: Gamepad2,
            title: "General Gaming",
            desc: "Core Windows gaming settings and Game Mode.",
            tweaks: () => generalTweaks,
        },
        {
            id: "visuals",
            icon: Palette,
            title: "Visual Optimizations",
            desc: "Disable overlays, DVR, and full-screen optimizations.",
            tweaks: () => visualTweaks,
        },
        {
            id: "priority",
            icon: Zap,
            title: "Process & Priority",
            desc: "MMCSS scheduling and CPU priority boosting.",
            tweaks: () => priorityTweaks,
        },
    ] as const;

    $: currentSection = sections.find((s) => s.id === currentView);
    $: currentTweaks = currentSection?.tweaks() ?? [];
</script>

<div class="gaming-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            {#each sections.filter((s) => s.tweaks().length > 0) as section}
                <Card
                    icon={section.icon}
                    title={section.title}
                    description={section.desc}
                    status="{section.tweaks().length} tweaks"
                    onclick={() => (currentView = section.id)}
                />
            {/each}
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            <div class="section-content">
                {#if currentSection}
                    <SectionHeader
                        icon={currentSection.icon}
                        title={currentSection.title}
                        description={currentSection.desc}
                        actionLabel="Apply Safe Tweaks"
                        onAction={() => applySafeTweaks(currentTweaks)}
                    />
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={currentTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .gaming-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
    }

    .section-content {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
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
