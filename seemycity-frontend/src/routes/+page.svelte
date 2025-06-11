<script lang="ts">
	// Revert to default import + destructuring as suggested by the Vite error for CJS compatibility
	import { browser } from '$app/environment';
	import MapComponent from '$lib/components/map/MapComponent.svelte'; // Import the new component
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import SearchBar from '$lib/components/ui/SearchBar.svelte';
	import { onMount, onDestroy } from 'svelte'; // Add onDestroy

	let mapWrapperElement: HTMLElement | undefined = undefined;
	let isWrapperReady = false; // Flag to control MapComponent rendering
	let resizeObserver: ResizeObserver | null = null;

	// Note: onMount runs *after* the initial render where bind:this happens
	console.log('Page mounted.');

	// Use a reactive statement to set up the observer once mapWrapperElement is bound
	$: if (browser && mapWrapperElement && !resizeObserver) {
		console.log('Page.svelte: mapWrapperElement bound, setting up ResizeObserver.');
		resizeObserver = new ResizeObserver(entries => {
			if (!entries || entries.length === 0) return;
			const { width, height } = entries[0].contentRect;
			// console.log(`Page.svelte ResizeObserver: Wrapper dimensions ${width.toFixed(0)}x${height.toFixed(0)}`); // Verbose log

			if (width > 0 && height > 0 && !isWrapperReady) { // Only update if not already ready
				console.log('Page.svelte ResizeObserver: Valid dimensions detected. Setting isWrapperReady=true.');
				isWrapperReady = true;
				// Optional: Stop observing once ready
				// resizeObserver?.unobserve(mapWrapperElement);
				// resizeObserver?.disconnect();
				// resizeObserver = null;
			} else if ((width <= 0 || height <= 0) && isWrapperReady) {
				// Optional: Handle if it becomes zero AFTER being ready
				console.warn('Page.svelte ResizeObserver: Wrapper dimensions became invalid.');
				// isWrapperReady = false; // Reset if needed
			}
		});
		resizeObserver.observe(mapWrapperElement);
	}

	// Cleanup function when the component is destroyed
	onDestroy(() => {
		resizeObserver?.disconnect();
		resizeObserver = null; // Clear the observer instance
	});

</script>

<svelte:head>
	<title>SeeMyCity | South African Municipal Finance</title>
	<meta
		name="description"
		content="Explore the financial health of South African municipalities with an interactive map."
	/>
</svelte:head>

<div class="map-view-layout">
	<!-- Sidebar for controls, search, and information -->
	<aside class="sidebar">
		<PageHeader
			title="Explore the Map"
			subtitle="Find a municipality to see its financial health score"
		/>

		<div class="sidebar-content">
			<SearchBar />
			<!-- Other components like legends or filters will go here -->
			<div class="placeholder">
				<p>Search for a municipality or click one on the map to see details here.</p>
			</div>
		</div>
	</aside>

	<!-- Main content area for the map -->
	<main class="map-container">
		<!-- Map wrapper with fixed height -->
		<div class="map-wrapper" bind:this={mapWrapperElement}> 
			<MapComponent /> <!-- Render directly -->
		</div>
	</main>
</div>

<style lang="scss">
	@use '../styles/variables' as *;

	.map-view-layout {
		display: grid;
		grid-template-columns: 380px 1fr; // Fixed sidebar, flexible map
		height: 100%; // Occupy full viewport height
		width: 100%;
		overflow: hidden; // Prevent page scroll
	}

	.sidebar {
		background-color: var(--background-color);
		display: flex;
		flex-direction: column;
		padding: var(--spacing-lg);
		border-right: 1px solid var(--border-color);
		overflow-y: auto; // Allow sidebar to scroll if content overflows
	}

	.sidebar-content {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xl);
		margin-top: var(--spacing-lg);
	}

	.map-container {
		position: relative; // Needed for map controls
		background-color: var(--background-offset-color); // A light background for the map area
	}

	.map-wrapper {
		margin-top: 1rem; /* Keep the margin for spacing above map */
		border: 1px solid #ccc; 
		border-radius: 4px; 
		// overflow: hidden; /* Keep commented out or remove */
		position: relative; /* Keep for potential absolute positioning inside */
		height: 600px; /* << SET FIXED HEIGHT HERE (e.g., 600px) */
		/* background-color: #eee; /* Remove or keep temp background */
	}

	.placeholder {
		padding: var(--spacing-xl);
		text-align: center;
		background-color: var(--background-offset-color);
		border-radius: var(--border-radius-lg);
		border: 1px solid var(--border-color-light);
		color: var(--text-muted-color);
		font-size: var(--font-size-sm);
	}
</style>
