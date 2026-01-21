<script lang="ts">
    export let columns: string = "repeat(auto-fill, 280px)";
    export let gap: string = "20px";
</script>

<div class="card-grid" style="--columns: {columns}; --gap: {gap};">
    <slot />
</div>

<style>
    .card-grid {
        display: grid;
        grid-template-columns: var(--columns);
        justify-content: center;
        gap: var(--gap);
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
        min-height: 0;
    }
</style>
