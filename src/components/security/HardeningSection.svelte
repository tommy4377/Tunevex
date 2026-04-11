<script lang="ts">
    import { Lock } from "lucide-svelte";
    import { SectionHeader } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";

    export let tweaks: Tweak[] = [];

    async function hardenAll() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
        }
        tweaks = tweaks; // Trigger reactivity
    }
</script>

<div class="section-container">
    <SectionHeader
        icon={Lock}
        title="Security Hardening"
        description="Network hardening: disable SMBv1, LLMNR, NetBIOS, block SAM enumeration, and more."
        actionLabel="Apply Safe Hardening"
        onAction={hardenAll}
    />
    <div class="tweaks-wrapper">
        <TweakList {tweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    .tweaks-wrapper {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        display: flex;
        flex-direction: column;
        padding-bottom: 24px;
    }
</style>
