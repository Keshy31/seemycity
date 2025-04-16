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
    const opex = data.latestFinancials.expenditure;

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
  <div class="loading-state flex flex-col items-center justify-center pt-32 text-gray-600">
    <Icon icon="eos-icons:loading" class="text-4xl mb-2" />
    <p>Loading municipality data...</p>
  </div>
{:then pageData}
  <!-- Access resolved data -->
  <div class="container mx-auto p-4 pt-20">
    <!-- Header: Name, Province, Score -->
    <div class="flex justify-between items-center mb-6 p-4 rounded-lg shadow-md" style="background-color: var(--background, #FDF6E3); color: var(--text, #3C2F2F);">
      <div>
        <h1 class="text-3xl font-bold font-heading">{pageData.municipality.name}</h1>
        <p class="text-lg text-gray-600">{pageData.municipality.province}</p>
        <p class="text-sm text-gray-500">Category: {pageData.municipality.classification ?? 'N/A'}</p>
      </div>
      <div class="text-right">
        <p class="text-sm font-medium uppercase tracking-wider">Overall Score</p>
        <!-- Apply dynamic color STYLE -->
        <p class="text-5xl font-bold" style={getScoreColorStyle(pageData.latestFinancials?.overall_score)}>
          {formatScore(pageData.latestFinancials?.overall_score)}
        </p>
        <p class="text-xs text-gray-500">Financial Year: {pageData.latestFinancials?.year ?? 'N/A'}</p>
      </div>
    </div>

    <!-- Display only if latestFinancials exist -->
    {#if pageData.latestFinancials}
      <!-- Key Metrics Row -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
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
        <!-- Capex % -->
        <div class="metric-card">
          <Icon icon="mdi:home-group" class="metric-icon" />
          <p class="metric-label">Capital Spend %</p>
          <p class="metric-value">
            {capexPercentageFormatted}
          </p>
        </div>
        <!-- Efficiency Score -->
        <div class="metric-card">
          <Icon icon="mdi:scale-balance" class="metric-icon" />
          <p class="metric-label">Efficiency Score</p>
          <p class="metric-value" style={getScoreColorStyle(pageData.latestFinancials.efficiency_score)}>
            {formatScore(pageData.latestFinancials.efficiency_score)}
          </p>
        </div>
        <!-- Audit Outcome -->
        <div class="metric-card">
          <Icon icon={getAuditIcon(pageData.latestFinancials.audit_outcome)} class="metric-icon" style={getAuditColorStyle(pageData.latestFinancials.audit_outcome)} />
          <p class="metric-label">Audit Outcome</p>
          <p class="metric-value">{pageData.latestFinancials.audit_outcome ?? 'N/A'}</p>
        </div>
      </div>

      <!-- Score Breakdown Section -->
      <div class="mb-6 p-4 rounded-lg shadow-md bg-white">
        <h2 class="text-xl font-semibold mb-3 font-heading">Score Breakdown ({pageData.latestFinancials.year})</h2>
        <div class="space-y-3">
          <!-- Financial Health -->
          <div class="breakdown-item">
            <span class="font-medium">Financial Health (30%)</span>
            <span class="text-right" style={getScoreColorStyle(pageData.latestFinancials.financial_health_score)}>
              {formatScore(pageData.latestFinancials.financial_health_score)} / 100
            </span>
          </div>
          <!-- Infrastructure Investment -->
          <div class="breakdown-item">
            <span class="font-medium">Infrastructure Investment (25%)</span>
            <span class="text-right" style={getScoreColorStyle(pageData.latestFinancials.infrastructure_score)}>
              {formatScore(pageData.latestFinancials.infrastructure_score)} / 100
            </span>
          </div>
          <!-- Efficiency -->
          <div class="breakdown-item">
            <span class="font-medium">Efficiency (25%)</span>
            <span class="text-right" style={getScoreColorStyle(pageData.latestFinancials.efficiency_score)}>
              {formatScore(pageData.latestFinancials.efficiency_score)} / 100
            </span>
          </div>
          <!-- Accountability -->
          <div class="breakdown-item">
            <span class="font-medium">Accountability (20%)</span>
            <span class="text-right" style={getScoreColorStyle(pageData.latestFinancials.accountability_score)}>
              {formatScore(pageData.latestFinancials.accountability_score)} / 100
            </span>
          </div>
        </div>
      </div>

      <!-- Financial Details Section -->
      <div class="p-4 rounded-lg shadow-md bg-white">
        <h2 class="text-xl font-semibold mb-3 font-heading">Financial Details ({pageData.latestFinancials.year})</h2>
        <dl class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-3">
          <div>
            <p>
              <span class="font-medium">Total Revenue:</span> {formatCurrency(pageData.latestFinancials.revenue)}
            </p>
            <p>
              <span class="font-medium">Total Expenditure:</span> {formatCurrency(pageData.latestFinancials.expenditure)}
            </p>
          </div>
          <div>
            <p>
              <span class="font-medium">Capital Expenditure:</span> {formatCurrency(pageData.latestFinancials.capital_expenditure)}
            </p>
            <p>
              <span class="font-medium">Total Debt:</span> {formatCurrency(pageData.latestFinancials.debt)}
            </p>
          </div>
        </dl>
      </div>

    {:else}
      <!-- Message if latest financials are not available for this muni -->
      <div class="p-4 rounded-lg shadow-md bg-yellow-100 text-yellow-800">
        <p>Latest financial data is not available for {pageData.municipality.name}.</p>
      </div>
    {/if}

    <!-- Municipality Details Section -->
    <div class="mt-6 p-4 rounded-lg shadow-md bg-white">
      <h2 class="text-xl font-semibold mb-3 font-heading">About {pageData.municipality.name}</h2>
      <dl class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-3">
        <div><dt class="font-medium">Province:</dt><dd>{pageData.municipality.province}</dd></div>
        <div><dt class="font-medium">Classification:</dt><dd>{pageData.municipality.classification ?? 'N/A'}</dd></div>
        <div><dt class="font-medium">Population (Est.):</dt><dd>{formatPopulation(pageData.municipality.population)}</dd></div>
        <div><dt class="font-medium">Website:</dt><dd><a href={pageData.municipality.website} target="_blank" rel="noopener noreferrer" class="text-teal-600 hover:underline">{formatWebsite(pageData.municipality.website)}</a></dd></div>
      </dl>
    </div>

    <!-- Refresh Button -->
    <div class="mt-6 text-center">
      <button
        on:click={handleRefresh}
        disabled={isRefreshing}
        class="px-4 py-2 bg-teal-600 text-white rounded hover:bg-teal-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center mx-auto"
      >
        {#if isRefreshing}
          <Icon icon="eos-icons:loading" class="mr-2" />
          Refreshing...
        {:else}
          <Icon icon="mdi:refresh" class="mr-2" />
          Refresh Data
        {/if}
      </button>
    </div>

    <!-- Removed the #if block wrapper -->
  </div>
{:catch error}
  <!-- Error State (will be added later) -->
  <p>Error loading data: {error.message}</p>
{/await}

<!-- Basic Styling (Add to a global CSS/SCSS or refine here) -->
<style lang="scss">
  // Key Metric Card Styling
  .metric-card {
    background-color: #fff;
    padding: 1rem;
    border-radius: 0.5rem;
    box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.1);
    color: var(--text, #3C2F2F);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    .metric-label {
      font-size: 1.125rem;
      font-weight: 500;
      margin-bottom: 0.25rem;
    }
    .metric-value {
      font-size: 0.875rem;
      color: var(--primary-color, #008080);
      font-weight: bold;
    }
  }

  // Score Breakdown Item Styling
  .breakdown-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ddd;
    padding-bottom: 0.5rem;
    margin-bottom: 0.5rem;
    &:last-child {
      border-bottom: none;
      padding-bottom: 0;
      margin-bottom: 0;
    }
  }

  // Use Ubuntu font if available (ensure it's imported globally)
  .font-heading {
    font-family: 'Ubuntu', sans-serif;
  }

  // Add more theme-specific styles as needed based on ux.md & MEMORY rules
  .container {
    background-color: var(--background, #FDF6E3); // Apply background from rules/ux
  }

  dl { 
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr)); /* Equivalent to grid-cols-2 */
    gap: 1rem; /* Equivalent to gap-4 */
    font-size: 0.875rem; /* Equivalent to text-sm */
  }

  dt {
    font-weight: 500;
  }

  dd { 
    text-align: right;
  }
</style>