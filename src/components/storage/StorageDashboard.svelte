<script lang="ts">
    import { fade } from "svelte/transition";
    import { Folder, HardDrive, Brush, Archive } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        Card,
        CardGrid,
        BackButton,
        SectionHeader,
        InfoBanner,
    } from "../ui";
    import Compactor from "./Compactor.svelte";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView:
        | "dashboard"
        | "filesystem"
        | "nvme"
        | "maintenance"
        | "compactor" = "dashboard";

    // Filters
    $: filesystemTweaks = allTweaks.filter(
        (t) =>
            t.category === "FileSystem" &&
            (t.id.includes("ntfs") ||
                t.id.includes("timestamp") ||
                t.id.includes("search") ||
                t.id.includes("shortname")),
    );

    $: nvmeTweaks = allTweaks.filter(
        (t) =>
            t.category === "FileSystem" &&
            (t.id.includes("nvme") ||
                t.id.includes("trim") ||
                t.id.includes("write_cache") ||
                t.id.includes("power") ||
                t.id.includes("apm") ||
                t.id.includes("msi")),
    );

    $: maintenanceTweaks = allTweaks.filter(
        (t) =>
            t.category === "FileSystem" &&
            (t.id.includes("hibernation") ||
                t.id.includes("storage_sense") ||
                t.id.includes("restore") ||
                t.id.includes("prefetch")),
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

<div class="storage-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            <Card
                icon={Folder}
                title="Filesystem (NTFS)"
                description="Optimize metadata, timestamps, and search indexing."
                status="{filesystemTweaks.length} tweaks"
                onclick={() => (currentView = "filesystem")}
            />
            <Card
                icon={HardDrive}
                title="NVMe & SSD"
                description="Power settings, MSI mode, and write caching."
                status="{nvmeTweaks.length} tweaks"
                onclick={() => (currentView = "nvme")}
            />
            <Card
                icon={Brush}
                title="Maintenance Tools"
                description="Hibernation, storage sense, and cleanup utilities."
                status="{maintenanceTweaks.length} tweaks"
                onclick={() => (currentView = "maintenance")}
            />
            <Card
                icon={Archive}
                title="Compactor"
                description="Compress folders with Windows transparent compression. Safe for games and apps."
                status="Space Saver"
                featured={true}
                onclick={() => (currentView = "compactor")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            <div class="section-content">
                {#if currentView === "filesystem"}
                    <SectionHeader
                        icon={Folder}
                        title="Filesystem Optimization"
                        description="Improve NTFS performance and reduce overhead."
                        actionLabel="Apply Safe Tweaks"
                        onAction={() => applySafeTweaks(filesystemTweaks)}
                    />
                    <div class="tweaks-wrapper">
                        <TweakList
                            tweaks={filesystemTweaks}
                            showHeader={false}
                        />
                    </div>
                {:else if currentView === "nvme"}
                    <SectionHeader
                        icon={HardDrive}
                        title="NVMe & SSD"
                        description="Maximize drive throughput and responsiveness."
                        actionLabel="Apply Safe Tweaks"
                        onAction={() => applySafeTweaks(nvmeTweaks)}
                    />
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={nvmeTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "maintenance"}
                    <SectionHeader
                        icon={Brush}
                        title="Maintenance Tools"
                        description="Hibernation, storage sense, and cleanup."
                        actionLabel="Apply Safe Tweaks"
                        onAction={() => applySafeTweaks(maintenanceTweaks)}
                    />
                    <div class="tweaks-wrapper">
                        {#if maintenanceTweaks.length > 0}
                            <TweakList
                                tweaks={maintenanceTweaks}
                                showHeader={false}
                            />
                        {:else}
                            <div class="empty-state">
                                No maintenance tweaks available.
                            </div>
                        {/if}
                    </div>
                {:else if currentView === "compactor"}
                    <SectionHeader
                        icon={Archive}
                        title="Compactor"
                        description="Compress folders using Windows transparent compression."
                    />
                    <InfoBanner variant="info" message="">
                        <strong>Safe Compression:</strong> Already-compressed files
                        (images, videos, archives) and system files are automatically
                        skipped to prevent issues.
                    </InfoBanner>
                    <div class="tweaks-wrapper">
                        <Compactor />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .storage-container {
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
        min-height: 0;
        padding-bottom: 24px;
    }

    .empty-state {
        text-align: center;
        padding: 40px 20px;
        color: var(--text-muted);
        font-size: 14px;
    }
</style>
