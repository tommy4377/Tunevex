<script lang="ts">
    import { fade } from "svelte/transition";
    import DnsSection from "./DnsSection.svelte";
    import TcpSection from "./TcpSection.svelte";
    import AdapterSection from "./AdapterSection.svelte";
    import SecuritySection from "./SecuritySection.svelte";
    import MsiSection from "./MsiSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Sub-routes: 'dashboard', 'dns', 'tcp', 'adapter', 'security', 'msi'
    let currentView:
        | "dashboard"
        | "dns"
        | "tcp"
        | "adapter"
        | "security"
        | "msi" = "dashboard";

    // Precise Filters
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
            !t.id.includes("net_dns_benchmark"), // Remove legacy benchmark tweak from list
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
            !msiTweaks.includes(t) && // Ensure no overlap
            true, // Catch-all (includes soft_reset)
    );
</script>

<div class="network-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- DNS Card -->
            <div
                class="card dns-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "dns")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "dns")}
            >
                <div class="card-icon">🌐</div>
                <h3>DNS Optimizer</h3>
                <p>Benchmark and maximize DNS speed.</p>
                <div class="status">{dnsTweaks.length} tweaks</div>
            </div>

            <!-- TCP Card -->
            <div
                class="card tcp-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "tcp")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "tcp")}
            >
                <div class="card-icon">🚀</div>
                <h3>TCP/IP Stack</h3>
                <p>Optimize packet handling and latency.</p>
                <div class="status">{tcpTweaks.length} tweaks</div>
            </div>

            <!-- Adapter Card -->
            <div
                class="card adapter-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "adapter")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "adapter")}
            >
                <div class="card-icon">🔌</div>
                <h3>Adapter Settings</h3>
                <p>Fine-tune network card offloading.</p>
                <div class="status">{adapterTweaks.length} tweaks</div>
            </div>

            <!-- MSI Card -->
            <div
                class="card msi-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "msi")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "msi")}
            >
                <div class="card-icon">⚡</div>
                <h3>MSI Mode</h3>
                <p>Lower latency with Message Signaled Interrupts.</p>
                <div class="status">{msiTweaks.length} tweaks</div>
            </div>

            <!-- Security Card -->
            <div
                class="card security-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "security")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "security")}
            >
                <div class="card-icon">🛡️</div>
                <h3>Network Security</h3>
                <p>Harden protocols and block leaks.</p>
                <div class="status">{securityTweaks.length} tweaks</div>
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
        /* Padding removed here to allow full-width/height scroll masking */
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 20px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        /* Moved padding here so content scrolls 'into' the padding, not clipped by parent */
        padding: 24px;
        padding-top: 4px; /* Small top padding for hover clearance */
        min-height: 0;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 24px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        position: relative; /* Base for z-index */
        z-index: 1; /* Default layer */
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
        z-index: 10; /* Fix: Ensure card stays on top when scaled */
        position: relative; /* Often needed for z-index to work */
    }

    .card-icon {
        font-size: 32px;
        margin-bottom: 16px;
    }

    h3 {
        margin: 0 0 8px 0;
        font-size: 18px;
        font-weight: 600;
    }

    p {
        margin: 0 0 24px 0;
        color: var(--text-muted);
        font-size: 14px;
        line-height: 1.5;
        flex-grow: 1;
    }

    .status {
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 6px 12px;
        border-radius: 20px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px; /* Fix: Global padding for all detail views */
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
