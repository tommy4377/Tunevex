<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { DnsBenchmarkResult, Tweak } from "$lib/types";
    import { fade } from "svelte/transition";
    import { Check, Trophy } from "lucide-svelte";
    import TweakList from "../TweakList.svelte";

    export let tweaks: Tweak[] = [];

    let benchmarkResults: DnsBenchmarkResult[] = [];
    let loading = false;
    let error: string | null = null;
    let selectedProvider: string | null = null;

    async function runBenchmark() {
        loading = true;
        error = null;
        benchmarkResults = [];
        try {
            benchmarkResults = await invoke("benchmark_dns");
            // Auto-select fastest
            if (benchmarkResults.length > 0) {
                // Already sorted by backend, so first is fastest
                selectedProvider = benchmarkResults[0].provider;
            }
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    }

    async function applySafeTweaks() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        tweaks = tweaks;
    }

    async function applyDns(provider: string) {
        const item = benchmarkResults.find((r) => r.provider === provider);
        if (!item) return;

        // Visual selection
        selectedProvider = provider;

        try {
            await invoke("apply_dns_server", {
                primary: item.primary,
                secondary: item.secondary,
            });
            // You might want to show a success toast here
            console.log(`Applied DNS: ${provider}`);
        } catch (e) {
            console.error(e);
            error = String(e);
        }
    }
</script>

