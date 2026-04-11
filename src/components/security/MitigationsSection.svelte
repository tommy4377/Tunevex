<script lang="ts">
    import { Rocket } from "lucide-svelte";
    import { SectionHeader, InfoBanner } from "../ui";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let tweaks: Tweak[] = [];
</script>

<div class="section-container">
    <SectionHeader
        icon={Rocket}
        title="Exploit Mitigations"
        description="Control CPU vulnerability mitigations (Spectre/Meltdown), DEP, VBS, HVCI, and process protections."
    />

    <InfoBanner
        variant="danger"
        message="DANGER: These settings significantly reduce system security. Only disable for isolated gaming PCs."
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
