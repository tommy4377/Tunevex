<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    import Select from "./Select.svelte";
    import {
        LayoutTemplate,
        Monitor,
        ArrowUp,
        AlignLeft,
        Layers,
    } from "lucide-svelte";
    import TweakCard from "../TweakCard.svelte";
    import { createEventDispatcher } from "svelte";

    export let tweaks: Tweak[] = [];

    const dispatch = createEventDispatcher();

    // Helper to check if a tweak is enabled
    function isEnabled(id: string): boolean {
        const tweak = tweaks.find((t) => t.id === id);
        return tweak?.enabled ?? false;
    }

    // --- Computed Values (reactive) ---

    // Style: ep_style_win10 enabled = Win10, otherwise Win11
    $: styleValue = isEnabled("ep_style_win10") ? "win10" : "win11";

    // Position: Check which position tweak is enabled
    $: posValue = isEnabled("taskbar_pos_top")
        ? "top"
        : isEnabled("taskbar_pos_left")
          ? "left"
          : isEnabled("taskbar_pos_right")
            ? "right"
            : "bottom";

    // Size: Small icons enabled?
    $: sizeValue = isEnabled("taskbar_icons_small") ? "small" : "large";

    // Alignment: Left alignment enabled?
    $: alignValue = isEnabled("taskbar_align_left") ? "left" : "center";

    // Other tweaks (not handled by dropdowns)
    $: otherTweaks = tweaks.filter(
        (t) =>
            t.id !== "ep_style_win10" &&
            !t.id.startsWith("taskbar_pos_") &&
            t.id !== "taskbar_icons_small" &&
            t.id !== "taskbar_align_left",
    );

    // --- Loading State ---
    let loading: Record<string, boolean> = {};

    function setLoading(key: string, val: boolean) {
        loading = { ...loading, [key]: val };
    }

    // Trigger parent to refresh tweaks
    function triggerRefresh() {
        dispatch("refresh");
    }

    // --- Handlers ---

    async function handleStyleChange(e: CustomEvent) {
        setLoading("style", true);
        try {
            if (e.detail.value === "win10") {
                await invoke("apply_tweak", { id: "ep_style_win10" });
            } else {
                await invoke("undo_tweak", { id: "ep_style_win10" });
            }
            triggerRefresh();
        } catch (err) {
            console.error("Style change error:", err);
        } finally {
            setLoading("style", false);
        }
    }

    async function handlePosChange(e: CustomEvent) {
        setLoading("pos", true);
        const val = e.detail.value;
        try {
            // Undo all position tweaks first
            const posTweaks = [
                "taskbar_pos_top",
                "taskbar_pos_left",
                "taskbar_pos_right",
            ];
            for (const id of posTweaks) {
                if (isEnabled(id)) {
                    await invoke("undo_tweak", { id });
                }
            }

            // Apply new position (bottom = default, no tweak needed)
            if (val !== "bottom") {
                await invoke("apply_tweak", { id: `taskbar_pos_${val}` });
            }
            triggerRefresh();
        } catch (err) {
            console.error("Position change error:", err);
        } finally {
            setLoading("pos", false);
        }
    }

    async function handleSizeChange(e: CustomEvent) {
        setLoading("size", true);
        try {
            if (e.detail.value === "small") {
                await invoke("apply_tweak", { id: "taskbar_icons_small" });
            } else {
                await invoke("undo_tweak", { id: "taskbar_icons_small" });
            }
            triggerRefresh();
        } catch (err) {
            console.error("Size change error:", err);
        } finally {
            setLoading("size", false);
        }
    }

    async function handleAlignChange(e: CustomEvent) {
        setLoading("align", true);
        try {
            if (e.detail.value === "left") {
                await invoke("apply_tweak", { id: "taskbar_align_left" });
            } else {
                await invoke("undo_tweak", { id: "taskbar_align_left" });
            }
            triggerRefresh();
        } catch (err) {
            console.error("Align change error:", err);
        } finally {
            setLoading("align", false);
        }
    }
</script>

