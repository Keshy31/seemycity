<script lang="ts">
    import type { PageData } from './$types';
    import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
    import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
    import ComparisonContainer from '$lib/components/compare/ComparisonContainer.svelte';

    export let data: PageData;
</script>

<svelte:head>
    <title>Compare Municipalities | SeeMyCity</title>
    <meta name="description" content="Compare financial health details for selected municipalities." />
</svelte:head>

{#await data}
    <LoadingSpinner />
{:then resolvedData}
    {#if resolvedData.municipalities && resolvedData.municipalities.length > 0}
        <ComparisonContainer municipalities={resolvedData.municipalities} />
    {:else if resolvedData.error}
        <ErrorMessage message={`Error loading comparison data: ${resolvedData.error}`} />
    {:else}
        <div class="no-data-container">
            <Icon icon="mdi:information-outline" class="info-icon" />
            <p>No data found for the requested municipalities.</p>
            {#if resolvedData.requestedIds && resolvedData.requestedIds.length > 0}
                <p class="requested-ids">Requested IDs: {resolvedData.requestedIds.join(', ')}</p>
            {/if}
        </div>
    {/if}
{:catch error}
    <ErrorMessage message={`An unexpected error occurred: ${error.message}`} />
{/await}