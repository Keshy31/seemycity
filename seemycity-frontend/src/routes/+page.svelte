<script lang="ts">
	// Revert to default import + destructuring as suggested by the Vite error for CJS compatibility
	import maplibregl from 'maplibre-gl';
	const { Map, NavigationControl } = maplibregl;
	import 'maplibre-gl/dist/maplibre-gl.css';
	import { onMount, onDestroy } from 'svelte'; // Add onDestroy for cleanup consistency

	let mapContainer: HTMLDivElement;
	let map: Map | undefined; // Initialize map as undefined

	// Define basic options (style, center, zoom) - container is set dynamically
	const mapOptions = { // Type inference will handle this now
		style: 'https://demotiles.maplibre.org/style.json', // Use a basic style URL initially
		center: [22.9375, -30.5595] as [number, number], // Explicitly type as a tuple
		zoom: 5
	};

	onMount(() => { // Make sure onMount is synchronous
		if (!mapContainer) {
			console.error('Map container not found on mount');
			return;
		}

		// Initialize map directly inside onMount
		map = new Map({
			container: mapContainer,
			...mapOptions
		});

		map.addControl(new NavigationControl(), 'top-right');

		// The onMount return function IS the cleanup
		return () => {
			map?.remove();
		};
	});

	// TODO: Fetch GeoJSON data and add as source/layer
	// TODO: Implement choropleth styling based on scores
	// TODO: Add tooltips/popups on hover/click
</script>

<svelte:head>
	<title>SeeMyCity - Map</title>
	<meta name="description" content="Interactive map of South African municipal financial health" />
</svelte:head>

<section class="map-container">
	<h1>Municipal Financial Health Map</h1>
	<div class="map" bind:this={mapContainer}></div>
</section>

<style lang="scss">
	.map-container {
		width: 100%;
		height: calc(100vh - 60px); // Adjust based on header/footer height if any
		display: flex;
		flex-direction: column;
		align-items: center;

		h1 {
			margin: 1rem 0;
			// Add styling from ux.md if needed
		}
	}

	.map {
		width: 100%;
		height: 100%; // Take remaining height within the container
	}

	// Ensure MapLibre controls look okay with global styles
	:global(.maplibregl-ctrl-group button) {
		background-color: rgba(255, 255, 255, 0.8);
		border: none;
		&:hover {
			background-color: white;
		}
	}

	// If using a dark theme, you might need to adjust control styles further
</style>
