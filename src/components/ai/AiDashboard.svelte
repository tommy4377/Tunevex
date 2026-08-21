<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { Sparkles, Key, Trash2, Wifi } from "lucide-svelte";
  import type { Tweak } from "$lib/types";
  import Button from "../ui/Button.svelte";
  import ScanPanel from "./ScanPanel.svelte";
  import ChatPanel from "./ChatPanel.svelte";
  import DiagnosePanel from "./DiagnosePanel.svelte";

  type Tab = "scan" | "chat" | "diagnose";
  let tab: Tab = "scan";
  let hasKey = false;
  let keyInput = "";
  let savingKey = false;
  let keyError = "";
  let testingKey = false;
  let connectionStatus = "";
  export let allTweaks: Tweak[] = [];

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
      await testConnection();
    } catch (e) {
      keyError = e as string;
    } finally {
      savingKey = false;
    }
  }

  async function testConnection() {
    testingKey = true;
    connectionStatus = "";
    keyError = "";
    try {
      await invoke("test_gemini_connection");
      connectionStatus = "Connection verified.";
    } catch (e) {
      keyError = `Key saved, but Gemini could not be reached: ${e as string}`;
    } finally {
      testingKey = false;
    }
  }

  async function deleteKey() {
    await invoke("delete_gemini_key");
    hasKey = false;
  }

  function openAiStudio() {
    void openUrl("https://aistudio.google.com/app/apikey");
  }
</script>

<div class="ai-dashboard">
  {#if !hasKey}
    <div class="setup-card">
      <div class="setup-icon"><Sparkles size={32} /></div>
      <h2>Connect Gemini AI</h2>
      <p>
        Create an API or authorization key at
        <button type="button" onclick={openAiStudio} class="subtle-link">
          aistudio.google.com
        </button>
        . Availability and billing depend on your Google AI Studio account.
      </p>
      <p class="security-note">
        Your key is stored in <strong>Windows Credential Manager</strong>,
        encrypted with your Windows login. It never touches the filesystem.
      </p>
      <div class="key-input-row">
        <input
          type="password"
          placeholder="Paste your AI Studio key"
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
        <span class="powered-by">powered by Gemini 3.5 Flash</span>
      </div>
      <div class="key-actions">
        <button class="remove-key" onclick={testConnection} disabled={testingKey} title="Test Gemini connection">
          <Wifi size={13} /> {testingKey ? "Testing..." : "Test"}
        </button>
        <button class="remove-key" onclick={deleteKey} title="Remove API key">
          <Trash2 size={13} /> Remove Key
        </button>
      </div>
    </div>

    <div class="tabs">
      <button class:active={tab === "scan"} onclick={() => tab = "scan"}>Scan</button>
      <button class:active={tab === "chat"} onclick={() => tab = "chat"}>Chat</button>
      <button class:active={tab === "diagnose"} onclick={() => tab = "diagnose"}>Diagnose</button>
      <button class="disconnect-btn" onclick={deleteKey}>Disconnect</button>
    </div>

    <div class="tab-content">
      {#if connectionStatus}<p class="connection-ok">{connectionStatus}</p>{/if}
      {#if keyError}<p class="error">{keyError}</p>{/if}
      {#if tab === "scan"}
        <ScanPanel {allTweaks} />
      {:else if tab === "chat"}
        <ChatPanel {allTweaks} />
      {:else}
        <DiagnosePanel />
      {/if}
    </div>
  {/if}
</div>

<style>
  .ai-dashboard {
    padding: 0 24px 24px;
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
  .subtle-link { color: var(--accent-color); text-decoration: none; opacity: 0.85; transition: opacity 0.15s; border: 0; background: none; padding: 0; cursor: pointer; font: inherit; }
  .subtle-link:hover { opacity: 1; text-decoration: underline; }
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
  .remove-key:disabled { opacity: 0.55; cursor: wait; }
  .key-actions { display: flex; gap: 8px; }
  .connection-ok { color: var(--success); font-size: 12px; margin: 0 0 10px; }
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

  @media (max-width: 940px) {
    .ai-dashboard { padding-inline: 18px; padding-bottom: 18px; }
    .header { padding-top: 20px; margin-bottom: 16px; }
    .tabs { margin-bottom: 14px; }
  }
</style>
