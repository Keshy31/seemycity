<script lang="ts">
    import type { MunicipalityDetail } from '$lib/types';
    import {
        formatScore,
        getScoreStatusIcon,
        formatCurrency,
        formatPercentage, 
        formatPopulation 
    } from '$lib/utils/formatUtils';
    import Icon from '@iconify/svelte';

    export let municipality: MunicipalityDetail;

    // Helper to get the latest financial record
    $: latestFinancials = municipality?.financials?.[0];

    // --- Calculate derived metrics for display ---

    // Revenue per Capita
    $: revenuePerCapita = (() => {
        const revenue = latestFinancials?.revenue;
        const population = municipality?.population;
        if (revenue == null || population == null || population <= 0) return 'N/A';
        const perCapita = revenue / population;
        return new Intl.NumberFormat('en-ZA', {
            style: 'currency',
            currency: 'ZAR',
            minimumFractionDigits: 0,
            maximumFractionDigits: 0
        }).format(perCapita);
    })();

    // Total Expenditure (OpEx + CapEx)
    $: totalExpenditure = (latestFinancials?.capital_expenditure ?? 0) + (latestFinancials?.operational_expenditure ?? 0);

    // Operational Spend %
    $: opexPercentage = formatPercentage(
        latestFinancials?.operational_expenditure,
        totalExpenditure
    );

    // CapEx Ratio %
    $: capexPercentage = formatPercentage(
        latestFinancials?.capital_expenditure,
        totalExpenditure
    );

    // Audit Outcome
    $: auditOutcome = latestFinancials?.audit_outcome ?? 'N/A';

    // Total Debt
    $: totalDebt = formatCurrency(latestFinancials?.debt);

</script>

<div class="comparison-card-new">
    <div class="card-header-new">
        <h3 class="municipality-name-new">{municipality.name}</h3>
        <div class="score-display-new">
            <span class="score-value-new">{formatScore(latestFinancials?.overall_score)}</span>
            <span class="score-icon-new">{getScoreStatusIcon(latestFinancials?.overall_score)}</span>
        </div>
    </div>

    <hr class="divider" />

    <div class="card-body-new">
        <div class="metric-row-new">
            <span class="metric-label-new"><Icon icon="mdi:cash-multiple" /> Revenue / Capita</span>
            <span class="metric-value-new">{revenuePerCapita}</span>
        </div>
        <div class="metric-row-new">
            <span class="metric-label-new"><Icon icon="mdi:chart-pie-outline" /> OpEx Spend %</span>
            <span class="metric-value-new">{opexPercentage}</span>
        </div>
        <div class="metric-row-new">
            <span class="metric-label-new"><Icon icon="mdi:chart-gantt" /> CapEx Ratio %</span>
            <span class="metric-value-new">{capexPercentage}</span>
        </div>
        <div class="metric-row-new">
            <span class="metric-label-new"><Icon icon="mdi:bank-minus" /> Total Debt</span>
            <span class="metric-value-new">{totalDebt}</span>
        </div>
        <div class="metric-row-new audit-row">
            <span class="metric-label-new"><Icon icon="mdi:clipboard-check-outline" /> Audit Outcome</span>
            <span class="metric-value-new audit-outcome-value-new">{auditOutcome}</span>
        </div>
    </div>
</div>

<style lang="scss">
    @use '../../../styles/variables' as *;

    .comparison-card-new {
        background-color: var(--background-offset-light, #fdfdfd);
        border: 1px solid var(--border-color-light, #eee);
        border-radius: var(--border-radius-xl, 16px); 
        padding: var(--spacing-lg, 1.5rem); 
        box-shadow: var(--box-shadow-md); 
        width: 320px; 
        flex-shrink: 0;
        transition: all var(--transition-duration) ease-in-out;
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md, 1rem); 

        &:hover {
            transform: translateY(-5px) scale(1.02);
            box-shadow: var(--box-shadow-lg);
        }
    }

    .card-header-new {
        display: flex;
        justify-content: space-between;
        align-items: flex-start; 
        gap: var(--spacing-md);
    }

    .municipality-name-new {
        font-family: var(--font-family-headings, 'Ubuntu', sans-serif);
        font-size: var(--font-size-xl, 1.4rem); 
        font-weight: var(--font-weight-bold, 700);
        color: var(--text-heading); 
        margin: 0;
        line-height: 1.3;
        flex-grow: 1; 
    }

    .score-display-new {
        display: flex;
        flex-direction: column; 
        align-items: flex-end; 
        text-align: right;
        flex-shrink: 0; 
    }

    .score-value-new {
        font-family: var(--font-family-headings, 'Ubuntu', sans-serif);
        font-size: var(--font-size-xxl, 2.5rem); 
        font-weight: var(--font-weight-bold, 700);
        color: var(--text-heading); 
        line-height: 1;
        margin-bottom: -0.1em; 
    }

    .score-icon-new {
        font-size: var(--font-size-lg, 1.2rem);
    }

    .divider {
        border: none;
        height: 1px;
        background-color: var(--border-color, #e5e7eb);
        margin: 0; 
    }

    .card-body-new {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md, 1rem); 
    }

    .metric-row-new {
        display: flex;
        justify-content: space-between; 
        align-items: flex-start; 
        gap: var(--spacing-md);
        line-height: 1.4;
    }

    .metric-label-new {
        display: inline-flex; 
        align-items: center;
        gap: var(--spacing-sm, 0.5rem);
        font-size: var(--font-size-sm, 0.9rem);
        color: var(--text-secondary);
        flex-shrink: 0; 

        .iconify {
            font-size: 1.2rem;
            color: var(--accent-color-dark); 
            flex-shrink: 0;
            margin-top: -1px; 
        }
    }

    .metric-value-new {
        font-size: var(--font-size-md, 1rem); 
        font-weight: var(--font-weight-semibold, 600);
        color: var(--text-primary);
        text-align: right;
        white-space: normal; 
        word-break: break-word; 
    }

    .audit-row {
         align-items: baseline; 
    }

    .audit-outcome-value-new {
        font-size: var(--font-size-sm, 0.9rem);
    }

</style>