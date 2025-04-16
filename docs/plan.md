### `plan.md` - Development Plan

#### Overview
**Product Name**: Municipal Financial Dashboard
**Purpose**: This development plan outlines the tasks to build an MVP of the Municipal Financial Dashboard in a logical sequence. The approach prioritizes a basic web version deployed online as soon as possible, using Fly.io for hosting and Postgres, to enable early feedback while backend and data integration are built out incrementally.

**Goals**:
- Deploy a minimal frontend to Fly.io quickly for visibility.
- Integrate Municipal Money API data with Postgres caching on Fly.io.
- Deliver a full MVP with map, single, and comparison views.

**Assumptions**:
- Fly.io deployment is available from the start.
- External data (GeoJSON, population) can be sourced manually.
- Iterative development allows frontend-first with backend catch-up.

---

#### Phases and Tasks

##### Phase 1: Initial Frontend Setup & Deployment
- **Goal**: Get a basic web version live on Fly.io ASAP for early visibility.
- **Status**: **Completed**
- **Tasks**:
    - [x] **Initialize SvelteKit Project**: `npm create svelte@latest seemycity-frontend`
    - [x] **Basic Routing**: Setup `src/routes/+page.svelte` (homepage) and `src/routes/[id]/+page.svelte` (detail page placeholder)
    - [x] **Static Adapter**: Configure `@sveltejs/adapter-static` in `svelte.config.js`.
    - [x] **Dockerfile**: Create `Dockerfile` using Node base image for build stage and Nginx for serving.
    - [x] **Fly.io Setup**: Create `fly.toml`, launch app `fly launch --no-deploy`.
    - [x] **Initial Deploy**: Run `fly deploy`. Debug Dockerfile/Nginx issues until homepage is served.
    - [x] **Styling (Basic)**: Apply background, text color, and Ubuntu font globally via `+layout.svelte`.
    - [x] **Configure CSS/SCSS**: Installed `sass`, using plain CSS/SCSS for styling (replaces previous Tailwind attempt).
    - [ ] **Add MapLibre GL JS map placeholder**: (Moved slightly later, can be done during Phase 3)
- **Milestone**: Basic Web Up - **Achieved**
- **Note on Prerendering (Initial Deployment)**:
    *   For the initial static deployment (Phase 1), global prerendering (`export const prerender = true;` in `src/routes/+layout.ts`) has been *disabled*, and `@sveltejs/adapter-static` is configured with `strict: false` in `svelte.config.js`. The homepage (`/`) is explicitly prerendered via `src/routes/+page.ts`.
    *   This avoids build errors caused by the dynamic `/[id]` route before we have data or links to generate specific municipality pages.
    *   Prerendering for `/[id]` pages will need to be re-enabled/configured later (likely Phase 5 or 6) once the backend API or build-time data source can provide the list of municipality IDs needed to generate the individual static pages. The `strict: true` setting should also be reconsidered then. Alternatively, SPA fallback mode could be configured.

