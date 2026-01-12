<script lang="ts">
    import { fade } from "svelte/transition";
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

    // Sub-routes
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

    // Filters by tweak ID patterns
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

    // Hardening tweaks: including new ones (delivery opt, update medic, uac virtualization, auto maintenance)
    $: hardeningTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_block_sam_enum" ||
                t.id === "sec_disable_remote_assistance" ||
                t.id === "sec_disable_netbios" ||
                t.id === "sec_disable_llmnr" ||
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

    // SmartScreen tweaks
    $: smartscreenTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_smartscreen_apps" ||
                t.id === "sec_disable_smartscreen_edge" ||
                t.id === "sec_disable_smartscreen_store" ||
                t.id === "sec_disable_app_reputation" ||
                t.id === "sec_disable_protected_popup"),
    );

    // Error Reporting tweaks
    $: errorTweaks = allTweaks.filter(
        (t) =>
            t.category === "SecurityPrivacy" &&
            (t.id === "sec_disable_wer" ||
                t.id === "sec_disable_crash_dumps" ||
                t.id === "sec_disable_wer_service" ||
                t.id === "sec_disable_problem_dialog" ||
                t.id === "sec_disable_corp_wer"),
    );
</script>

<div class="security-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Defender Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "defender")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "defender")}
            >
                <div class="card-icon">🛡️</div>
                <h3>Windows Defender</h3>
                <p>Real-time protection, cloud features, and exclusions.</p>
                <div class="status">{defenderTweaks.length} tweaks</div>
            </div>

            <!-- Firewall Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "firewall")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "firewall")}
            >
                <div class="card-icon">🔥</div>
                <h3>Firewall</h3>
                <p>Windows Firewall settings and network rules.</p>
                <div class="status">{firewallTweaks.length} tweaks</div>
            </div>

            <!-- UAC Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "uac")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "uac")}
            >
                <div class="card-icon">👤</div>
                <h3>UAC Settings</h3>
                <p>User Account Control prompts and elevation.</p>
                <div class="status">{uacTweaks.length} tweaks</div>
            </div>

            <!-- Mitigations Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "mitigations")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "mitigations")}
            >
                <div class="card-icon">🚀</div>
                <h3>Exploit Mitigations</h3>
                <p>Spectre, DEP, VBS, HVCI, and process mitigations.</p>
                <div class="status danger">
                    {mitigationTweaks.length} tweaks
                </div>
            </div>

            <!-- Hardening Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "hardening")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "hardening")}
            >
                <div class="card-icon">🔒</div>
                <h3>Security Hardening</h3>
                <p>SMB, NetBIOS, LLMNR, maintenance, and updates.</p>
                <div class="status">{hardeningTweaks.length} tweaks</div>
            </div>

            <!-- Auth Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "auth")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "auth")}
            >
                <div class="card-icon">🔑</div>
                <h3>Authentication</h3>
                <p>Windows Hello, lock screen, and login settings.</p>
                <div class="status">{authTweaks.length} tweaks</div>
            </div>

            <!-- SmartScreen Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "smartscreen")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "smartscreen")}
            >
                <div class="card-icon">🔍</div>
                <h3>SmartScreen</h3>
                <p>App filtering, Edge protection, and Store checks.</p>
                <div class="status danger">
                    {smartscreenTweaks.length} tweaks
                </div>
            </div>

            <!-- Error Reporting Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "errorreporting")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "errorreporting")}
            >
                <div class="card-icon">📊</div>
                <h3>Error Reporting</h3>
                <p>Crash reports, memory dumps, and WER service.</p>
                <div class="status safe">{errorTweaks.length} tweaks</div>
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

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
        gap: 20px;
        margin-top: 16px;
        overflow-y: auto;
        flex: 1;
        padding: 20px;
        padding-top: 4px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 20px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
    }

    .card-icon {
        font-size: 28px;
        margin-bottom: 12px;
    }
    h3 {
        margin: 0 0 6px 0;
        font-size: 16px;
        font-weight: 600;
    }
    p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 13px;
        line-height: 1.4;
        flex-grow: 1;
    }

    .status {
        font-size: 11px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 5px 10px;
        border-radius: 16px;
    }

    .status.danger {
        color: #ef4444;
        background: rgba(239, 68, 68, 0.1);
    }

    .status.safe {
        color: #10b981;
        background: rgba(16, 185, 129, 0.1);
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        overflow: hidden; /* Contain scrollbar within this view */
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
