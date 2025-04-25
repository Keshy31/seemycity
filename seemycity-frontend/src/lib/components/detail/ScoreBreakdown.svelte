<script lang="ts">
    import type { FinancialYearData } from '$lib/types';
    import {
        formatCurrency,
        formatPercentage,
        formatScore,
        getScoreColorStyle
    } from '$lib/utils/formatUtils';
    import {
        calculateCapexRatio,
        calculateOpexRatio,
        calculateDebtRatio,
        calculateRevenuePerCapita
    } from '$lib/utils/calculations'; // Assume these exist/will be created
    import Icon from '@iconify/svelte';
    import { getAuditOutcomeText } from '$lib/utils/auditUtils';
    import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
    import { slide } from 'svelte/transition'; // Import slide transition

    export let financials: FinancialYearData | null | undefined;
    export let population: number | null | undefined;

    // --- Calculate derived metrics ---
    $: capexRatio = calculateCapexRatio(financials?.capital_expenditure, financials?.operational_expenditure);
    $: opexRatio = calculateOpexRatio(financials?.operational_expenditure, financials?.revenue);
    $: debtRatio = calculateDebtRatio(financials?.debt, financials?.revenue);
    $: revenuePerCapita = calculateRevenuePerCapita(financials?.revenue, population);
    $: totalExpenditure = financials?.capital_expenditure + financials?.operational_expenditure;
    $: operationalEfficiencyMetric = financials && financials.revenue && financials.revenue > 0
        ? formatPercentage(financials.operational_expenditure, financials.revenue)
        : 'N/A';

</script>

<div class="score-breakdown-card">
    <!-- Use <details> for built-in expand/collapse -->
    <details>
        <summary> 
            <h3>
                <Icon icon="mdi:calculator-variant-outline" />
                How the Score is Calculated
                <span class="toggle-icon">▶</span> <!-- Default state icon -->
            </h3>
        </summary>

        {#if financials}
            <!-- Apply slide transition to the content -->
            <div class="pillars-grid" transition:slide={{ duration: 200 }}>
                <!-- Pillar 1: Financial Health -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:finance" /> Financial Health (30%)</h4>
                        <span class="pillar-score" style="{getScoreColorStyle(financials.financial_health_score)}">{formatScore(financials.financial_health_score)} / 100</span>
                    </div>
                    <div class="pillar-metrics">
                        <span>Debt Ratio: {formatPercentage(debtRatio, 1)}</span>
                        <span>Revenue per Capita: {formatCurrency(revenuePerCapita)}</span>
                    </div>
                    <ProgressBar value={financials.financial_health_score} />
                    <!-- Optional: Add visual bar/indicator here -->
                </div>

                <!-- Pillar 2: Infrastructure Investment -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:domain" /> Infrastructure Investment (25%)</h4>
                        <span class="pillar-score" style="{getScoreColorStyle(financials.infrastructure_score)}">{formatScore(financials.infrastructure_score)} / 100</span>
                    </div>
                    <div class="pillar-metrics">
                        <span>CapEx Ratio: {formatPercentage(capexRatio, 1)}</span>
                    </div>
                    <ProgressBar value={financials.infrastructure_score} />
                    <!-- Optional: Add visual bar/indicator here -->
                </div>

                <!-- Pillar 3: Efficiency & Service Delivery -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:scale-balance" /> Efficiency & Service Delivery (25%)</h4>
                        <span class="pillar-score" style="{getScoreColorStyle(financials.efficiency_score)}">{formatScore(financials.efficiency_score)} / 100</span>
                    </div>
                    <div class="pillar-metrics">
                        <span>OpEx Ratio: {operationalEfficiencyMetric}</span>
                    </div>
                    <ProgressBar value={financials.efficiency_score} />
                    <!-- Optional: Add visual bar/indicator here -->
                </div>

                <!-- Pillar 4: Accountability -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:account-check-outline" /> Accountability (20%)</h4>
                        <span class="pillar-score" style="{getScoreColorStyle(financials.accountability_score)}">{formatScore(financials.accountability_score)} / 100</span>
                    </div>
                    <div class="pillar-metrics">
                        <span>Audit Outcome: {getAuditOutcomeText(financials.audit_outcome)}</span>
                    </div>
                    <ProgressBar value={financials.accountability_score} />
                    <!-- Optional: Add visual bar/indicator here -->
                </div>
            </div>
        {:else}
            <p class="unavailable">Score breakdown data is not available for this year.</p>
        {/if}
    </details> 
</div>

<style lang="scss">
    @import '../../../styles/variables'; // Corrected path (no extension/underscore)

    .score-breakdown-card {
        background-color: var(--background-offset-light); // Use CSS var
        border-radius: var(--border-radius-medium);
        padding: 1.5rem;
        margin-top: 1.5rem;
        box-shadow: var(--box-shadow-sm);

        h3 {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 1.3rem;
            color: var(--text-color);
            margin-top: 0;
            margin-bottom: 1.5rem;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.75rem;

            // Style the summary element for better cursor etc.
            cursor: pointer;
 
            .toggle-icon {
                font-size: 0.9em; // Smaller toggle icon
                transition: transform 0.2s ease-in-out;
                float: right; // Use float to push to the right, or adjust layout
                line-height: inherit; // Align vertically with h3 if needed
            }
        }
    }

    // Rotate toggle icon when open - Use attribute selector
    details[open] > summary .toggle-icon {
        transform: rotate(90deg);
    }

    // Remove default marker from details/summary
    summary {
        list-style: none;
        display: block; // Necessary for ::marker removal
    }
    summary::-webkit-details-marker { // Chrome/Safari
        display: none;
    }
    summary::marker { // Firefox
        display: none;
    }

    .pillars-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1.5rem;
    }

    .pillar-item {
        background-color: var(--background-color); // Use base background
        padding: 1rem;
        border-radius: var(--border-radius-small);
        border: 1px solid var(--border-color);
    }

    .pillar-header {
        display: flex;
        justify-content: space-between;
        align-items: baseline;
        margin-bottom: 0.75rem;

        h4 {
            margin: 0;
            font-size: 1.1rem;
            color: var(--text-color);
            font-weight: 600;
        }

        .pillar-score {
            font-weight: bold;
            font-size: 1rem;
            white-space: nowrap;
        }
    }

    .pillar-metrics {
        font-size: 0.9rem;
        color: var(--text-muted-color);
        display: flex;
        flex-direction: column;
        gap: 0.4rem;

        span {
            display: block; // Ensure they stack nicely
        }
    }

    :global(.pillar-item .progress-bar-container) {
        margin-top: 0.75rem; // Add some space above the progress bar
    }

    .unavailable {
        color: var(--text-muted-color);
        font-style: italic;
        text-align: center;
        padding: 1rem;
    }

    // Basic responsiveness
    @media (max-width: 600px) {
        .pillars-grid {
            grid-template-columns: 1fr; // Stack on small screens
        }
        .pillar-header {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.25rem;
        }
    }

    // Add styles for pillar icons
    .pillar-header h4 {
        display: flex;
        align-items: center;
        gap: 0.4rem; // Space between icon and text
    }
</style>