import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// required for svelte5 + jsdom as jsdom does not support matchMedia
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	enumerable: true,
	value: vi.fn().mockImplementation((query) => ({
		matches: false,
		media: query,
		onchange: null,
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
});

// jsdom lacks createObjectURL; maplibre-gl calls it at module scope to set up
// its web worker, so importing any component that imports maplibre-gl throws
// without this shim.
Object.defineProperty(window.URL, 'createObjectURL', {
	writable: true,
	value: vi.fn(() => 'blob:vitest-mock')
});

// add more mocks here if you need them
