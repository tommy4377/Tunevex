<script lang="ts">
    import { ArrowLeft } from "lucide-svelte";

    export let onclick: () => void;
    export let label: string = "Back to Dashboard";
</script>

<button class="back-btn" on:click={onclick}>
    <ArrowLeft size={16} />
    <span>{label}</span>
</button>

<style>
    .back-btn {
        display: inline-flex;
        align-items: center;
        gap: 8px;
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
