<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Sparkles, ArrowRight, AlertTriangle, CheckCircle, XCircle } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  interface RecommendedTweak {
    id: string;
    priority: string;
    reason: string;
  }

  interface TweakConflict {
    tweak_ids: string[];
    issue: string;
  }

  interface AnalysisResult {
    add: RecommendedTweak[];
    remove: RecommendedTweak[];
    conflicts: TweakConflict[];
    system_summary: string;
  }

  let loading = false;
  let analysis: AnalysisResult | null = null;
  let error = "";

  async function runAnalysis() {
    loading = true;
    error = "";
    try {
      analysis = await invoke<AnalysisResult>("ai_analyze");
    } catch (e) {
      error = e as string;
    } finally {
      loading = false;
    }
  }

  function getPriorityColor(priority: string): string {
    switch (priority) {
      case "high": return "var(--danger)";
      case "medium": return "#f59e0b";
      case "low": return "var(--accent-color)";
      default: return "var(--text-muted)";
    }
  }

  async function applyTweak(id: string) {
    try {
      await invoke("apply_tweak", { id });
    } catch (e) {
      console.error("Failed to apply tweak:", e);
    }
  }

  async function removeTweak(id: string) {
    try {
      await invoke("undo_tweak", { id });
    } catch (e) {
      console.error("Failed to remove tweak:", e);
    }
  }
</script>

<div class="scan-panel">
  <div class="scan-header">
    <h2><Sparkles size={18} /> System Analysis</h2>
    <p>AI-powered recommendations based on your system configuration</p>
  </div>

  <Button onclick={runAnalysis} disabled={loading} variant="accent">
    {#if loading}
      <div class="spinner"></div> Analyzing...
    {:else}
      <Sparkles size={14} /> Run Analysis
    {/if}
  </Button>

  {#if error}
    <div class="error-box">
      <AlertTriangle size={16} /> {error}
    </div>
  {/if}

  {#if analysis}
    <div class="results">
      <div class="summary">
        <h3>System Summary</h3>
        <p>{analysis.system_summary}</p>
      </div>

      {#if analysis.add.length > 0}
        <div class="section">
          <h3><Sparkles size={16} /> Recommended to Add</h3>
          {#each analysis.add as item}
            <div class="rec-item">
              <div class="rec-info">
                <span class="rec-id">{item.id}</span>
                <span class="rec-reason">{item.reason}</span>
              </div>
              <div class="rec-actions">
                <span class="priority" style="color: {getPriorityColor(item.priority)}">{item.priority}</span>
                <Button variant="ghost" onclick={() => applyTweak(item.id)}>
                  <ArrowRight size={12} /> Apply
                </Button>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      {#if analysis.remove.length > 0}
        <div class="section">
          <h3><XCircle size={16} /> Recommended to Remove</h3>
          {#each analysis.remove as item}
            <div class="rec-item">
              <div class="rec-info">
                <span class="rec-id">{item.id}</span>
                <span class="rec-reason">{item.reason}</span>
              </div>
              <div class="rec-actions">
                <span class="priority" style="color: {getPriorityColor(item.priority)}">{item.priority}</span>
                <Button variant="danger" onclick={() => removeTweak(item.id)}>
                  Remove
                </Button>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      {#if analysis.conflicts.length > 0}
        <div class="section conflicts">
          <h3><AlertTriangle size={16} /> Conflicts Detected</h3>
          {#each analysis.conflicts as conflict}
            <div class="conflict-item">
              <span class="conflict-ids">{conflict.tweak_ids.join(" + ")}</span>
              <span class="conflict-issue">{conflict.issue}</span>
            </div>
          {/each}
        </div>
      {/if}

      {#if analysis.add.length === 0 && analysis.remove.length === 0 && analysis.conflicts.length === 0}
        <div class="no-issues">
          <CheckCircle size={32} />
          <p>No optimizations needed - your system looks great!</p>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .scan-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .scan-header h2 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 18px;
    margin: 0;
    color: var(--text-primary);
  }
  .scan-header p {
    color: var(--text-muted);
    font-size: 14px;
    margin: 4px 0 0 0;
  }
  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
  .error-box {
    background: rgba(239,68,68,0.1);
    border: 1px solid rgba(239,68,68,0.3);
    border-radius: var(--radius-sm);
    padding: 12px;
    color: #ef4444;
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .results {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  .summary {
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
    padding: 16px;
  }
  .summary h3 {
    font-size: 14px;
    margin: 0 0 8px 0;
    color: var(--text-muted);
  }
  .summary p {
    margin: 0;
    color: var(--text-primary);
    font-size: 14px;
  }
  .section {
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
    padding: 16px;
  }
  .section h3 {
    font-size: 14px;
    margin: 0 0 12px 0;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .rec-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    background: rgba(255,255,255,0.03);
    border-radius: var(--radius-sm);
    margin-bottom: 8px;
  }
  .rec-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .rec-id {
    font-weight: 600;
    font-size: 13px;
    color: var(--accent-color);
  }
  .rec-reason {
    font-size: 12px;
    color: var(--text-muted);
  }
  .rec-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .priority {
    font-size: 11px;
    text-transform: uppercase;
    font-weight: 600;
  }
  .conflicts .conflict-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px;
    background: rgba(245,158,11,0.1);
    border-radius: var(--radius-sm);
    margin-bottom: 8px;
  }
  .conflict-ids {
    font-weight: 600;
    color: #f59e0b;
    font-size: 13px;
  }
  .conflict-issue {
    font-size: 12px;
    color: var(--text-muted);
  }
  .no-issues {
    text-align: center;
    padding: 40px;
    color: #22c55e;
  }
  .no-issues p {
    margin: 12px 0 0 0;
    color: var(--text-muted);
  }
</style>