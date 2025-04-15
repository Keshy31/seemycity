import { svelteTesting } from '@testing-library/svelte/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: { 
		proxy: {
			// Proxy /api requests to the backend server
			'/api': {
				target: 'http://127.0.0.1:4000', // Your Rust backend address
				changeOrigin: true, // Recommended for CORS and virtual hosts
				secure: false, // Optional: If your backend uses HTTPS with a self-signed cert
				// Optional: rewrite path if your backend expects a different path prefix
				// rewrite: (path) => path.replace(/^\/api/, '') 
			},
		},
		// Optional: Define the port Vite runs on if needed (default is 5173)
		// port: 5173, 
	},
	test: {
		workspace: [
			{
				extends: './vite.config.ts',
				plugins: [svelteTesting()],
				test: {
					name: 'client',
					environment: 'jsdom',
					clearMocks: true,
					include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['src/lib/server/**'],
					setupFiles: ['./vitest-setup-client.ts']
				}
			},
			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});
