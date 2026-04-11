<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { AlertTriangle, CheckCircle, RefreshCw, Undo2 } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  interface DiagnosisCause {
    tweak_id: string;
    confidence: string;
    explanation: string;
  }

  interface DiagnosisResult {
    likely_causes: DiagnosisCause[];
    suggested_fix: string;
    safe_to_revert: string[];
  }

  let problem = "";
  let loading = false;
  let diagnosis: DiagnosisResult | null = null;
  let error = "";

  async function runDiagnosis() {
    if (!problem.trim() || loading) return;
    
    loading = true;
    error = "";
    diagnosis = null;
    
    try {
      diagnosis = await invoke<DiagnosisResult>("ai_diagnose", {
        problem: problem
      });
    } catch (e) {
      error = e as string;
    } finally {
      loading = false;
    }
  }

  function getConfidenceColor(confidence: string): string {
    switch (confidence) {
      case "high": return "#ef4444";
      case "medium": return "#f59e0b";
      case "low": return "var(--accent-color)";
      default: return "var(--text-muted)";
    }
  }

  async function revertTweak(id: string) {
    try {
      await invoke("undo_tweak", { id });
    } catch (e) {
      console.error("Failed to revert tweak:", e);
    }
  }
</script>

<div class="diagnose-panel">
  <div class="diagnose-header">
    <h2><AlertTriangle size={18} /> Problem Diagnosis</h2>
    <p>Describe an issue you're experiencing, and AI will identify which tweaks may be causing it</p>
  </div>

  <div class="input-section">
    <textarea
      bind:value={problem}
      placeholder="e.g., My computer crashes when playing games, or WiFi disconnects randomly..."
      disabled={loading}
      rows="3"
    ></textarea>
    <Button onclick={runDiagnosis} disabled={loading || !problem.trim()} variant="accent">
      {#if loading}
        <RefreshCw size={14} class="spin" /> Diagnose...
      {:else}
        <AlertTriangle size={14} /> Diagnose Problem
      {/if}
    </Button>
  </div>

  {#if error}
    <div class="error-box">{error}</div>
  {/if}

  {#if diagnosis}
    <div class="results">
      {#if diagnosis.likely_causes.length > 0}
        <div class="section">
          <h3> Likely Causes</h3>
          {#each diagnosis.likely_causes as cause}
            <div class="cause-item">
              <div class="cause-header">
                <span class="cause-id">{cause.tweak_id}</span>
                <span class="confidence" style="color: {getConfidenceColor(cause.confidence)}">
                  {cause.confidence} confidence
                </span>
              </div>
              <p class="cause-explanation">{cause.explanation}</p>
              {#if diagnosis.safe_to_revert.includes(cause.tweak_id)}
                <Button variant="ghost" onclick={() => revertTweak(cause.tweak_id)}>
                  <Undo2 size={12} /> Revert This Tweak
                </Button>
              {/if}
            </div>
          {/each}
        </div>
      {/if}

      {#if diagnosis.suggested_fix}
        <div class="section fix-section">
          <h3> Suggested Fix</h3>
          <div class="fix-content">{diagnosis.suggested_fix}</div>
        </div>
      {/if}

      {#if diagnosis.safe_to_revert.length > 0}
        <div class="section safe-revert">
          <h3><CheckCircle size={16} /> Safe to Revert</h3>
          <p>These tweaks can be safely disabled without causing further issues:</p>
          <div class="revert-list">
            {#each diagnosis.safe_to_revert as id}
              <span class="revert-tag">{id}</span>
            {/each}
          </div>
        </div>
      {/if}

      {#if diagnosis.likely_causes.length === 0 && diagnosis.suggested_fix === ""}
        <div class="no-issues">
          <CheckCircle size={32} />
          <p>No likely causes found. Your applied tweaks may not be related to this issue.</p>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .diagnose-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .diagnose-header h2 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 18px;
    margin: 0;
    color: var(--text-primary);
  }
  .diagnose-header p {
    color: var(--text-muted);
    font-size: 14px;
    margin: 4px 0 0 0;
  }
  .input-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .input-section textarea {
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
    padding: 16px;
    color: var(--text-primary);
    font-size: 14px;
    resize: vertical;
    min-height: 80px;
    outline: none;
  }
  .input-section textarea:focus { border-color: var(--accent-color); }
  :global(.spin) {
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
  }
  .results {
    display: flex;
    flex-direction: column;
    gap: 16px;
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
  .cause-item {
    padding: 12px;
    background: rgba(255,255,255,0.03);
    border-radius: var(--radius-sm);
    margin-bottom: 10px;
  }
  .cause-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }
  .cause-id {
    font-weight: 600;
    color: var(--accent-color);
    font-size: 13px;
  }
  .confidence {
    font-size: 11px;
    text-transform: uppercase;
    font-weight: 600;
  }
  .cause-explanation {
    margin: 0 0 10px 0;
    color: var(--text-muted);
    font-size: 13px;
    line-height: 1.5;
  }
  .fix-section .fix-content {
    white-space: pre-wrap;
    color: var(--text-primary);
    font-size: 14px;
    line-height: 1.6;
  }
  .safe-revert p {
    margin: 0 0 10px 0;
    color: var(--text-muted);
    font-size: 13px;
  }
  .revert-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .revert-tag {
    background: rgba(34,197,94,0.1);
    border: 1px solid rgba(34,197,94,0.3);
    border-radius: 4px;
    padding: 4px 10px;
    font-size: 12px;
    color: #22c55e;
  }
  .no-issues {
    text-align: center;
    padding: 40px;
    color: var(--text-muted);
  }
  .no-issues p { margin: 12px 0 0 0; }
</style>