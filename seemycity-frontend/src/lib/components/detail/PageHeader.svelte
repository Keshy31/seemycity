<script lang="ts">
  import { getScoreColorStyle, getScoreStatusIcon } from '$lib/utils/colorUtils'; 
  import Icon from '@iconify/svelte'; 

  // Props received from the parent page
  export let municipalityName: string;
  export let provinceName: string | undefined;
  export let classification: string | undefined;
  export let overallScore: number | undefined;
  export let financialYear: string | undefined;

  // Determine score color style - we pass the score, not the function
  $: scoreStyle = overallScore !== undefined ? getScoreColorStyle(overallScore) : '';
  // Determine the status icon based on the score
  $: scoreIcon = overallScore !== undefined ? getScoreStatusIcon(overallScore) : 'mdi:help-circle-outline'; 
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
      <span class="score-label">Overall Score ({financialYear})</span>
      <div class="score-display"> 
        <span class="overall-score" style={scoreStyle}>{overallScore.toFixed(0)}</span>
        <Icon icon={scoreIcon} class="score-status-icon" style={scoreStyle} /> 
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  /* Styles moved from [id]/+page.svelte */
  .header-section {
    display: flex;
    flex-wrap: wrap; 
    justify-content: space-between;
    align-items: flex-start; 
    gap: var(--spacing-md); 
    padding: var(--spacing-lg);
    margin-bottom: var(--spacing-xl); 
    background-color: var(--background-offset-color); 
    border-radius: var(--border-radius-md);
    box-shadow: var(--box-shadow-sm);
  }

  .header-info {
    flex-grow: 1; 
  }

  .municipality-name {
    margin-bottom: var(--spacing-xs); 
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
    text-align: right; 
    min-width: 150px; 
    flex-shrink: 0; 
    display: flex; 
    flex-direction: column; 
    align-items: flex-end; 
  }

  .score-label {
    display: block; 
    font-size: var(--font-size-sm);
    color: var(--text-muted-color);
    margin-bottom: var(--spacing-xs);
  }

  .score-display { 
    display: flex;
    align-items: center; 
    gap: var(--spacing-sm); 
  }

  .overall-score {
    font-size: 2.8rem; 
    font-weight: var(--font-weight-bold);
    line-height: 1.1;
    // Color is applied via inline style prop
  }

  .score-status-icon { 
    font-size: 2.2rem; 
    // Color is applied via inline style prop to match score
    // Add animation styles later
  }

  // Keyframes for the pulse animation
  @keyframes pulse {
    0%, 100% {
      opacity: 0.8;
     }
   }
</style>