<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { PageData } from './$types'; // Import PageData for typing
  import { invalidateAll } from '$app/navigation'; // For refresh

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

  function formatCurrency(value: number | null | undefined): string {
    if (value == null) return 'N/A';
    return value.toLocaleString('en-ZA', {
      style: 'currency',
      currency: 'ZAR',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    });
  }

  function formatPopulation(pop: number | null | undefined): string {
    if (pop == null) return 'N/A';
    return Math.round(pop).toLocaleString('en-ZA');
  }

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

  // Map audit outcomes to icons
  function getAuditIcon(outcome: string | null | undefined): string {
    if (!outcome) return 'mdi:help-circle-outline'; // Unknown/NA
    switch (outcome.toLowerCase().trim()) {
      case 'unqualified - no findings':
      case 'unqualified opinion with no findings': // Handle variations
        return 'mdi:check-circle-outline'; // Clean
      case 'unqualified - emphasis of matter items':
      case 'financially unqualified opinion': // Common alias
      case 'unqualified opinion with findings': // Handle variations
        return 'mdi:information-outline'; // Clean but with notes
      case 'qualified':
      case 'qualified opinion':
        return 'mdi:alert-circle-outline'; // Issues found
      case 'adverse':
      case 'adverse opinion':
        return 'mdi:close-circle-outline'; // Major issues
      case 'disclaimer':
      case 'disclaimer of opinion':
      case 'disclaimer with findings': // Handle variations
        return 'mdi:comment-question-outline'; // Unable to audit
      case 'outstanding':
        return 'mdi:clock-alert-outline'; // Not submitted
      default:
        console.warn('Unknown audit outcome:', outcome);
        return 'mdi:help-circle-outline'; // Default for unknown
    }
  }

  // Map audit outcomes to colors
  function getAuditColorStyle(outcome: string | null | undefined): string {
    if (!outcome) return 'color: var(--neutral-grey, #888);';
    switch (outcome.toLowerCase().trim()) {
      case 'unqualified - no findings':
      case 'unqualified opinion with no findings':
        return 'color: var(--audit-clean-color, #2E8B57);'; // Green
      case 'unqualified - emphasis of matter items':
      case 'financially unqualified opinion':
      case 'unqualified opinion with findings':
        return 'color: var(--audit-emphasis-color, #4682B4);'; // Blue
      case 'qualified':
      case 'qualified opinion':
        return 'color: var(--audit-qualified-color, #F28C38);'; // Orange
      case 'adverse':
      case 'adverse opinion':
      case 'disclaimer':
      case 'disclaimer of opinion':
      case 'disclaimer with findings':
        return 'color: var(--audit-adverse-disclaimer-color, #CD5C5C);'; // Red
      case 'outstanding':
        return 'color: var(--audit-outstanding-color, #B0B0B0);'; // Grey
      default:
        return 'color: var(--neutral-grey, #888);';
    }
  }

  // Helper to format website URL for display
  function formatWebsite(url: string | null | undefined): string {
    if (!url) return 'N/A';
    try {
      const parsedUrl = new URL(url);
      return parsedUrl.hostname.replace(/^www\./, ''); // Remove www.
    } catch (e) {
      return url; // Return original if parsing fails
    }
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
    <!-- Header: Name, Province, Score -->
    <div class="header-section" style="background-color: var(--background, #FDF6E3); color: var(--text, #3C2F2F);">
      <div>
        <h1 class="municipality-name">{pageData.municipality.name}</h1>
        <p class="province-name">{pageData.municipality.province}</p>
        <p class="classification">Category: {pageData.municipality.classification ?? 'N/A'}</p>
      </div>
      <div class="score-section">
        <p class="score-label">Overall Score</p>
        <!-- Apply dynamic color STYLE -->
        <p class="overall-score" style={getScoreColorStyle(pageData.latestFinancials?.overall_score)}>
          {formatScore(pageData.latestFinancials?.overall_score)}
        </p>
        <p class="financial-year">Financial Year: {pageData.latestFinancials?.year ?? 'N/A'}</p>
      </div>
    </div>

    <!-- Display only if latestFinancials exist -->
    {#if pageData.latestFinancials}
      <!-- Key Metrics Row -->
      <div class="key-metrics-grid">
        <!-- Revenue Per Capita (Calculation needs population) -->
        <div class="metric-card">
          <Icon icon="mdi:cash-multiple" class="metric-icon" />
          <p class="metric-label">Revenue / Capita</p>
          <p class="metric-value">
            {formatCurrency(
              pageData.municipality.population && pageData.latestFinancials.revenue
                ? pageData.latestFinancials.revenue / pageData.municipality.population
                : null
            )}
          </p>
        </div>

        <!-- CapEx % -->
        <div class="metric-card">
          <Icon icon="mdi:domain" class="metric-icon" />
          <p class="metric-label">CapEx % of Total Exp.</p>
          <p class="metric-value">{capexPercentageFormatted}</p>
        </div>

        <!-- Debt (Raw value) -->
        <div class="metric-card">
          <Icon icon="mdi:bank-minus" class="metric-icon" />
          <p class="metric-label">Total Debt</p>
          <p class="metric-value">{formatCurrency(pageData.latestFinancials.debt)}</p>
        </div>

        <!-- Audit Outcome -->
        <div class="metric-card">
          <Icon
            icon={getAuditIcon(pageData.latestFinancials.audit_outcome)}
            class="metric-icon"
            style={getAuditColorStyle(pageData.latestFinancials.audit_outcome)}
          />
          <p class="metric-label">Audit Outcome</p>
          <p class="metric-value">{pageData.latestFinancials.audit_outcome ?? 'N/A'}</p>
        </div>
      </div>

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
  // Placeholder for SCSS styles - we will add these in Step 2/3
  .detail-container {
    padding: 1rem; // Example basic padding
  }

  .header-section {
    // Will add flex/grid styles here later
    padding: 1rem;
    margin-bottom: 1.5rem;
    border-radius: 0.5rem; // Example
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); // Example
  }

  .municipality-name {
    // Heading styles here
  }
  .province-name {
    // Style here
  }
  .classification {
     // Style here
  }

  .score-section {
     // Style here
  }
  .score-label {
     // Style here
  }
  .overall-score {
     // Style here
  }
  .financial-year {
     // Style here
  }

  .key-metrics-grid {
    // Grid styles here
    display: grid;
    gap: 1rem;
    margin-bottom: 1.5rem;
     // Example responsive grid
    grid-template-columns: repeat(2, 1fr);
    @media (min-width: 768px) { // Example breakpoint
      grid-template-columns: repeat(4, 1fr);
    }
  }

  .metric-card {
    // Card styles here
    padding: 1rem;
    border: 1px solid #eee; // Example
    border-radius: 0.5rem;
    text-align: center; // Example
  }
  .metric-icon {
    font-size: 2rem; // Example
    margin-bottom: 0.5rem;
  }
  .metric-label {
    font-size: 0.875rem;
    margin-bottom: 0.25rem;
    color: #555; // Example
  }
  .metric-value {
    font-size: 1.125rem;
    font-weight: 600; // Example
  }

  .score-breakdown-section {
    // Section styles
    margin-bottom: 1.5rem;
    padding: 1rem;
    border: 1px solid #eee; // Example
    border-radius: 0.5rem;
  }

  .section-title {
    // Title styles
    font-size: 1.25rem;
    margin-bottom: 1rem;
  }

  .score-row {
    // Row styles (e.g., flex)
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
    // Label styles
    flex-grow: 1; // Example
    margin-right: 1rem;
  }
  .score-value {
    // Score value styles
    font-weight: bold;
  }
   .score-suffix {
      font-size: 0.8em;
      margin-left: 0.25em;
      color: #777; // Example
   }

  .no-data-message {
    // Styles for message when no financials
    padding: 1.5rem;
    text-align: center;
    color: #777; // Example
    border: 1px dashed #ddd; // Example
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .about-section {
    // Section styles
     margin-top: 1.5rem;
     padding: 1rem;
     border: 1px solid #eee; // Example
     border-radius: 0.5rem;
  }

  .about-grid {
    // Grid styles
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
    // Container styles (e.g., flex)
    margin-top: 1.5rem;
    display: flex;
    justify-content: flex-end; // Align buttons right
    gap: 0.5rem;
  }

  .button {
    // Basic button styles
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

</style>