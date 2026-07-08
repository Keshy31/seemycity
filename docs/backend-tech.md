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
  - Error handling leverages `AppError::SqlxError(#[from] sqlx::Error)` for automatic conversion.
  - Key query functions (`src/db/`):
    *   `municipalities::get_municipality_base_info_db`: static info for one municipality (`MunicipalityDb`).
    *   `municipalities::get_municipalities_summary_for_map`: map summary (id, name, province, population, latest score, simplified geometry) → `Vec<MapFeature>`.
    *   `municipalities::get_all_municipality_populations`: (id, population) list for the cache warmer.
    *   `financials::get_all_financial_years_db`: all cached year-rows (incl. timestamps) for one municipality.
    *   `financials::upsert_complete_financial_record`: `INSERT ... ON CONFLICT (municipality_id, year) DO UPDATE` of raw data + scores.
  - Compile-time checking works offline via the committed `.sqlx/` data (`cargo sqlx prepare` after query changes).

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

#### Core Framework

*   **Language:** Rust (Stable)
*   **Web Framework:** Actix Web
    *   Used for handling HTTP requests, routing, and middleware.
*   **Asynchronous Runtime:** Tokio (integrated via `#[actix_web::main]` and `#[tokio::test]`)

---

#### Database Interaction

*   **Database:** PostgreSQL
*   **Extension:** PostGIS (for geospatial queries)
*   **ORM/Query Builder:** `sqlx`
    *   Chosen for its compile-time query checking and async support.
    *   Connection pooling is managed via `sqlx::postgres::PgPoolOptions`.
    *   Error handling leverages `AppError::SqlxError(#[from] sqlx::Error)` for automatic conversion.
    *   Key query functions: see the list under "Database Client" above (single source).

---

#### API Client (Municipal Money)

*   **Purpose:** Interacts with the South African National Treasury's Municipal Money open data portal API.
*   **HTTP Client:** `reqwest`
    *   Used for making asynchronous HTTP GET requests to the API.
*   **Serialization/Deserialization:** `serde` / `serde_json`
    *   Used for parsing JSON responses from the API into Rust structs.
*   **Error Handling:** `thiserror`
    *   Used to define custom, structured error types (`ApiClientError`) for better error propagation and handling.
*   **Structure (`src/api/muni_money/`):**
    *   `client.rs`: Contains the main `MunicipalMoneyClient` struct, manages the `reqwest` client, and handles generic request logic.
    *   `types.rs`: Defines structs representing the API's JSON response structure (e.g., generic `FactsApiResponse`, specific `AuditApiResponse`, `Cell`, `FinancialFact`, `AuditOpinionFact`) and the custom `ApiClientError` enum.
    *   `financials.rs`: Contains functions specific to fetching financial data points (e.g., `get_total_revenue`, `get_total_operational_expenditure`), including logic to handle specific API parameters (item codes, amount types).
    *   `audit.rs`: Contains functions specific to fetching audit outcome data (`get_audit_outcome`).
*   **Status:** Core client logic implemented. Audit outcome fetching refactored for type safety and integration tests pass (as of 2025-04-15). Financial data fetchers verified.

---

#### Project Structure

*   **Type:** Library (`lib.rs`) and Binary (`main.rs`).
*   **Reasoning:** This structure allows integration tests (which are separate binaries) to easily import and use code defined within the main application's library (`seemycity_backend`).
*   **Modules:** Code is organized into modules (`api`, `db`, `handlers`, `models`, `config`, `errors`) for better separation of concerns.

---

#### Backend Architecture

##### Proposed File Structure (Refactored)

```
seemycity-backend/
├── Cargo.toml
├── .env
├── .gitignore
├── src/
│   ├── main.rs         # Entry point, server setup
│   ├── api/
│   │   ├── mod.rs      # Declares muni_money module, re-exports client/types
│   │   └── muni_money/ # Municipal Money API interaction logic
│   │       ├── mod.rs      # Declares submodules
│   │       ├── types.rs    # API-specific structs (FactsApiResponse, AuditApiResponse, FinancialFact, AuditOpinionFact, etc.), errors
│   │       ├── client.rs   # MunicipalMoneyClient, base reqwest logic
│   │       ├── financials.rs # get_total_revenue, get_total_operational_expenditure, etc.
│   │       └── audit.rs    # get_audit_outcome
│   ├── config.rs       # Configuration loading
│   ├── db/
│   │   ├── mod.rs      # Declares submodules
│   │   ├── municipalities.rs # Queries for municipalities table
│   │   ├── financials.rs   # Queries for financial_data table
│   │   ├── geometries.rs   # Queries for municipal_geometries table
│   ├── handlers/
│   │   ├── mod.rs      # Declares municipalities module
│   │   └── municipalities.rs # Actix request handlers (/api/...)
│   ├── models.rs       # Core application data structures (shared between layers)
│   ├── scoring.rs      # Financial score calculation logic
│   └── errors.rs       # Application-level error types (AppError)
├── target/             # Compiled output
└── tests/              # Integration/Unit tests (to be added)
    └── muni_money_integration_test.rs # Tests hitting the live MuniMoney API
```

