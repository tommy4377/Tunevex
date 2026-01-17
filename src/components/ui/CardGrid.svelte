<script lang="ts">
    export let columns: string = "repeat(auto-fill, minmax(240px, 1fr))";
    export let gap: string = "20px";
</script>

<div class="card-grid" style="--columns: {columns}; --gap: {gap};">
    <slot />
</div>

<style>
    .card-grid {
        display: grid;
        grid-template-columns: var(--columns);
        gap: var(--gap);
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
        min-height: 0;
    }
</style>
