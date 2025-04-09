<script lang="ts">
	// Revert to default import + destructuring as suggested by the Vite error for CJS compatibility
	import { browser } from '$app/environment';
	import MapComponent from '$lib/components/MapComponent.svelte'; // Import the new component
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
	<title>SeeMyCity - Map</title>
	<meta name="description" content="Map view of Municipal Financial Health" />
</svelte:head>

<div class="page-content">
	<h1>Municipal Financial Health Map</h1>
	
	<!-- Map wrapper with fixed height -->
	<div class="map-wrapper"> 
		<MapComponent /> <!-- Render directly -->
	</div>

	<!-- Other page content can go here -->

</div>

<!-- Styles -->
<style lang="scss">
	/* Styles for the main content area of this specific page */
	.page-content {
		padding: 1rem; /* Restore padding here */
		display: flex;
		flex-direction: column;
		flex-grow: 1; /* Make this container grow to fill main */
		min-height: 0; /* Help flex calculations */
	}

	h1 {
		margin-bottom: 0.5rem;
		flex-shrink: 0; /* Prevent h1 from shrinking if content grows */
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

	/* Styles specific to the map container itself are in MapComponent.svelte */

</style>
