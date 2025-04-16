// src/routes/compare/[ids]/+page.ts
import type { PageLoad } from './$types';
// Adjust this path based on your project structure
import type { MunicipalityDetail } from '$lib/types'; 

export const load: PageLoad = async ({ params, fetch }) => {
	const idsString = params.ids;
	const muniIds = idsString
		.split(',')
		.map((id) => id.trim()) // Remove whitespace
		.filter((id) => id.length > 0); // Remove empty strings

	if (muniIds.length === 0) {
		// Or return an error, or handle as needed
		return { municipalities: [], requestedIds: [] };
	}

	console.log(`[compare/+page.ts] Loading data for IDs: ${muniIds.join(', ')}`);

	try {
		const fetchPromises = muniIds.map(async (id) => {
			console.log(`[compare/+page.ts] Fetching data for ID: ${id}`);
			try {
				// Ensure API_BASE_URL is defined or fallback (adjust as needed)
				const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000'; 
				const response = await fetch(`${apiUrl}/api/municipalities/${id}`);

				if (!response.ok) {
					throw new Error(`HTTP error! status: ${response.status} for ID ${id}`);
				}
				const data: MunicipalityDetail = await response.json();
				return data;
			} catch (err) {
				console.error(`Error fetching or parsing data for ${id}:`, err);
				return null; // Indicate failure for this specific ID
			}
		});

		// Wait for all fetches to complete
		const results = await Promise.all(fetchPromises);

		// Filter out any null results (failed fetches)
		const successfulMunicipalities = results.filter(
			(muni: MunicipalityDetail | null): muni is MunicipalityDetail => muni !== null
		);

		return {
			municipalities: successfulMunicipalities,
			requestedIds: muniIds // Optionally return the list of IDs requested
		};

	} catch (error) {
		console.error(`Error in compare/+page.ts load function:`, error);
		// Consider throwing a SvelteKit error for page-level failure
		// import { error as svelteKitError } from '@sveltejs/kit';
		// throw svelteKitError(500, 'Failed to load comparison data');
		return {
			municipalities: [],
			error: 'Failed to load comparison data.', // Pass error message to the page
			requestedIds: muniIds
		};
	}
};