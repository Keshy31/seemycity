<script lang="ts">
	import '../app.scss'; // Global styles: tokens (_variables), reset/base (_base), typography
</script>

<main>
	<slot />
</main>

<style lang="scss">
	// Theme, body, and typography defaults live in src/styles/_base.scss.
	// This block only gives the routed content a full-height flex column so the
	// map view can fill the viewport.
	main {
		flex-grow: 1;
		width: 100%;
		display: flex;
		flex-direction: column;
		min-height: 0; // Allow children to size within the flex column
	}
</style>