##### API Endpoints

- **`/api/municipalities`**: Returns a GeoJSON `FeatureCollection` containing all municipalities.
    *   Each feature includes basic properties like `id`, `name`, `province`, `population`, and the latest calculated `overall_score`.
    *   Fetches data primarily from the `municipalities` and `municipal_geometries` tables, joining with `financial_data` to get the latest score using `get_municipalities_summary_for_map`.
    *   Handler: `get_municipalities_list_handler`.
    *   Payload defined in: `docs/data-spec.md#31-map-view-payload-apimunicipalities`.
- **`/api/municipalities/{id}`**: Returns detailed information for a single municipality specified by its ID (e.g., "BUF").
    *   **Data Flow & Caching:** See detailed flow below.
    *   **Response:** Constructs and returns a `MunicipalityDetail` object containing base info, geometry, and an array of `FinancialYearData` (including scores).
    *   Handler: `get_municipality_detail_handler`.
    *   Payload defined in: `docs/data-spec.md#32-detail-view-payload-apimunicipalitiesid`.
- **`/api/health`**: Simple health check endpoint.

See the dedicated [`docs/data-spec.md`](./data-spec.md#2-database-schema-postgresql--postgis) for the complete database schema definition.

---

#### Data Structures & API Payloads

The canonical definitions for all backend data structures (Rust structs) and API payloads are maintained in [data-spec.md](./data-spec.md#1-core-data-structures). Please refer to that document for up-to-date field lists and type details.

**Backend-specific notes:**
- Database mapping and serialization logic live in `src/models.rs`.
- Any deviations or extensions should be documented in data-spec.md and referenced here.

---

#### Financial Score Calculation

The financial health score calculation (pillars, weights, normalization, and formulas) follows the canonical rubric defined in [prd.md](./prd.md#2-functional-requirements). Please refer to that document for the latest details and rationale.

**Backend implementation note:**  
The calculation logic is implemented in `src/scoring.rs` and invoked by API handlers when new financial data is fetched or updated.

---

#### Data Flow (`/api/municipalities/{id}` Handler) — *as implemented July 2026*

1.  **Extract ID & base info:** 404 if the municipality is unknown.
2.  **`ensure_financials_fresh`** (shared with the cache warmer):
    a.  Loads all cached `financial_data` rows for the municipality.
    b.  **Walks candidate years newest-first** (`current_year - 1` back through `YEAR_FALLBACK_DEPTH = 3`) until one yields a **scorable** row (all four pillars → `overall_score IS NOT NULL`). The newest year often publishes figures months before its audit opinion, so "any data" is not enough to stop.
    c.  A cached row younger than **`CACHE_TTL_DAYS = 7`** is trusted as-is — including an all-NULL row, which acts as a **negative cache** ("upstream has no data for this year").
    d.  A missing/expired row triggers a full refresh: **4 concurrent upstream calls** (`tokio::join!`) — one `incexp_v2` fetch shared by revenue *and* opex (`get_revenue_and_expenditure`), plus capex, debt, audit. Individual failures degrade to NULL fields.
    e.  If **every** call fails at transport level, nothing is persisted (an outage must never masquerade as "no data") and the **`UpstreamHealth` circuit breaker** opens for 5 minutes — subsequent requests serve cached (even stale) data instantly.
    f.  **Score healing:** scores are re-derived from stored raw metrics for every cached row; rows whose stored scores disagree with the current formula are upserted with corrected values. Formula changes therefore propagate to all history (and the map) lazily, with zero upstream calls.
3.  **Response:** all-NULL negative-cache rows are filtered out; remaining years sorted newest-first into `financials[]`. `geometry` is intentionally `null` (the detail view renders no map; the map endpoint serves simplified geometry).

#### Map endpoint (`GET /api/municipalities`)

- Single SQL query: `ROW_NUMBER()` CTE for each municipality's latest non-NULL score + `ST_AsGeoJSON(ST_SimplifyPreserveTopology(geom, 0.002), 5)` — payload ~941 KB raw / ~305 KB gzipped (was 18 MB).
- Whole response cached in memory for 60 s (`MapResponseCache`), `Cache-Control: public, max-age=60`; ~15-30 ms warm in release builds.
- Canonical score property name: **`overall_score`** (shared with detail payload and DB column). NULL = "no data" → grey on the map.
- `?limit=` must be positive (400 otherwise).

#### Background cache warmer

`warm_all_municipalities` runs 15 s after startup and every 24 h (disable with `CACHE_WARMER=false`): iterates all municipalities through `ensure_financials_fresh`, skipping fresh rows (repeat passes are near-free), aborting early if the circuit breaker opens. Keeps the map fully scored without depending on detail-page traffic. Live result 2026-07-07: 204/213 scored in 72 s.

---

#### Testing

*   **Framework:** Rust's built-in test framework (`#[test]`, `cargo test`).
*   **Integration Tests:** Placed in the `tests/` directory.
    *   Use `#[tokio::test]` for async test functions.
    *   API-dependent tests are marked with `#[ignore]` to prevent running them automatically during regular `cargo test` runs (run via `cargo test -- --ignored`).
    *   Assertions involving `Option<Decimal>` values returned from API functions compare against `Option<Decimal>` values, e.g., `assert!(result >= Some(Decimal::ZERO))`.

---

#### Municipal Money API Data Fetching Logic — *current implementation + known issues*

All financial queries hit `/cubes/{cube}/aggregate` cut by `demarcation.code`, `financial_period.period` (or `financial_year_end.year` for audits/UIFW) and `amount_type.code:AUDA` (audited actuals).

**As implemented (`src/api/muni_money/financials.rs`):**

1.  **Revenue + Operational Expenditure** — one shared `incexp_v2` fetch (`get_revenue_and_expenditure`); revenue sums item codes `0200`–`2500`, opex sums `3000`–`4000`.
2.  **Capital Expenditure** — `capital_v2`, sum of all returned items.
3.  **Debt** — `financial_position_v2`, sum of item codes 310–500 (total-liabilities proxy).
4.  **Audit Outcome** — `audit_opinions` cube, cut by `financial_year_end.year`, first cell's `opinion.label`.

**✓ Item sets re-pinned and AFS-validated (2026-07-07, Phase 8-A1b):**

The `incexp_v2` sums now use numeric ranges validated against Cape Town's audited
FY2024 AFS (note 37.4.1 budget reconciliation, mSCOA basis) and a rollup-identity
check across 8 municipalities:

- **Revenue = items `0200`–`2800`** (mSCOA operating revenue: rates, service
  charges, transfers, fines, gains). CPT check: cube 61.84bn vs AFS-mSCOA
  61.47bn (+0.6%).
- **Expenditure = items `3000`–`4300`** (payroll through other losses, including
  `4100` operational cost). CPT check: cube 58.67bn vs AFS-mSCOA 58.45bn (+0.4%).
- **Item `2900` "Other expenditure" is a mislabeled TOTAL-REVENUE rollup** —
  `|2900 − Σrevenue| = 0.00%` for every municipality tested. Excluded from both
  ranges; usable as a per-municipality checksum by the data-confidence layer.
- Operational transfers (grants) are item **`2200`** — the basis for the coming
  own-revenue metric. (Item `1600` is generic "Operational Revenue", *not*
  transfers, whatever older notes claimed.)
- `4600`/`4700` (capital transfers received) and `4900`+ (tax/JV/minorities)
  stay excluded from operating figures.
- Note: figures are on the **mSCOA basis** (grossed up ~9% vs GRAP by inventory
  classification); ratios remain internally consistent since both sides gross up.
- Beware: **OpenUpSA's `municipal-data` repo `codes.py` does not match the live
  cube's facts** — the cube itself + published AFS are the only trustworthy
  references.
- Unit tests cover range membership and the 2900 exclusion
  (`src/api/muni_money/financials.rs::tests`).

**Planned additional cubes (validated by probe):**

- **`uifwexp`** — unauthorised / irregular / fruitless & wasteful expenditure; keyed by `financial_year_end.year` + `item` (no amount_type). Feeds accountability v2.
- **`repmaint_v2`** — repairs & maintenance, standard AUDA shape. Feeds infrastructure v2 (Treasury norm: 8% of asset value).
- `aged_debtor_v2` (collection rates) and `cflow_v2` (liquidity) are v3 candidates.

**Upstream reliability caveat:** the Treasury API can return **empty-but-HTTP-200 responses while degraded** (observed 2026-07-07 — it produced 9 false "no data" municipalities including eThekwini). Transport failures are handled by the circuit breaker; *empty successes are not detectable today* and are a requirement on the Phase 8-A data-confidence layer.

---

#### Handlers (`src/handlers/municipalities.rs`)

*   **`get_municipality_detail_handler`** — `GET /api/municipalities/{id}`; see "Data Flow" above.
*   **`get_municipalities_list_handler`** — `GET /api/municipalities`; see "Map endpoint" above.
*   **Shared plumbing in the same module:** `ensure_financials_fresh` (year-walk + healing, used by handler and warmer), `refresh_financial_year` (one muni-year fetch/score/upsert round), `warm_all_municipalities`, `MapResponseCache`, `UpstreamHealth` (circuit breaker).

---

#### API Endpoints

*   **`GET /api/municipalities`**
    - Fetches GeoJSON FeatureCollection for the map view.
    - Handler: `get_municipalities_list_handler`.
    - Query: `get_municipalities_summary_for_map`.
    - Returns a `geojson::FeatureCollection`.
*   **`GET /api/municipalities/{id}`**
    - Fetches detailed info for a single municipality (identified by `id`), including an array of all available historical financial data (`financials`).
    - Handler: `get_municipality_detail_handler`.
    - Query: `get_municipality_detail`.
    - Returns a `MunicipalityDetail` struct (containing the `financials` array).

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
