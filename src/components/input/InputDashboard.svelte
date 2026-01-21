<script lang="ts">
    import { fade } from "svelte/transition";
    import { Mouse, Keyboard, Usb } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import MouseDashboard from "./MouseDashboard.svelte";
    import KeyboardDashboard from "./KeyboardDashboard.svelte";
    import UsbDashboard from "./UsbDashboard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "mouse" | "keyboard" | "usb" = "dashboard";

    // Counts
    $: mouseCount = allTweaks.filter(
        (t) => t.category === "MouseInput" && t.id.includes("mouse"),
    ).length;

    $: keyboardCount = allTweaks.filter(
        (t) =>
            (t.category === "Input" || t.category === "MouseInput") &&
            (t.id.includes("keyboard") || t.id.includes("filter")),
    ).length;

    $: usbCount = allTweaks.filter((t) => t.id.includes("usb")).length;
</script>

<div class="input-container">
    {#if currentView === "dashboard"}
        <CardGrid>
            <Card
                icon={Mouse}
                title="Mouse Optimization"
                description="Reduce input lag and adjust acceleration curves."
                status="{mouseCount} tweaks"
                onclick={() => (currentView = "mouse")}
            />
            <Card
                icon={Keyboard}
                title="Keyboard Response"
                description="Data queue sizes and repeat rates."
                status="{keyboardCount} tweaks"
                onclick={() => (currentView = "keyboard")}
            />
            <Card
                icon={Usb}
                title="USB & Devices"
                description="MSI Mode for USB controllers and pollution reduction."
                status="{usbCount} tweaks"
                onclick={() => (currentView = "usb")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            <div class="section-content">
                {#if currentView === "mouse"}
                    <MouseDashboard {allTweaks} />
                {:else if currentView === "keyboard"}
                    <KeyboardDashboard {allTweaks} />
                {:else if currentView === "usb"}
                    <UsbDashboard {allTweaks} />
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .input-container {
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

    .section-content {
        flex: 1;
        overflow: hidden;
    }
</style>
