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
    *   `municipalities::get_all_municipalities_basic`: Fetches basic info (id, name, province) for all municipalities.
    *   `municipalities::get_municipality_base_info_db`: Fetches detailed static info for a single municipality from the `municipalities` table (`MunicipalityDb` struct).
    *   `municipalities::get_municipalities_summary_for_map`: Fetches summary data (id, name, province, latest score, geometry) for the map view. Returns `Vec<MapFeature>`.
    *   `financials::get_financial_data_for_year`: Fetches cached financial data and scores for a specific municipality and year. Returns `Option<FinancialRecord>`.
    *   `financials::upsert_complete_financial_record`: Accepts raw financial data and calculated scores. Performs an `INSERT ... ON CONFLICT DO UPDATE SET ...` to save/update the record, including scores, for a given municipality and year.
    *   `geometries::get_municipality_geojson`: Fetches the GeoJSON geometry for a single municipality.

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
    *   Key query functions (`src/db/`):
        *   `municipalities::get_municipality_base_info_db`: Fetches detailed static info for a single municipality from the `municipalities` table (`MunicipalityDb` struct).
        *   `municipalities::get_municipalities_summary_for_map`: Fetches summary data (id, name, province, latest score, geometry) for the map view. Returns `Vec<MapFeature>`.
        *   `financials::get_financial_data_for_year`: Fetches cached financial data and scores for a specific municipality and year. Returns `Option<FinancialRecord>`.
        *   `financials::upsert_complete_financial_record`: Accepts raw financial data and calculated scores. Performs an `INSERT ... ON CONFLICT DO UPDATE SET ...` to save/update the record, including scores, for a given municipality and year.
        *   `geometries::get_municipality_geojson`: Fetches the GeoJSON geometry for a single municipality.

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

#### Data Flow (`/api/municipalities/{id}` Handler)

1.  **Extract ID:** The handler receives the municipality ID (`muni_id`) from the URL path.
2.  **Fetch Base Info:** It calls `db::municipalities::get_municipality_base_info_db` to retrieve static details (name, province, population) for the `muni_id` from the `municipalities` table. If not found, returns a 404 Not Found.
3.  **Determine Target Year:** Calculates the most recent financial year needed (e.g., `current_year - 1`).
4.  **Cache Check:**
    a.  Calls `db::financials::get_financial_data_for_year` to check for existing data and scores for the `muni_id` and `target_year`.
    b.  If found, checks the `updated_at` timestamp against `CACHE_TTL_SECONDS`.
    c.  If recent enough, converts the cached `FinancialRecord` to `FinancialYearData` and stores it in `financial_data_to_use`. Skips to Step 8.
5.  **Fresh Data Fetch (if Cache Miss or Stale):** If `financial_data_to_use` is `None`:
    a.  Logs the fetch action.
    b.  Initiates **concurrent** calls to the `MunicipalMoneyClient` for `muni_id` and `target_year` to get raw financial figures (revenue, operational_expenditure, capex, debt, audit outcome).
    c.  Awaits results using `tokio::join!`. Propagates errors.
6.  **Score Calculation:**
    a.  Constructs a `ScoringInput` struct using the fetched financial data and the population from the base info (Step 2).
    b.  Calls `scoring::calculate_financial_score` with the `ScoringInput`.
    c.  Handles the `Option<ScoreBreakdown>` result. If `None`, logs a warning but proceeds (scores will be missing).
7.  **Upsert Data & Scores:**
    a.  Calls `db::financials::upsert_complete_financial_record` to save/update the fetched raw financial data *and* the calculated `ScoreBreakdown` (if available) into the `financial_data` table for `muni_id` and `target_year`.
    b.  Converts the upserted `FinancialRecord` (which includes scores) into `FinancialYearData` and stores it in `financial_data_to_use`.
8.  **Fetch Geometry:** Calls `db::geometries::get_municipality_geojson` to retrieve the geometry.
9.  **Construct Response:** Combines `base_info`, `financial_data_to_use`, and `geometry` into the `MunicipalityDetail` response struct.
10. **Return JSON:** Serializes `MunicipalityDetail` to JSON and returns HTTP 200 OK.

---

#### Testing

