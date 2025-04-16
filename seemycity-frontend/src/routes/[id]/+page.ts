// src/routes/[id]/+page.ts
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// Define the expected structure based on models.rs
// Use 'number' for fields serialized from Decimal/f32
// Renamed from FinancialYearDetails for clarity and consistency with backend
interface FinancialYearData {
	year: number;
	revenue: number | null;
	expenditure: number | null;
	capital_expenditure: number | null;
	debt: number | null;
	audit_outcome: string | null;
	overall_score: number | null;
	financial_health_score: number | null;
	infrastructure_score: number | null;
	efficiency_score: number | null;
	accountability_score: number | null;
}

// Renamed from MunicipalityDetails for clarity and consistency with backend
interface MunicipalityDetail {
	id: string;
	name: string;
	province: string;
	population: number | null;
	classification: string | null;
	website: string | null;
	financials: FinancialYearData[];
	geometry: object | null; // Represent GeoJSON geometry as a generic object for now
}

export const load: PageLoad = async ({ params, fetch }) => {
	// Use muniId for clarity
	const muniId = params.id;

	if (!muniId) {
		throw error(400, 'Municipality ID is required');
	}

	try {
		// Construct the API URL
		const apiUrl = `/api/municipalities/${muniId}`;
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