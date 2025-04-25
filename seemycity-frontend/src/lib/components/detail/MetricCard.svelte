<script lang="ts">
  import Icon from '@iconify/svelte';

  // Props for the card content
  export let icon: string; // Iconify icon name (e.g., 'mdi:cash')
  export let label: string;
  export let value: string | number | undefined | null; // Can be string, number, or N/A
  export let valueColorStyle: string = ''; // Optional style string for value color (e.g., for audit outcome)

  // Format value for display
  $: displayValue = value === null || value === undefined ? 'N/A' : value;
</script>

<div class="metric-card">
  <Icon icon={icon} class="metric-icon" />
  <div class="metric-label">{label}</div>
  <div class="metric-value" style={valueColorStyle}>{displayValue}</div>
</div>

<style lang="scss">
  .metric-card {
    background-color: var(--background-offset-color);
    padding: var(--spacing-md);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    text-align: center;
    box-shadow: var(--box-shadow-sm);
    transition: transform var(--transition-duration) ease-in-out,
                box-shadow var(--transition-duration) ease-in-out;

    &:hover {
        transform: translateY(-3px); // Subtle lift effect
        box-shadow: var(--box-shadow-md); // Slightly larger shadow
    }
  }

  .metric-icon {
    font-size: 2.5rem;
    margin-bottom: var(--spacing-sm);
    color: var(--primary-color); // Use primary color for icons (can be overridden if needed)
  }

  .metric-label {
    font-size: var(--font-size-sm);
    margin-bottom: var(--spacing-xs);
    color: var(--text-muted-color);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .metric-value {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-medium);
    color: var(--text-color); // Default text color, overridden by valueColorStyle if provided
  }
</style>