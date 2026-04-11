<script lang="ts">
    export let level: "safe" | "careful" | "dangerous" | "default" = "default";
    export let label: string = "";
</script>

<span class="badge {level}">
    {label || level.charAt(0).toUpperCase() + level.slice(1)}
</span>

<style>
    .badge {
        display: inline-flex;
        align-items: center;
        font-size: 10px;
        font-weight: 600;
        padding: 3px 8px;
        border-radius: var(--radius-pill);
        white-space: nowrap;
        letter-spacing: 0.02em;
    }
    .safe      { background: rgba(52, 211, 153, 0.14); color: #34d399; }
    .careful   { background: rgba(251, 191, 36,  0.14); color: #fbbf24; }
    .dangerous { background: rgba(248, 113, 113, 0.14); color: #f87171; }
    .default   { background: rgba(100, 116, 139, 0.14); color: #94a3b8; }
</style>
