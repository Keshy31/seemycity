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
	<Icon {icon} class="metric-icon" />
	<div class="metric-label">{label}</div>
	<div class="metric-value" style={valueColorStyle}>{displayValue}</div>
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.metric-card {
		background-color: var(--background-offset-light);
		padding: var(--spacing-lg);
		border: 1px solid var(--border-color-light);
		border-radius: var(--border-radius-xl);
		text-align: center;
		box-shadow: var(--box-shadow-md);
		transition:
			transform 0.2s ease-in-out,
			box-shadow 0.2s ease-in-out;

		&:hover {
			transform: translateY(-4px);
			box-shadow: var(--box-shadow-lg);
		}
	}

	.metric-icon {
		font-size: 2rem; // Slightly smaller for a cleaner look
		margin-bottom: var(--spacing-sm);
		color: var(--color-accent); // Use the main accent color (teal)
	}

	.metric-label {
		font-size: var(--font-size-xs); // Smaller for better hierarchy
		margin-bottom: var(--spacing-xs);
		color: var(--text-color-muted);
		text-transform: uppercase;
		letter-spacing: 0.08em; // Increased spacing for clarity
		font-weight: var(--font-weight-bold);
	}

	.metric-value {
		font-size: var(--font-size-xl);
		font-weight: var(--font-weight-medium);
		color: var(
			--text-color-default
		); // Default text color, overridden by valueColorStyle if provided
		line-height: 1.2;
	}
</style>