<div class="dns-section">
    <div class="header">
        <h2>DNS Benchmark & Settings</h2>
        <p>
            Test latency to public DNS providers and configure DNS caching
            strategies.
        </p>
        <button
            class="benchmark-btn"
            on:click={runBenchmark}
            disabled={loading}
        >
            {loading ? "Running Benchmark..." : "Start Speed Test"}
        </button>
    </div>

    {#if error}
        <div class="error">{error}</div>
    {/if}

    <div class="results-list">
        {#each benchmarkResults as res, i}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
                class="result-row"
                class:fastest={i === 0}
                class:selected={selectedProvider === res.provider}
                role="button"
                tabindex="0"
                on:click={() => applyDns(res.provider)}
                on:keydown={(e) => e.key === "Enter" && applyDns(res.provider)}
                in:fade
            >
                <div class="info">
                    <div class="name-row">
                        <span class="provider-name">{res.provider}</span>
                        <span class="ips">{res.primary}, {res.secondary}</span>
                    </div>
                    <span class="description">{res.description}</span>
                </div>

                <div class="metrics">
                    <div class="bar-container">
                        <div
                            class="bar"
                            style="width: {Math.max(
                                5,
                                100 - (res.avg_latency_ms || 999) / 2,
                            )}%"
                            class:good={(res.avg_latency_ms || 999) < 50}
                            class:ok={(res.avg_latency_ms || 999) >= 50 &&
                                (res.avg_latency_ms || 999) < 150}
                        ></div>
                    </div>
                    <span class="latency">
                        {res.avg_latency_ms
                            ? `${res.avg_latency_ms} ms`
                            : "Timeout"}
                    </span>
                </div>

                {#if selectedProvider === res.provider}
                    <div class="selected-badge">✓ Running</div>
                {/if}

                {#if i === 0}
                    <div class="badge"><Trophy size={12} /> Fastest</div>
                {/if}
            </div>
        {/each}

        {#if benchmarkResults.length === 0 && !loading}
            <div class="placeholder">
                Click "Start Speed Test" to analyze DNS performance.
            </div>
        {/if}

        <!-- Added TweakList for DNS Tweaks like TTL -->
        {#if tweaks.length > 0}
            <div class="dns-tweaks-area">
                <div class="section-header">
                    <div class="header-text">
                        <h3>Advanced DNS Settings</h3>
                        <p>Additional DNS configuration.</p>
                    </div>
                    <button
                        class="optimize-btn safe"
                        on:click={() => applySafeTweaks()}
                    >
                        <Check size={14} />
                        Apply Safe Tweaks
                    </button>
                    <!-- Script needs to go in main script block -->
                </div>
                <TweakList {tweaks} showHeader={false} />
            </div>
        {/if}
    </div>
</div>

<style>
    .dns-tweaks-area {
        margin-top: 24px;
        border-top: var(--border-glass);
        padding-top: 24px;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 16px;
    }

    .header-text h3 {
        margin: 0 0 4px 0;
        font-size: 16px;
        color: var(--text-primary);
        font-weight: 600;
    }
    .header-text p {
        margin: 0;
        font-size: 13px;
        color: var(--text-secondary);
    }

    .optimize-btn.safe {
        background: var(--btn-safe-bg);
        color: var(--btn-safe-color);
        border: 1px solid var(--btn-safe-border);
        padding: 6px 12px;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        font-size: 13px;
    }
    .optimize-btn.safe:hover {
        background: var(--btn-safe-hover-bg);
    }
    .dns-section {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden; /* Fix: Container handles overflow */
        height: 100%;
    }

    .header {
        margin-bottom: 16px;
        border-bottom: var(--border-glass);
        padding-bottom: 16px;
        flex-shrink: 0;
    }

    h2 {
        font-size: 20px;
        margin-bottom: 8px;
    }

    p {
        color: var(--text-secondary);
        font-size: 14px;
        margin-bottom: 16px;
    }

    .benchmark-btn {
        background: var(--accent);
        color: white;
        border: none;
        padding: 10px 20px;
        border-radius: 6px;
        font-weight: 600;
        cursor: pointer;
        transition: background 0.2s;
    }

    .benchmark-btn:hover {
        background: #4eb0fa; /* Brighter accent */
    }

    .benchmark-btn:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }

    .results-list {
        display: flex;
        flex-direction: column;
        gap: 8px;
        overflow-y: auto; /* Fix: Make results scrollable */
        flex: 1;
        padding: 12px; /* Fix: Add padding to prevent badge clipping (top/left -8px) */
        padding-bottom: 24px; /* Ensure last item is visible */
    }

    .result-row {
        background: rgba(255, 255, 255, 0.03);
        border: var(--border-glass);
        padding: 12px 16px;
        border-radius: 8px;
        display: flex;
        align-items: center;
        gap: 16px;
        cursor: pointer;
        transition: all 0.2s;
        position: relative;
    }

    .result-row:hover {
        background: rgba(255, 255, 255, 0.06);
    }

    .result-row.selected {
        border-color: var(--accent);
        background: rgba(59, 130, 246, 0.1);
    }

    .info {
        flex: 2;
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .name-row {
        display: flex;
        align-items: baseline;
        gap: 8px;
    }

    .provider-name {
        font-weight: 600;
        font-size: 15px;
    }

    .ips {
        font-size: 12px;
        color: var(--text-secondary);
        font-family: monospace;
    }

    .description {
        font-size: 12px;
        color: var(--text-secondary);
        font-style: italic;
    }

    .metrics {
        flex: 1;
        display: flex;
        align-items: center;
        gap: 12px;
        min-width: 150px;
    }

    .bar-container {
        flex: 1;
        background: rgba(255, 255, 255, 0.1);
        height: 6px;
        border-radius: 3px;
        overflow: hidden;
    }

    .bar {
        height: 100%;
        background: #ef4444; /* Default Bad */
        border-radius: 3px;
        transition: width 0.5s ease-out;
    }

    .bar.good {
        background: #22c55e;
    }
    .bar.ok {
        background: #eab308;
    }

    .latency {
        width: 50px;
        text-align: right;
        font-feature-settings: "tnum";
        font-size: 13px;
        font-weight: 500;
    }

    .badge {
        position: absolute;
        top: -8px;
        left: -8px;
        background: #eab308;
        color: black;
        font-size: 10px;
        font-weight: bold;
        padding: 2px 8px;
        border-radius: 10px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        z-index: 2;
    }

    .selected-badge {
        font-size: 12px;
        color: var(--accent);
        font-weight: 600;
        margin-left: 8px;
    }

    .placeholder {
        text-align: center;
        padding: 40px;
        color: var(--text-secondary);
        font-style: italic;
        border: 2px dashed var(--border-glass);
        border-radius: 12px;
    }
</style>
