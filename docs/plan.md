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
- **Tasks**:
  1.  **Initialize SvelteKit Project**: `npm create svelte@latest seemycity-frontend` - 
  2.  **Basic Routing**: Setup `src/routes/+page.svelte` (homepage) and `src/routes/[id]/+page.svelte` (detail page placeholder) - 
  3.  **Static Adapter**: Configure `@sveltejs/adapter-static` in `svelte.config.js`. - 
  4.  **Dockerfile**: Create `Dockerfile` using Node base image for build stage and Nginx for serving. - 
  5.  **Fly.io Setup**: Create `fly.toml`, launch app `fly launch --no-deploy`. - 
  6.  **Initial Deploy**: Run `fly deploy`. Debug Dockerfile/Nginx issues until homepage is served. - 
  7.  **Styling (Basic)**: Apply background, text color, and Ubuntu font globally via `+layout.svelte`. - 
  8.  **TODO**: Configure Tailwind CSS.
  9.  **TODO**: Add Leaflet map placeholder.
- **Milestone**: Basic Web Up. - **Achieved**
- **Note on Prerendering (Initial Deployment)**:
    *   For the initial static deployment (Phase 1), global prerendering (`export const prerender = true;` in `src/routes/+layout.ts`) has been *disabled*, and `@sveltejs/adapter-static` is configured with `strict: false` in `svelte.config.js`. The homepage (`/`) is explicitly prerendered via `src/routes/+page.ts`.
    *   This avoids build errors caused by the dynamic `/[id]` route before we have data or links to generate specific municipality pages.
    *   Prerendering for `/[id]` pages will need to be re-enabled/configured later (likely Phase 5 or 6) once the backend API or build-time data source can provide the list of municipality IDs needed to generate the individual static pages. The `strict: true` setting should also be reconsidered then. Alternatively, SPA fallback mode could be configured.