##### Phase 2: Database Setup on Fly.io
- **Goal**: Establish Postgres on Fly.io for data storage and initial data load.
- **Status**: **Completed**
- **Tasks**:
    - [x] **Setup Postgres Instance**: Run `fly postgres create` on Fly.io, enable PostGIS extension.
    - [x] **Create Tables**: Define database schema **in [`docs/data-spec.md`](./data-spec.md#2-database-schema-postgresql--postgis)** (tables: `municipalities`, `municipal_geometries`, `financial_data`).
    - [x] **Connect Locally**: Establish connection for manual imports (e.g., `psql`).
    - [x] **Load Municipality & Geometry Data**: Download/obtain GeoJSON/boundaries and core municipality info. Import into `municipalities` and `municipal_geometries` tables. (User confirmed manual completion).
    - [x] **Load Population Data**: Source and import population estimates per municipality.
    - [-] **(Deferred to Phase 7)** Load District/Province association data.
- **Milestone**: DB Ready - **Achieved**

##### Phase 3: Frontend Expansion (MVP)
- **Goal**: Build core UI views using dummy data.
- **Tasks**:
    - [x] Create Map View (`/`) with MapLibre GL JS.
    - [x] Implement click popups showing name/score.
    - [x] Implement navigation from popup header to Single View.
    - [x] Create Single View (`/[id]`) displaying detailed muni info (province, pop, metrics) and score breakdown.
    - [x] Create Comparison View (`/compare/[ids]`) showing side-by-side metrics.
    - [x] Refactor and enhance dummy data (`dummyStore.ts`) based on PRD/Tech specs.
- **Status**: **Completed**
- **Milestone**: Full UI (dummy) Ready.

##### Phase 4: Backend Development
- **Goal**: Build the Rust backend with Actix Web, `sqlx`, and integrate the Municipal Money API client.
- **Status**: **Completed** (Core API endpoints implemented)
- **Tasks:**
    *   [x] Initialize Rust project (`cargo new`, add initial dependencies: actix-web, sqlx, tokio, serde, reqwest, dotenvy, env_logger, rust_decimal, chrono)
    *   [x] Setup basic Actix Web server (root endpoint `/`, health check `/health`)
    *   [x] Implement modular structure (`src/handlers`, `src/models`, `src/db`, `src/api`, `src/errors`, `src/utils`, `src/config`)
    *   [x] Define core data models (`src/models.rs`)
    *   [x] Integrate PostgreSQL using `sqlx` (`src/db/queries.rs`: connect, define core query functions like `get_municipality_base_info_db`, `upsert_financial_data`)
    *   [x] Build Municipal Money API client (`src/api/muni_money/`): Structure, request logic, response parsing, error handling (`ApiClientError`).
    *   [x] Implement core API endpoints (`src/handlers/municipalities.rs`):
        *   [x] `GET /api/municipalities/{id}`: Fetch data using DB queries and API client, combine results, perform `upsert_financial_data`.
        *   [x] `GET /api/municipalities`: Implemented to return GeoJSON summary for map view.
    *   [x] Implement Caching Logic: Implemented time-based cache check (`CACHE_TTL_SECONDS`) within the `/api/municipalities/{id}` handler.
    *   [x] Setup integration tests for API client (`tests/muni_money_integration_test.rs`).
    *   [x] Implement application-level error handling (`src/errors.rs: AppError`).
    *   [x] Configure environment variables (`.env`, `src/config.rs`).
    *   [x] Refactor `/municipalities/{id}` endpoint to fetch missing data, trigger scoring, and upsert results.
    *   [x] Refactor scoring logic (`calculate_financial_score`) to use detailed sub-score functions.
    *   [ ] Implement caching strategy (for calculated scores).
    *   [ ] Add endpoints for district/province aggregation (Post-MVP).
- **Milestone**: **Backend Ready** (API Structure & Endpoints): **Achieved**

##### Phase 5: Frontend-Backend Integration
- **Goal**: Connect SvelteKit frontend to the Rust API, replacing dummy data.
- **Status**: **In Progress** (Initial map fetch complete)
- **Tasks**:
    - [x] **Update Map View (Initial Fetch)**: Fetch `/api/municipalities`, connect to backend via proxy, display basic GeoJSON shapes.
    - [x] **Update Map View (Data Styling)**: Implement data-driven styling to color municipalities based on fetched scores (requires backend scoring logic).
    - [x] **Update Single View**: Fetch `/api/municipality/{id}`, display real metrics. Add refresh. Deploy.
    - [ ] **Update Comparison View**: Fetch multiple `/api/municipality/{id}`, show real comparisons. Deploy.
- **Milestone**: Real Data Integration (Initial Map Fetch): **Achieved**

##### Phase 6: Polish and Testing
- **Goal**: Refine UI/UX, add tests, fix bugs, finalize documentation.
- **Status**: **Not Started**
- **Tasks**:
    - [ ] **Polish UI**: Apply transitions, add feedback (e.g., toasts), test responsiveness/themes. Deploy.
    - [ ] **Testing**: Verify functionality, performance, accessibility. Handle edge cases (missing data).
    - [ ] **Optimize `/api/municipalities` DB Query**: Current query takes ~13s. Investigate potential index optimizations on `municipalities`, `municipal_geometries`, and `financial_data` tables (especially on `year`, `municipality_id`, and geometry columns).
    - [ ] **Reduce `/api/municipalities` Payload Size**: Current response is ~33MB. Explore using simplified geometries (`ST_SimplifyPreserveTopology`) or centroids (`ST_Centroid`) instead of full geometries for the map overview.
    - [ ] **Bug Fixes**: Address identified issues. Deploy fixes.
    - [ ] **Documentation**: Update README with final setup/deployment steps.
- **Milestone**: MVP Done

##### Phase 7: Post-MVP Enhancements (Hierarchical View)
- **Goal**: Enhance map exploration with hierarchical drill-down and aggregated scores.
- **Status**: **Not Started**
- **Tasks**:
    - [ ] **Source/Load District & Province Data**: Obtain and load GeoJSON boundaries for districts and provinces, and associate municipalities with districts/provinces in the database.
    - [ ] **Implement Hierarchical Map Navigation**: Update frontend map component to allow zooming/clicking between Province, District, and Municipality views.
    - [ ] **Add Backend Logic for Score Aggregation**: Implement API endpoints and logic to calculate average scores for provinces and districts based on their constituent municipalities.
    - [ ] **Update Frontend Views**: Display aggregated scores on the map and potentially in dedicated sidebars/tooltips when viewing Province/District levels.
- **Milestone**: Hierarchical View

---

#### Logical Sequence
1.  **Frontend First**: Basic SvelteKit + map deployed to Fly.io. **(Done)**
2.  **DB Setup**: Postgres on Fly.io with static data. **(Done)**
3.  **Frontend Expansion**: Full UI with dummy data, iterative deploys.
4.  **Backend Build**: Rust API to process real data. **(Done)**
5.  **Integration**: Connect SvelteKit frontend to Rust API, replacing dummy data. **(In Progress)**
6.  **Polish**: Final UX tweaks and testing.
7.  **Hierarchical View**: Enhance map exploration with hierarchical drill-down and aggregated scores.

---

#### Milestones Summary
- **Basic Web Up**: Achieved (Phase 1)
- **DB Ready**: Achieved (Phase 2)
- **Full UI (Dummy Data)**: Achieved (Phase 3)
- **Backend Ready** (API Structure & Endpoints): **Achieved** (Phase 4)
- **Real Data Integration** (Initial Map Fetch): **Achieved** (Phase 5)
- **Real Data Integration** (Full): **Pending** (Phase 5)
- **MVP Done**: Target after Phase 6
- **Hierarchical View**: Target after Phase 7

---

#### Risks
- **Data Sourcing**: Population/District data delays - proceed with available data for now.
- **API Complexity**: Municipal Money API integration might require adjustments.
- **Fly.io**: Potential deployment complexities with dual apps (frontend/backend).

---

#### Resources
- **Tools**: Cargo, npm, Fly.io CLI, psql.
- **Docs**: Municipal Money API, Actix Web, SvelteKit, MapLibre GL JS, PostGIS.

---

### Feedback
This plan reflects the current progress, prioritizing getting the database populated before expanding the frontend significantly.

**Current Focus**: Implementing the map data styling (Phase 5) and backend financial scoring logic (Phase 4).