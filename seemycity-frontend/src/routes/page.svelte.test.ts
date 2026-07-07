import { describe, test, expect } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/svelte';
import Page from './+page.svelte';

describe('/+page.svelte', () => {
	// The page requires the load function's data prop. GeoJSON is left null so
	// the MapLibre component (which needs a real WebGL context) is not mounted.
	const data = { municipalityGeoJSON: null, error: null };

	test('should render the sidebar heading', () => {
		render(Page, { props: { data } });
		expect(screen.getByRole('heading', { level: 1, name: 'Explore the Map' })).toBeInTheDocument();
	});

	test('should render the error state when the load reported one', () => {
		render(Page, { props: { data: { municipalityGeoJSON: null, error: 'API unreachable' } } });
		expect(screen.getByRole('alert')).toHaveTextContent('API unreachable');
	});
});
