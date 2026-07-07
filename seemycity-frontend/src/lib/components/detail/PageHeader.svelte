<script lang="ts">
  // Corrected import path and added new formatters
  import {
    getScoreColorStyle,
    getScoreStatusIcon,
    formatPopulation
  } from '$lib/utils/formatUtils';
  import Icon from '@iconify/svelte';

  // Props received from the parent page, updated to match ux.md.
  // Score and population can be null from the API (missing data ≠ zero).
  export let municipalityName: string;
  export let provinceName: string | null | undefined = undefined;
  export let population: number | null | undefined = undefined;
  export let websiteUrl: string | null | undefined = undefined;
  export let overallScore: number | null | undefined = undefined;
  export let financialYear: string | number | null | undefined = undefined;

  $: scoreStyle = getScoreColorStyle(overallScore);
  $: scoreIcon = getScoreStatusIcon(overallScore);
</script>

<header class="page-header">
  <div class="header-main-row">
    <h1 class="municipality-name">{municipalityName}</h1>
    <div class="header-actions">
      <div class="score-display">
        {#if overallScore != null}
          <span class="score-value" style={scoreStyle}>{overallScore.toFixed(0)}</span>
          <span class="score-label">/ 100</span>
        {:else}
          <span class="score-label">Insufficient data</span>
        {/if}
        <span class="score-status-icon">
          <Icon icon={scoreIcon} style={scoreStyle} />
        </span>
      </div>
      {#if websiteUrl}
        <a href={websiteUrl} target="_blank" rel="noopener noreferrer" class="website-link">
          <span>Website</span>
          <Icon icon="mdi:open-in-new" />
        </a>
      {/if}
    </div>
  </div>

  <div class="header-sub-row">
    {#if provinceName}
      <span class="sub-item">Province: <strong>{provinceName}</strong></span>
    {/if}
    {#if population}
      <span class="sub-item">Population: <strong>{formatPopulation(population)}</strong></span>
    {/if}
    {#if financialYear}
      <span class="sub-item">Financials: <strong>{financialYear}</strong></span>
    {/if}
  </div>
</header>

<style lang="scss">
  @use '../../../styles/variables' as *;

  .page-header {
    padding: var(--spacing-lg) var(--spacing-xl);
    background-color: var(--background-offset-light);
    border-bottom: 1px solid var(--border-color);
    margin-bottom: var(--spacing-xl);
  }

  .header-main-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--spacing-lg);
    flex-wrap: wrap;
  }

  .municipality-name {
    font-family: var(--font-family-heading);
    font-size: 2.25rem; // h2 size
    font-weight: var(--font-weight-bold);
    color: var(--text-color);
    margin: 0;
    line-height: 1.2;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--spacing-xl);
    flex-wrap: wrap;
  }

  .score-display {
    display: flex;
    align-items: baseline;
    gap: var(--spacing-xs);
    animation: pulse-in 0.5s 0.2s ease-out backwards;
  }

  .score-value {
    font-size: 2.5rem;
    font-weight: var(--font-weight-bold);
    line-height: 1;
  }

  .score-label {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-light);
    color: var(--text-color-muted);
    padding-left: 0.1em;
  }

  .score-status-icon {
    font-size: 2rem;
    line-height: 1;
    margin-left: var(--spacing-sm);
    display: inline-flex;
    align-items: center;
  }

  .website-link {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm);
    background-color: var(--primary-color);
    color: var(--text-inverse-color);
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--border-radius-md);
    text-decoration: none;
    font-weight: var(--font-weight-medium);
    transition: background-color 0.2s ease;

    &:hover {
      background-color: var(--accent-color-hover);
    }
  }

  .header-sub-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--spacing-lg);
    margin-top: var(--spacing-md);
    font-size: var(--font-size-sm);
    color: var(--text-color-muted);
  }

  .sub-item strong {
    color: var(--text-color);
    font-weight: var(--font-weight-medium);
  }

  @keyframes pulse-in {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
</style>