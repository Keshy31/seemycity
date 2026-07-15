<script lang="ts">
	import type { MunicipalityDetail } from '$lib/types';
	import ComparisonCard from './ComparisonCard.svelte';
	import Icon from '@iconify/svelte';

	// Accept the array of municipalities to compare
	export let municipalities: MunicipalityDetail[];
</script>

<div class="comparison-container">
	<div class="header-row">
		<h1 class="page-title">Municipality Comparison</h1>
		<!-- Potential placeholder for future actions like 'Add Municipality' -->
	</div>

	{#if municipalities && municipalities.length > 0}
		<div class="cards-wrapper">
			{#each municipalities as muni (muni.id)}
				<div class="card-snap-container">
					<ComparisonCard municipality={muni} />
				</div>
			{/each}
		</div>
	{:else}
		<div class="empty-state">
			<Icon icon="mdi:plus-box-multiple-outline" />
			<h2>Add Municipalities to Compare</h2>
			<p>Use the search or map to find and add municipalities to your comparison list.</p>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.comparison-container {
		padding: var(--spacing-lg);
		max-width: 100%;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: var(--spacing-lg);
	}

	.header-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 var(--spacing-md);
	}

	.page-title {
		font-family: var(--font-family-headings);
		font-size: var(--font-size-h2);
		font-weight: 700;
		color: var(--text-heading-color);
		margin: 0;
	}

	.cards-wrapper {
		display: flex;
		gap: var(--spacing-lg);
		overflow-x: auto;
		padding: 1rem; // Add padding to see shadows and for scrollbar space
		margin: -1rem; // Negative margin to counteract padding for alignment

		/* Scroll Snapping */
		scroll-snap-type: x mandatory;
		scroll-padding: 1rem; // Match padding

		/* Hide scrollbar visually but keep functionality on hover */
		&::-webkit-scrollbar {
			height: 8px;
		}
		&::-webkit-scrollbar-track {
			background: transparent;
		}
		&::-webkit-scrollbar-thumb {
			background-color: var(--border-color-light);
			border-radius: 10px;
			border: 2px solid var(--background-color);
		}

		/* Fade-out effect on the edges to indicate scrollability */
		-webkit-mask-image: linear-gradient(
			90deg,
			transparent 0%,
			black 5%,
			black 95%,
			transparent 100%
		);
		mask-image: linear-gradient(90deg, transparent 0%, black 5%, black 95%, transparent 100%);
	}

	.card-snap-container {
		scroll-snap-align: start;
		flex: 0 0 auto; // Prevent cards from shrinking
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: var(--spacing-xxl) var(--spacing-lg);
		border: 2px dashed var(--border-color);
		border-radius: var(--border-radius-lg);
		background-color: var(--background-offset-color);
		color: var(--text-muted-color);
		min-height: 400px;

		:global(.iconify) {
			font-size: 4rem;
			margin-bottom: var(--spacing-lg);
			color: var(--primary-color-light);
		}

		h2 {
			font-size: var(--font-size-xl);
			color: var(--text-color);
			margin: 0 0 var(--spacing-sm) 0;
		}

		p {
			max-width: 400px;
			margin: 0;
		}
	}
</style>
