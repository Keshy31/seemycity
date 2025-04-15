// src/routes/[id]/+page.ts
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// Define structure for the items within the financials array
interface FinancialYearDetails {
	year: number;
	revenue?: number | null;
	expenditure?: number | null;
	capital_expenditure?: number | null;
	debt?: number | null;
	audit_outcome?: string | null;
	overall_score?: number | null; // Backend sends Decimal (number/string), adjust if needed
	financial_health_score?: number | null;
	infrastructure_score?: number | null;
	efficiency_score?: number | null;
	accountability_score?: number | null;
}

// Update main interface to include the financials array
// Match this with the Rust MunicipalityDetail struct
interface MunicipalityDetails {
	id: string;
	name: string;
	province: string; // Changed from province_name
	population?: number | null; // Backend sends f32, might be received as number
	classification?: string | null; // Changed from category
	website?: string | null;
	financials: FinancialYearDetails[];
    geometry?: any; // Add if needed
}

export const load: PageLoad = async ({ params, fetch }) => {
	const { id } = params; // Get the municipality ID from the route parameter

	if (!id) {
		throw error(400, 'Municipality ID is required.');
	}

	try {
		// Construct the API URL
		const apiUrl = `/api/municipalities/${id}`;
		console.log(`[+page.ts] Fetching municipality details from: ${apiUrl}`);

		const response = await fetch(apiUrl);

		if (!response.ok) {
			let errorBody = { message: response.statusText };
			try {
				errorBody = await response.json();
			} catch (e) {
				// Ignore if response body is not JSON
			}
			console.error(`[+page.ts] API Error (${response.status}): ${errorBody.message || 'Unknown error'}`);
			throw error(response.status, `Failed to load municipality data: ${errorBody.message || response.statusText}`);
		}

		const municipalityData: MunicipalityDetails = await response.json();

		if (!municipalityData || typeof municipalityData !== 'object' || !municipalityData.id) {
            console.error('[+page.ts] Received unexpected data structure:', municipalityData);
			throw error(500, `Received invalid data structure for municipality ID ${id}`);
		}

		console.log('[+page.ts] Successfully fetched municipality data:', municipalityData);

		// Return the data to be used by the +page.svelte component
		return {
			municipality: municipalityData
		};

	} catch (err: any) {
		console.error('[+page.ts] Error loading municipality data:', err);

		// Handle SvelteKit errors specifically (re-throw)
		if (err.status && err.body) {
			throw error(err.status, err.body.message);
		}

		// Handle generic fetch errors or errors thrown above
		throw error(500, `An error occurred while fetching municipality data: ${err.message || 'Unknown error'}`);
	}
};