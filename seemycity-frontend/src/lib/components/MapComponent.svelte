<script lang="ts">
	import { onMount, createEventDispatcher, afterUpdate } from 'svelte';
	import maplibregl from 'maplibre-gl';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import type { Map, GeoJSONSource, ExpressionSpecification } from 'maplibre-gl';
	import type { FeatureCollection } from 'geojson';
	import { SCORE_BANDS, NO_DATA_BAND, bandForScore } from '$lib/utils/scoreBands';

	export let geojson: FeatureCollection | null = null;

	const dispatch = createEventDispatcher();

	let mapContainer: HTMLElement;
	let map: Map;
	let isMapLoaded = false;

	/** Reads a design token off :root so the map ramp always matches the UI. */
	function token(name: string, fallback: string): string {
		const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
		return value || fallback;
	}

	/**
	 * Quantized 5-band fill: a `step` expression whose thresholds and colors come
	 * from SCORE_BANDS, so the map can never drift from the legend.
	 */
	function bandFillExpression(): ExpressionSpecification {
		// step wants: base color, then (threshold, color) pairs ascending.
		const worstFirst = [...SCORE_BANDS].reverse();
		const steps: (string | number)[] = [];
		for (const band of worstFirst.slice(1)) {
			steps.push(band.min, token(band.cssVar, '#888888'));
		}
		return [
			'case',
			// A null score means "no data" — render grey, distinct from a low score.
			// Scores are always 0-100, so coalescing missing/null to -1 lets a
			// simple >= 0 test separate "has data" from "no data".
			['>=', ['coalesce', ['get', 'overall_score'], -1], 0],
			[
				'step',
				['coalesce', ['get', 'overall_score'], 0],
				token(worstFirst[0].cssVar, '#888888'),
				...steps
			],
			token(NO_DATA_BAND.cssVar, '#d6d3d1')
		] as ExpressionSpecification;
	}

	/** Builds tooltip DOM with textContent only — feature names never touch innerHTML. */
	function tooltipContent(props: { name?: string; overall_score?: number | null }): HTMLElement {
		const root = document.createElement('div');
		root.className = 'map-tooltip';

		const name = document.createElement('div');
		name.className = 'map-tooltip-name';
		name.textContent = props.name ?? 'Unknown';
		root.appendChild(name);

		const score = props.overall_score ?? null;
		const band = bandForScore(score);

		const verdictRow = document.createElement('div');
		verdictRow.className = 'map-tooltip-band';
		const swatch = document.createElement('span');
		swatch.className = 'map-tooltip-swatch';
		swatch.style.backgroundColor = token((band ?? NO_DATA_BAND).cssVar, '#d6d3d1');
		const bandText = document.createElement('span');
		bandText.textContent = band
			? `${Math.round(score!)} / 100 · ${band.label}`
			: NO_DATA_BAND.label;
		verdictRow.append(swatch, bandText);
		root.appendChild(verdictRow);

		const verdict = document.createElement('div');
		verdict.className = 'map-tooltip-verdict';
		verdict.textContent = (band ?? NO_DATA_BAND).verdict;
		root.appendChild(verdict);

		const hint = document.createElement('div');
		hint.className = 'map-tooltip-hint';
		hint.textContent = 'Click for the full diagnosis';
		root.appendChild(hint);

		return root;
	}

	function addDataLayers(mapInstance: Map) {
		if (!mapInstance) return;

		mapInstance.addSource('municipalities', {
			type: 'geojson',
			data: geojson || { type: 'FeatureCollection', features: [] }
		});

		// Municipality fills, quantized to the 5 score bands.
		mapInstance.addLayer({
			id: 'municipalities-fill',
			type: 'fill',
			source: 'municipalities',
			paint: {
				'fill-color': bandFillExpression(),
				'fill-opacity': 0.75,
				'fill-outline-color': 'rgba(28, 25, 23, 0.15)'
			}
		});

		// Add a layer for the outlines
		mapInstance.addLayer({
			id: 'municipalities-outline',
			type: 'line',
			source: 'municipalities',
			paint: {
				'line-color': '#ffffff',
				'line-width': 1,
				'line-opacity': 0.6
			}
		});

		// Handle clicks on the municipalities layer
		mapInstance.on('click', 'municipalities-fill', (e) => {
			if (e.features && e.features.length > 0) {
				const feature = e.features[0];
				const muniId = feature.properties.id;
				if (muniId) {
					dispatch('municipalityClick', { id: muniId });
				}
			}
		});

		// Hover tooltip: one popup instance, repositioned as the pointer moves.
		const popup = new maplibregl.Popup({
			closeButton: false,
			closeOnClick: false,
			maxWidth: '260px',
			offset: 12
		});

		mapInstance.on('mousemove', 'municipalities-fill', (e) => {
			if (!e.features?.length) return;
			mapInstance.getCanvas().style.cursor = 'pointer';
			popup
				.setLngLat(e.lngLat)
				.setDOMContent(tooltipContent(e.features[0].properties))
				.addTo(mapInstance);
		});

		mapInstance.on('mouseleave', 'municipalities-fill', () => {
			mapInstance.getCanvas().style.cursor = '';
			popup.remove();
		});
	}

	onMount(() => {
		const apiKey = import.meta.env.VITE_MAPTILER_API_KEY;
		if (!apiKey) {
			console.warn(
				'VITE_MAPTILER_API_KEY is not set — using a plain background instead of a basemap.'
			);
		}
		// Without a valid style the map never fires `load` and the choropleth
		// layers are never added. The keyless fallback is an inline style (no
		// network at all): the choropleth needs no basemap to be useful, and an
		// external fallback URL just trades one availability dependency for another.
		const style = apiKey
			? `https://api.maptiler.com/maps/dataviz/style.json?key=${apiKey}`
			: {
					version: 8 as const,
					sources: {},
					layers: [
						{
							id: 'background',
							type: 'background' as const,
							paint: { 'background-color': token('--background-offset-color', '#f5f5f4') }
						}
					]
				};

		map = new maplibregl.Map({
			container: mapContainer,
			style,
			center: [24.5, -28.8],
			zoom: 4.5
		});

		map.addControl(new maplibregl.NavigationControl({}), 'top-right');

		if (import.meta.env.DEV) {
			// Dev-only handle for debugging map state from the console.
			(window as unknown as Record<string, unknown>).__seemycityMap = map;
		}

		map.on('load', () => {
			isMapLoaded = true;
			addDataLayers(map);
		});

		return () => {
			if (map) map.remove();
		};
	});

	// This ensures that if the geojson data arrives after the map has loaded,
	// the map source is updated correctly. Only push to the source when the
	// prop actually changed — afterUpdate fires on every component update.
	let lastAppliedGeojson: FeatureCollection | null = null;
	afterUpdate(() => {
		if (isMapLoaded && map && geojson !== lastAppliedGeojson) {
			const source = map.getSource('municipalities') as GeoJSONSource;
			if (source) {
				source.setData(geojson || { type: 'FeatureCollection', features: [] });
				lastAppliedGeojson = geojson;
			}
		}
	});
