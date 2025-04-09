<script>
	import '../app.scss'; // Import global styles
	// Javascript for the layout goes here (we'll add things later)
	console.log('Layout loaded!');
</script>

<main>
	<!-- This is where the content from +page.svelte will be inserted -->
	<slot />
</main>

<style>
	/* Apply styles to the whole page */
	:global(body) {
		background-color: #fdf6e3; /* Solarized Light Background */
		color: #3c2f2f; /* Dark text color */
		font-family: 'Ubuntu', sans-serif; /* Use Ubuntu font */
		margin: 0; /* Remove default body margin */
		padding: 0; /* Remove default body padding */
		box-sizing: border-box; /* Better box model */
		height: 100%; /* Ensure body takes full height */
		line-height: 1.6; /* Improve readability */
	}

	/* Ensure html and body take full height */
	:global(html) {
		height: 100%;
	}

	/* Ensure root svelte element takes full height */
	:global(#svelte) {
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	/* CSS for the layout goes here */
	main {
		/* padding: 1rem; */ /* Remove padding, apply it lower down */
		flex-grow: 1; /* Allow main content to grow */
		width: 100%;
		display: flex; /* Make it a flex container */
		flex-direction: column; /* Stack children vertically */
		// overflow: hidden; /* Prevent content overflow issues and help height calc */
		min-height: 0; /* Add this to help flex calculations */
	}
</style>
