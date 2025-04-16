// src/routes/[id]/+page.ts
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
// Import shared types
import type { FinancialYearData, MunicipalityDetail } from '../../lib/types';

export const load: PageLoad = async ({ params, fetch }) => {
	// Use muniId for clarity
	const muniId = params.id;

	if (!muniId) {
		throw error(400, 'Municipality ID is required');
	}

	try {
		// Construct the API URL using the environment variable
		const baseApiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000'; // Fallback for local dev
		const apiUrl = `${baseApiUrl}/api/municipalities/${muniId}`;
		console.log(`[+page.ts] Fetching municipality details from: ${apiUrl}`);

		const response = await fetch(apiUrl);

		if (!response.ok) {
			// Improved error message handling
			let errorMessage = `Failed to fetch data: ${response.statusText}`;
			try {
				const errorBody = await response.json();
				errorMessage = errorBody.message || errorMessage; // Use backend message if available
			} catch (e) {
				// Ignore if response body isn't JSON or empty
			}
			console.error(`[+page.ts] API Error (${response.status}): ${errorMessage}`);
			throw error(response.status, errorMessage);
		}

		// Use the corrected interface name
		const municipalityData: MunicipalityDetail = await response.json();

		// Validate the core structure minimally
		if (!municipalityData || typeof municipalityData !== 'object' || !municipalityData.id) {
            console.error('[+page.ts] Received unexpected data structure:', municipalityData);
			throw error(500, `Received invalid data structure for municipality ID ${muniId}`);
		}

		// Sort financials array by year descending to easily get the latest
        // Ensure financials exist and is an array before sorting
        if (Array.isArray(municipalityData.financials)) {
            municipalityData.financials.sort((a, b) => b.year - a.year);
        } else {
            municipalityData.financials = []; // Ensure it's an empty array if null/undefined
        }

        // Extract the latest financial data (first element after sorting)
        const latestFinancials: FinancialYearData | null = municipalityData.financials.length > 0
            ? municipalityData.financials[0]
            : null;

		console.log(`[+page.ts] Successfully fetched data for ${municipalityData.name}`);

		// Return both the full data and the latest financials separately
		return {
			municipality: municipalityData,
            latestFinancials: latestFinancials
		};

	} catch (err: any) {
		console.error('[+page.ts] Error loading municipality data:', err);

		// Handle SvelteKit errors specifically (re-throw if it has status)
		if (err.status) {
			throw err; // Re-throw SvelteKit error object
		}

		// Handle generic fetch errors or errors thrown above
		throw error(500, `An unexpected error occurred while fetching municipality data: ${err.message || 'Unknown error'}`);
	}
};