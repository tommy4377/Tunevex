<script lang="ts">
    export let columns: string = "repeat(auto-fill, minmax(220px, 1fr))";
    export let gap: string = "20px";
</script>

<div class="card-grid" style="--columns: {columns}; --gap: {gap};">
    <slot />
</div>

<style>
    .card-grid {
        display: grid;
        grid-template-columns: var(--columns);
        grid-auto-rows: minmax(168px, auto);
        align-content: start;
        justify-content: start;
        gap: var(--gap);
        overflow-y: auto;
        flex: 1;
        padding: 16px 24px 24px;
        min-height: 0;
    }
</style>
