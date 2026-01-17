<script lang="ts">
    import { fade } from "svelte/transition";
    import { Target, Plug, HardDrive, Timer } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Card, CardGrid, BackButton, SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "scheduling" | "power" | "memory" | "timer" =
        "dashboard";

    // Filters
    $: schedulingTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("priority") ||
                t.id.includes("fth") ||
                t.id.includes("svchost") ||
                t.id.includes("page_combining") ||
                t.id.includes("sleep_study")),
    );

    $: powerTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("power") ||
                t.id.includes("performance") ||
                t.id.includes("usb") ||
                t.id.includes("pcie") ||
                t.id.includes("core_parking") ||
                t.id.includes("idle") ||
                t.id.includes("acpi") ||
                t.id.includes("throttle")),
    );

    $: memoryTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("ntfs") ||
                t.id.includes("memory") ||
                t.id.includes("cache") ||
                t.id.includes("paging") ||
                t.id.includes("prefetch")),
    );

    $: timerTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("timer") ||
                t.id.includes("tsc") ||
                t.id.includes("tick") ||
                t.id.includes("clock") ||
                t.id.includes("boot") ||
                t.id.includes("processor_check_interval")),
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
            id: "scheduling",
            icon: Target,
            title: "Scheduling & Priority",
            desc: "Optimize CPU thread handling and priorities.",
            tweaks: () => schedulingTweaks,
        },
        {
            id: "power",
            icon: Plug,
            title: "Power Management",
            desc: "High performance power plans and throttling.",
            tweaks: () => powerTweaks,
        },
        {
            id: "memory",
            icon: HardDrive,
            title: "Memory & Storage",
            desc: "RAM management, caching, and paging file.",
            tweaks: () => memoryTweaks,
        },
        {
            id: "timer",
            icon: Timer,
            title: "Timer & Boot",
            desc: "System timers, HPET, and boot configuration.",
            tweaks: () => timerTweaks,
        },
    ] as const;

    $: currentSection = sections.find((s) => s.id === currentView);
    $: currentTweaks = currentSection?.tweaks() ?? [];
</script>

<div class="cpu-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            {#each sections as section}
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
    .cpu-container {
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
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
