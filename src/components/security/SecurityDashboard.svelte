<script lang="ts">
    import { fade } from "svelte/transition";
    import {
        Shield,
        Flame,
        User,
        Rocket,
        Lock,
        Key,
        Search,
        BarChart3,
    } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import DefenderSection from "./DefenderSection.svelte";
    import FirewallSection from "./FirewallSection.svelte";
    import UacSection from "./UacSection.svelte";
    import MitigationsSection from "./MitigationsSection.svelte";
    import HardeningSection from "./HardeningSection.svelte";
    import AuthSection from "./AuthSection.svelte";
    import SmartScreenSection from "./SmartScreenSection.svelte";
    import ErrorReportingSection from "./ErrorReportingSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView:
        | "dashboard"
        | "defender"
        | "firewall"
        | "uac"
        | "mitigations"
        | "hardening"
        | "auth"
        | "smartscreen"
        | "errorreporting" = "dashboard";

    // Filters
    $: defenderTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_realtime" ||
                t.id === "sec_disable_cloud" ||
                t.id === "sec_disable_samples" ||
                t.id === "sec_dev_exclusions" ||
                t.id === "sec_disable_pua" ||
                t.id === "sec_disable_defender"),
    );

    $: firewallTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id.startsWith("sec_fw_") || t.id === "sec_disable_firewall"),
    );

    $: uacTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id.startsWith("sec_uac_") || t.id === "sec_disable_uac"),
    );

    $: mitigationTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_spectre" ||
                t.id === "sec_disable_dep" ||
                t.id === "sec_disable_kvas" ||
                t.id === "sec_disable_hvci" ||
                t.id === "sec_disable_vbs" ||
                t.id === "sec_disable_sehop" ||
                t.id === "sec_disable_cfg" ||
                t.id === "sec_disable_all_mitigations"),
    );

    $: hardeningTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_block_sam_enum" ||
                t.id === "sec_disable_remote_assistance" ||
                t.id === "sec_disable_netbios" ||
                t.id === "net_disable_netbios" ||
                t.id === "sec_disable_llmnr" ||
                t.id === "net_disable_llmnr" ||
                t.id === "net_restrict_anonymous_access" ||
                t.id === "net_restrict_anonymous_enum" ||
                t.id === "net_disable_smb_throttling" ||
                t.id === "sec_disable_smbv1" ||
                t.id === "sec_enable_smb_signing" ||
                t.id === "sec_disable_delivery_opt" ||
                t.id === "sec_disable_update_medic" ||
                t.id === "sec_disable_uac_virtualization" ||
                t.id === "sec_disable_auto_maintenance"),
    );

    $: authTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_hello" ||
                t.id === "sec_disable_lockscreen" ||
                t.id === "sec_no_password_reveal" ||
                t.id === "sec_no_signin_sleep" ||
                t.id === "sec_auto_login" ||
                t.id === "sec_disable_rdp"),
    );

    $: smartscreenTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_smartscreen_apps" ||
                t.id === "sec_disable_smartscreen_edge" ||
                t.id === "sec_disable_smartscreen_store" ||
                t.id === "sec_disable_app_reputation" ||
                t.id === "sec_disable_protected_popup"),
    );

    $: errorTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_wer" ||
                t.id === "sec_disable_crash_dumps" ||
                t.id === "sec_disable_wer_service" ||
                t.id === "sec_disable_problem_dialog" ||
                t.id === "sec_disable_corp_wer"),
    );

    const sections = [
        {
            id: "defender",
            icon: Shield,
            title: "Windows Defender",
            desc: "Real-time protection, cloud features, and exclusions.",
            tweaks: () => defenderTweaks,
        },
        {
            id: "firewall",
            icon: Flame,
            title: "Firewall",
            desc: "Windows Firewall settings and network rules.",
            tweaks: () => firewallTweaks,
        },
        {
            id: "uac",
            icon: User,
            title: "UAC Settings",
            desc: "User Account Control prompts and elevation.",
            tweaks: () => uacTweaks,
        },
        {
            id: "mitigations",
            icon: Rocket,
            title: "Exploit Mitigations",
            desc: "Spectre, DEP, VBS, HVCI, and process mitigations.",
            tweaks: () => mitigationTweaks,
            variant: "danger",
        },
        {
            id: "hardening",
            icon: Lock,
            title: "Security Hardening",
            desc: "SMB, NetBIOS, LLMNR, maintenance, and updates.",
            tweaks: () => hardeningTweaks,
        },
        {
            id: "auth",
            icon: Key,
            title: "Authentication",
            desc: "Windows Hello, lock screen, and login settings.",
            tweaks: () => authTweaks,
        },
        {
            id: "smartscreen",
            icon: Search,
            title: "SmartScreen",
            desc: "App filtering, Edge protection, and Store checks.",
            tweaks: () => smartscreenTweaks,
            variant: "danger",
        },
        {
            id: "errorreporting",
            icon: BarChart3,
            title: "Error Reporting",
            desc: "Crash reports, memory dumps, and WER service.",
            tweaks: () => errorTweaks,
            variant: "safe",
        },
    ] as const;
</script>

<div class="security-container">
    {#if currentView === "dashboard"}
        <CardGrid columns="repeat(3, 1fr)" gap="16px">
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

            {#if currentView === "defender"}
                <DefenderSection tweaks={defenderTweaks} />
            {:else if currentView === "firewall"}
                <FirewallSection tweaks={firewallTweaks} />
            {:else if currentView === "uac"}
                <UacSection tweaks={uacTweaks} />
            {:else if currentView === "mitigations"}
                <MitigationsSection tweaks={mitigationTweaks} />
            {:else if currentView === "hardening"}
                <HardeningSection tweaks={hardeningTweaks} />
            {:else if currentView === "auth"}
                <AuthSection tweaks={authTweaks} />
            {:else if currentView === "smartscreen"}
                <SmartScreenSection tweaks={smartscreenTweaks} />
            {:else if currentView === "errorreporting"}
                <ErrorReportingSection tweaks={errorTweaks} />
            {/if}
        </div>
    {/if}
</div>

<style>
    .security-container {
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
        overflow-y: auto;
    }
</style>
