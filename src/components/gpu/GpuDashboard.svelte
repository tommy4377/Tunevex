<script lang="ts">
    import { fade } from "svelte/transition";
    import { Rocket, Zap, Info } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        Card,
        CardGrid,
        BackButton,
        SectionHeader,
        InfoBanner,
    } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "general" | "msi" = "dashboard";

    // Filters
    $: schedulingTweaks = allTweaks.filter(
        (t) =>
            t.category === "GpuOptimization" &&
            (t.id.includes("hags") ||
                t.id.includes("preemption") ||
                t.id.includes("mpo") ||
                t.id.includes("priority")),
    );

    $: msiTweaks = allTweaks.filter(
        (t) => t.category === "GpuOptimization" && t.id.includes("msi"),
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

<div class="gpu-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            <Card
                icon={Rocket}
                title="General Optimization"
                description="HAGS, Game Mode, and Priority adjustments."
                status="{schedulingTweaks.length} tweaks"
                onclick={() => (currentView = "general")}
            />
            <Card
                icon={Zap}
                title="MSI Mode"
                description="Message Signaled Interrupts for GPU."
                status="{msiTweaks.length} tweaks"
                onclick={() => (currentView = "msi")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            <div class="section-content">
                {#if currentView === "general"}
                    <SectionHeader
                        icon={Rocket}
                        title="General Optimization"
                        description="Core GPU scheduling and system settings."
                        actionLabel="Apply Safe Tweaks"
                        onAction={() => applySafeTweaks(schedulingTweaks)}
                    />
                    <div class="tweaks-wrapper">
                        <TweakList
                            tweaks={schedulingTweaks}
                            showHeader={false}
                        />
                    </div>
                {:else if currentView === "msi"}
                    <SectionHeader
                        icon={Zap}
                        title="GPU MSI Mode"
                        description="Enable Message Signaled Interrupts for lower latency."
                    />
                    <InfoBanner
                        variant="info"
                        message="High Priority is recommended for dedicated GPUs."
                    />
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={msiTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .gpu-container {
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
