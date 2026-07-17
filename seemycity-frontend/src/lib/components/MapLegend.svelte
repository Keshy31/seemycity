<script lang="ts">
	import type { FeatureCollection } from 'geojson';
	import { SCORE_BANDS, NO_DATA_BAND, bandForScore } from '$lib/utils/scoreBands';

	export let geojson: FeatureCollection | null = null;

	// Count municipalities per band (+ the no-data bucket) from the live payload,
	// so the legend always agrees with what the map is actually showing.
	$: counts = (() => {
		const byLabel = new Map<string, number>(SCORE_BANDS.map((b) => [b.label, 0]));
		let noData = 0;
		for (const feature of geojson?.features ?? []) {
			const score = (feature.properties as { overall_score?: number | null })?.overall_score;
			const band = bandForScore(score);
			if (band) byLabel.set(band.label, (byLabel.get(band.label) ?? 0) + 1);
			else noData++;
		}
		return { byLabel, noData };
	})();
</script>

<div class="legend" role="group" aria-label="Score bands">
	<div class="legend-title">Overall score</div>
	<ul>
		{#each SCORE_BANDS as band (band.label)}
			<li>
				<span class="swatch" style="background-color: var({band.cssVar})"></span>
				<span class="label">{band.label}</span>
				<span class="range">{band.min}–{band.max}</span>
				<span class="count">{counts.byLabel.get(band.label) ?? 0}</span>
			</li>
		{/each}
		<li class="no-data">
			<span class="swatch" style="background-color: var({NO_DATA_BAND.cssVar})"></span>
			<span class="label">{NO_DATA_BAND.label}</span>
			<span class="range"></span>
			<span class="count">{counts.noData}</span>
		</li>
	</ul>
</div>

<style lang="scss">
	.legend {
		position: absolute;
		bottom: var(--spacing-lg);
		left: var(--spacing-lg);
		z-index: 10;
		background-color: color-mix(in srgb, var(--surface-color) 92%, transparent);
		backdrop-filter: blur(4px);
		border: 1px solid var(--border-color-light);
		border-radius: var(--border-radius-lg);
		padding: var(--spacing-sm) var(--spacing-md);
		box-shadow: var(--shadow-md);
		min-width: 200px;

		@media (max-width: 768px) {
			bottom: var(--spacing-sm);
			left: var(--spacing-sm);
			min-width: 170px;
			padding: var(--spacing-xs) var(--spacing-sm);
		}
	}

	.legend-title {
		font-size: var(--font-size-xs);
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-muted-color);
		margin-bottom: var(--spacing-xs);
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 3px;
	}

	li {
		display: grid;
		grid-template-columns: 12px 1fr auto auto;
		align-items: center;
		gap: var(--spacing-sm);
		font-size: var(--font-size-xs);
		line-height: 1.4;

		&.no-data {
			margin-top: 3px;
			padding-top: 5px;
			border-top: 1px dashed var(--border-color-light);
		}
	}

	.swatch {
		width: 12px;
		height: 12px;
		border-radius: 3px;
		border: 1px solid rgba(28, 25, 23, 0.12);
	}

	.label {
		color: var(--text-color);
		font-weight: 500;
	}

	.range {
		color: var(--text-muted-color);
		font-variant-numeric: tabular-nums;
	}

	.count {
		color: var(--text-heading-color);
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		min-width: 2ch;
		text-align: right;
	}
</style>
