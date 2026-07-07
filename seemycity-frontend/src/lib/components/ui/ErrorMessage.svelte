<script lang="ts">
	import Icon from '@iconify/svelte';

	export let message: string = 'An error occurred while loading data.'; // Default message
	export let errorDetails: string | undefined = undefined; // Optional more detailed error
</script>

<div class="error-container" role="alert">
	<Icon icon="mdi:alert-circle-outline" class="error-icon" />
	<div class="error-text-content">
		<h4 class="error-title">{message}</h4>
		{#if errorDetails}
			<p class="error-details">{errorDetails}</p>
		{/if}
	</div>
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.error-container {
		display: flex;
		align-items: center;
		padding: var(--spacing-lg);
		text-align: left;
		background-color: var(--background-offset-light);
		border: 1px solid var(--error-color);
		border-radius: var(--border-radius-lg);
		gap: var(--spacing-md);
		color: var(--text-color-default);
	}

	.error-icon {
		font-size: 2rem;
		color: var(--error-color);
		flex-shrink: 0; // Prevent icon from shrinking
	}

	.error-text-content {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.error-title {
		margin: 0;
		color: var(--text-color-default);
		font-size: var(--font-size-base);
		font-weight: var(--font-weight-bold);
	}

	.error-details {
		margin: 0;
		font-size: var(--font-size-sm);
		color: var(--text-color-muted);
		max-width: 500px;
	}
</style>
