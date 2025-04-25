<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { PageData } from './$types'; // Import PageData for typing
  import { invalidateAll } from '$app/navigation'; // For refresh
  import PageHeader from '$lib/components/detail/PageHeader.svelte';
  import KeyMetricsGrid from '$lib/components/detail/KeyMetricsGrid.svelte'; // Import the grid component
  // Import necessary formatters used directly in THIS template
  import { formatCurrency, formatPopulation, formatWebsite } from '$lib/utils/formatUtils';

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

  function formatScore(score: number | null | undefined): string {
    if (score == null) return 'N/A';
    // Clamp score between 0 and 100 before formatting
    const clampedScore = Math.max(0, Math.min(100, score));
    return clampedScore.toFixed(1); // Use one decimal place
  }

  // Use inline styles for score colors based on UX guidelines
  function getScoreColorStyle(score: number | null | undefined): string {
    if (score == null) return 'color: var(--neutral-grey, #888);'; // Neutral
    if (score >= 70) return 'color: var(--score-high-color, #2E8B57);'; // Green
    if (score >= 40) return 'color: var(--score-medium-color, #F28C38);'; // Orange
    return 'color: var(--score-low-color, #CD5C5C);'; // Red
  }

  // Background color variant for score indicators if needed
  function getScoreBackgroundStyle(score: number | null | undefined): string {
    if (score == null) return 'background-color: var(--neutral-grey, #888);'; // Neutral
    if (score >= 70) return 'background-color: var(--score-high-color, #2E8B57);';
    if (score >= 40) return 'background-color: var(--score-medium-color, #F28C38);';
    return 'background-color: var(--score-low-color, #CD5C5C);';
  }

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
    {#if pageData.latestFinancials}
      <!-- Key Metrics Grid - Render the component -->
      <KeyMetricsGrid financials={pageData.latestFinancials} population={pageData.municipality.population} />

      <!-- Score Breakdown Section -->
      <div class="score-breakdown-section">
        <h2 class="section-title">Score Breakdown</h2>
        <!-- Financial Health -->
        <div class="score-row">
          <span class="score-pillar-label">Financial Health (30%)</span>
          <span class="score-value" style={getScoreColorStyle(pageData.latestFinancials.financial_health_score)}>
            {formatScore(pageData.latestFinancials.financial_health_score)}
          </span>
          <span class="score-suffix">/ 100</span>
        </div>
        <!-- Infrastructure Investment -->
        <div class="score-row">
          <span class="score-pillar-label">Infrastructure Inv. (25%)</span>
          <span class="score-value" style={getScoreColorStyle(pageData.latestFinancials.infrastructure_score)}>
            {formatScore(pageData.latestFinancials.infrastructure_score)}
          </span>
           <span class="score-suffix">/ 100</span>
        </div>
        <!-- Efficiency & Service Delivery -->
        <div class="score-row">
          <span class="score-pillar-label">Efficiency & Service Delivery (25%)</span>
          <span class="score-value" style={getScoreColorStyle(pageData.latestFinancials.efficiency_score)}>
            {formatScore(pageData.latestFinancials.efficiency_score)}
          </span>
           <span class="score-suffix">/ 100</span>
        </div>
        <!-- Accountability -->
        <div class="score-row">
          <span class="score-pillar-label">Accountability (20%)</span>
          <span class="score-value" style={getScoreColorStyle(pageData.latestFinancials.accountability_score)}>
            {formatScore(pageData.latestFinancials.accountability_score)}
          </span>
           <span class="score-suffix">/ 100</span>
        </div>
      </div>

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
    padding: var(--spacing-lg); // Use spacing variable
    max-width: var(--container-max-width); // Limit width
    margin: 0 auto; // Center container
  }

  .score-breakdown-section {
    margin-bottom: var(--spacing-lg);
    padding: var(--spacing-md);
    background-color: var(--card-background-color, #fff);
    border: 1px solid var(--border-color, #eee);
  }

  .section-title {
    font-size: 1.25rem;
    margin-bottom: var(--spacing-md);
  }

  .score-row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    padding: 0.5rem 0;
    border-bottom: 1px dashed #eee; // Example separator

    &:last-child {
        border-bottom: none;
    }
  }
  .score-pillar-label {
    flex-grow: 1; // Example
    margin-right: 1rem;
  }
  .score-value {
    font-weight: bold;
  }
   .score-suffix {
      font-size: 0.8em;
      margin-left: 0.25em;
      color: #777; // Example
   }

  .no-data-message {
    padding: var(--spacing-lg);
    text-align: center;
    color: #777; // Example
    border: 1px dashed #ddd; // Example
    border-radius: 0.5rem;
    margin-bottom: var(--spacing-lg);
  }

  .about-section {
    margin-top: var(--spacing-lg);
    padding: var(--spacing-md);
    border: 1px solid #eee; // Example
    border-radius: 0.5rem;
  }

  .about-grid {
    display: grid;
    gap: 0.75rem 1.5rem; // row-gap column-gap
     @media (min-width: 768px) { // Example breakpoint
      grid-template-columns: repeat(2, 1fr);
    }
  }

   .about-grid dt {
      font-weight: 600; // Example
      color: #444; // Example
   }
   .about-grid dd {
      margin-left: 0; // Reset default dl margin
   }

   .website-link {
      color: var(--accent-teal, #008080); // Use variable
      text-decoration: none;
      &:hover {
         text-decoration: underline;
      }
   }
   .external-link-icon {
      margin-left: 0.25rem;
      vertical-align: middle; // Align icon nicely
      font-size: 0.9em;
   }


  .action-buttons {
    margin-top: var(--spacing-lg);
    display: flex;
    justify-content: flex-end; // Align buttons right
    gap: 0.5rem;
  }

  .button {
    padding: 0.5rem 1rem;
    border: 1px solid transparent;
    border-radius: 0.375rem;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.375rem; // Space between icon and text
    transition: background-color 0.2s;

    &:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
  }

  .refresh-button {
    background-color: var(--accent-teal, #008080); // Use variable
    color: white;
    &:hover:not(:disabled) {
       background-color: darken(#008080, 10%); // Example hover
    }
  }


  // --- Loading and Error States ---
  .loading-state, .error-state {
     display: flex;
     flex-direction: column;
     align-items: center;
     justify-content: center;
     padding-top: 5rem; // Example vertical centering space
     color: #666; // Example
  }

  .loading-icon, .error-icon {
      font-size: 2.5rem; // Example
      margin-bottom: 0.5rem;
  }

   .error-state h2 {
      font-size: 1.25rem;
      margin-bottom: 0.5rem;
      color: #CC0000; // Example error color
   }

   .error-state p {
      margin-bottom: 1rem;
   }

   .retry-button {
      background-color: #666;
      color: white;
       &:hover:not(:disabled) {
         background-color: darken(#666, 10%); // Example hover
      }
   }

  .page-container {
    max-width: 960px; /* Or your preferred max width */
    margin: var(--spacing-xl) auto;
    padding: 0 var(--spacing-md);
  }
</style>