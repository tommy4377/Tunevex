<script lang="ts">
    import { fade } from "svelte/transition";
    import { Rocket, Zap, Info, Monitor } from "lucide-svelte";
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

    let currentView: "dashboard" | "general" | "msi" | "vendor" = "dashboard";

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

    $: vendorTweaks = allTweaks.filter(
        (t) =>
            t.category === "GpuOptimization" &&
            (t.id.includes("nvidia") ||
                t.id.includes("amd") ||
                t.id.includes("tdr")),
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
</script>

<div class="gpu-container">
    {#if currentView === "dashboard"}
        <div class="centered-cards">
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
            <Card
                icon={Monitor}
                title="Vendor Tweaks"
                description="NVIDIA, AMD, and TDR settings."
                status="{vendorTweaks.length} tweaks"
                onclick={() => (currentView = "vendor")}
            />
        </div>
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
                {:else if currentView === "vendor"}
                    <SectionHeader
                        icon={Monitor}
                        title="Vendor Tweaks"
                        description="NVIDIA, AMD, and TDR specific settings."
                    />
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={vendorTweaks} showHeader={false} />
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

    .centered-cards {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;
        padding: 24px;
        flex: 1;
        max-width: 400px;
        margin: 0 auto;
        width: 100%;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
    }

    .section-content {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        padding-right: 4px; /* Space for scrollbar */
    }

    .tweaks-wrapper {
        flex: 1;
        display: flex;
        flex-direction: column;
        max-width: 600px;
        margin: 0 auto;
        width: 100%;
        overflow-y: auto;
        overflow-x: hidden;
        padding-bottom: 24px;
    }
</style>