</script>

<div class="map-container-full" bind:this={mapContainer}></div>

<style lang="scss">
	.map-container-full {
		width: 100%;
		height: 100%;
		position: absolute;
		top: 0;
		left: 0;
	}

	// Popup DOM is created by MapLibre outside Svelte's scope.
	:global(.maplibregl-popup-content) {
		padding: var(--spacing-sm) var(--spacing-md);
		border-radius: var(--border-radius-md);
		box-shadow: var(--shadow-lg);
		font-family: var(--font-family-base);
	}

	:global(.map-tooltip) {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	:global(.map-tooltip-name) {
		font-weight: 600;
		color: var(--text-heading-color);
		font-size: var(--font-size-sm);
	}

	:global(.map-tooltip-band) {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: var(--font-size-sm);
		color: var(--text-color);
		font-variant-numeric: tabular-nums;
	}

	:global(.map-tooltip-swatch) {
		width: 10px;
		height: 10px;
		border-radius: 2px;
		flex-shrink: 0;
	}

	:global(.map-tooltip-verdict) {
		font-size: var(--font-size-xs);
		color: var(--text-muted-color);
		line-height: 1.35;
	}

	:global(.map-tooltip-hint) {
		margin-top: 4px;
		font-size: var(--font-size-xs);
		color: var(--primary-color);
		font-weight: 500;
	}
</style>
