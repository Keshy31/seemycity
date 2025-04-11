### `backend-tech.md` - Backend Technical Specifications

This document outlines the technical details for the Rust backend of the Municipal Financial Dashboard.

---

#### Technology Stack

##### Backend Framework
- **Framework**: Actix Web
  - Why: Mature, high-performance, async-capable web framework for Rust. Excellent community support and ecosystem.
  - Use: Building REST API endpoints, handling HTTP requests/responses, middleware.

##### Database Client
- **Library**: sqlx
  - Why: Type-safe, async SQL query builder and executor with compile-time checking against the database schema. Prevents runtime errors and SQL injection vulnerabilities.
  - Use: Interacting with the PostgreSQL database for caching API data and retrieving static information.

##### Asynchronous Runtime
- **Library**: Tokio
  - Why: De facto standard async runtime for Rust, underpinning Actix Web and sqlx. Provides tools for managing async tasks, I/O, and timers.
  - Use: Enables non-blocking operations for handling concurrent API requests and database queries efficiently.

##### HTTP Client
- **Library**: reqwest
  - Why: Ergonomic, async HTTP client for making requests to external APIs (Municipal Money API).
  - Use: Fetching data from the Municipal Money API endpoints.

##### Serialization/Deserialization
- **Library**: Serde
  - Why: Standard Rust library for efficiently serializing Rust data structures into formats like JSON and deserializing JSON responses from APIs into Rust types.
  - Use: Parsing JSON responses from Municipal Money API, serializing data for API responses to the frontend.

##### Language
- **Language**: Rust
  - Why: Performance, memory safety without garbage collection, strong type system, excellent concurrency support. Aligns with the goal of a high-performance backend.

---

#### Backend Architecture

##### Proposed File Structure (Standard Actix Project)

```
seemycity-backend/       # Root directory for the backend project
├── Cargo.toml          # Rust project manifest (dependencies, metadata)
├── .env                # Environment variables (DATABASE_URL, API keys - *gitignore this!*)
├── .gitignore
├── src/                # Main source code
│   ├── main.rs         # Application entry point, Actix server setup
│   ├── api/            # Modules related to fetching data from external APIs
│   │   ├── mod.rs
│   │   └── municipal_money.rs # Logic for interacting with Treasury API
│   ├── config.rs       # Configuration loading (e.g., from .env)
│   ├── db/             # Modules for database interaction (sqlx)
│   │   ├── mod.rs
│   │   └── queries.rs    # SQL query functions (e.g., get_municipality_data, cache_data)
│   ├── handlers/       # Actix request handlers (define API endpoints)
│   │   ├── mod.rs
│   │   └── municipalities.rs # Handlers for /api/municipalities, /api/municipality/{id}
│   ├── models.rs       # Data structures (structs for API responses, DB records)
│   └── errors.rs       # Custom error types for the application
└── tests/              # Integration tests
    └── api_tests.rs
```

##### API Endpoints
- **`/api/municipalities`**: Returns a GeoJSON FeatureCollection containing all municipalities. Each feature includes basic info (id, name) and aggregated score in its `properties`, and the geometry converted from PostGIS using the `ST_AsGeoJSON` function in the SQL query.
- **`/api/municipality/{id}`**: Returns detailed financial data and score breakdown for a specific municipality ID (standard JSON, not GeoJSON).
- **`/api/health`**: Simple health check endpoint.

##### Database Schema

The database will store static municipality information, geometry, and cached financial data. The schema is defined as follows:

