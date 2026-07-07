<script lang="ts">
	import Icon from '@iconify/svelte';

	// The text entered into the search bar
	export let value: string = '';

	// The placeholder text to display
	export let placeholder: string = 'Search by name or code...';
</script>

<div class="search-wrapper">
	<Icon icon="mdi:magnify" class="search-icon" />
	<input
		type="search"
		class="search-input"
		aria-label="Search municipalities by name or code"
		{placeholder}
		bind:value
		on:input
		on:keydown
	/>
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.search-wrapper {
		position: relative;
		display: flex;
		align-items: center;
		background-color: var(--background-offset-color);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius-md);
		padding: 0 var(--spacing-md);
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;

		&:focus-within {
			border-color: var(--primary-color);
			box-shadow: 0 0 0 3px var(--primary-color-translucent);
		}
	}

	// :global() because the Icon component's <svg> is outside Svelte's scoping.
	.search-wrapper :global(.search-icon) {
		font-size: 1.25rem;
		color: var(--text-muted-color);
		margin-right: var(--spacing-sm);
		flex-shrink: 0;
	}

	.search-input {
		width: 100%;
		border: none;
		background: none;
		padding: var(--spacing-sm) 0;
		font-family: var(--font-family-sans-serif);
		font-size: var(--font-size-base);
		color: var(--text-color);
		outline: none;

		&::placeholder {
			color: var(--text-muted-color);
		}

		// Remove the default clear button (X) in WebKit browsers
		&::-webkit-search-decoration,
		&::-webkit-search-cancel-button,
		&::-webkit-search-results-button,
		&::-webkit-search-results-decoration {
			-webkit-appearance: none;
		}
	}
</style>
