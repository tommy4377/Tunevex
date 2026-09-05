<script lang="ts">
    import { Usb } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { SectionHeader, InfoBanner } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    $: usbTweaks = allTweaks.filter(
        (t) =>
            t.id.includes("usb") ||
            t.id.includes("usbstor") ||
            t.id.includes("xhci"),
    );

    async function applySafeTweaks(tweaks: Tweak[]) {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    tweak.enabled = await invoke<boolean | null>("apply_tweak", { id: tweak.id });
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        allTweaks = allTweaks;
    }
</script>

<div class="section-container">
    <SectionHeader
        icon={Usb}
        title="USB & Devices"
        description="Optimize USB controllers and manage polling behavior."
        actionLabel="Apply Safe Tweaks"
        onAction={() => applySafeTweaks(usbTweaks)}
    />

    <InfoBanner variant="info" message="">
        Enabling <strong>MSI Mode</strong> for USB Controllers can significantly
        reduce input variance and latency for connected devices.
    </InfoBanner>

    <div class="tweaks-wrapper">
        <TweakList tweaks={usbTweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .tweaks-wrapper {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        padding-bottom: 24px;
    }
</style>
