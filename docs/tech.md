### `tech.md` - Technical Specifications

#### Overview
**Product Name**: Municipal Financial Dashboard  
**Purpose**: The Municipal Financial Dashboard is a web application that delivers a high-performance, engaging experience for exploring South African municipal financial health. This document outlines the technical architecture, tools, and approaches to meet the product’s requirements (prd.md) and user experience goals (ux.md).

**Goals**:  
- Fetch and process financial data efficiently from the Municipal Money API.  
- Cache data in Postgres for speed and flexibility.  
- Render an interactive, warm UI with Svelte and Leaflet.js.  
- Ensure scalability and maintainability for future growth.

**Date**: April 08, 2025.

---

#### Architecture

##### High-Level Overview
- **Backend**: Rust-based server handles API requests, data processing, and Postgres interactions.  
- **Database**: Postgres stores cached financial data and static municipality details.  
- **Frontend**: SvelteKit delivers a static, reactive UI with Leaflet.js for maps.  
- **Deployment**: Fly.io hosts the full stack (Rust + Postgres + Svelte).

##### Data Flow
1. **External API**: Rust fetches data from Municipal Money API (http://municipaldata.treasury.gov.za/api).  
2. **Processing**: Rust normalizes data, calculates scores, and caches in Postgres.  
3. **Internal API**: Rust serves processed data to Svelte via REST endpoints.  
4. **UI**: Svelte renders map, single, and comparison views with Leaflet.js.

---

#### Technology Stack

##### Backend
- **Framework**: Actix Web  
  - Why: Fast, async-capable, type-safe—ideal for high-performance data handling.  
  - Use: REST API (e.g., `/api/municipality/CPT`), data processing.  
- **Database Client**: sqlx  
  - Why: Type-safe, async Postgres queries with compile-time validation.  
  - Use: CRUD operations for `municipalities` and `financial_data` tables.  
- **Language**: Rust  
  - Why: Speed, memory safety, concurrency—core to the platform’s vision.

##### Database
- **DBMS**: PostgreSQL  
  - Why: Robust, supports geographic data (via PostGIS), allows manual manipulation.
  - Extensions: PostGIS - used for the `geom` column in `municipal_geometries` to store spatial data.
- **Schema Source of Truth**: The definitive schema is maintained in `schema.sql`. The definitions below reflect that file.
- **Tables**:
  - `municipalities`: Stores static details for South African municipalities.
    ```sql
    -- From schema.sql
    CREATE TABLE municipalities (
        id varchar NOT NULL PRIMARY KEY, -- Corresponds to municipal_geometries.munic_id
        "name" text NOT NULL,
        province text NOT NULL,
        population real NULL,
        classification text NULL,
        address text NULL,
        website text NULL,
        phone text NULL,
        district_id varchar NULL,
        district_name text NULL
    );
    ```
  - `municipal_geometries`: Stores geographic boundaries for South African municipalities.
    ```sql
    -- From schema.sql
    CREATE TABLE municipal_geometries (
        ogc_fid serial4 NOT NULL PRIMARY KEY,
        geom public.geometry(geometry, 4326) NULL, -- PostGIS geometry type
        munic_id varchar NOT NULL,
        CONSTRAINT municipal_geometries_municipalities_fk FOREIGN KEY (munic_id) REFERENCES public.municipalities(id) ON DELETE CASCADE ON UPDATE CASCADE
    );
    CREATE INDEX municipal_geometries_geom_geom_idx ON public.municipal_geometries USING gist (geom);
    CREATE INDEX municipal_geometries_munic_id_idx ON public.municipal_geometries USING btree (munic_id);
    ```
  - `financial_data`: Stores cached financial metrics and calculated scores.
    ```sql
    -- From schema.sql
    CREATE TABLE financial_data (
        id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
        municipality_id varchar NULL, -- References municipalities.id
        "year" int4 NOT NULL,
        revenue numeric NULL,
        expenditure numeric NULL,
        capital_expenditure numeric NULL,
        debt numeric NULL,
        audit_outcome text NULL,
        score numeric NULL,
        created_at timestamptz DEFAULT now() NULL,
        CONSTRAINT financial_data_municipalities_fk FOREIGN KEY (municipality_id) REFERENCES public.municipalities(id) ON DELETE SET NULL ON UPDATE CASCADE
    );
    CREATE INDEX financial_data_municipality_id_year_idx ON public.financial_data USING btree (municipality_id, year);
    ```

##### Frontend
- **Framework**: SvelteKit  
  - Why: Lightweight, reactive, supports static output for simple deployment.  
  - Use: Routing (map → single → comparison), data fetching from Rust API.  
- **Map Library**: Leaflet.js (via `svelte-leaflet`)  
  - Why: Lightweight, GeoJSON-ready, fits MVP’s simplicity.  
  - Use: Choropleth map with zoom and hover effects.  
- **CSS**: Tailwind CSS  
  - Why: Rapid, utility-first styling for warm, responsive design.  
  - Use: Layouts, cards, animations (via Tailwind classes).  
- **Icons**: Iconify (`@iconify/svelte`)  
  - Why: Lightweight, customizable icons for metrics.  
- **Animations**: Svelte Built-ins  
  - Why: Native `fade`, `slide`, and `scale`—no extra dependencies.  
  - Use: View transitions, pulsing badges.

##### Deployment
- **Platform**: Fly.io  
  - Why: Free tier (1 CPU, 256MB RAM), supports Rust + Postgres, easy scaling.  
  - Setup: Rust app + Postgres instance + static Svelte files.

---

#### Project Structure

The project is organized into the following main directories at the root (`c:\Users\kesha\CascadeProjects\seemycity`):

*   **/seemycity-frontend**: Contains the SvelteKit frontend application code. All frontend-related commands (like `npm install`, `npm run dev`) should be run from within this directory.
*   **/seemycity-backend** (Planned): This directory will house the Rust (Actix Web) backend application code.
*   **/docs**: Contains project documentation files like `plan.md` and `tech.md`.
*   **/database** (Potential): May be used later for database-specific files like migrations or more complex seeding scripts. (Currently, `schema.sql` resides in the root).
*   **.gitignore**: Specifies intentionally untracked files that Git should ignore.
*   **schema.sql**: Defines the initial database schema.
*   **README.md** (To be created): General project overview and setup instructions.

This structure separates the frontend and backend concerns clearly.

---

#### Technical Approach

##### Data Sources

1.  **Municipal Money API (National Treasury)**
    *   **Website**: [municipaldata.treasury.gov.za](https://municipaldata.treasury.gov.za/)
    *   **API Base URL**: `https://municipaldata.treasury.gov.za/api/`
    *   **Structure**: The API uses the [Cubes](https://cubes.readthedocs.io/en/latest/) framework for OLAP-style data browsing. Key concepts:
        *   **Cubes**: Represent individual datasets (e.g., `municipalities`, `incexp_v2`, `cflow_v2`).
        *   **Dimensions**: Categorical attributes used for slicing and dicing data (e.g., `municipality`, `financial_year`, `province`). Dimensions have **Members** (their distinct values, like 'CPT', 'JHB', 'WC', 'GP') and **Attributes** (details about members, like `municipality.long_name`, `municipality.province_code`).
        *   **Measures**: Numerical values that can be aggregated (e.g., `amount` in financial datasets).
    *   **Key Endpoint Types**:
        *   `/cubes/{cube_name}/model`: Describes the structure (dimensions, measures) of a specific cube. Useful for understanding what data is available.
        *   `/cubes/{cube_name}/members/{dimension_name}`: Lists the distinct members (and their attributes) for a given dimension within a cube. **Crucial for getting static lists like municipalities (`/cubes/municipalities/members/municipality`).**
        *   `/cubes/{cube_name}/facts`: Retrieves raw, detailed data records (facts) linking dimension members to measures. Allows filtering using `?cut=...`.
        *   `/cubes/{cube_name}/aggregate`: Returns summarized data, aggregating measures based on specified dimensions using `?drilldown=...` and `&aggregates=...`.
    *   **Usage Plan**:
        *   Use `/cubes/municipalities/members/municipality` to fetch the initial list of municipalities and their basic details (code, name, province, classification) for the `municipalities` table.
        *   Use `/aggregate` or `/facts` endpoints on financial cubes (e.g., `incexp_v2`, `audit_opinions`) for the `financial_data` table, filtering by municipality and year.

2.  **Static Geospatial Data (Boundaries)**
    *   **Source**: [Municipal Demarcation Board via ArcGIS Hub](https://spatialhub-mdb-sa.opendata.arcgis.com/) (Specifically datasets like "Local Municipal Boundary").
    *   **Format**: GeoJSON (preferred for easy use with PostGIS and Leaflet).
    *   **Requirement**: Must contain municipality boundaries that can be matched to the codes retrieved from the Municipal Money API.
    *   **Storage**: `municipal_geometries.geom` column (PostGIS `geometry` type).

3.  **Static Population Data**
    *   **Source**: To be determined (e.g., StatsSA Census data or estimates).
    *   **Requirement**: Population figures per municipality, matchable to codes from the Municipal Money API.
    *   **Storage**: `municipalities.population` column.

##### Data Ingestion
- **Source**: Municipal Money API (GET requests, JSON responses).  
  - Example: `GET /api/cubes/income_expenditure?municipality=CPT&year=2024`.  
- **Static Data**:  
  - GeoJSON: Sourced from OpenStreetMap or SA government portals.  
  - Population: Sourced from StatsSA (e.g., 2022 estimates).  
  - Loaded manually into Postgres via CSV/JSON import.  
- **Process**:  
  1. Rust (Actix) fetches API data on demand or refresh.  
  2. Normalizes (e.g., `revenue / population`) and calculates score.  
  3. Stores in Postgres with `sqlx`.

##### Data Processing
- **Scoring**:  
  - Rust function:  
    ```rust
    fn calculate_score(revenue: f64, population: i32, debt: f64, expenditure: f64, capital_expenditure: f64, audit: &str) -> f64 {
        let rev_per_capita = (revenue / population as f64).min(10000.0) / 10000.0 * 100.0; // Normalize
        let debt_ratio = (debt / revenue).min(1.0) * 100.0; // 0-100%
        let capex_ratio = (capital_expenditure / expenditure) * 100.0;
        let exp_ratio = (expenditure / revenue).min(2.0) * 100.0; // Cap at 200%
        let audit_score = match audit {
            "Clean" => 100.0,
            "Qualified" => 50.0,
            _ => 0.0,
        };
        0.30 * rev_per_capita + 0.25 * capex_ratio + 0.25 * (100.0 - exp_ratio) + 0.20 * audit_score
    }
    ```
  - Output: Stored in `financial_data.score`.

##### API Endpoints
- **Rust (Actix)**:  
  - `GET /api/municipalities`: List all (id, name, score, geojson).  
  - `GET /api/municipality/{id}`: Single municipality details + metrics.  
  - `POST /api/refresh/{id}`: Update financial data from API.  
- **Response**: JSON (e.g., `{ "id": "CPT", "score": 84, "metrics": {...} }`).

##### Frontend Rendering
- **SvelteKit**:  
  - Routes: `/` (map), `/[id]` (single), `/compare/[ids]` (comparison).  
  - Fetches: `fetch('/api/municipality/CPT')` in `+page.svelte`.  
- **Leaflet**:  
  - Loads `geojson` from `/api/municipalities`, applies choropleth styles.  
  - Events: Hover (tooltip), click (route to single view).  
- **Tailwind**: Classes like `bg-cream text-charcoal p-4 rounded-lg` for cards.

##### Performance
- **Backend**: Async Rust + Postgres indexing (municipality_id, year).  
- **Frontend**: Lazy-load map tiles, memoize Svelte components.  
- **Target**: <2s initial load (cached), <5s API fetch.

---

#### Dependencies
- **Rust**: `actix-web`, `sqlx`, `serde`, `tokio`.  
- **Svelte**: `sveltekit`, `svelte-leaflet`, `tailwindcss`, `@iconify/svelte`.  
- **Postgres**: PostGIS extension.  
- **Build**: Cargo (Rust), npm (Svelte).

#### Security
- **API**: No auth for MVP (public data); rate-limit endpoints.  
- **Postgres**: Connection string in env vars (e.g., `.env`).  
- **Svelte**: Static output avoids server-side risks.

#### Scalability
- **Rust**: Actix handles concurrent requests natively.  
- **Postgres**: Sharding or replication if data grows (post-MVP).  
- **Fly.io**: Scale CPU/RAM as needed.

---

#### Assumptions
- API returns consistent JSON structures.  
- GeoJSON aligns with Municipal Money IDs.  
- Fly.io free tier suffices for MVP load.

#### Risks
- **API Rate Limits**: Cache mitigates; monitor usage.  
- **GeoJSON Size**: Large files may slow map—optimize or simplify.  
- **Postgres Setup**: Manual import needs testing.

---

### Feedback
This tech spec locks in the stack (Rust/Actix, Postgres/sqlx, SvelteKit/Leaflet) and outlines a clear approach from data to UI. It’s high-performance yet simple for the MVP. Any concerns or tweaks (e.g., endpoint names, tools) before I move to `plan.md`?