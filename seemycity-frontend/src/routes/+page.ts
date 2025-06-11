export const prerender = true;

import type { PageLoad } from './$types';
import type { MunicipalitySearchResult } from '$lib/types';

export const load: PageLoad = async ({ fetch }) => {
  console.log('[+page.ts] Loading all municipalities for search...');
  try {
    const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000';
    const response = await fetch(`${apiUrl}/api/municipalities`);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const municipalities: MunicipalitySearchResult[] = await response.json();

    return {
      municipalities
    };
  } catch (error) {
    console.error('Error loading municipality list:', error);
    // Return an empty array and an error message for the page to handle
    return {
      municipalities: [],
      error: 'Could not load municipality data. Please try again later.'
    };
  }
};