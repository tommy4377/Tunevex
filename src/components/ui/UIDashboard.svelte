<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";
    import { Target, Moon, Folder, Settings, Palette } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter UI tweaks
    $: uiTweaks = allTweaks.filter((t) => t.category === "InterfaceUx");

    // Organize by section
    $: taskbarTweaks = uiTweaks.filter(
        (t) => t.id.includes("taskbar") || t.id.includes("TaskbarGlomLevel"),
    );
    $: visualTweaks = uiTweaks.filter(
        (t) => t.id.includes("context") || t.id.includes("dark"),
    );
    $: explorerTweaks = uiTweaks.filter(
        (t) =>
            t.id.includes("extension") ||
            t.id.includes("file") ||
            t.id.includes("compact") ||
            t.id.includes("hidden"),
    );
    $: otherTweaks = uiTweaks.filter(
        (t) =>
            !taskbarTweaks.includes(t) &&
            !visualTweaks.includes(t) &&
            !explorerTweaks.includes(t),
    );

    const sectionsMeta = [
        {
            id: "taskbar",
            icon: Target,
            title: "Taskbar",
            desc: "Taskbar layout & behavior",
            tweaks: () => taskbarTweaks,
        },
        {
            id: "appearance",
            icon: Moon,
            title: "Appearance",
            desc: "Dark mode & context menus",
            tweaks: () => visualTweaks,
        },
        {
            id: "explorer",
            icon: Folder,
            title: "Explorer",
            desc: "File Explorer tweaks",
            tweaks: () => explorerTweaks,
        },
        {
            id: "other",
            icon: Settings,
            title: "Other",
            desc: "Additional tweaks",
            tweaks: () => otherTweaks,
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
        <div class="header">
            <div class="header-icon"><Palette size={28} /></div>
            <h2>Interface Tweaks</h2>
            <p class="subtitle">Customize Windows appearance and behavior</p>
        </div>

        <CardGrid columns="repeat(auto-fill, minmax(200px, 1fr))" gap="16px">
            {#each sections as section}
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
            <BackButton onclick={goBack} label="Back" />
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

    .header {
        text-align: center;
    }

    .header-icon {
        color: var(--accent-color);
        margin-bottom: 8px;
    }

    .header h2 {
        margin: 0;
        font-size: 28px;
        color: var(--text-color);
    }

    .subtitle {
        color: var(--text-muted);
        margin: 8px 0 0;
        font-size: 14px;
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
