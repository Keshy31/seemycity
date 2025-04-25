<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { PageData } from './$types'; // Import PageData for typing
  import { invalidateAll } from '$app/navigation'; // For refresh
  import PageHeader from '$lib/components/detail/PageHeader.svelte';
  import KeyMetricsGrid from '$lib/components/detail/KeyMetricsGrid.svelte';
  import ScoreBreakdown from '$lib/components/detail/ScoreBreakdown.svelte';
  import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  // Import necessary formatters used directly in THIS template
  import { formatCurrency, formatPopulation, formatWebsite, formatScore, getScoreColorStyle, getScoreBackgroundStyle } from '$lib/utils/formatUtils';

  /** @type {import('./$types').PageData} */
  export let data: PageData; // Data is passed from +page.ts load function

  // State for refresh button
  let isRefreshing = false;
  async function handleRefresh() {
    isRefreshing = true;
    await invalidateAll(); // Re-runs the load function
    isRefreshing = false;
  }

  // --- Formatting Helpers ---
  // These are now imported from formatUtils.ts

  // Access the municipality data directly from the prop
  // $: muniDetails = data.municipality;

  // Helper to safely get the first financial record
  // $: firstFinancialRecord = municipality?.financials?.[0];

  // Calculate Capex % based on Total Expenditure (OpEx + CapEx)
  $: capexPercentageFormatted = (() => {
    // Ensure latestFinancials are loaded
    if (!data?.latestFinancials) return 'N/A';

    const capex = data.latestFinancials.capital_expenditure;
    // Update to use operational_expenditure
    const opex = data.latestFinancials.operational_expenditure; 

    if (capex == null || opex == null) return 'N/A';

    const totalExpenditure = opex + capex;
    if (totalExpenditure === 0) return 'N/A'; // Avoid division by zero

    return ((capex / totalExpenditure) * 100).toFixed(1) + '%';
  })();

  // $: efficiencyScoreFormatted = firstFinancialRecord?.efficiency_score != null ? `${formatScore(firstFinancialRecord.efficiency_score)} / 100` : 'N/A';

  // $: revenuePerCapita = formatRevenuePerCapita(firstFinancialRecord?.revenue, municipality?.population);

  function formatRevenuePerCapita(revenue: number | null | undefined, population: number | null | undefined): string {
    if (revenue == null || population == null || population <= 0) return 'N/A';
    const perCapita = revenue / population;
    return `R ${perCapita.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })} / capita`;
  }
</script>

<svelte:head>
  <!-- Use optional chaining and nullish coalescing for safety, referencing the 'data' prop -->
  <title>Details for {data?.municipality?.name ?? 'Municipality'} | SeeMyCity</title>
  <meta name="description" content={`Financial health details for ${data?.municipality?.name ?? 'a municipality'}.`} />
</svelte:head>

{#await data}
  <!-- Loading State -->
  <div class="loading-state">
    <Icon icon="eos-icons:loading" class="loading-icon" />
    <p>Loading municipality data...</p>
  </div>
{:then pageData}
  <!-- Access resolved data -->
  <div class="detail-container">
    <!-- *** ADDED PageHeader COMPONENT HERE *** -->
    <PageHeader 
      municipalityName={pageData.municipality.name}
      overallScore={pageData.latestFinancials?.overall_score} 
      financialYear={pageData.latestFinancials?.year}
      provinceName={pageData.municipality.province}
      classification={pageData.municipality.classification}
    />

    {#if pageData.latestFinancials}
      <!-- Key Financial Metrics -->
      <KeyMetricsGrid 
        financials={pageData.latestFinancials} 
        population={pageData.municipality.population}
      />

      <!-- Score Breakdown Section -->
      <ScoreBreakdown 
        financials={pageData.latestFinancials}
        population={pageData.municipality.population}
      />

      <!-- Placeholder for potential future sections like Charts, News, etc. -->

    {:else}
      <!-- Message if no financial data is available -->
      <div class="no-data-message">
        <p>No financial data available for the latest year.</p>
      </div>
    {/if}

    <!-- About Section -->
    <div class="about-section">
      <h2 class="section-title">About {pageData.municipality.name}</h2>
      <dl class="about-grid">
        <div><dt>Province:</dt><dd>{pageData.municipality.province}</dd></div>
        <div><dt>Classification:</dt><dd>{pageData.municipality.classification ?? 'N/A'}</dd></div>
        <div><dt>Population:</dt><dd>{formatPopulation(pageData.municipality.population)}</dd></div>
        <div>
          <dt>Website:</dt>
          {#if pageData.municipality.website}
            <a href={pageData.municipality.website} target="_blank" rel="noopener noreferrer" class="website-link">
              {formatWebsite(pageData.municipality.website)}
              <Icon icon="mdi:external-link" class="external-link-icon" />
            </a>
          {:else}
            N/A
          {/if}
        </div>
      </dl>
    </div>

    <!-- Action Buttons -->
    <div class="action-buttons">
      <!-- TODO: Add Compare Functionality -->
      <!-- <button class="button compare-button">
        <Icon icon="mdi:compare-horizontal" /> Add to Compare
      </button> -->

      <button class="button refresh-button" on:click={handleRefresh} disabled={isRefreshing}>
        <Icon icon={isRefreshing ? 'eos-icons:loading' : 'mdi:refresh'} />
        {isRefreshing ? 'Refreshing...' : 'Refresh Data'}
      </button>
    </div>
  </div>
{:catch error}
  <!-- Error State -->
  <div class="error-state">
    <Icon icon="mdi:alert-octagon-outline" class="error-icon" />
    <h2>Error Loading Data</h2>
    <p>{error.message}</p>
    <button class="button retry-button" on:click={handleRefresh}>
      <Icon icon="mdi:refresh" /> Try Again
    </button>
  </div>
{/await}

<style lang="scss">
  .detail-container {
    padding: 2rem;
    max-width: 1200px;
    margin: 2rem auto;
    background-color: var(--background-color);
    border-radius: var(--border-radius-large);
    box-shadow: var(--box-shadow-sm);
  }

  .loading-state,
  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 300px; /* Ensure it takes up space */
    color: var(--text-secondary);
    font-size: 1.2rem;
    gap: 1rem;
  }

  .about-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem 2rem; /* Row and column gap */
    background-color: var(--background-offset-light);
    padding: 1.5rem;
    border-radius: var(--border-radius-medium);
    margin-top: 1.5rem;

    dt {
      font-weight: 600;
      color: var(--text-secondary);
      margin-bottom: 0.25rem;
    }

    dd {
      margin-left: 0;
      color: var(--text-primary);
    }
  }

  .website-link {
    color: var(--accent-color);
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    transition: color 0.2s ease;

    &:hover {
      color: var(--accent-color-hover);
      text-decoration: underline;
    }
  }

  .external-link-icon {
    font-size: 0.9em; /* Make icon slightly smaller */
    opacity: 0.8;
  }

  .action-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  .button {
    /* Basic button styles - consider moving to global? */
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: var(--border-radius-small);
    background-color: var(--accent-color);
    color: white;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;

    &:hover {
      background-color: var(--accent-color-hover);
    }

    &:disabled {
      background-color: var(--accent-color-disabled);
      cursor: not-allowed;
    }
  }

  @media (max-width: 768px) {
    .detail-container {
      padding: 1rem;
      margin: 1rem;
    }

    .about-grid {
      grid-template-columns: 1fr; /* Stack on smaller screens */
    }

    .action-buttons {
      justify-content: center;
    }
  }
</style>