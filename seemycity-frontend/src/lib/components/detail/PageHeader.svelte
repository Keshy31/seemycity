<script lang="ts">
  import { getScoreColorStyle } from '$lib/utils/colorUtils'; // Helper for score color

  // Props received from the parent page
  export let municipalityName: string;
  export let provinceName: string | undefined;
  export let classification: string | undefined;
  export let overallScore: number | undefined;
  export let financialYear: string | undefined;

  // Determine score color style - we pass the score, not the function
  $: scoreStyle = overallScore !== undefined ? getScoreColorStyle(overallScore) : '';
</script>

<div class="header-section">
  <div class="header-info">
    <h1 class="municipality-name">{municipalityName}</h1>
    {#if provinceName}
      <p class="province-name">{provinceName}</p>
    {/if}
    {#if classification}
      <p class="classification">{classification}</p>
    {/if}
  </div>

  {#if overallScore !== undefined && financialYear}
    <div class="score-section">
      <span class="score-label">Overall Score</span>
      <span class="overall-score" style={scoreStyle}>{overallScore.toFixed(0)}</span>
      <p class="financial-year">({financialYear})</p>
    </div>
  {/if}
</div>

<style lang="scss">
  /* Styles moved from [id]/+page.svelte */
  .header-section {
    display: flex;
    flex-wrap: wrap; // Allow wrapping on small screens
    justify-content: space-between;
    align-items: flex-start; // Align items to the top
    gap: var(--spacing-md); // Gap between items
    padding: var(--spacing-lg);
    margin-bottom: var(--spacing-xl); // Larger bottom margin
    background-color: var(--background-offset-color); // Use offset background
    border-radius: var(--border-radius-md);
    box-shadow: var(--box-shadow-sm);
  }

  .header-info {
    flex-grow: 1; // Allow text info to take available space
  }

  .municipality-name {
    margin-bottom: var(--spacing-xs); // Less margin below h1
    // Base h1 styles already applied globally via _typography.scss
    // Re-apply any specific overrides if needed, but base should cover it.
    // Example: font-size: var(--font-size-h1);
  }

  .province-name,
  .classification {
    font-size: var(--font-size-lg);
    color: var(--text-muted-color);
    margin-bottom: var(--spacing-sm);
  }

  .score-section {
    text-align: right; // Align score to the right
    min-width: 150px; // Ensure score section has some width
    flex-shrink: 0; // Prevent shrinking too much on wrap
  }

  .score-label {
    display: block; // Ensure label is on its own line
    font-size: var(--font-size-sm);
    color: var(--text-muted-color);
    margin-bottom: var(--spacing-xs);
  }

  .overall-score {
    font-size: 2.8rem; // Larger score font size
    font-weight: var(--font-weight-bold);
    line-height: 1.1;
    // Color is applied via inline style prop
  }

  .financial-year {
    font-size: var(--font-size-sm);
    color: var(--text-muted-color);
    margin-top: var(--spacing-xs);
  }
</style>