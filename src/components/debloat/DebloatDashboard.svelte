<script lang="ts">
    import { fade } from "svelte/transition";
    import AppsSection from "./AppsSection.svelte";
    import FeaturesSection from "./FeaturesSection.svelte";
    import EdgeSection from "./EdgeSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Sub-routes for detailed views
    let currentView: "dashboard" | "apps" | "features" | "edge" = "dashboard";

    // Filter tweaks
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
                t.id === "debloat_prevent_reinstall"),
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

    import { onMount, onDestroy } from "svelte";
    import { listen } from "@tauri-apps/api/event";

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
        <div class="dashboard-grid" in:fade>
            <!-- Apps & Bloatware Card -->
            <div
                class="card apps-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "apps")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "apps")}
            >
                <div class="card-icon">📦</div>
                <h3>Apps & Bloatware</h3>
                <p>Remove pre-installed junk and third-party apps.</p>
                <div class="status">{appsTweaks.length} tweaks</div>
            </div>

            <!-- Features Card -->
            <div
                class="card features-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "features")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "features")}
            >
                <div class="card-icon">🧩</div>
                <h3>Windows Features</h3>
                <p>Disable unused Windows components.</p>
                <div class="status">{featuresTweaks.length} tweaks</div>
            </div>

            <!-- Edge Card -->
            <div
                class="card edge-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "edge")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "edge")}
            >
                <div class="card-icon">🌐</div>
                <h3>Microsoft Edge</h3>
                <p>Debloat Edge and remove annoying features.</p>
                <div class="status">{edgeTweaks.length} tweaks</div>
            </div>
        </div>
    {:else}
        <div class="detail-view" in:fade>
            <button
                class="back-btn"
                on:click={() => (currentView = "dashboard")}
            >
                ← Back to Dashboard
            </button>

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
        /* Padding removed here to allow full-width/height scroll masking */
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 24px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        /* Moved padding here so content scrolls 'into' the padding, not clipped by parent */
        padding: 24px;
        padding-top: 4px; /* Small top padding for hover clearance */
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 24px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        position: relative; /* Base for z-index */
        z-index: 1; /* Default layer */
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
        z-index: 10; /* Fix: Ensure card stays on top when scaled */
        position: relative; /* Often needed for z-index to work */
    }

    .card-icon {
        font-size: 32px;
        margin-bottom: 16px;
    }

    h3 {
        margin: 0 0 8px 0;
        font-size: 18px;
        font-weight: 600;
    }

    p {
        margin: 0 0 24px 0;
        color: var(--text-muted);
        font-size: 14px;
        line-height: 1.5;
        flex-grow: 1;
    }

    .status {
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 6px 12px;
        border-radius: 20px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px; /* Fix: Global padding for all detail views */
    }

    .back-btn {
        align-self: flex-start;
        background: none;
        border: none;
        color: var(--text-muted);
        font-size: 14px;
        cursor: pointer;
        padding: 8px 0;
        margin-bottom: 16px;
        transition: color 0.2s;
    }

    .back-btn:hover {
        color: var(--text-color);
    }
</style>
