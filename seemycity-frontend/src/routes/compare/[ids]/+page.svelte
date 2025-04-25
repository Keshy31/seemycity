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
    <div class="loading-container">
        <p>Loading comparison data...</p>
        <Icon icon="line-md:loading-twotone-loop" class="loading-icon" />
    </div>
{:then resolvedData}
    {#if resolvedData.error}
        <div class="error-container">
            <Icon icon="material-symbols:error-outline" class="error-icon" />
            <p>Error loading comparison data:</p>
            <p class="error-message">{resolvedData.error}</p>
        </div>
    {:else if resolvedData.municipalities && resolvedData.municipalities.length > 0}
        <div class="comparison-container">
            <h1 class="page-title">Municipality Comparison</h1>

            <div class="comparison-grid" style="grid-template-columns: auto repeat({resolvedData.municipalities.length}, 1fr);"> 
                <div class="header-column"> 
                    <p>Municipality</p>
                    <p>Province</p>
                    <p>Population</p>
                    <p>Year</p>
                    <p>Revenue</p>
                    <p>Expenditure</p>
                    <p>Debt</p>
                    <p>Audit Outcome</p>
                </div>

                {#each resolvedData.municipalities as muni}
                    <div class="data-column"> 
                        <h2 class="municipality-name">{muni.name}</h2>
                        <p>{muni.province}</p>
                        <p>{formatPopulation(muni.population)}</p>
                        <p>{muni.financials?.[0]?.year ?? 'N/A'}</p>
                        <p>{formatCurrency(muni.financials?.[0]?.revenue)}</p>
                        <p>{formatCurrency(muni.financials?.[0]?.operational_expenditure)}</p>
                        <p>{formatCurrency(muni.financials?.[0]?.debt)}</p>
                        <p>{muni.financials?.[0]?.audit_outcome ?? 'N/A'}</p>
                    </div>
                {/each}
            </div>
        </div>
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
    <div class="error-container">
        <Icon icon="material-symbols:error-outline" class="error-icon" />
        <p>An unexpected error occurred while loading comparison data:</p>
        <p class="error-message">{error.message}</p>
    </div>
{/await}

<style lang="scss">
    // --- Containers ---
    .loading-container, .error-container, .no-data-container, .comparison-container {
        padding: 1rem;
        margin-top: 4rem; 
        text-align: center; 
    }

    .comparison-container {
        text-align: left; 
    }

    // --- Icons ---
    .loading-icon, .error-icon, .info-icon {
        display: block; 
        margin: 1rem auto 0.5rem auto; 
        font-size: 2.5rem; 
    }
     .error-icon {
        color: var(--score-low-color, #CD5C5C); 
     }
     .info-icon {
        color: var(--neutral-grey, #888); 
     }


    // --- Text ---
    .page-title {
        font-size: 1.875rem; 
        font-weight: 700; 
        margin-bottom: 1.5rem; 
        font-family: 'Ubuntu', sans-serif; 
    }

    .error-message, .requested-ids {
        font-size: 0.875rem; 
        margin-top: 0.25rem; 
    }

    .error-container {
        color: var(--score-low-color, #CD5C5C); 
    }

    .no-data-container {
         color: var(--neutral-grey, #888); 
    }

    // --- Comparison Grid (Keep existing structure, potentially adjust selectors/styles if needed) ---
    .comparison-grid {
        display: grid;
        gap: 0; 
        margin-top: 1.5rem;
        background-color: var(--background-offset, #fff); 
        padding: 1rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
        overflow-x: auto; 
    }

    // Header Column (Labels)
    .header-column {
        font-weight: 600;
        text-align: right;
        padding-right: 1rem;
        color: var(--text-muted, #555); 

        p {
            padding-top: 0.5rem;
            padding-bottom: 0.5rem;
            min-height: 2.5rem; 
             white-space: nowrap; 
        }
         p:first-child { 
             min-height: 3rem; 
         }
    }

    // Data Column (Per Municipality)
    .data-column {
        padding-left: 1rem;
        padding-right: 1rem;
        border-right: 1px solid var(--border-color, #e5e7eb); 

        &:last-child {
            border-right: none;
        }

        h2.municipality-name { 
            font-size: 1.25rem; 
            font-weight: 600; 
            padding-top: 0.5rem; 
            padding-bottom: 0.5rem; 
            border-bottom: 1px solid var(--border-color, #e5e7eb);
            height: 3rem; 
            margin-bottom: 0.5rem; 
            overflow: hidden; 
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        p {
            padding-top: 0.5rem; 
            padding-bottom: 0.5rem; 
            min-height: 2.5rem; 
            display: flex; 
            align-items: center;
        }
    }
</style>