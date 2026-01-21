<script lang="ts">
    import { fade } from "svelte/transition";
    import {
        Monitor,
        Smartphone,
        Settings,
        Megaphone,
        Shield,
        Brush,
    } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Card, CardGrid, BackButton, SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView:
        | "dashboard"
        | "windows"
        | "apps"
        | "settings"
        | "advertising"
        | "policies"
        | "maintenance" = "dashboard";

    // Filters
    $: windowsTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_disable_telemetry" ||
                t.id === "priv_disable_ceip" ||
                t.id === "priv_disable_wer" ||
                t.id === "priv_disable_input_telemetry" ||
                t.id === "priv_disable_telemetry_tasks"),
    );

    $: appTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_nvidia_telemetry" ||
                t.id === "priv_office_telemetry" ||
                t.id === "priv_vs_telemetry" ||
                t.id === "priv_vscode_telemetry" ||
                t.id === "priv_dotnet_telemetry" ||
                t.id === "priv_powershell_telemetry" ||
                t.id === "priv_chrome_telemetry" ||
                t.id === "priv_firefox_telemetry" ||
                t.id === "priv_disable_background_apps"),
    );

    $: settingsTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id.includes("activity") ||
                t.id.includes("timeline") ||
                t.id.includes("location") ||
                t.id.includes("speech") ||
                t.id.includes("sync") ||
                t.id === "priv_app_permissions" ||
                t.id === "priv_disable_lang_list" ||
                t.id === "priv_lockscreen_camera" ||
                t.id === "priv_config_wmp" ||
                t.id === "priv_disable_app_tracking" ||
                t.id === "priv_disable_perftrack"),
    );

    $: adTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id.includes("advertising") ||
                t.id.includes("tailored") ||
                t.id.includes("suggestions") ||
                t.id === "priv_disable_sync_notifs" ||
                t.id === "priv_all_in_one"),
    );

    $: policiesTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_disable_oobe_privacy" ||
                t.id === "priv_disallow_ms_accounts" ||
                t.id === "priv_disable_rsop" ||
                t.id === "priv_disable_experimentation" ||
                t.id === "priv_disable_smart_app_control"),
    );

    $: maintenanceTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_config_storage_sense" ||
                t.id === "priv_disable_reserved_storage" ||
                t.id === "priv_disable_maintenance_wakeup"),
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
            id: "windows",
            icon: Monitor,
            title: "Windows Telemetry",
            desc: "Core telemetry, DiagTrack, CEIP, and input collection.",
            tweaks: () => windowsTweaks,
        },
        {
            id: "apps",
            icon: Smartphone,
            title: "App Telemetry",
            desc: "NVIDIA, Office, VS Code, Chrome, Firefox, .NET.",
            tweaks: () => appTweaks,
        },
        {
            id: "settings",
            icon: Settings,
            title: "Privacy Settings",
            desc: "Activity, location, speech, app permissions, camera.",
            tweaks: () => settingsTweaks,
        },
        {
            id: "advertising",
            icon: Megaphone,
            title: "Advertising & Ads",
            desc: "Ad ID, tailored experiences, suggestions, explorer ads.",
            tweaks: () => adTweaks,
        },
        {
            id: "policies",
            icon: Shield,
            title: "System Policies",
            desc: "OOBE, MS Accounts, RSoP, Experiments, Smart App Control.",
            tweaks: () => policiesTweaks,
        },
        {
            id: "maintenance",
            icon: Brush,
            title: "Maintenance",
            desc: "Storage Sense, Reserved Storage, Wake-up timers.",
            tweaks: () => maintenanceTweaks,
        },
    ] as const;

    $: currentSection = sections.find((s) => s.id === currentView);
    $: currentTweaks = currentSection?.tweaks() ?? [];
</script>

<div class="privacy-container">
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
    .privacy-container {
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
        overflow: hidden;
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
