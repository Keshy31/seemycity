<script lang="ts">
  import Icon from '@iconify/svelte';

  /** @type {import('./$types').PageData} */
  export let data; // Data is passed from +page.ts load function

  // Access the municipality data directly from the prop
  $: muniDetails = data.municipality;

  // Helper to safely get the first financial record
  $: firstFinancialRecord = muniDetails?.financials?.[0];

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
  // Calculate Capex % based on Total Expenditure (OpEx + CapEx)
  $: capexPercentage = (() => {
    const capex = financials?.capital_expenditure;
    const opex = financials?.expenditure; // expenditure is OpEx
    if (capex == null || opex == null) return 'N/A';
    const totalExpenditure = opex + capex;
    if (totalExpenditure === 0) return 'N/A'; // Avoid division by zero
    return formatPercentage((capex / totalExpenditure) * 100);
  })();
  $: efficiencyScoreFormatted = financials?.efficiency_score != null ? `${formatScore(financials.efficiency_score)} / 100` : 'N/A';
</script>

<svelte:head>
	<title>{muniDetails?.name ?? 'Municipality Details'} - SeeMyCity</title>
	<meta name="description" content={`Financial health details for ${muniDetails?.name ?? 'a municipality'}.`} />
</svelte:head>

<div class="container mx-auto p-4 pt-20"> 
  {#if muniDetails && financials}
	<!-- Header: Name, Province, Score -->
	<div class="flex justify-between items-center mb-6 p-4 rounded-lg shadow-md" style="background-color: var(--background, #FDF6E3); color: var(--text, #3C2F2F);">
		<div>
			<h1 class="text-3xl font-bold font-heading">{muniDetails.name}</h1>
			<p class="text-lg text-gray-600">{muniDetails.province}</p>
			<p class="text-sm text-gray-500">Category: {muniDetails.classification ?? 'N/A'}</p>
		</div>
		<div class="text-right">
			<p class="text-sm font-medium uppercase tracking-wider">Overall Score</p>
			<!-- Apply dynamic color class -->
			<p class="text-5xl font-bold {getScoreColorClass(overallScore)}"> {formatScore(overallScore)}
			</p>
			<p class="text-xs text-gray-500">Financial Year: {financials.year}</p>
		</div>
	</div>

	<!-- Key Metrics Row -->
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

	<!-- Score Breakdown Section -->
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

	<!-- Financial Details Section -->
	<div class="p-4 rounded-lg shadow-md bg-white">
		<h2 class="text-xl font-semibold mb-3 font-heading">Financial Details ({financials.year})</h2>
		<dl class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-3">
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
		</dl>
	</div>

	{:else}
	<p>Municipality data or financials for the requested year not available.</p>
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
		background-color: #fff;
		padding: 1rem;
		border-radius: 0.5rem;
		box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.1);
		color: var(--text, #3C2F2F);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		.metric-icon {
			font-size: 2rem;
			margin-bottom: 0.5rem;
			color: var(--primary-color, #008080);
		}
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

	.metric-label { 
		font-size: 1.125rem; /* Equivalent to text-lg */
		font-weight: 500;
		margin-bottom: 0.25rem;
	}

	.metric-value {
		font-size: 0.875rem; /* Equivalent to text-sm */
		color: var(--primary-color, #008080);
		font-weight: bold;
	}

	.financial-detail-label { 
		font-size: 0.75rem; /* Equivalent to text-xs */
		color: #6b7280; /* Equivalent to text-gray-500 */
		font-weight: normal;
	}

	.financial-detail-value {
		font-size: 1.125rem; /* Equivalent to text-lg */
		font-weight: 600; /* Equivalent to font-semibold */
		margin-bottom: 0.5rem; /* Equivalent to mb-2 */
	}

	// Styles for dl/dt/dd for Financial Details (if using that structure)
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