<script>
	import { onMount } from 'svelte';

	// State to hold map components and readiness
	let LeafletMap = null;
	let TileLayer = null;
	let mapReady = false;

	// Map options based on MEMORY[user_15211870533011317499]
	const mapOptions = {
		center: [-30.5595, 22.9375], // Approx center of SA
		zoom: 6 // Initial zoom level
	};

	// Tile layer URL (OpenStreetMap)
	const tileUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
	const tileLayerOptions = {
		attribution:
			'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
	};

	// Only load Leaflet components on the client-side after mounting
	onMount(async () => {
		// Dynamically import the components
		const leaflet = await import('svelte-leaflet');
		LeafletMap = leaflet.LeafletMap;
		TileLayer = leaflet.TileLayer;

		// Import CSS dynamically as well (or ensure it's globally available if preferred)
		await import('leaflet/dist/leaflet.css');
		mapReady = true; // Signal that the map can now be rendered
	});

	// Javascript specific to this page goes here
	console.log('Homepage loaded!');
</script>

<svelte:head>
	<title>SeeMyCity - Home</title>
	<meta name="description" content="Visualizing South African municipal financial health." />
</svelte:head>

<div class="page-content">
	<h1>Welcome to SeeMyCity!</h1>
	<div class="map-container">
		{#if mapReady && LeafletMap && TileLayer}
			<LeafletMap options={mapOptions}>
				<svelte:component this={TileLayer} url={tileUrl} options={tileLayerOptions} />
				<!-- Markers, GeoJSON layers, etc. will go here later -->
			</LeafletMap>
		{:else}
			<p>Loading map...</p> <!-- Optional loading indicator -->
		{/if}
	</div>
</div>

<style>
	.map-container {
		height: 600px; /* Give the map a defined height */
		width: 100%;   /* Make map take full width */
		margin-top: 1rem;
		border: 1px solid #ccc; /* Optional: Add a border to see the container */
	}

	.page-content {
		/* Add styles specific to this page's content if needed */
		text-align: center;
		padding-top: 2rem;
	}

	h1 {
		color: #008080; /* Teal accent color from WindsurfRules */
		font-weight: 500; /* Slightly bolder */
	}
</style>
