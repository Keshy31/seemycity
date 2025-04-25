<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import type { FinancialYearData } from '$lib/types'; // Use the correct type

  // Import necessary utils
  import { getAuditIcon, getAuditOutcomeColorStyle } from '$lib/utils/auditUtils';
  import { formatCurrency, formatPercentage } from '$lib/utils/formatUtils';

  // Props: Financial data and population (needed for Revenue/Capita)
  export let financials: FinancialYearData | null | undefined;
  export let population: number | null | undefined;

  // --- Calculated Values ---

  // Revenue Per Capita
  $: revenuePerCapita = financials && financials.revenue !== null && population && population > 0
    ? formatCurrency(financials.revenue / population)
    : 'N/A';

  // Total Expenditure for percentage calculations
  $: totalExpenditure = financials
    ? (financials.operational_expenditure ?? 0) + (financials.capital_expenditure ?? 0)
    : 0;

  // Capital Spend Percentage
  $: capexPercentage = financials && totalExpenditure > 0
    ? formatPercentage(financials.capital_expenditure, totalExpenditure)
    : 'N/A';

  // Operational Spend Percentage
  $: opexPercentage = financials && totalExpenditure > 0
    ? formatPercentage(financials.operational_expenditure, totalExpenditure)
    : 'N/A';

</script>

<div class="key-metrics-grid">
  {#if financials}
    <!-- 💰 Revenue / Capita -->
    <MetricCard
      icon="mdi:cash-multiple"
      label="Revenue / Capita"
      value={revenuePerCapita}
    />

    <!-- 🏡 Capital Spend % -->
     <MetricCard
      icon="mdi:domain"
      label="Capital Spend %"
      value={capexPercentage}
    />

    <!-- ⚖️ Operational Spend % -->
    <MetricCard
      icon="mdi:receipt-text-check-outline"
      label="Operational Spend %"
      value={opexPercentage}
    /> <!-- Potential icon for Opex -->

    <!-- 🌟 Audit Outcome -->
    <MetricCard
      icon={getAuditIcon(financials.audit_outcome)}
      label="Audit Outcome"
      value={financials.audit_outcome ?? 'N/A'}
      valueColorStyle={getAuditOutcomeColorStyle(financials.audit_outcome)}
    />
  {:else}
    <!-- Optional: Show placeholders or a loading state if financials is null -->
    <p>Loading metrics...</p> <!-- Or render placeholder MetricCards -->
  {/if}
</div>

<style lang="scss">
  .key-metrics-grid {
    display: grid;
    gap: var(--spacing-lg);
    margin-bottom: var(--spacing-xl);
    // Responsive columns: 2 on small screens, 4 on larger screens
    grid-template-columns: repeat(2, 1fr);
     @media (min-width: 768px) { // Adjust breakpoint as needed
      grid-template-columns: repeat(4, 1fr);
    }
  }
</style>