<script lang="ts">
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
  import { onMount } from 'svelte';

  // Define structure for the items within the financials array
  interface FinancialYearDetails {
    year: number;
    revenue?: number | null;
    expenditure?: number | null;
    capital_expenditure?: number | null;
    debt?: number | null;
    audit_outcome?: string | null;
    overall_score?: number | null;
    financial_health_score?: number | null;
    infrastructure_score?: number | null;
    efficiency_score?: number | null;
    accountability_score?: number | null;
  }

  // Update main interface to include the financials array
  interface MunicipalityDetails {
    id: string;
    name: string;
    province: string;
    population?: number | null;
    classification?: string | null;
    website?: string | null;
    financials: FinancialYearDetails[];
  }

  $: id = $page.params.id;

  let muniDetails: MunicipalityDetails | null = null;
  let isLoading = true;
  let error: string | null = null;

  // Helper to safely get the first financial record
  $: firstFinancialRecord = muniDetails?.financials?.[0];

  onMount(async () => {
    if (!id) {
      error = "Municipality ID not found in URL.";
      isLoading = false;
      return;
    }
    isLoading = true;
    error = null;
    try {
      const response = await fetch(`/api/municipalities/${id}`);
      if (!response.ok) {
        if (response.status === 404) {
          throw new Error(`Municipality with ID '${id}' not found.`);
        } else {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
      }
      const rawData = await response.json();
      console.log('Fetched Muni Details (Raw):', rawData); // Log raw data

      // Basic validation before assigning
      if (rawData && typeof rawData === 'object' && Array.isArray(rawData.financials)) {
         muniDetails = rawData as MunicipalityDetails;
      } else {
          console.error('Received unexpected data structure:', rawData);
          throw new Error('Received unexpected data structure from API.');
      }

    } catch (e: any) {
      console.error('Error fetching municipality details:', e);
      error = e.message || 'Failed to load municipality data.';
      muniDetails = null; // Clear data on error
    } finally {
      isLoading = false;
    }
  });

  function formatCurrency(value: number | null | undefined): string {
    if (value == null) return 'N/A';
    if (Math.abs(value) >= 1_000_000_000) {
      return `R ${(value / 1_000_000_000).toFixed(1)} billion`;
    }
    if (Math.abs(value) >= 1_000_000) {
      return `R ${(value / 1_000_000).toFixed(1)} million`;
    }
    return `R ${value.toLocaleString()}`; // Fallback for smaller numbers
  }

  function formatScore(score: number | null | undefined): string {
    if (score == null) return 'N/A';
    const clampedScore = Math.max(0, Math.min(100, score));
    return clampedScore.toFixed(0); // Display score as whole number (e.g., 75)
  }

  function getScoreColorClass(score: number | null | undefined): string {
    if (score == null) return 'score-grey'; // Default/NA color
    if (score >= 75) return 'score-green';
    if (score >= 50) return 'score-orange';
    return 'score-red';
  }

  function formatPercentage(value: number | null | undefined): string {
    if (value == null) return 'N/A';
    return `${value.toFixed(1)} %`;
  }

  function formatRevenuePerCapita(revenue: number | null | undefined, population: number | null | undefined): string {
    if (revenue == null || population == null || population <= 0) return 'N/A';
    const perCapita = revenue / population;
    return `R ${perCapita.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })} / capita`;
  }

  $: financials = muniDetails?.financials?.[0]; // Get the first (latest) year's data
  $: population = muniDetails?.population;
  $: overallScore = financials?.overall_score;
  $: revenuePerCapita = formatRevenuePerCapita(financials?.revenue, population);
  $: capexPercentage = (financials?.capital_expenditure != null && financials?.expenditure != null && financials.expenditure !== 0)
    ? formatPercentage((financials.capital_expenditure / financials.expenditure) * 100)
    : 'N/A';
  $: efficiencyScoreFormatted = financials?.efficiency_score != null ? `${formatScore(financials.efficiency_score)} / 100` : 'N/A';

</script>

<svelte:head>
	<title>{muniDetails ? muniDetails.name : 'Municipality Details'} - SeeMyCity</title>
	<meta name="description" content={`Financial health details for ${muniDetails?.name ?? 'a municipality'}.`} />
</svelte:head>

<div class="container mx-auto p-4 pt-20"> 
  {#if isLoading}
	<p>Loading municipality details...</p>
	{:else if error}
	<p class="text-red-500">Error loading data: {error}</p>
	{:else if muniDetails && financials}
	<!-- Header: Name, Province, Score -->
	<div class="flex justify-between items-center mb-6 p-4 rounded-lg shadow-md" style="background-color: var(--background, #FDF6E3); color: var(--text, #3C2F2F);">
		<div>
			<h1 class="text-3xl font-bold font-heading">{muniDetails.name}</h1>
			<p class="text-lg text-gray-600">{muniDetails.province_name}</p>
			<p class="text-sm text-gray-500">Category: {muniDetails.category}</p>
		</div>
		<div class="text-right">
			<p class="text-sm font-medium uppercase tracking-wider">Overall Score</p>
			<!-- Apply dynamic color class -->
			<p class="text-5xl font-bold {getScoreColorClass(overallScore)}"> {formatScore(overallScore)}
			</p>
			<p class="text-xs text-gray-500">Financial Year: {financials.year}</p>
		</div>
	</div>

	<!-- Key Metrics Row (NEW) -->
	<div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
		<!-- Revenue Per Capita -->
		<div class="metric-card">
			<Icon icon="mdi:cash-multiple" class="metric-icon" />
			<p class="metric-label">Revenue / Capita</p>
			<p class="metric-value">{revenuePerCapita}</p>
		</div>
		<!-- Capex % -->
		<div class="metric-card">
			<Icon icon="mdi:home-group" class="metric-icon" />
			<p class="metric-label">Capex %</p>
			<p class="metric-value">{capexPercentage}</p>
		</div>
		<!-- Efficiency Score -->
		<div class="metric-card">
			<Icon icon="mdi:scale-balance" class="metric-icon" />
			<p class="metric-label">Efficiency Score</p>
			<p class="metric-value">{efficiencyScoreFormatted}</p>
		</div>
		<!-- Audit Outcome -->
		<div class="metric-card">
			<Icon icon="mdi:star-check-outline" class="metric-icon" />
			<p class="metric-label">Audit Outcome</p>
			<p class="metric-value">{financials.audit_outcome ?? 'N/A'}</p>
		</div>
	</div>

	<!-- Score Breakdown Section (NEW) -->
	<div class="mb-6 p-4 rounded-lg shadow-md bg-white">
		<h2 class="text-xl font-semibold mb-3 font-heading">Score Breakdown</h2>
		<div class="space-y-3">
			<!-- Financial Health -->
			<div class="breakdown-item">
				<span class="font-medium">Financial Health (30%)</span>
				<span class="text-right {getScoreColorClass(financials.financial_health_score)}"> {formatScore(financials.financial_health_score)} / 100
				</span>
			</div>
			<!-- Infrastructure Investment -->
			<div class="breakdown-item">
				<span class="font-medium">Infrastructure Investment (25%)</span>
				<span class="text-right {getScoreColorClass(financials.infrastructure_score)}"> {formatScore(financials.infrastructure_score)} / 100
				</span>
			</div>
			<!-- Efficiency -->
			<div class="breakdown-item">
				<span class="font-medium">Efficiency (25%)</span>
				<span class="text-right {getScoreColorClass(financials.efficiency_score)}"> {formatScore(financials.efficiency_score)} / 100
				</span>
			</div>
			<!-- Accountability -->
			<div class="breakdown-item">
				<span class="font-medium">Accountability (20%)</span>
				<span class="text-right {getScoreColorClass(financials.accountability_score)}"> {formatScore(financials.accountability_score)} / 100
				</span>
			</div>
		</div>
	</div>

	<!-- Other Financial Details (Example) -->
	<div class="p-4 rounded-lg shadow-md bg-white">
		<h2 class="text-xl font-semibold mb-3 font-heading">Financial Details ({financials.year})</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<div>
				<p>
					<span class="font-medium">Total Revenue:</span> {formatCurrency(financials.revenue)}
				</p>
				<p>
					<span class="font-medium">Total Expenditure:</span> {formatCurrency(financials.expenditure)}
				</p>
			</div>
			<div>
				<p>
					<span class="font-medium">Capital Expenditure:</span> {formatCurrency(financials.capital_expenditure)}
				</p>
				<p>
					<span class="font-medium">Total Debt:</span> {formatCurrency(financials.debt)}
				</p>
			</div>
		</div>
	</div>

	{:else}
	<p>Municipality data not found or format is incorrect.</p>
	{/if}
</div>

<!-- Basic Styling (Add to a global CSS/SCSS or refine here) -->
<style lang="scss">
	// Score Colors
	.score-green {
		color: #28a745; // Green
	}
	.score-orange {
		color: #fd7e14; // Orange
	}
	.score-red {
		color: #dc3545; // Red
	}
	.score-grey {
		color: #6c757d; // Grey
	}

	// Key Metric Card Styling
	.metric-card {
		@apply bg-white p-4 rounded-lg shadow text-center flex flex-col items-center justify-center;
		color: var(--text, #3C2F2F);

		.metric-icon {
			@apply text-3xl mb-2;
			color: var(--primary-color, #008080);
		}
		.metric-label {
			@apply text-sm font-medium text-gray-500 mb-1;
		}
		.metric-value {
			@apply text-lg font-semibold;
		}
	}

	// Score Breakdown Item Styling
	.breakdown-item {
		@apply flex justify-between items-center border-b border-gray-200 pb-2;
		&:last-child {
			border-bottom: none;
			padding-bottom: 0;
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

</style>