##### Phase 2: Database Setup on Fly.io
- **Goal**: Establish Postgres on Fly.io for data storage and manual access.  
- **Tasks**:  
  4. **Setup Postgres Instance**:  
     - Run `fly postgres create` on Fly.io, enable PostGIS extension. - **Achieved** 
     - Create `municipalities` and `financial_data` tables (SQL from tech.md). - **Achieved** 
     - Connect locally for manual imports (e.g., `psql`).  
  5. **Source and Load Static Data**:  
     - GeoJSON: Download from the [Municipal Demarcation Board ArcGIS Hub](https://spatialhub-mdb-sa.opendata.arcgis.com/) (focus on Local Municipal Boundaries). - **In Progress (Manual)** 
     - Population: Extract from StatsSA (e.g., 2022 estimates). - **In Progress (Manual)** 
     - Import into `municipalities` table via CSV or SQL script.  
- **Milestone**: DB Ready. - *(Pending data load)*

##### Phase 3: Frontend Expansion
- **Goal**: Build out UI with dummy data, refine UX, and deploy iteratively.  
- **Tasks**:  
  6. **Enhance Map View**:  
     - Fetch static GeoJSON from Postgres (mock API call or hardcoded), render choropleth with dummy scores.  
     - Add hover tooltips (“Cape Town’s at 84—pretty solid!”), province filter dropdown.  
     - Deploy to Fly.io.  
  7. **Build Single View**:  
     - Fetch dummy data for `/[id]` (e.g., Cape Town metrics), display pulsing score badge, metric cards (💰, 🏡).  
     - Add expandable breakdown (“What’s behind this score?”) with slide animation.  
     - Deploy update.  
  8. **Build Comparison View**:  
     - Route `/compare/[ids]`, show dummy side-by-side table (Cape Town vs. Johannesburg).  
     - Add “Swap” button (flip animation), “Add Another” link.  
     - Deploy update.  

##### Phase 4: Backend Development
- **Goal**: Add Rust backend to fetch and process real data, connect to frontend.  
- **Tasks**:  
  9. **Initialize Rust Project**:  
     - Run `cargo init`, add Actix Web, sqlx, serde, reqwest.  
     - Setup basic server with `/api/municipalities` endpoint (dummy response).  
  10. **Postgres Integration**:  
      - Connect to Fly.io Postgres with `sqlx`, query `municipalities` table.  
      - Test basic CRUD (e.g., fetch all municipalities).  
  11. **API Client and Processing**:  
      - Fetch Municipal Money API data (`GET /api/cubes/income_expenditure`).  
      - Implement `calculate_score` function (tech.md), normalize metrics.  
      - Cache results in `financial_data` table.  
  12. **API Endpoints**:  
      - `GET /api/municipalities`: Return all with scores, GeoJSON.  
      - `GET /api/municipality/{id}`: Single municipality details.  
      - `POST /api/refresh/{id}`: Update financial data.  
      - Test with Postman.  
  13. **Deploy Backend**:  
      - Add Rust app to Fly.io (`fly init` in separate dir), configure `fly.toml`.  
      - Deploy alongside SvelteKit frontend, expose API (e.g., `app.fly.dev/api`).  

##### Phase 5: Frontend-Backend Integration
- **Goal**: Connect Svelte to Rust API, replace dummy data with real data.  
- **Tasks**:  
  14. **Update Map View**:  
      - Fetch `/api/municipalities`, render real scores and GeoJSON.  
      - Deploy update.  
  15. **Update Single View**:  
      - Fetch `/api/municipality/{id}`, display real metrics and score.  
      - Add “Refresh” button to call `/api/refresh/{id}`.  
      - Deploy update.  
  16. **Update Comparison View**:  
      - Fetch multiple `/api/municipality/{id}`, show real comparisons.  
      - Deploy update.  

##### Phase 6: Polish and Testing
- **Goal**: Refine UX, fix bugs, and validate the MVP.  
- **Tasks**:  
  17. **Polish UI**:  
      - Apply Svelte transitions (fade, pulse), “Data refreshed!” toast.  
      - Test dark mode toggle (cream → charcoal).  
      - Deploy final UI tweaks.  
  18. **Testing**:  
      - Verify map load, API fetch speed (Cape Town, Johannesburg).  
      - Check mobile stacking, accessibility (keyboard, screen reader).  
      - Handle missing data (“Unavailable” labels).  
  19. **Bug Fixes**:  
      - Address issues (e.g., slow map, API errors).  
      - Deploy fixes.  
  20. **Documentation**:  
      - Update README with setup, deployment steps.

---

#### Logical Sequence
1. **Frontend First**: Basic SvelteKit + map deployed to Fly.io.  
2. **DB Setup**: Postgres on Fly.io with static data.  
3. **Frontend Expansion**: Full UI with dummy data, iterative deploys.  
4. **Backend Build**: Rust API to process real data.  
5. **Integration**: Connect frontend to backend, use real data.  
6. **Polish**: Final UX tweaks and testing.

#### Milestones
- **Basic Web Up**: After Phase 1—live placeholder site. - **Achieved**
- **DB Ready**: After Phase 2—Postgres populated.  
- **Full UI**: After Phase 3—interactive dummy version.  
- **Real Data**: After Phase 5—backend integration complete.  
- **MVP Done**: After Phase 6—polished and tested.

#### Risks
- **Early Deploy**: Dummy data may confuse—label as “Preview.”  
- **Data Sourcing**: GeoJSON/population delays—start with 2-3 municipalities.  
- **Fly.io**: Setup hiccups—test locally first if needed.

#### Resources
- **Tools**: Cargo, npm, Fly.io CLI, psql.  
- **Docs**: Municipal Money API, Actix Web, SvelteKit, Leaflet.js.

---

### Feedback
This revised plan gets a basic web version live ASAP (Phase 1), builds out the frontend while the backend catches up, and uses Fly.io with Postgres from the start. You’ll see progress online early, and we can squash bugs as we go. All set to start coding, or any final tweaks to the sequence?