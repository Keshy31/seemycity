<script lang="ts">
    import type { MunicipalityDetail } from '$lib/types'; // Use the type defined in $lib/types
    import {
        formatScore,
        getScoreStatusIcon,
        formatCurrency,
        formatPercentage,
        formatPopulation // Needed for Rev per Capita calc
    } from '$lib/utils/formatUtils';
    import Icon from '@iconify/svelte';

    export let municipality: MunicipalityDetail; // Update prop type

    // Helper to get the latest financial record (assuming financials array is sorted descending by year)
    $: latestFinancials = municipality?.financials?.[0];

    // --- Calculate derived metrics for display ---

    // Revenue per Capita
    $: revenuePerCapita = (() => {
        const revenue = latestFinancials?.revenue;
        const population = municipality?.population;
        if (revenue == null || population == null || population <= 0) return 'N/A';
        const perCapita = revenue / population;
        // Format as currency without cents
        return new Intl.NumberFormat('en-ZA', {
            style: 'currency',
            currency: 'ZAR',
            minimumFractionDigits: 0,
            maximumFractionDigits: 0
        }).format(perCapita);
    })();

    // Total Expenditure (OpEx + CapEx) - handle nulls
    $: totalExpenditure =
        (latestFinancials?.capital_expenditure ?? 0) +
        (latestFinancials?.operational_expenditure ?? 0);

    // Operational Spend % (OpEx / Total Expenditure)
    $: opexPercentage = formatPercentage(
        latestFinancials?.operational_expenditure,
        totalExpenditure // Use calculated total expenditure
    );

    // CapEx Ratio % (CapEx / Total Expenditure)
    $: capexPercentage = formatPercentage(
        latestFinancials?.capital_expenditure,
        totalExpenditure // Use calculated total expenditure
    );

    // Audit Outcome - Use directly if available
    $: auditOutcome = latestFinancials?.audit_outcome ?? 'N/A';
</script>

<div class="comparison-card">
    <div class="card-header">
        <h3 class="municipality-name">{municipality.name}</h3>
        <div class="score-display">
            <span class="score-value">{formatScore(latestFinancials?.overall_score)}</span>
            <span class="score-icon">{getScoreStatusIcon(latestFinancials?.overall_score)}</span>
        </div>
    </div>

    <div class="card-body">
        <div class="metric-row">
            <Icon icon="mdi:cash-multiple" class="metric-icon" />
            <span class="metric-label">Revenue / Capita</span>
            <span class="metric-value">{revenuePerCapita}</span>
        </div>
        <div class="metric-row">
            <Icon icon="mdi:chart-pie-outline" class="metric-icon" />
            <span class="metric-label">OpEx Spend %</span>
            <span class="metric-value">{opexPercentage}</span>
        </div>
        <div class="metric-row">
            <Icon icon="mdi:chart-gantt" class="metric-icon" />
            <span class="metric-label">CapEx Ratio %</span>
            <span class="metric-value">{capexPercentage}</span>
        </div>
        <div class="metric-row">
            <Icon icon="mdi:clipboard-check-outline" class="metric-icon" />
            <span class="metric-label">Audit Outcome</span>
            <span class="metric-value">{auditOutcome}</span>
        </div>
    </div>
</div>

<style lang="scss">
    @use '../../../styles/variables' as *;

    .comparison-card {
        background-color: var(--background-offset-light, #fdfdfd);
        border: 1px solid var(--border-color, #e5e7eb);
        border-radius: var(--border-radius-lg, 12px);
        padding: var(--spacing-md, 1rem);
        box-shadow: var(--box-shadow-sm);
        width: 280px; // Fixed width for consistency in the row
        flex-shrink: 0; // Prevent cards from shrinking
        transition: all var(--transition-duration) ease-in-out;

        &:hover {
            transform: translateY(-4px);
            box-shadow: var(--box-shadow-lg);
        }
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start; // Align items to the top when name wraps
        border-bottom: 1px solid var(--border-color-light, #eee);
        padding-bottom: var(--spacing-sm, 0.75rem);
        margin-bottom: var(--spacing-md, 1rem);
    }

    .municipality-name {
        font-size: var(--font-size-lg, 1.25rem);
        font-weight: var(--font-weight-semibold, 600);
        color: var(--text-primary);
        margin: 0;
        // Allow name to wrap
    }

    .score-display {
        display: flex;
        align-items: center; // Center score number and icon vertically
        gap: var(--spacing-xs, 0.25rem);
    }

    .score-value {
        font-size: var(--font-size-xl, 1.5rem);
        font-weight: var(--font-weight-bold, 700);
        color: var(--text-primary); // Color applied via icon instead? Or keep separate?
    }

    .score-icon {
        font-size: var(--font-size-lg, 1.25rem);
    }

    .card-body {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md, 1rem); // Space between metric rows
    }

    .metric-row {
        display: flex;
        align-items: flex-start; // Align icon/label/value to top when value wraps
        gap: var(--spacing-sm, 0.75rem);
    }

    .metric-icon {
        font-size: 1.4rem;
        color: var(--accent-color-dark); // Use accent color for icons
        flex-shrink: 0;
    }

    .metric-label {
        flex-grow: 1; // Take up remaining space
        font-size: var(--font-size-sm, 0.9rem);
        color: var(--text-secondary);
        text-align: left;
    }

    .metric-value {
        font-size: var(--font-size-md, 1rem);
        font-weight: var(--font-weight-semibold, 600); // Make value slightly bolder
        color: var(--text-primary);
        text-align: right;
    }
</style>