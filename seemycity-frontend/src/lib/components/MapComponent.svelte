<script lang="ts">
	import { onMount, createEventDispatcher, afterUpdate } from 'svelte';
	import maplibregl from 'maplibre-gl';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import type { Map, GeoJSONSource } from 'maplibre-gl';
	import type { FeatureCollection } from 'geojson';

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

	function addDataLayers(mapInstance: Map) {
		if (!mapInstance) return;

		// Score palette comes from _variables.scss; thresholds mirror
		// getScoreColorVarName (>=70 high, >=40 medium, <40 low).
		const scoreLow = token('--score-low-color', '#dc2626');
		const scoreMedium = token('--score-medium-color', '#d97706');
		const scoreHigh = token('--score-high-color', '#059669');
		const scoreHighStrong = token('--score-high-strong-color', '#047857');
		const scoreNone = token('--score-none-color', '#d6d3d1');

		mapInstance.addSource('municipalities', {
			type: 'geojson',
			data: geojson || { type: 'FeatureCollection', features: [] }
		});

		// Add a layer for the municipality fills with data-driven styling for color.
		mapInstance.addLayer({
			id: 'municipalities-fill',
			type: 'fill',
			source: 'municipalities',
			paint: {
				'fill-color': [
					'case',
					// A null score means "no data" — render grey, distinct from a low score.
					// Scores are always 0-100, so coalescing missing/null to -1 lets a
					// simple >= 0 test separate "has data" from "no data".
					['>=', ['coalesce', ['get', 'overall_score'], -1], 0],
					[
						'interpolate',
						['linear'],
						['coalesce', ['get', 'overall_score'], 0],
						0,
						scoreLow,
						40,
						scoreMedium,
						70,
						scoreHigh,
						100,
						scoreHighStrong
					],
					scoreNone
				],
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

		// Change the cursor to a pointer when hovering over the municipalities
		mapInstance.on('mouseenter', 'municipalities-fill', () => {
			mapInstance.getCanvas().style.cursor = 'pointer';
		});

		mapInstance.on('mouseleave', 'municipalities-fill', () => {
			mapInstance.getCanvas().style.cursor = '';
		});
	}

	onMount(() => {
		const apiKey = import.meta.env.VITE_MAPTILER_API_KEY;
		if (!apiKey) {
			console.warn(
				'VITE_MAPTILER_API_KEY is not set — falling back to the keyless MapLibre demo basemap.'
			);
		}
		// Without a valid style the map never fires `load` and the choropleth layers
		// are never added, so an explicit keyless fallback keeps the app functional.
		const styleUrl = apiKey
			? `https://api.maptiler.com/maps/dataviz/style.json?key=${apiKey}`
			: 'https://demotiles.maplibre.org/style.json';

		map = new maplibregl.Map({
			container: mapContainer,
			style: styleUrl,
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
</style>
