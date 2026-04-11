<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Send, User, Bot, Loader } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  interface ChatMessage {
    role: string;
    content: string;
  }

  let message = "";
  let loading = false;
  let messages: ChatMessage[] = [];
  let error = "";

  async function sendMessage() {
    if (!message.trim() || loading) return;
    
    const userMsg = message;
    message = "";
    messages = [...messages, { role: "user", content: userMsg }];
    
    loading = true;
    error = "";
    try {
      const response = await invoke<string>("ai_chat", {
        message: userMsg,
        history: messages.slice(0, -1)
      });
      messages = [...messages, { role: "model", content: response }];
    } catch (e) {
      error = e as string;
    } finally {
      loading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }
</script>

<div class="chat-panel">
  <div class="chat-header">
    <h2><Bot size={18} /> AI Assistant</h2>
    <p>Ask questions about your system, tweaks, or Windows optimization</p>
  </div>

  <div class="messages">
    {#if messages.length === 0}
      <div class="empty-state">
        <Bot size={48} />
        <p>Start a conversation with the AI assistant</p>
        <p class="hint">Try asking: "What tweaks should I add for gaming?"</p>
      </div>
    {:else}
      {#each messages as msg}
        <div class="message" class:user={msg.role === "user"} class:model={msg.role === "model"}>
          <div class="msg-icon">
            {#if msg.role === "user"}
              <User size={16} />
            {:else}
              <Bot size={16} />
            {/if}
          </div>
          <div class="msg-content">{msg.content}</div>
        </div>
      {/each}
    {/if}
    
    {#if loading}
      <div class="message model">
        <div class="msg-icon"><Loader size={16} class="spin" /></div>
        <div class="msg-content">Thinking...</div>
      </div>
    {/if}
  </div>

  {#if error}
    <div class="error-box">{error}</div>
  {/if}

  <div class="input-row">
    <textarea
      bind:value={message}
      placeholder="Ask a question..."
      onkeydown={handleKeydown}
      disabled={loading}
      rows="1"
    ></textarea>
    <Button onclick={sendMessage} disabled={loading || !message.trim()} variant="accent">
      <Send size={14} />
    </Button>
  </div>
</div>

<style>
  .chat-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 16px;
  }
  .chat-header h2 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 18px;
    margin: 0;
    color: var(--text-primary);
  }
  .chat-header p {
    color: var(--text-muted);
    font-size: 14px;
    margin: 4px 0 0 0;
  }
  .messages {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
  }
  .empty-state {
    text-align: center;
    padding: 40px;
    color: var(--text-muted);
  }
  .empty-state p { margin: 12px 0 0 0; }
  .empty-state .hint { font-size: 13px; opacity: 0.7; }
  .message {
    display: flex;
    gap: 12px;
    max-width: 85%;
  }
  .message.user {
    align-self: flex-end;
    flex-direction: row-reverse;
  }
  .message.model { align-self: flex-start; }
  .msg-icon {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  .user .msg-icon {
    background: var(--accent-color);
    color: white;
  }
  .model .msg-icon {
    background: var(--layer-hover);
    color: var(--accent-color);
  }
  .msg-content {
    padding: 12px 16px;
    border-radius: 12px;
    font-size: 14px;
    line-height: 1.5;
    white-space: pre-wrap;
  }
  .user .msg-content {
    background: var(--accent-color);
    color: white;
  }
  .model .msg-content {
    background: rgba(255,255,255,0.05);
    color: var(--text-primary);
  }
  .error-box {
    background: rgba(239,68,68,0.1);
    border: 1px solid rgba(239,68,68,0.3);
    border-radius: var(--radius-sm);
    padding: 12px;
    color: #ef4444;
    font-size: 13px;
  }
  .input-row {
    display: flex;
    gap: 8px;
  }
  .input-row textarea {
    flex: 1;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 12px;
    color: var(--text-primary);
    font-size: 14px;
    resize: none;
    min-height: 44px;
    max-height: 120px;
    outline: none;
  }
  .input-row textarea:focus { border-color: var(--accent-color); }
  :global(.spin) {
    animation: spin 1s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>