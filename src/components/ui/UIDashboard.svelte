<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter UI Classic tweaks
    $: uiTweaks = allTweaks.filter(
        (t) => t.category === "InterfaceUx" && t.id.startsWith("ui."),
    );

    // Organize by section
    $: patchTweaks = uiTweaks.filter((t) => t.id === "ui.patch_uxtheme");
    $: taskbarTweaks = uiTweaks.filter((t) =>
        ["ui.taskbar_classic", "ui.taskbar_top", "ui.taskbar_center"].includes(
            t.id,
        ),
    );
    $: startTweaks = uiTweaks.filter((t) =>
        [
            "ui.start_orb_win7",
            "ui.start_orb_clover",
            "ui.start_win7_menu",
        ].includes(t.id),
    );
    $: visualTweaks = uiTweaks.filter((t) =>
        [
            "ui.context_classic",
            "ui.dark_full",
            "ui.explorer_ribbon_dark",
            "ui.explorer_ribbon_light",
        ].includes(t.id),
    );
    $: themeTweaks = uiTweaks.filter((t) =>
        ["ui.msstyles_win7", "ui.msstyles_plain8"].includes(t.id),
    );
    $: systemTweaks = uiTweaks.filter((t) =>
        ["ui.details_bottom", "ui.search_classic", "ui.tray_win10"].includes(
            t.id,
        ),
    );
    $: profileTweaks = uiTweaks.filter((t) => t.id === "ui.profile_win7_full");

    interface Section {
        title: string;
        icon: string;
        color: string;
        tweaks: Tweak[];
        description: string;
    }

    $: sections = [
        {
            title: "UxTheme Patch",
            icon: "🔓",
            color: "#ef4444",
            tweaks: patchTweaks,
            description: "Required for custom themes",
        },
        {
            title: "Taskbar",
            icon: "🎯",
            color: "#3b82f6",
            tweaks: taskbarTweaks,
            description: "Classic taskbar options",
        },
        {
            title: "Start Menu",
            icon: "🔵",
            color: "#8b5cf6",
            tweaks: startTweaks,
            description: "Orbs & menu style",
        },
        {
            title: "Visual",
            icon: "👁️",
            color: "#10b981",
            tweaks: visualTweaks,
            description: "Context menus & ribbon",
        },
        {
            title: "Themes",
            icon: "🎨",
            color: "#f59e0b",
            tweaks: themeTweaks,
            description: "Apply custom msstyles",
        },
        {
            title: "System",
            icon: "⚙️",
            color: "#6b7280",
            tweaks: systemTweaks,
            description: "Search & tray",
        },
    ] as Section[];

    type View = "dashboard" | "detail";
    let currentView: View = "dashboard";
    let selectedSection = "";
    let selectedTweaks: Tweak[] = [];

    function openSection(section: Section) {
        selectedSection = section.title;
        selectedTweaks = section.tweaks;
        currentView = "detail";
    }

    function goBack() {
        currentView = "dashboard";
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
            <h2>🪟 UI Classic</h2>
            <p class="subtitle">
                Complete StartAllBack-style Windows UI customization
            </p>
        </div>

        <!-- Quick Profile Button -->
        <div class="quick-profile">
            {#each profileTweaks as profile}
                <button
                    class="profile-btn"
                    class:enabled={profile.enabled}
                    on:click={() =>
                        profile.enabled
                            ? revertTweak(profile)
                            : applyTweak(profile)}
                >
                    {profile.name}
                    <span class="badge"
                        >{profile.enabled ? "✓ Active" : "Apply All"}</span
                    >
                </button>
            {/each}
        </div>

        <div class="dashboard-grid">
            {#each sections as section}
                <button
                    class="section-card"
                    style="--accent: {section.color}"
                    on:click={() => openSection(section)}
                >
                    <div class="card-header">
                        <span class="card-icon">{section.icon}</span>
                        <h3>{section.title}</h3>
                    </div>
                    <p class="card-desc">{section.description}</p>
                    <span class="count">{section.tweaks.length} options</span>
                </button>
            {/each}
        </div>

        <div class="info-box">
            <strong>💡 Getting Started:</strong>
            <ol>
                <li>
                    Apply <strong>UxTheme Patch</strong> first (enables custom themes)
                </li>
                <li>Restart your computer</li>
                <li>Apply any <strong>Theme</strong> (Win7, Plain8)</li>
                <li>
                    Customize <strong>Taskbar</strong>,
                    <strong>Start Menu</strong>, etc.
                </li>
            </ol>
        </div>
    {:else}
        <div class="detail-view">
            <button class="back-btn" on:click={goBack}>← Back</button>
            <h2>{selectedSection}</h2>
            <div class="tweak-list">
                {#each selectedTweaks as tweak}
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

    .quick-profile {
        display: flex;
        justify-content: center;
    }

    .profile-btn {
        background: linear-gradient(135deg, #3b82f6, #8b5cf6);
        border: none;
        color: white;
        padding: 14px 28px;
        border-radius: var(--radius);
        cursor: pointer;
        font-weight: 600;
        font-size: 15px;
        display: flex;
        align-items: center;
        gap: 12px;
        transition: all 0.2s;
    }

    .profile-btn:hover {
        transform: scale(1.02);
        box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
    }

    .profile-btn.enabled {
        background: linear-gradient(135deg, #10b981, #059669);
    }

    .badge {
        background: rgba(255, 255, 255, 0.2);
        padding: 4px 10px;
        border-radius: 12px;
        font-size: 12px;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 16px;
        align-content: start;
    }

    .section-card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 20px;
        text-align: left;
        cursor: pointer;
        transition: all 0.2s;
    }

    .section-card:hover {
        background: rgba(255, 255, 255, 0.06);
        border-color: var(--accent);
        transform: translateY(-2px);
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 10px;
        margin-bottom: 8px;
    }

    .card-icon {
        font-size: 24px;
    }

    .section-card h3 {
        margin: 0;
        font-size: 16px;
        color: var(--text-color);
    }

    .card-desc {
        margin: 0;
        font-size: 12px;
        color: var(--text-muted);
    }

    .count {
        display: inline-block;
        margin-top: 8px;
        font-size: 11px;
        color: var(--accent);
        background: rgba(59, 130, 246, 0.1);
        padding: 2px 8px;
        border-radius: 10px;
    }

    .info-box {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.3);
        border-radius: var(--radius);
        padding: 16px 20px;
        font-size: 13px;
        color: var(--text-color);
    }

    .info-box ol {
        margin: 8px 0 0 20px;
        padding: 0;
    }

    .info-box li {
        margin: 4px 0;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 16px;
        overflow: hidden;
    }

    .back-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-muted);
        padding: 8px 16px;
        border-radius: var(--radius-md);
        cursor: pointer;
        align-self: flex-start;
        transition: all 0.2s;
    }

    .back-btn:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
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
