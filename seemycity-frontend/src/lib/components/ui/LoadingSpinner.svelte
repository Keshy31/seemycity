<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="spinner-container" role="status" aria-live="polite">
	<Icon icon="mdi:loading" class="spinner-icon" />
	<span class="spinner-text">Loading...</span>
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.spinner-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-xl);
		gap: var(--spacing-md);
	}

	.spinner-text {
		color: var(--text-muted-color);
		font-size: var(--font-size-sm);
		font-weight: var(--font-weight-medium);
		letter-spacing: 0.05em;
	}

	// The Icon component renders an <svg> that Svelte's scoping can't reach —
	// target the class globally from within the scoped container.
	.spinner-container :global(.spinner-icon) {
		font-size: 2.5rem;
		color: var(--primary-color);
		animation: spin 0.9s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