*   **Framework:** Rust's built-in test framework (`#[test]`, `cargo test`).
*   **Integration Tests:** Placed in the `tests/` directory.
    *   Use `#[tokio::test]` for async test functions.
    *   API-dependent tests are marked with `#[ignore]` to prevent running them automatically during regular `cargo test` runs (run via `cargo test -- --ignored`).
    *   Assertions involving `Option<Decimal>` values returned from API functions compare against `Option<Decimal>` values, e.g., `assert!(result >= Some(Decimal::ZERO))`.

---

#### Municipal Money API Data Fetching Logic

The backend retrieves the core financial metrics for scoring as follows. All queries should filter by the target `demarcation.code` (municipality) and `financial_period.period` (year). The appropriate `amount_type.code` should be prioritized (e.g., 'AUDA' - Audited Actual) using the `/aggregate` endpoint.

1.  **Total Revenue ([`get_total_revenue`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:10:0-58:1))**
    *   **Cube**: `incexp_v2`
    *   **Method**: Use the `fetch_incexp_aggregate` function which calls the `/aggregate` endpoint with `drilldown=demarcation.code|demarcation.label|item.code|item.label`, `cut=amount_type.code:AUDA|financial_period.period:{year}|demarcation.code:"{muni_code}"`, and `aggregates=amount.sum`. Sum the `amount.sum` for the following `item.code`s observed in the response (example from CPT, 2023, AUDA):
        *   `0200`: Property rates - *(Added based on CPT 2023 data)*
        *   `0300`: Service charges - Electricity revenue
        *   `0400`: Service charges - Water revenue
        *   `0500`: Service charges - Sanitation revenue
        *   `0600`: Service charges - Refuse revenue
        *   `0700`: Rental of facilities and equipment - *(Label updated based on CPT 2023 data)*
        *   `0800`: Interest earned - external investments
        *   `1000`: Interest earned - outstanding debtors - *(Label updated based on CPT 2023 data)*
        *   `1100`: Dividends received - *(Label updated based on CPT 2023 data)*
        *   `1200`: Fines, Penalties and Forfeits - *(Added based on CPT 2023 data)*
        *   `1300`: Licences and permits - *(Added based on CPT 2023 data)*
        *   `1400`: Agency services - *(Label updated based on CPT 2023 data)*
        *   `1600`: Transfers recognised - operational
        *   `1700`: Other revenue - *(Added based on CPT 2023 data)*
        *   `1800`: Gains on disposal of PPE - *(Added based on CPT 2023 data)*
    *   *Note: This list is based on observed data for CPT/2023/AUDA and may differ for other municipalities/years. Codes like 0900, 1500, 1900-2700 from the previous list were not present or had zero amounts in the test data.*

2.  **Total Operational Expenditure ([`get_total_operational_expenditure`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:60:0-94:1))**
    *   **Cube**: `incexp_v2`
    *   **Method**: Sum the `amount.sum` for `item.code` = `2000` (`Operational Expenditure`) filtering by `amount_type.code:AUDA`. This corresponds to the `Total Operating Expenditure` item. Returns `Option<Decimal>`. 

3.  **Capital Expenditure ([`get_capital_expenditure`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:147:0-186:1))**
    *   **Cube**: `capital_v2`
    *   **Method**: Sum the `amount.sum` for `item.code` = `4100` (`Total Capital Expenditure`) filtering by `amount_type.code:AUDA`. Returns `Option<Decimal>`. 
4.  **Audit Outcome ([`get_audit_outcome`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/audit.rs:9:0-50:1))**
    *   **Cube**: `audit_opinions_v2`
    *   **Method**: Uses the `fetch_audit_opinion` function, which queries the `/facts` endpoint with filters for `demarcation.code`, `financial_year.year`, and `latest_opinion.label`. It extracts the `latest_opinion.label` value from the first fact in the response.
    *   *Note: Assumes the API returns a single, relevant fact.* Returns `Option<String>`.

---

#### Handlers (`src/handlers/municipalities.rs`)

*   **`get_municipality_detail_handler`**: 
    *   Handles `GET /api/municipalities/{municipality_code}`.
    *   Fetches base static municipality details using `get_municipality_base_info_db`.
    *   Fetches financial details (revenue, operational_expenditure, debt, audit) for the specified year from the Municipal Money API via the `MunicipalMoneyClient`.
    *   Combines the base info and fetched financial data into a `MunicipalityDetail` response.
    *   Uses `upsert_complete_financial_record` to save the fetched financial metrics to the database.
    *   Currently, caching logic using `crate::utils::cache::Cache` is commented out.

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
