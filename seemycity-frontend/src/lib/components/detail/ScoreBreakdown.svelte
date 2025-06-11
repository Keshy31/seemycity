<script lang="ts">
    import type { FinancialYearData } from '$lib/types';
    import {
        formatCurrency,
        formatPercentage,
        formatScore,
        getScoreColorVarName // Use the new var name utility
    } from '$lib/utils/formatUtils';
    import {
        calculateCapexRatio,
        calculateOpexRatio,
        calculateDebtRatio,
        calculateRevenuePerCapita
    } from '$lib/utils/calculations';
    import Icon from '@iconify/svelte';
    import { getAuditOutcomeText } from '$lib/utils/auditUtils';
    import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
    import { slide } from 'svelte/transition';

    export let financials: FinancialYearData | null | undefined;
    export let population: number | null | undefined;

    // --- Calculate derived metrics ---
    $: capexRatio = calculateCapexRatio(
        financials?.capital_expenditure,
        financials?.operational_expenditure
    );
    $: opexRatio = calculateOpexRatio(financials?.operational_expenditure, financials?.revenue);
    $: debtRatio = calculateDebtRatio(financials?.debt, financials?.revenue);
    $: revenuePerCapita = calculateRevenuePerCapita(financials?.revenue, population);
    $: totalExpenditure =
        (financials?.capital_expenditure ?? 0) + (financials?.operational_expenditure ?? 0);
    $: operationalEfficiencyMetric =
        financials && financials.revenue && financials.revenue > 0
            ? formatPercentage(financials.operational_expenditure, financials.revenue)
            : 'N/A';
</script>

<div class="score-breakdown-card">
    <details>
        <summary>
            <h3>
                <Icon icon="mdi:calculator-variant-outline" />
                How the Score is Calculated
            </h3>
            <div class="toggle-icon">
                <Icon icon="mdi:chevron-right" />
            </div>
        </summary>

        {#if financials}
            <div class="pillars-grid" transition:slide={{ duration: 250 }}>
                <!-- Pillar 1: Financial Health -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:finance" /> Financial Health (30%)</h4>
                        <span class="pillar-score" style="color: var({getScoreColorVarName(financials.financial_health_score)});">{formatScore(financials.financial_health_score)} / 100</span>
                    </div>
                    <ProgressBar value={financials.financial_health_score} />
                    <div class="pillar-metrics">
                        <span>Debt Ratio: <strong>{formatPercentage(debtRatio, 1)}</strong></span>
                        <span>Revenue/Capita: <strong>{formatCurrency(revenuePerCapita)}</strong></span>
                    </div>
                </div>

                <!-- Pillar 2: Infrastructure Investment -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:domain" /> Infrastructure Investment (25%)</h4>
                        <span class="pillar-score" style="color: var({getScoreColorVarName(financials.infrastructure_score)});">{formatScore(financials.infrastructure_score)} / 100</span>
                    </div>
                    <ProgressBar value={financials.infrastructure_score} />
                    <div class="pillar-metrics">
                        <span>CapEx Ratio: <strong>{formatPercentage(capexRatio, 1)}</strong></span>
                    </div>
                </div>

                <!-- Pillar 3: Efficiency & Service Delivery -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:scale-balance" /> Efficiency & Service Delivery (25%)</h4>
                        <span class="pillar-score" style="color: var({getScoreColorVarName(financials.efficiency_score)});">{formatScore(financials.efficiency_score)} / 100</span>
                    </div>
                    <ProgressBar value={financials.efficiency_score} />
                    <div class="pillar-metrics">
                        <span>OpEx Ratio: <strong>{operationalEfficiencyMetric}</strong></span>
                    </div>
                </div>

                <!-- Pillar 4: Accountability -->
                <div class="pillar-item">
                    <div class="pillar-header">
                        <h4><Icon icon="mdi:account-check-outline" /> Accountability (20%)</h4>
                        <span class="pillar-score" style="color: var({getScoreColorVarName(financials.accountability_score)});">{formatScore(financials.accountability_score)} / 100</span>
                    </div>
                    <ProgressBar value={financials.accountability_score} />
                    <div class="pillar-metrics">
                        <span>Audit Outcome: <strong>{getAuditOutcomeText(financials.audit_outcome)}</strong></span>
                    </div>
                </div>
            </div>
        {:else}
            <div class="loading-state" transition:slide={{ duration: 250 }}>
                <p>Loading score breakdown...</p>
            </div>
        {/if}
    </details>
</div>

<style lang="scss">
    @use '../../../styles/variables' as *;

    .score-breakdown-card {
        background-color: var(--background-offset-color);
        border-radius: var(--border-radius-lg);
        border: 1px solid var(--border-color);
        margin-top: var(--spacing-xl);
        overflow: hidden; // Ensures children with border-radius clip correctly
    }

    summary {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--spacing-md) var(--spacing-lg);
        cursor: pointer;
        list-style: none; // Remove default marker
        transition: background-color 0.2s ease-in-out;

        &::-webkit-details-marker {
            display: none; // For Safari/Chrome
        }

        &:hover {
            background-color: var(--background-offset-darker-color);
        }

        h3 {
            display: flex;
            align-items: center;
            gap: var(--spacing-md);
            font-size: var(--font-size-lg);
            font-weight: 600;
            margin: 0;
            color: var(--text-heading-color);
        }

        .toggle-icon {
            font-size: 1.5rem;
            transition: transform 0.25s ease-in-out;
            color: var(--text-muted-color);
        }
    }

    /* Rotate icon when details are open */
    details[open] > summary .toggle-icon {
        transform: rotate(90deg);
    }

    .pillars-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: var(--spacing-lg);
        padding: var(--spacing-lg);
        border-top: 1px solid var(--border-color);
        background-color: var(--background-color);
    }

    .pillar-item {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
        background-color: var(--background-offset-color);
        padding: var(--spacing-md);
        border-radius: var(--border-radius-md);
        border: 1px solid var(--border-color-light);
    }

    .pillar-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: var(--spacing-md);

        h4 {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
            font-size: var(--font-size-md);
            font-weight: 600;
            margin: 0;
        }

        .pillar-score {
            font-weight: 700;
            font-size: var(--font-size-md);
            white-space: nowrap;
        }
    }

    .pillar-metrics {
        font-size: var(--font-size-sm);
        color: var(--text-muted-color);
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
        margin-top: var(--spacing-sm);

        strong {
            color: var(--text-color);
            font-weight: 600;
        }
    }

    .loading-state {
        padding: var(--spacing-xl);
        text-align: center;
        color: var(--text-muted-color);
    }
</style>