```sql
-- Enable PostGIS extension if not already enabled (Fly images usually have it, but this ensures it)
CREATE EXTENSION IF NOT EXISTS postgis;

-- Table to store municipality details 
CREATE TABLE municipalities (
	id varchar NOT NULL,          -- Corresponds to municipal_geometries.munic_id
	"name" text NOT NULL,
	province text NOT NULL,
	-- geojson column removed as requested
	population real NULL, -- Changed from int4 to real
	classification text NULL,
	address text NULL,
	website text NULL,
	phone text NULL,
	district_id varchar NULL,
	district_name text NULL,
	CONSTRAINT municipalities_pkey PRIMARY KEY (id)
);

-- Table to store municipality geometry
CREATE TABLE municipal_geometries (
	ogc_fid serial4 NOT NULL,
	geom public.geometry(geometry, 4326) NULL,
	munic_id varchar NOT NULL, -- Renamed from cat_b and set to NOT NULL
	CONSTRAINT municipal_geometries_pkey PRIMARY KEY (ogc_fid),
    CONSTRAINT municipal_geometries_municipalities_fk FOREIGN KEY (munic_id) REFERENCES public.municipalities(id) ON DELETE CASCADE ON UPDATE CASCADE -- Updated FK column name
);
CREATE INDEX municipal_geometries_geom_geom_idx ON public.municipal_geometries USING gist (geom);
CREATE INDEX municipal_geometries_munic_id_idx ON public.municipal_geometries USING btree (munic_id); -- Updated index column name and index name

-- Table to store cached financial data and scores
CREATE TABLE financial_data (
	id uuid NOT NULL DEFAULT gen_random_uuid(), -- Use default UUID generation
	municipality_id varchar NULL,
	"year" int4 NOT NULL,
	revenue numeric NULL,
	expenditure numeric NULL,
	capital_expenditure numeric NULL,
	debt numeric NULL,
	audit_outcome text NULL,
	score numeric NULL,
	created_at timestamptz DEFAULT now() NULL,
	CONSTRAINT financial_data_pkey PRIMARY KEY (id),
    CONSTRAINT financial_data_municipalities_fk FOREIGN KEY (municipality_id) REFERENCES public.municipalities(id) ON DELETE SET NULL ON UPDATE CASCADE -- Consider ON DELETE behavior
);
CREATE INDEX financial_data_municipality_id_year_idx ON public.financial_data USING btree (municipality_id, year);

-- Optional: Add comments for clarity (good practice)
COMMENT ON TABLE municipalities IS 'Stores static details for South African municipalities.';
COMMENT ON TABLE municipal_geometries IS 'Stores geographic boundaries for South African municipalities.';
COMMENT ON TABLE financial_data IS 'Stores cached financial metrics and calculated scores for municipalities, linked by municipality_id and year.';
```

##### Data Flow
1.  Frontend request hits an Actix handler.
2.  Handler checks local cache (Postgres) for requested data (municipality, year).
3.  **Cache Miss**:
    *   Use `reqwest` client (`api::municipal_money`) to call relevant Municipal Money API endpoints based on the logic below.
    *   Parse JSON responses using `serde`.
    *   Process data: Sum items for revenue/expenditure, map audit outcomes, calculate scores.
    *   Store processed data in Postgres using `sqlx` (`db::queries`).
    *   Return processed data to frontend.
4.  **Cache Hit**:
    *   Retrieve data directly from Postgres using `sqlx`.
    *   Return cached data to frontend.

---

#### Municipal Money API Data Fetching Logic

The backend retrieves the core financial metrics for scoring as follows. All queries should filter by the target `demarcation.code` (municipality) and `financial_year_end.year`. The appropriate `amount_type.code` should be prioritized (e.g., 'AUDA' - Audited Actual, 'ADJB' - Adjusted Budget, 'ORGB' - Original Budget) based on availability for the financial cubes.

1.  **Total Revenue (`revenue`)**
    *   **Cube**: `incexp_v2`
    *   **Method**: Sum the `amount` measure for the following `item.code`s:
        *   `0200`: Property Rates
        *   `0300`: Service Charges - Electricity Revenue
        *   `0400`: Service Charges - Water Revenue
        *   `0500`: Service Charges - Sanitation Revenue
        *   `0600`: Service Charges - Refuse Revenue
        *   `0700`: Service Charges - Other
        *   `0800`: Rental of Facilities and Equipment
        *   `0900`: Interest Earned - External Investments
        *   `1000`: Interest Earned - Outstanding Debtors
        *   `1100`: Dividends Earned
        *   `1200`: Fines, Penalties and Forfeits
        *   `1300`: Licences and Permits
        *   `1400`: Agency Services
        *   `1500`: Transfers Recognised - Operational
        *   `1600`: Transfers Recognised - Capital
        *   `1700`: Other Revenue
        *   `1800`: Gains on Disposal of PPE
        *   `1900`: Gains on Disposal of Investment Properties
        *   `2000`: Revenue from Recovery of Unauthorised, Irregular, Fruitless and Wasteful Expenditure
        *   `2100`: Fuel Levy Allocation
        *   `2200`: Library Services Revenue
        *   `2300`: Housing Services Revenue
        *   `2400`: Contributed Assets
        *   `2500`: Operational Revenue

