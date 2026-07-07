<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { PageData } from './$types';
	import { invalidateAll } from '$app/navigation';
	import PageHeader from '$lib/components/detail/PageHeader.svelte';
	import KeyMetricsGrid from '$lib/components/detail/KeyMetricsGrid.svelte';
	import ScoreBreakdown from '$lib/components/detail/ScoreBreakdown.svelte';
	import { formatPopulation, formatWebsite } from '$lib/utils/formatUtils';

	export let data: PageData; // Resolved by the +page.ts load function before render

	// State for refresh button
	let isRefreshing = false;
	async function handleRefresh() {
		isRefreshing = true;
		await invalidateAll(); // Re-runs the load function
		isRefreshing = false;
	}
</script>

<svelte:head>
	<title>Details for {data.municipality.name} | SeeMyCity</title>
	<meta name="description" content={`Financial health details for ${data.municipality.name}.`} />
</svelte:head>

<div class="detail-container">
	<PageHeader
		municipalityName={data.municipality.name}
		overallScore={data.latestFinancials?.overall_score}
		financialYear={data.latestFinancials?.year}
		provinceName={data.municipality.province}
		population={data.municipality.population}
		websiteUrl={data.municipality.website}
	/>

	{#if data.latestFinancials}
		<KeyMetricsGrid financials={data.latestFinancials} population={data.municipality.population} />

		<ScoreBreakdown financials={data.latestFinancials} population={data.municipality.population} />
	{:else}
		<div class="no-data-message">
			<p>No financial data available for the latest year.</p>
		</div>
	{/if}

	<!-- About Section -->
	<div class="about-section">
		<h2 class="section-title">About {data.municipality.name}</h2>
		<dl class="about-grid">
			<div>
				<dt>Province:</dt>
				<dd>{data.municipality.province}</dd>
			</div>
			<div>
				<dt>Classification:</dt>
				<dd>{data.municipality.classification ?? 'N/A'}</dd>
			</div>
			<div>
				<dt>Population:</dt>
				<dd>{formatPopulation(data.municipality.population)}</dd>
			</div>
			<div>
				<dt>Website:</dt>
				{#if data.municipality.website}
					<a
						href={data.municipality.website}
						target="_blank"
						rel="noopener noreferrer"
						class="website-link"
					>
						{formatWebsite(data.municipality.website)}
						<Icon icon="mdi:external-link" class="external-link-icon" />
					</a>
				{:else}
					N/A
				{/if}
			</div>
		</dl>
	</div>

	<!-- Action Buttons -->
	<div class="action-buttons">
		<button class="button refresh-button" on:click={handleRefresh} disabled={isRefreshing}>
			<Icon icon={isRefreshing ? 'eos-icons:loading' : 'mdi:refresh'} />
			{isRefreshing ? 'Refreshing...' : 'Refresh Data'}
		</button>
	</div>
</div>

<style lang="scss">
	.detail-container {
		padding: 2rem;
		max-width: 1200px;
		margin: 2rem auto;
		background-color: var(--background-color);
		border-radius: var(--border-radius-large);
		box-shadow: var(--box-shadow-sm);
	}

	.about-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1rem 2rem; /* Row and column gap */
		background-color: var(--background-offset-light);
		padding: 1.5rem;
		border-radius: var(--border-radius-medium);
		margin-top: 1.5rem;

		dt {
			font-weight: 600;
			color: var(--text-secondary);
			margin-bottom: 0.25rem;
		}

		dd {
			margin-left: 0;
			color: var(--text-primary);
		}
	}

	.website-link {
		color: var(--accent-color);
		text-decoration: none;
		display: inline-flex;
		align-items: center;
		gap: 0.3rem;
		transition: color 0.2s ease;

		&:hover {
			color: var(--accent-color-hover);
			text-decoration: underline;
		}
	}

	.external-link-icon {
		font-size: 0.9em; /* Make icon slightly smaller */
		opacity: 0.8;
	}

	.action-buttons {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border-color);
	}

	.button {
		/* Basic button styles - consider moving to global? */
		padding: 0.6rem 1.2rem;
		border: none;
		border-radius: var(--border-radius-small);
		background-color: var(--accent-color);
		color: white;
		font-size: 0.95rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;

		&:hover {
			background-color: var(--accent-color-hover);
		}

		&:disabled {
			background-color: var(--accent-color-disabled);
			cursor: not-allowed;
		}
	}

	@media (max-width: 768px) {
		.detail-container {
			padding: 1rem;
			margin: 1rem;
		}

		.about-grid {
			grid-template-columns: 1fr; /* Stack on smaller screens */
		}

		.action-buttons {
			justify-content: center;
		}
	}
</style>
