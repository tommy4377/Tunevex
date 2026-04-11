<script lang="ts">
    import { fade } from "svelte/transition";
    import { Globe, Rocket, Plug, Zap, Shield } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import DnsSection from "./DnsSection.svelte";
    import TcpSection from "./TcpSection.svelte";
    import AdapterSection from "./AdapterSection.svelte";
    import SecuritySection from "./SecuritySection.svelte";
    import MsiSection from "./MsiSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView:
        | "dashboard"
        | "dns"
        | "tcp"
        | "adapter"
        | "security"
        | "msi" = "dashboard";

    // Filters
    $: securityTweaks = allTweaks.filter(
        (t) =>
            t.category === "Network" &&
            (t.id.includes("llmnr") ||
                t.id.includes("anonymous") ||
                t.id.includes("smb") ||
                t.id.includes("netbios") ||
                t.id.includes("security")),
    );

    $: dnsTweaks = allTweaks.filter(
        (t) =>
            t.category === "Network" &&
            t.id.includes("dns") &&
            !t.id.includes("net_dns_google") &&
            !t.id.includes("net_dns_cloudflare") &&
            !t.id.includes("net_dns_quad9") &&
            !t.id.includes("net_dns_benchmark"),
    );

    $: msiTweaks = allTweaks.filter(
        (t) => t.category === "Network" && t.id.includes("msi"),
    );

    $: tcpTweaks = allTweaks.filter(
        (t) =>
            t.category === "Network" &&
            !t.id.includes("dns") &&
            !securityTweaks.includes(t) &&
            !msiTweaks.includes(t) &&
            (t.id.includes("tcp") ||
                t.id.includes("ttl") ||
                t.id.includes("sack") ||
                t.id.includes("pmtu") ||
                t.id.includes("user_port") ||
                t.id.includes("network_throttling") ||
                t.id.includes("system_responsiveness")),
    );

    $: adapterTweaks = allTweaks.filter(
        (t) =>
            t.category === "Network" &&
            !t.id.includes("dns") &&
            !securityTweaks.includes(t) &&
            !tcpTweaks.includes(t) &&
            !msiTweaks.includes(t),
    );
</script>

<div class="network-container">
    {#if currentView === "dashboard"}
        <CardGrid columns="repeat(auto-fill, minmax(200px, 1fr))">
            <Card
                icon={Globe}
                title="DNS Optimizer"
                description="Benchmark and maximize DNS speed."
                status="{dnsTweaks.length} tweaks"
                onclick={() => (currentView = "dns")}
            />
            <Card
                icon={Rocket}
                title="TCP/IP Stack"
                description="Optimize packet handling and latency."
                status="{tcpTweaks.length} tweaks"
                onclick={() => (currentView = "tcp")}
            />
            <Card
                icon={Plug}
                title="Adapter Settings"
                description="Fine-tune network card offloading."
                status="{adapterTweaks.length} tweaks"
                onclick={() => (currentView = "adapter")}
            />
            <Card
                icon={Zap}
                title="MSI Mode"
                description="Lower latency with Message Signaled Interrupts."
                status="{msiTweaks.length} tweaks"
                onclick={() => (currentView = "msi")}
            />
            <Card
                icon={Shield}
                title="Network Security"
                description="Harden protocols and block leaks."
                status="{securityTweaks.length} tweaks"
                onclick={() => (currentView = "security")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            {#if currentView === "dns"}
                <DnsSection tweaks={dnsTweaks} />
            {:else if currentView === "tcp"}
                <TcpSection tweaks={tcpTweaks} />
            {:else if currentView === "adapter"}
                <AdapterSection tweaks={adapterTweaks} />
            {:else if currentView === "security"}
                <SecuritySection tweaks={securityTweaks} />
            {:else if currentView === "msi"}
                <MsiSection tweaks={msiTweaks} />
            {/if}
        </div>
    {/if}
</div>

<style>
    .network-container {
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
</style>
