<script lang="ts">
    export let level: "safe" | "careful" | "dangerous" | "default" = "default";
    export let label: string = "";
</script>

<span class="badge {level}">
    {label || level.charAt(0).toUpperCase() + level.slice(1)}
</span>

<style>
    .badge {
        font-size: 11px;
        font-weight: 600;
        padding: 4px 10px;
        border-radius: 20px;
        white-space: nowrap;
    }

    .safe {
        background: rgba(34, 197, 94, 0.15);
        color: #22c55e;
    }

    .careful {
        background: rgba(245, 158, 11, 0.15);
        color: #f59e0b;
    }

    .dangerous {
        background: rgba(239, 68, 68, 0.15);
        color: #ef4444;
    }

    .default {
        background: rgba(100, 116, 139, 0.15);
        color: #64748b;
    }
</style>
