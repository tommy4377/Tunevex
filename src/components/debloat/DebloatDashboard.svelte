<script lang="ts">
    import { fade } from "svelte/transition";
    import { Package, Puzzle, Globe } from "lucide-svelte";
    import { onMount, onDestroy } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import { Card, CardGrid, BackButton } from "../ui";
    import AppsSection from "./AppsSection.svelte";
    import FeaturesSection from "./FeaturesSection.svelte";
    import EdgeSection from "./EdgeSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "apps" | "features" | "edge" = "dashboard";

    // Filters
    $: appsTweaks = allTweaks.filter(
        (t) =>
            t.category === "DebloatTelemetry" &&
            (t.id.startsWith("debloat_ms_") ||
                t.id.startsWith("debloat_hp") ||
                t.id.startsWith("debloat_dell") ||
                t.id.startsWith("debloat_lenovo") ||
                t.id.startsWith("debloat_asus") ||
                t.id.startsWith("debloat_msi") ||
                t.id.startsWith("debloat_acer") ||
                t.id.startsWith("debloat_razer") ||
                t.id.startsWith("debloat_mcafee") ||
                t.id.startsWith("debloat_norton") ||
                t.id === "debloat_onedrive" ||
                t.id === "debloat_thirdparty" ||
                t.id === "debloat_gaming" ||
                t.id === "debloat_widgets" ||
                t.id === "debloat_teams_chat_taskbar" ||
                t.id === "debloat_prevent_reinstall" ||
                t.id === "debloat_remove_edge_full" ||
                t.id === "debloat_remove_store" ||
                t.id === "debloat_disable_defender"),
    );

    $: featuresTweaks = allTweaks.filter(
        (t) =>
            t.category === "DebloatTelemetry" &&
            (t.id.includes("printer_features") ||
                t.id.includes("_ie_") ||
                t.id.includes("mediaplayer") ||
                t.id.includes("wordpad")),
    );

    $: edgeTweaks = allTweaks.filter(
        (t) =>
            t.category === "DebloatTelemetry" &&
            t.id.startsWith("debloat_edge_"),
    );

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let unlisten: any;

    onMount(async () => {
        unlisten = await listen("app-navigation", (event) => {
            const direction = event.payload as string;
            if (direction === "back" && currentView !== "dashboard") {
                currentView = "dashboard";
            }
        });
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });
</script>

<div class="debloat-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            <Card
                icon={Package}
                title="Apps & Bloatware"
                description="Remove pre-installed junk and third-party apps."
                status="{appsTweaks.length} tweaks"
                onclick={() => (currentView = "apps")}
            />
            <Card
                icon={Puzzle}
                title="Windows Features"
                description="Disable unused Windows components."
                status="{featuresTweaks.length} tweaks"
                onclick={() => (currentView = "features")}
            />
            <Card
                icon={Globe}
                title="Microsoft Edge"
                description="Debloat Edge and remove annoying features."
                status="{edgeTweaks.length} tweaks"
                onclick={() => (currentView = "edge")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            {#if currentView === "apps"}
                <AppsSection tweaks={appsTweaks} />
            {:else if currentView === "features"}
                <FeaturesSection tweaks={featuresTweaks} />
            {:else if currentView === "edge"}
                <EdgeSection tweaks={edgeTweaks} />
            {/if}
        </div>
    {/if}
</div>

<style>
    .debloat-container {
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
</style>