<div class="taskbar-settings">
    <!-- Config Grid -->
    <div class="config-grid">
        <!-- Style -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box style">
                    <LayoutTemplate size={20} />
                </div>
                <div class="text-info">
                    <h3>Style</h3>
                    <p>Taskbar engine</p>
                </div>
                <div class="action-area">
                    <Select
                        value={styleValue}
                        options={[
                            { value: "win11", label: "Windows 11" },
                            { value: "win10", label: "Windows 10" },
                        ]}
                        loading={loading["style"] ?? false}
                        on:change={handleStyleChange}
                    />
                </div>
            </div>
        </div>

        <!-- Position -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box pos">
                    <ArrowUp size={20} />
                </div>
                <div class="text-info">
                    <h3>Position</h3>
                    <p>Screen edge</p>
                </div>
                <div class="action-area">
                    <Select
                        value={posValue}
                        options={[
                            { value: "bottom", label: "Bottom" },
                            { value: "top", label: "Top" },
                            { value: "left", label: "Left" },
                            { value: "right", label: "Right" },
                        ]}
                        loading={loading["pos"] ?? false}
                        on:change={handlePosChange}
                    />
                </div>
            </div>
        </div>

        <!-- Size -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box size">
                    <Monitor size={20} />
                </div>
                <div class="text-info">
                    <h3>Icon Size</h3>
                    <p>Large or small</p>
                </div>
                <div class="action-area">
                    <Select
                        value={sizeValue}
                        options={[
                            { value: "large", label: "Large" },
                            { value: "small", label: "Small" },
                        ]}
                        loading={loading["size"] ?? false}
                        on:change={handleSizeChange}
                    />
                </div>
            </div>
        </div>

        <!-- Alignment -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box align">
                    <AlignLeft size={20} />
                </div>
                <div class="text-info">
                    <h3>Alignment</h3>
                    <p>Icon position</p>
                </div>
                <div class="action-area">
                    <Select
                        value={alignValue}
                        options={[
                            { value: "center", label: "Center" },
                            { value: "left", label: "Left" },
                        ]}
                        loading={loading["align"] ?? false}
                        on:change={handleAlignChange}
                    />
                </div>
            </div>
        </div>
    </div>

    <!-- Other Tweaks (as toggle cards) -->
    {#if otherTweaks.length > 0}
        <h3 class="section-title">Behavior</h3>
        <div class="toggles-grid">
            {#each otherTweaks as tweak (tweak.id)}
                <TweakCard {tweak} on:toggle />
            {/each}
        </div>
    {/if}
</div>

<style>
    .taskbar-settings {
        display: flex;
        flex-direction: column;
        gap: 24px;
        padding-bottom: 24px;
    }

    .config-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
        gap: 16px;
    }

    .config-card {
        background: var(--layer-card, rgba(255, 255, 255, 0.03));
        border: 1px solid var(--border-color, rgba(255, 255, 255, 0.08));
        border-radius: 12px;
        padding: 16px;
        transition: all 0.2s;
    }

    .config-card:hover {
        background: rgba(255, 255, 255, 0.05);
        border-color: rgba(255, 255, 255, 0.15);
    }

    .card-content {
        display: flex;
        align-items: center;
        gap: 12px;
        justify-content: space-between;
        width: 100%;
    }

    .icon-box {
        width: 40px;
        height: 40px;
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .icon-box.style {
        background: rgba(139, 92, 246, 0.15);
        color: #a78bfa;
    }

    .icon-box.pos {
        background: rgba(59, 130, 246, 0.15);
        color: #60a5fa;
    }

    .icon-box.size {
        background: rgba(16, 185, 129, 0.15);
        color: #34d399;
    }

    .icon-box.align {
        background: rgba(251, 146, 60, 0.15);
        color: #fb923c;
    }

    .text-info {
        flex: 1;
        min-width: 0;
    }

    .text-info h3 {
        margin: 0 0 2px 0;
        font-size: 14px;
        font-weight: 600;
        color: var(--text-color);
    }

    .text-info p {
        margin: 0;
        font-size: 12px;
        color: var(--text-muted);
        white-space: nowrap;
    }

    .action-area {
        flex-shrink: 0;
    }

    .section-title {
        font-size: 16px;
        font-weight: 600;
        margin: 0;
        padding-left: 4px;
        color: var(--text-color);
    }

    .toggles-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 16px;
    }
</style>
