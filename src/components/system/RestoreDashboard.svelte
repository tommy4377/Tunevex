<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";

    let description = "Tunevex Restore Point";
    let status = "";
    let isCreating = false;
    let isSuccess = false;

    async function createPoint() {
        if (!description.trim()) {
            status = "Please enter a description.";
            return;
        }
        isCreating = true;
        status = "Creating restore point... This may take a few minutes.";
        isSuccess = false;

        try {
            const result = await invoke("create_restore_point", {
                description,
            });
            status = result as string;
            isSuccess = true;
        } catch (e) {
            status = `Error: ${e}`;
            isSuccess = false;
        } finally {
            isCreating = false;
        }
    }
</script>

<div class="restore-container" in:fade>
    <div class="header">
        <h2>🛡️ System Restore</h2>
        <p>
            Create a restore point before applying major tweaks to ensure
            safety.
        </p>
    </div>

    <div class="card">
        <div class="input-group">
            <label for="desc">Restore Point Description</label>
            <input
                id="desc"
                type="text"
                bind:value={description}
                placeholder="e.g. Before Tweaks"
                disabled={isCreating}
            />
        </div>

        <button
            class="create-btn"
            on:click={createPoint}
            disabled={isCreating}
            class:loading={isCreating}
        >
            {#if isCreating}
                ⏳ Creating...
            {:else}
                ➕ Create Restore Point
            {/if}
        </button>

        {#if status}
            <div
                class="status-msg"
                class:success={isSuccess}
                class:error={!isSuccess}
            >
                {status}
            </div>
        {/if}
    </div>

    <div class="info-box">
        <h3>ℹ️ Note</h3>
        <p>System Restore must be enabled on your C: drive for this to work.</p>
        <p>
            This process uses the <code>Checkpoint-Computer</code> PowerShell command.
        </p>
    </div>
</div>

<style>
    .restore-container {
        height: 100%;
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 24px;
        color: var(--text-primary);
        max-width: 600px;
        margin: 0 auto;
    }

    .header h2 {
        font-size: 24px;
        margin: 0 0 8px 0;
    }
    .header p {
        color: var(--text-secondary);
        margin: 0;
    }

    .restore-card {
        background: var(--layer-card);
        backdrop-filter: blur(20px);
        border: var(--border-glass);
        border-radius: var(--radius-card);
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 20px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    label {
        font-size: 14px;
        font-weight: 500;
        color: var(--text-secondary);
    }

    input {
        background: rgba(0, 0, 0, 0.2);
        border: var(--border-glass);
        border-radius: 8px;
        padding: 10px 14px;
        color: var(--text-primary);
        font-size: 14px;
        outline: none;
        transition: border-color 0.2s;
    }
    input:focus {
        border-color: var(--accent);
    }
    input:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .create-btn {
        background: var(--accent);
        color: white;
        border: none;
        padding: 12px;
        border-radius: 8px;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
    }
    .create-btn:hover:not(:disabled) {
        opacity: 0.9;
    }
    .create-btn:disabled {
        background: var(--text-secondary);
        cursor: not-allowed;
    }

    .status-msg {
        padding: 12px;
        border-radius: 8px;
        font-size: 13px;
        line-height: 1.4;
    }
    .status-msg.success {
        background: rgba(16, 185, 129, 0.1);
        color: #10b981;
        border: 1px solid rgba(16, 185, 129, 0.2);
    }
    .status-msg.error {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
        border: 1px solid rgba(239, 68, 68, 0.2);
    }

    .info-box {
        background: rgba(59, 130, 246, 0.05); /* Blue tint */
        border-radius: var(--radius-card);
        padding: 16px;
        font-size: 13px;
    }
    .info-box h3 {
        margin: 0 0 8px 0;
        font-size: 14px;
        color: #60a5fa;
    }
    .info-box p {
        margin: 4px 0;
        color: var(--text-secondary);
    }
    code {
        background: rgba(0, 0, 0, 0.3);
        padding: 2px 4px;
        border-radius: 4px;
        font-family: monospace;
    }
</style>
