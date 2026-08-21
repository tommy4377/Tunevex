<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";
    import { Moon, Folder, Menu, Layout } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter UI tweaks
    $: uiTweaks = allTweaks.filter((t) => t.category === "InterfaceUx");

    $: contextTweaks = uiTweaks.filter(
        (t) =>
            t.id.includes("context_menu") ||
            t.id.includes("take_ownership") ||
            t.id.includes("context"),
    );

    $: explorerTweaks = uiTweaks.filter(
        (t) =>
            t.id.includes("extension") ||
            t.id.includes("compact") ||
            t.id.includes("hidden") ||
            t.id.includes("system_files") ||
            t.id.includes("jpeg") ||
            t.id.includes("home_namespace") ||
            t.id.includes("menu_show_delay") ||
            t.id.includes("launch_to_this_pc") ||
            t.id.includes("icon_cache") ||
            t.id.includes("auto_end_tasks") ||
            t.id.includes("aero_shake") ||
            t.id.includes("seconds_clock") ||
            t.id.includes("enthusiast") ||
            t.id.includes("no_resolve") ||
            t.id.includes("printscreen"),
    );

    $: appearanceTweaks = uiTweaks.filter(
        (t) => t.id.includes("dark_mode") || t.id.includes("theme") || t.id.includes("dynamic_lighting"),
    );

    $: taskbarTweaks = uiTweaks.filter((t) => t.id.includes("taskbar") || t.id.includes("auto_end_tasks") || t.id.includes("seconds_clock"));

    const sectionsMeta = [
        {
            id: "context",
            icon: Menu,
            title: "Context Menu",
            desc: "Classic menu and right-click options.",
            tweaks: () => contextTweaks,
        },
        {
            id: "explorer",
            icon: Folder,
            title: "File Explorer",
            desc: "Extensions, hidden files, compact mode.",
            tweaks: () => explorerTweaks,
        },
        {
            id: "appearance",
            icon: Moon,
            title: "Appearance",
            desc: "Dark mode and visual themes.",
            tweaks: () => appearanceTweaks,
        },
        {
            id: "taskbar",
            icon: Layout,
            title: "Taskbar",
            desc: "Taskbar and system tray settings.",
            tweaks: () => taskbarTweaks,
        },
    ] as const;

    $: sections = sectionsMeta.filter((s) => s.tweaks().length > 0);

    type View = "dashboard" | "detail";
    let currentView: View = "dashboard";
    let selectedSection: (typeof sectionsMeta)[number] | null = null;

    function openSection(section: (typeof sectionsMeta)[number]) {
        selectedSection = section;
        currentView = "detail";
    }

    function goBack() {
        currentView = "dashboard";
        selectedSection = null;
    }

    // Refresh tweaks from backend (used by TaskbarSection)
    async function refreshTweaks() {
        try {
            const fresh: Tweak[] = await invoke("get_tweaks");
            allTweaks = fresh;
        } catch (e) {
            console.error("Failed to refresh tweaks:", e);
        }
    }

    async function applyTweak(tweak: Tweak) {
        try {
            await invoke("apply_tweak", { id: tweak.id });
            tweak.enabled = true;
            allTweaks = [...allTweaks];
        } catch (e) {
            console.error("Failed to apply tweak:", e);
        }
    }

    async function revertTweak(tweak: Tweak) {
        try {
            await invoke("undo_tweak", { id: tweak.id });
            tweak.enabled = false;
            allTweaks = [...allTweaks];
        } catch (e) {
            console.error("Failed to revert tweak:", e);
        }
    }
</script>

<div class="ui-dashboard" in:fade>
    {#if currentView === "dashboard"}
        <CardGrid columns="repeat(auto-fit, minmax(240px, 1fr))">
            {#each sections.filter((s) => s.tweaks().length > 0) as section}
                <Card
                    icon={section.icon}
                    title={section.title}
                    description={section.desc}
                    status="{section.tweaks().length} options"
                    onclick={() => openSection(section)}
                />
            {/each}
        </CardGrid>
    {:else if selectedSection}
        <div class="detail-view">
            <BackButton onclick={goBack} />
            <h2>{selectedSection.title}</h2>
            <div class="tweak-list">
                {#each selectedSection.tweaks() as tweak}
                    <TweakCard
                        {tweak}
                        on:apply={() => applyTweak(tweak)}
                        on:revert={() => revertTweak(tweak)}
                    />
                {/each}
            </div>
        </div>
    {/if}
</div>

<style>
    .ui-dashboard {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        padding: 24px;
        gap: 20px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 16px;
        overflow: hidden;
    }

    .detail-view h2 {
        margin: 0;
        font-size: 20px;
        color: var(--text-color);
    }

    .tweak-list {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
</style>
