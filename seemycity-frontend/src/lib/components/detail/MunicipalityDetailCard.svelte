<script lang="ts">
	import type { MunicipalityDetail } from '$lib/types';
	import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';

	export let id: string;

	let details: MunicipalityDetail | null = null;
	let isLoading = true;
	let error: string | null = null;
	// Guards against out-of-order responses when the user clicks rapidly on the map.
	let requestSeq = 0;

	async function fetchDetails(muniId: string) {
		if (!muniId) return;
		const seq = ++requestSeq;
		isLoading = true;
		error = null;
		details = null;

		try {
			const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000';
			const response = await fetch(`${apiUrl}/api/municipalities/${muniId}`);
			if (seq !== requestSeq) return; // a newer request superseded this one

			if (!response.ok) {
				throw new Error(`Failed to fetch data for ${muniId}`);
			}
			details = await response.json();
		} catch (e: unknown) {
			if (seq !== requestSeq) return;
			error = e instanceof Error ? e.message : String(e);
		} finally {
			if (seq === requestSeq) {
				isLoading = false;
			}
		}
	}

	// Fetch data whenever the id prop changes
	$: if (id) {
		fetchDetails(id);
	}
</script>

<div class="detail-card-wrapper">
	{#if isLoading}
		<LoadingSpinner />
	{:else if error}
		<ErrorMessage message={error} />
	{:else if details}
		<div class="detail-card">
			<div class="header">
				<h3 class="name">{details.name}</h3>
				<span class="id-chip">{details.id}</span>
			</div>
			<div class="stats">
				<div class="stat-item">
					<span class="label">Population</span>
					<span class="value"
						>{details.population ? details.population.toLocaleString() : 'N/A'}</span
					>
				</div>
				<div class="stat-item">
					<span class="label">Province</span>
					<span class="value">{details.province ?? 'N/A'}</span>
				</div>
			</div>
			<a href={`/${details.id}`} class="view-more-link">View Full Details &rarr;</a>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.detail-card-wrapper {
		padding: var(--spacing-sm) 0;
	}

	.detail-card {
		background-color: var(--background-offset-color);
		border: 1px solid var(--border-color-light);
		border-radius: var(--border-radius-lg);
		padding: var(--spacing-lg);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-lg);
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: var(--spacing-md);
	}

	.name {
		font-size: var(--font-size-lg);
		font-weight: 700;
		color: var(--text-heading-color);
		margin: 0;
		line-height: 1.2;
	}

	.id-chip {
		font-family: var(--font-family-mono);
		font-size: var(--font-size-xs);
		color: var(--text-muted-color);
		background-color: var(--background-color);
		padding: 4px 8px;
		border-radius: var(--border-radius-sm);
		border: 1px solid var(--border-color);
		white-space: nowrap;
	}

	.stats {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--spacing-md);
		border-top: 1px solid var(--border-color-light);
		border-bottom: 1px solid var(--border-color-light);
		padding: var(--spacing-md) 0;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.label {
		font-size: var(--font-size-sm);
		color: var(--text-muted-color);
		text-transform: uppercase;
	}

	.value {
		font-size: var(--font-size-md);
		font-weight: 500;
		color: var(--text-color);
	}

	.view-more-link {
		align-self: flex-start;
		text-decoration: none;
		color: var(--primary-color);
		font-weight: 600;
		transition: color 0.2s ease;

		&:hover {
			color: var(--primary-color-dark);
		}
	}
</style>
