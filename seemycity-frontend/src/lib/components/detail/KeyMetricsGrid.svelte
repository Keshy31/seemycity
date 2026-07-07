<script lang="ts">
	import MetricCard from './MetricCard.svelte';
	import type { FinancialYearData } from '$lib/types'; // Use the correct type

	// Import necessary utils
	import { getAuditIcon, getAuditOutcomeColorVarName } from '$lib/utils/auditUtils';
	import { formatCurrency, formatPercentage } from '$lib/utils/formatUtils';

	// Props: Financial data and population (needed for Revenue/Capita)
	export let financials: FinancialYearData | null | undefined;
	export let population: number | null | undefined;

	// --- Calculated Values ---

	// Revenue Per Capita
	$: revenuePerCapita =
		financials && financials.revenue !== null && population && population > 0
			? formatCurrency(financials.revenue / population)
			: 'N/A';

	// Total Expenditure for percentage calculations
	$: totalExpenditure = financials
		? (financials.operational_expenditure ?? 0) + (financials.capital_expenditure ?? 0)
		: 0;

	// Capital Spend Percentage
	$: capexPercentage =
		financials && totalExpenditure > 0
			? formatPercentage(financials.capital_expenditure, totalExpenditure)
			: 'N/A';

	// Operational Spend Percentage (Corrected: based on Total Expenditure)
	$: opexPercentage =
		financials && totalExpenditure > 0
			? formatPercentage(financials.operational_expenditure, totalExpenditure)
			: 'N/A';

	// Total Debt
	$: totalDebt = formatCurrency(financials?.debt);
</script>

<div class="key-metrics-grid">
	{#if financials}
		<!-- 💰 Revenue / Capita -->
		<MetricCard icon="mdi:cash-multiple" label="Revenue / Capita" value={revenuePerCapita} />

		<!-- 🏡 Capital Spend % -->
		<MetricCard icon="mdi:domain" label="Capital Spend %" value={capexPercentage} />

		<!-- ⚖️ Operational Spend % -->
		<MetricCard
			icon="mdi:receipt-text-check-outline"
			label="Operational Spend %"
			value={opexPercentage}
		/>
		<!-- Potential icon for Opex -->

		<!-- 🏦 Total Debt -->
		<MetricCard icon="mdi:bank-minus" label="Total Debt" value={totalDebt} />

		<!-- 🌟 Audit Outcome -->
		<MetricCard
			icon={getAuditIcon(financials.audit_outcome)}
			label="Audit Outcome"
			value={financials.audit_outcome ?? 'N/A'}
			valueColorStyle={`color: var(${getAuditOutcomeColorVarName(financials.audit_outcome)});`}
		/>
	{:else}
		<!-- Skeleton Loader: Show placeholder cards -->
		{#each Array.from({ length: 5 }, (_, i) => i) as i (i)}
			<MetricCard icon="" label="Loading..." value="" />
		{/each}
	{/if}
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.key-metrics-grid {
		display: grid;
		gap: var(--spacing-lg);
		margin-bottom: var(--spacing-xl);
		/* Responsive Grid: 1 col on mobile, 2 on tablet, up to 5 on desktop */
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));

		@media (min-width: 600px) {
			grid-template-columns: repeat(2, 1fr);
		}

		@media (min-width: 900px) {
			grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		}

		@media (min-width: 1200px) {
			grid-template-columns: repeat(5, 1fr);
		}
	}
</style>
