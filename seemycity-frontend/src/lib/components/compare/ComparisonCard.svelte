<script lang="ts">
    import type { MunicipalityDetail } from '$lib/types';
    import {
        formatScore,
        getScoreStatusIcon,
        formatCurrency,
        formatPercentage,
        getScoreColorVarName
    } from '$lib/utils/formatUtils';
    import { getAuditOutcomeColorVarName, getAuditOutcomeText } from '$lib/utils/auditUtils';
    import Icon from '@iconify/svelte';

    export let municipality: MunicipalityDetail;

    // Helper to get the latest financial record
    $: latestFinancials = municipality?.financials?.[0];

    // --- Dynamic styles ---
    $: scoreColorVar = getScoreColorVarName(latestFinancials?.overall_score);
    $: auditColorVar = getAuditOutcomeColorVarName(latestFinancials?.audit_outcome);

    // --- Calculate derived metrics for display ---

    // Revenue per Capita
    $: revenuePerCapita = (() => {
        const revenue = latestFinancials?.revenue;
        const population = municipality?.population;
        if (revenue == null || population == null || population <= 0) return 'N/A';
        return formatCurrency(revenue / population);
    })();

    // Total Expenditure (OpEx + CapEx)
    $: totalExpenditure =
        (latestFinancials?.capital_expenditure ?? 0) + (latestFinancials?.operational_expenditure ?? 0);

    // Operational Spend %
    $: opexPercentage = formatPercentage(latestFinancials?.operational_expenditure, totalExpenditure);

    // CapEx Ratio %
    $: capexPercentage = formatPercentage(latestFinancials?.capital_expenditure, totalExpenditure);

    // Audit Outcome
    $: auditOutcome = getAuditOutcomeText(latestFinancials?.audit_outcome);

    // Total Debt
    $: totalDebt = formatCurrency(latestFinancials?.debt);
</script>

<div class="comparison-card">
    <div class="card-header">
        <h3 class="municipality-name">{municipality.name}</h3>
        <div class="score-display" style="color: var({scoreColorVar});">
            <span class="score-value">{formatScore(latestFinancials?.overall_score)}</span>
            <Icon class="score-icon" icon={getScoreStatusIcon(latestFinancials?.overall_score)} />
        </div>
    </div>

    <hr class="divider" />

    <div class="card-body">
        <div class="metric-row">
            <span class="metric-label"><Icon icon="mdi:cash-multiple" /> Revenue / Capita</span>
            <span class="metric-value">{revenuePerCapita}</span>
        </div>
        <div class="metric-row">
            <span class="metric-label"><Icon icon="mdi:chart-pie-outline" /> OpEx Spend %</span>
            <span class="metric-value">{opexPercentage}</span>
        </div>
        <div class="metric-row">
            <span class="metric-label"><Icon icon="mdi:domain-plus" /> CapEx Spend %</span>
            <span class="metric-value">{capexPercentage}</span>
        </div>
        <div class="metric-row">
            <span class="metric-label"><Icon icon="mdi:bank-minus" /> Total Debt</span>
            <span class="metric-value">{totalDebt}</span>
        </div>
        <div class="metric-row audit-row">
            <span class="metric-label"><Icon icon="mdi:clipboard-check-outline" /> Audit Outcome</span>
            <span class="metric-value audit-outcome-value" style="color: var({auditColorVar});">{auditOutcome}</span>
        </div>
    </div>
</div>

<style lang="scss">
    @use '../../../styles/variables' as *;

    .comparison-card {
        background-color: var(--background-offset-color);
        border: 1px solid var(--border-color);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-lg);
        box-shadow: var(--box-shadow-sm);
        width: 340px;
        flex-shrink: 0;
        transition: all 0.2s ease-in-out;
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);

        &:hover {
            transform: translateY(-4px);
            box-shadow: var(--box-shadow-md);
        }
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: var(--spacing-md);
    }

    .municipality-name {
        font-family: var(--font-family-headings);
        font-size: var(--font-size-xl);
        font-weight: 600;
        color: var(--text-heading-color);
        margin: 0;
        line-height: 1.3;
        flex-grow: 1;
    }

    .score-display {
        display: flex;
        align-items: center;
        gap: var(--spacing-xs);
        text-align: right;
        flex-shrink: 0;
        color: var(--text-color); // Fallback color
    }

    .score-value {
        font-family: var(--font-family-headings);
        font-size: var(--font-size-xxl);
        font-weight: 700;
        line-height: 1;
    }

    .score-icon {
        font-size: var(--font-size-xl);
        margin-top: -2px; // Fine-tune alignment
    }

    .divider {
        border: none;
        height: 1px;
        background-color: var(--border-color);
        margin: 0;
    }

    .card-body {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
    }

    .metric-row {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: var(--spacing-md);
        line-height: 1.4;
    }

    .metric-label {
        display: inline-flex;
        align-items: center;
        gap: var(--spacing-sm);
        font-size: var(--font-size-sm);
        color: var(--text-muted-color);
        flex-shrink: 0;
        font-weight: 500;

        :global(.iconify) {
            font-size: 1.2rem;
            color: var(--primary-color-light);
        }
    }

    .metric-value {
        font-size: var(--font-size-md);
        font-weight: 600;
        color: var(--text-color);
        text-align: right;
        white-space: normal;
        word-break: break-word;
    }

    .audit-row {
        align-items: baseline;
    }

    .audit-outcome-value {
        font-weight: 700;
        font-size: var(--font-size-sm);
    }
</style>