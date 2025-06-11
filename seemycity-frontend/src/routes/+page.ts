import type { PageLoad } from './$types';
import type { FeatureCollection } from 'geojson';

export const load: PageLoad = async ({ fetch }) => {
	console.log('[+page.ts] Loading all municipalities for map...');
	try {
		const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000';
		const response = await fetch(`${apiUrl}/api/municipalities`);

		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}

		// The API returns a GeoJSON FeatureCollection. We will pass this directly to the page.
		const municipalityGeoJSON: FeatureCollection = await response.json();

		return {
			municipalityGeoJSON
		};
	} catch (error) {
		console.error('Error loading municipality GeoJSON:', error);
		return {
			// Return null for the geojson and a specific error message.
			municipalityGeoJSON: null,
			error: 'Could not load municipality map data. Please try again later.'
		};
	}
};