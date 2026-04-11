<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Search } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  interface Cause { tweak_id: string; confidence: "high"|"medium"|"low"; explanation: string; }
  interface Diagnosis { likely_causes: Cause[]; suggested_fix: string; safe_to_revert: string[]; }

  let problemText = "";
  let diagnosis: Diagnosis | null = null;
  let loading = false;
  let error = "";

  const confidenceColor = { high: "#f87171", medium: "#fbbf24", low: "#34d399" };

  async function diagnose() {
    if (!problemText.trim()) return;
    loading = true; error = "";
    try {
      diagnosis = await invoke<Diagnosis>("ai_diagnose", { problem: problemText });
    } catch (e) { error = e as string; }
    finally { loading = false; }
  }

  async function revertCauses() {
    if (!diagnosis) return;
    for (const id of diagnosis.safe_to_revert)
      await invoke("undo_tweak", { id });
  }
</script>

<div class="diagnose-panel">
  <textarea
    bind:value={problemText}
    placeholder="Describe your problem in detail:&#10;• Game stutters every 2 seconds&#10;• WiFi disconnects after sleep&#10;• High DPC latency in LatencyMon&#10;• Audio crackling during load"
    rows={5}
  />
  <Button onclick={diagnose} disabled={loading || !problemText.trim()}>
    <Search size={13} /> {loading ? "Analyzing…" : "Find Cause"}
  </Button>

  {#if error}<p class="error">{error}</p>{/if}

  {#if diagnosis}
    <h3>Likely causes</h3>
    {#each diagnosis.likely_causes as c}
      <div class="cause-card" style="border-color: {confidenceColor[c.confidence]}33">
        <div class="cause-header">
          <code>{c.tweak_id}</code>
          <span class="conf" style="color: {confidenceColor[c.confidence]}">{c.confidence}</span>
        </div>
        <p>{c.explanation}</p>
      </div>
    {/each}

    <div class="fix-box">
      <h3>Suggested fix</h3>
      <pre>{diagnosis.suggested_fix}</pre>
    </div>

    {#if diagnosis.safe_to_revert.length}
      <Button onclick={revertCauses}>
        Revert Likely Causes ({diagnosis.safe_to_revert.length})
      </Button>
    {/if}
  {/if}
</div>

<style>
  .diagnose-panel { display: flex; flex-direction: column; gap: 16px; }
  textarea { background: rgba(255,255,255,0.04); border: var(--border-glass);
    border-radius: var(--radius-md); padding: 12px; color: var(--text-color);
    font-size: 13px; resize: vertical; outline: none; font-family: inherit; }
  textarea:focus { border-color: var(--accent-color); }
  h3 { font-size: 13px; font-weight: 600; color: var(--text-muted); margin: 0; }
  .cause-card { padding: 12px 16px; background: var(--layer-card);
    border: 1px solid; border-radius: var(--radius-md); }
  .cause-header { display: flex; justify-content: space-between; margin-bottom: 6px; }
  code { font-size: 12px; color: var(--accent-color); }
  .conf { font-size: 11px; font-weight: 600; text-transform: uppercase; }
  .cause-card p { margin: 0; font-size: 13px; color: var(--text-muted); }
  .fix-box { background: var(--layer-card); border: var(--border-glass);
    border-radius: var(--radius-md); padding: 16px; }
  pre { margin: 8px 0 0; font-size: 12px; color: var(--text-secondary);
    white-space: pre-wrap; line-height: 1.6; }
  .error { color: var(--danger); font-size: 13px; }
</style>