2.  **Total Debt (`debt`)**
    *   **Cube**: `financial_position_v2`
    *   **Method**: Get the `amount` measure for `item.code`: `0500` (TOTAL LIABILITIES).

3.  **Total Expenditure (`expenditure`)**
    *   **Cube**: `incexp_v2`
    *   **Method**: Sum the `amount` measure for the following `item.code`s:
        *   `3000`: Employee Related Costs
        *   `3100`: Remuneration of Councillors
        *   `3200`: Bad Debts Written Off
        *   `3300`: Collection Cost
        *   `3400`: Depreciation and Asset Impairment
        *   `3500`: Finance Charges
        *   `3600`: Bulk Purchases
        *   `3700`: Other Materials
        *   `3800`: Contracted Services
        *   `3900`: Grants and Subsidies Paid
        *   `4000`: General Expenses
        *   `4100`: Operational Costs
        *   `4200`: Repairs and Maintenance
        *   `4300`: Loss on Disposal of PPE
        *   `4400`: Loss on Disposal of Investment Property
        *   `4500`: Unauthorised, Irregular, Fruitless and Wasteful Expenditure Written-off
        *   `4600`: Transfers and subsidies - capital (monetary allocations)
        *   `4700`: Transfers and subsidies - capital (in-kind)
        *   `4800`: Provision for Landfill Site Rehabilitation
        *   `4900`: Operating Lease Expense

4.  **Capital Expenditure (`capital_expenditure`)**
    *   **Cube**: `capital_v2`
    *   **Method**: Get the aggregate `amount.sum`.

5.  **Audit Outcome (`audit_outcome`)**
    *   **Cube**: `audit_opinions`
    *   **Method**: Fetch the `opinion.label` (or `opinion.code`) and map it to the frontend categories:
        *   `Unqualified - No findings` (`unqualified`) -> `'Clean'`
        *   `Unqualified - Emphasis of Matter items` (`unqualified_emphasis_of_matter`) -> `'Unqualified'`
        *   `Qualified` (`qualified`) -> `'Qualified'`
        *   `Adverse opinion` (`adverse`) -> `'Adverse'`
        *   `Disclaimer of opinion` (`disclaimer`) -> `'Disclaimer'`
        *   `Outstanding` (`outstanding`) -> `'Unavailable'`

---

#### Database Interaction

- **DBMS**: PostgreSQL + PostGIS extension.
- **Schema Source of Truth**: `schema.sql` in the project root.
- **Caching Strategy**: Store results from Municipal Money API (keyed by municipality code and year) in the `financial_data` table. Include a `last_updated` timestamp. Refresh cache based on a TTL (e.g., daily) or when new data is detected from the API.
- **Static Data**: The `municipalities` and `municipal_geometries` tables store static info loaded initially.

---

#### Performance
- Leverage Rust's async capabilities for non-blocking I/O (API calls, DB queries).
- Use database connection pooling (provided by `sqlx`).
- Implement efficient caching in Postgres to minimize external API calls.
- Use appropriate indexing on database tables (e.g., `municipality_id`, `year` in `financial_data`).
- Target API Response Time: < 500ms (cached), < 5 seconds (uncached API fetch).

---

#### Security
- **API Rate Limiting**: Implement rate limiting on backend endpoints (e.g., using Actix middleware) to prevent abuse.
- **Database Credentials**: Store `DATABASE_URL` in environment variables (`.env` file, ignored by Git) and load via configuration. Do not hardcode credentials.
- **Input Validation**: Sanitize and validate any user input used in database queries (though primarily reading from Treasury API here). `sqlx` helps prevent SQL injection.

---

#### Scalability
- Actix Web's actor model and async nature provide good concurrency.
- Stateless API design allows horizontal scaling (running multiple instances).
- Database performance can be scaled by optimizing queries, indexing, or eventually replication/sharding (Post-MVP).
- Deployment on Fly.io allows easy scaling of compute resources (CPU/RAM).

---

#### Dependencies
- `actix-web`
- `sqlx` (with features: `runtime-tokio-rustls`, `postgres`, `macros`, `chrono`, `json`)
- `tokio` (with features: `full`)
- `serde` (with features: `derive`)
- `serde_json`
- `reqwest` (with features: `json`)
- `dotenv`
- `config` (Optional, for more advanced config management)
- `chrono` (For timestamps)
