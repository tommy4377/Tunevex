<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Sparkles, MessageSquare, AlertTriangle, Key, Trash2 } from "lucide-svelte";
  import Button from "../ui/Button.svelte";
  import ScanPanel from "./ScanPanel.svelte";
  import ChatPanel from "./ChatPanel.svelte";
  import DiagnosePanel from "./DiagnosePanel.svelte";

  type Tab = "analyze" | "chat" | "diagnose";
  let tab: Tab = "analyze";
  let hasKey = false;
  let keyInput = "";
  let savingKey = false;
  let keyError = "";

  onMount(async () => {
    hasKey = await invoke<boolean>("get_gemini_key_status");
  });

  async function saveKey() {
    savingKey = true;
    keyError = "";
    try {
      await invoke("save_gemini_key", { key: keyInput });
      hasKey = true;
      keyInput = "";
    } catch (e) {
      keyError = e as string;
    } finally {
      savingKey = false;
    }
  }

  async function deleteKey() {
    await invoke("delete_gemini_key");
    hasKey = false;
  }
</script>

<div class="ai-dashboard">
  {#if !hasKey}
    <div class="setup-card">
      <div class="setup-icon"><Sparkles size={32} /></div>
      <h2>Connect Gemini AI</h2>
      <p>
        Get a free API key at
        <a href="https://aistudio.google.com/app/apikey" target="_blank">
          aistudio.google.com
        </a>
        — no billing required.
      </p>
      <p class="security-note">
        🔒 Your key is stored in <strong>Windows Credential Manager</strong>,
        encrypted with your Windows login. It never touches the filesystem.
      </p>
      <div class="key-input-row">
        <input
          type="password"
          placeholder="AIzaSy..."
          bind:value={keyInput}
          onkeydown={(e) => e.key === "Enter" && saveKey()}
        />
        <Button onclick={saveKey} disabled={savingKey || !keyInput}>
          <Key size={13} /> {savingKey ? "Saving..." : "Save Key"}
        </Button>
      </div>
      {#if keyError}
        <p class="error">{keyError}</p>
      {/if}
    </div>

  {:else}
    <div class="header">
      <div class="title-row">
        <Sparkles size={20} />
        <h1>AI Advisor</h1>
        <span class="powered-by">powered by Gemini 2.0 Flash</span>
      </div>
      <button class="remove-key" onclick={deleteKey} title="Remove API key">
        <Trash2 size={13} /> Remove Key
      </button>
    </div>

    <div class="tabs">
      <button class:active={tab === "analyze"} onclick={() => tab = "analyze"}>
        <Sparkles size={14} /> Analyze
      </button>
      <button class:active={tab === "chat"} onclick={() => tab = "chat"}>
        <MessageSquare size={14} /> Chat
      </button>
      <button class:active={tab === "diagnose"} onclick={() => tab = "diagnose"}>
        <AlertTriangle size={14} /> Diagnose Problem
      </button>
    </div>

    <div class="tab-content">
      {#if tab === "analyze"}
        <ScanPanel />
      {:else if tab === "chat"}
        <ChatPanel />
      {:else}
        <DiagnosePanel />
      {/if}
    </div>
  {/if}
</div>

<style>
  .ai-dashboard {
    padding: 0 32px 32px;
    height: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
  .setup-card {
    margin: auto;
    max-width: 480px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
    padding: 40px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: center;
    text-align: center;
  }
  .setup-icon { color: var(--accent-color); }
  .setup-card h2 { font-size: 22px; font-weight: 700; margin: 0; }
  .setup-card p { color: var(--text-muted); font-size: 14px; margin: 0; }
  .security-note {
    background: rgba(var(--accent-rgb), 0.08);
    border: 1px solid rgba(var(--accent-rgb), 0.2);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    font-size: 13px !important;
    color: var(--text-secondary) !important;
    text-align: left !important;
  }
  .key-input-row {
    display: flex;
    gap: 8px;
    width: 100%;
  }
  .key-input-row input {
    flex: 1;
    background: rgba(255,255,255,0.05);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    color: var(--text-color);
    font-size: 13px;
    outline: none;
  }
  .key-input-row input:focus { border-color: var(--accent-color); }
  .error { color: var(--danger); font-size: 12px; }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-top: 32px;
  }
  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--accent-color);
  }
  .title-row h1 { font-size: 24px; font-weight: 700; margin: 0; color: var(--text-color); }
  .powered-by { font-size: 11px; color: var(--text-muted); margin-top: 2px; }
  .remove-key {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-size: 12px;
    padding: 5px 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 5px;
    transition: color 0.2s, border-color 0.2s;
  }
  .remove-key:hover { color: var(--danger); border-color: var(--danger); }
  .tabs {
    display: flex;
    gap: 4px;
    margin-bottom: 20px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 4px;
    width: fit-content;
  }
  .tabs button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 7px 16px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.18s;
  }
  .tabs button:hover { color: var(--text-color); }
  .tabs button.active {
    background: rgba(var(--accent-rgb), 0.15);
    color: var(--accent-color);
  }
  .tab-content { flex: 1; overflow-y: auto; }
</style>