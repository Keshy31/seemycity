import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			// default options are suitable for static deployment
			pages: 'build',
			assets: 'build',
			fallback: null, // or 'index.html' or '200.html' if needed for SPA routing
			precompress: false
		})
	}
};

export default config;
