<script lang="ts">
    import { fade } from "svelte/transition";
    import { Brush, Settings, Monitor } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Card, CardGrid, BackButton, SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "maintenance" | "services" | "system" =
        "dashboard";

    // Filters
    $: maintenanceTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            (t.id.includes("maintenance") ||
                t.id.includes("restore") ||
                t.id.includes("cleanup") ||
                t.id.includes("bso_d") ||
                t.id.includes("restart")),
    );

    $: servicesTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            (t.id.includes("service") ||
                t.id.includes("update") ||
                t.id.includes("fax") ||
                t.id.includes("print") ||
                t.id.includes("bloat")),
    );

    $: systemTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            !maintenanceTweaks.includes(t) &&
            !servicesTweaks.includes(t),
    );

    async function applySafeTweaks(tweaks: Tweak[]) {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    tweak.enabled = await invoke<boolean | null>("apply_tweak", { id: tweak.id });
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        allTweaks = allTweaks;
    }

    const sections = [
        {
            id: "system",
            icon: Monitor,
            title: "System & Hardware",
            desc: "General system tweaks and hardware configurations.",
            tweaks: () => systemTweaks,
        },
    ] as const;

    $: currentSection = sections.find((s) => s.id === currentView);
    $: currentTweaks = currentSection?.tweaks() ?? [];
</script>

<div class="system-container">
    {#if currentView === "dashboard"}
        <CardGrid columns="repeat(auto-fit, minmax(210px, 1fr))">
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
    .system-container {
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
