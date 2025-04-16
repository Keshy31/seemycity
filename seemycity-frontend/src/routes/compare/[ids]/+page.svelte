<script lang="ts">
    import type { PageData } from './$types';
    import Icon from '@iconify/svelte';

    export let data: PageData;

    function formatCurrency(value: number | null | undefined): string {
        if (value == null) return 'N/A';
        return `R ${value.toLocaleString('en-ZA', { maximumFractionDigits: 0 })}`;
    }

    function formatScore(score: number | string | null | undefined): string {
        if (score == null) return 'N/A';
        if (typeof score === 'string') {
            try {
                score = parseFloat(score);
            } catch (e) {
                return 'N/A';
            }
        }
        return score.toFixed(1);
    }

    function formatPopulation(value: number | null | undefined): string {
        if (value == null) return 'N/A';
        return value.toLocaleString('en-ZA');
    }

    function formatPercentage(value: number | null | undefined): string {
        if (value == null) return 'N/A';
        return `${value.toFixed(1)}%`;
    }
</script>

<svelte:head>
    <title>Compare Municipalities | SeeMyCity</title>
    <meta name="description" content="Compare financial health details for selected municipalities." />
</svelte:head>

{#await data}
    <div class="container mx-auto p-4 pt-20 text-center">
        <p>Loading comparison data...</p>
        <Icon icon="line-md:loading-twotone-loop" class="text-3xl text-teal-600 mx-auto my-4" />
    </div>
{:then resolvedData}
    {#if resolvedData.error}
        <div class="container mx-auto p-4 pt-20 text-center text-red-600">
            <Icon icon="material-symbols:error-outline" class="text-4xl mx-auto mb-2" />
            <p>Error loading comparison data:</p>
            <p>{resolvedData.error}</p>
        </div>
    {:else if resolvedData.municipalities && resolvedData.municipalities.length > 0}
        <div class="container mx-auto p-4 pt-20">
            <h1 class="text-3xl font-bold mb-6 font-heading">Municipality Comparison</h1>

            <div class="comparison-grid bg-white p-4 rounded-lg shadow-md">
                <div class="font-semibold text-right pr-4">
                    <p class="py-2">Municipality</p>
                    <p class="py-2">Province</p>
                    <p class="py-2">Population</p>
                    <p class="py-2">Year</p>
                    <p class="py-2">Revenue</p>
                    <p class="py-2">Expenditure</p>
                    <p class="py-2">Debt</p>
                    <p class="py-2">Audit Outcome</p>
                </div>

                {#each resolvedData.municipalities as muni}
                    <div class="border-l border-r px-4">
                        <h2 class="text-xl font-semibold mb-2 py-2">{muni.name}</h2>
                        <p class="py-2">{muni.province}</p>
                        <p class="py-2">{formatPopulation(muni.population)}</p>
                        <p class="py-2">{muni.financials?.[0]?.year ?? 'N/A'}</p>
                        <p class="py-2">{formatCurrency(muni.financials?.[0]?.revenue)}</p>
                        <p class="py-2">{formatCurrency(muni.financials?.[0]?.operational_expenditure)}</p>
                        <p class="py-2">{formatCurrency(muni.financials?.[0]?.debt)}</p>
                        <p class="py-2">{muni.financials?.[0]?.audit_outcome ?? 'N/A'}</p>
                    </div>
                {/each}
            </div>
        </div>
    {:else}
        <div class="container mx-auto p-4 pt-20 text-center text-gray-600">
            <Icon icon="mdi:information-outline" class="text-4xl mx-auto mb-2" />
            <p>No data found for the requested municipalities.</p>
            {#if resolvedData.requestedIds && resolvedData.requestedIds.length > 0}
                <p class="text-sm mt-2">Requested IDs: {resolvedData.requestedIds.join(', ')}</p>
            {/if}
        </div>
    {/if}
{:catch error}
    <div class="container mx-auto p-4 pt-20 text-center text-red-600">
        <Icon icon="material-symbols:error-outline" class="text-4xl mx-auto mb-2" />
        <p>An unexpected error occurred while loading comparison data:</p>
        <p class="text-sm mt-1">{error.message}</p>
    </div>
{/await}

<style lang="scss">
    .comparison-grid {
        display: grid;
        grid-template-columns: auto 1fr 1fr;
        gap: 1rem;
        margin-top: 1.5rem;
        background-color: #fff;
        padding: 1rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }

    .comparison-grid > div {
        padding-top: 0.5rem;
        padding-bottom: 0.5rem;
    }

    .comparison-grid > div:first-child {
        font-weight: 600;
        text-align: right;
        padding-right: 1rem;
        color: #4b5563;
        border-right: 1px solid #e5e7eb;
    }

    .comparison-grid > div:not(:first-child) {
        padding-left: 1rem;
        padding-right: 1rem;
        border-right: 1px solid #e5e7eb;
    }

    .comparison-grid > div:last-child {
        border-right: none;
    }

    .comparison-grid > div:not(:first-child) h2 {
        font-size: 1.25rem;
        font-weight: 600;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem;
        border-bottom: 1px solid #e5e7eb;
        height: 3rem;
        margin-bottom: 0.5rem;
    }

    .comparison-grid > div:not(:first-child) p {
        padding-top: 0.5rem;
        padding-bottom: 0.5rem;
        min-height: 2.5rem;
    }
</style>