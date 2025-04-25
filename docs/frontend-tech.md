### `frontend-tech.md` - Frontend Technical Specifications

This document details the technical specifications for the SvelteKit frontend of the Municipal Financial Dashboard.

---

#### Technology Stack

##### Frontend Framework
- **Framework**: SvelteKit
  - Why: Modern, compiler-based approach for high performance, component-based architecture, built-in routing and server-side rendering (used for static export).
  - Use: Structure the application, manage routes, handle component logic.

##### Mapping Library
- **Library**: MapLibre GL JS
  - Why: High-performance, open-source vector map rendering, compatible with standard GeoJSON and vector tile sources. Chosen over Leaflet (mentioned in memory) due to better vector tile performance.
  - Use: Display municipal boundaries from the loaded GeoJSON source, apply choropleth styles based on feature properties (scores), add tooltips, and handle click interactions.

##### Styling
- **Approach**: Plain CSS/SCSS (No Tailwind CSS)
  - Why: User preference, provides fine-grained control, leverages Svelte's scoped styles.
  - Use: Styles are defined within Svelte components using `<style lang="scss">` blocks for automatic scoping. Global styles and CSS variables (e.g., for color palette from `ux.md`) are in `src/app.scss`. Semantic class names (e.g., `.info-card`) are preferred.

##### Icons
- **Library**: Iconify (`@iconify/svelte`)
  - Why: Lightweight access to numerous icon sets, customizable via CSS.
  - Use: Metric indicators, status badges, UI elements.

##### Animations
- **Approach**: Svelte Built-ins (transitions, animations)
  - Why: Simple, integrated way to achieve subtle UI animations (pulse, fade, slide) as specified in `ux.md`.
  - Use: Enhancing user feedback and visual appeal on interactions or data loading.

---

#### Frontend Architecture

##### File Structure

```
seemycity-frontend/
├── .dockerignore
├── .gitignore
├── .npmrc
├── .prettierignore
├── .prettierrc
├── .svelte-kit/          # SvelteKit build artifacts (transient)
├── Dockerfile            # For containerization (Fly.io)
├── README.md             # Frontend-specific readme
├── build/                # Static build output (transient)
├── eslint.config.js      # ESLint configuration
├── fly.toml              # Fly.io deployment configuration
├── node_modules/         # Project dependencies (transient)
├── package-lock.json     # Dependency lock file
├── package.json          # Project metadata and dependencies
├── static/               # Static assets (e.g., favicon, fonts)
│   └── favicon.png
├── src/                  # Main application source code
│   ├── app.d.ts          # Ambient TypeScript definitions
│   ├── app.html          # Main HTML template
│   ├── app.scss          # Global SCSS styles/variables
│   ├── demo.spec.ts      # Example test
│   ├── hooks.server.ts   # Server-side hooks (if needed)
│   ├── lib/              # Reusable components, utilities, data types
│   │   ├── components/   # UI Components 
│   │   │   ├── MapComponent.svelte
│   │   │   ├── detail/         # Components specific to single/comparison views
│   │   │   │   ├── PageHeader.svelte
│   │   │   │   ├── KeyMetricsGrid.svelte
│   │   │   │   ├── MetricCard.svelte
│   │   │   │   └── ScoreBreakdown.svelte
│   │   │   ├── ui/             # General reusable UI elements
│   │   │   │   ├── ErrorMessage.svelte
│   │   │   │   ├── LoadingSpinner.svelte
│   │   │   │   └── ProgressBar.svelte
│   │   │   └── dummyStore.ts # Exports dummyMunicipalitiesGeoJSON (for map) & dummyMunicipalityDetails (object mapping ID to details with financials array, latest_score, etc.). Contains helper functions.
│   │   └── index.ts      # Barrel file for lib exports (optional)
│   └── routes/           # Application pages and API routes
│       ├── +layout.svelte  # Root layout component
│       ├── +layout.ts      # Layout load function (if needed)
│       ├── +page.svelte    # Main map view page component
│       ├── +page.ts        # Main map view load function (fetches data)
│       ├── [id]/           # Single municipality view route
│       │   └── +page.svelte # Component for single view
│       ├── compare/        # Comparison view route
│       │   └── +page.svelte # Component for comparison view
│       └── page.svelte.test.ts # Test for main page
├── svelte.config.js      # SvelteKit configuration
├── tsconfig.json         # TypeScript configuration
├── vite.config.ts        # Vite configuration
└── vitest-setup-client.ts # Vitest setup
```

##### Key Components / Views
- **Map View (`/`)**: Displays the main interactive map with municipalities color-coded by score. Fetches aggregated data.
- **Single View (`/[id]`)**: Shows detailed financial metrics and score breakdown for a selected municipality. Fetches detailed data for one municipality.
- **Comparison View (`/compare`)**: Allows selecting multiple municipalities for side-by-side comparison (Post-MVP).

##### Data Flow
- Svelte components use `fetch` within `load` functions (`+page.ts`, `+layout.ts`) or component lifecycle functions (`onMount`) to request data from the backend API endpoints (`/api/municipalities`, `/api/municipalities/{id}`).
- The structure of the data received from the API is defined in the **API Payloads** section of [`docs/data-spec.md`](../docs/data-spec.md#3-api-payloads).
- The main map view fetches a GeoJSON FeatureCollection from `/api/municipalities`. Other views fetch standard JSON from endpoints like `/api/municipalities/{id}`.
- Data is stored in component state or Svelte stores for reactivity, generally matching the structures defined in the **Frontend Data Structures** section of [`docs/data-spec.md`](../docs/data-spec.md#12-frontend-svelte-typescript).
- MapLibre GL JS is initialized in the Map component (`MapComponent.svelte`), loading the fetched GeoJSON FeatureCollection as a data source. Data-driven styling (e.g., `fill-color`) is applied based on the `overall_score` property within the GeoJSON features to visually represent financial health.
- Single and Comparison views fetch financial data that now includes all calculated scores.

---

#### Score Calculation Reference

The financial score calculation logic (pillars, weights, normalization, and formulas) is defined in the canonical rubric in [prd.md](../docs/prd.md#2-functional-requirements). Please refer to that document for the latest details and rationale.

**Frontend implementation notes:**
- The frontend receives all calculated scores (overall and pillar scores) from the backend via API payloads.
- Scores are displayed in the map, single, and comparison views using color-coding, badges, and breakdown components as described in the UX documentation.
- No score calculations are performed on the frontend; all logic is backend-driven.

---

#### Data Structures & API Payloads

The canonical definitions for all frontend data structures (TypeScript interfaces) and API payloads are maintained in [data-spec.md](../docs/data-spec.md#1-core-data-structures). Please refer to that document for up-to-date field lists and type details.

**Frontend-specific notes:**
- Types should be kept in sync with backend payloads as defined in data-spec.md.
- TypeScript interfaces are located in `src/lib/types.ts`.
- Any deviations or extensions should be documented in data-spec.md and referenced here.

---

#### Performance
- Utilize SvelteKit's static adapter (`adapter-static`) for pre-rendered HTML.
- Lazy-load components or data where appropriate.
- Optimize map data loading (e.g., simplification, vector tiles if GeoJSON is too large).
- Target Load Time: < 2 seconds (initial load), < 1 second (subsequent navigation).

---

#### Dependencies
- `sveltekit`
- `maplibre-gl`
- `sass`
- `@iconify/svelte`
- `vite`, `vitest`, `eslint`, `prettier` (Development)
