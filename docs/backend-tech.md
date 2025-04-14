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

#### Core Framework

*   **Language:** Rust (Stable)
*   **Web Framework:** Actix Web
    *   Used for handling HTTP requests, routing, and middleware.
*   **Asynchronous Runtime:** Tokio (integrated via `#[actix_web::main]` and `#[tokio::test]`)

---

#### Database Interaction

*   **Database:** PostgreSQL
*   **Extension:** PostGIS (for geospatial queries - planned)
*   **ORM/Query Builder:** `sqlx`
    *   Chosen for its compile-time query checking and async support.
    *   Connection pooling is managed via `sqlx::postgres::PgPoolOptions`.

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
    *   `financials.rs`: Contains functions specific to fetching financial data points (e.g., `get_total_revenue`, `get_total_debt`), including logic to handle specific API parameters (item codes, amount types).
    *   `audit.rs`: Contains functions specific to fetching audit outcome data (`get_audit_outcome`).
*   **Status:** Core client logic implemented. Audit outcome fetching refactored for type safety and integration tests pass (as of 2025-04-14). Financial data fetchers still need live API verification.

---

#### Configuration

*   **Method:** Environment Variables
*   **Loading:** `dotenvy` crate
    *   Loads variables from a `.env` file during development/testing.
    *   Reads system environment variables in production.
*   **Management:** A `config.rs` module loads and validates necessary configuration (e.g., `DATABASE_URL`, `MUNI_MONEY_API_URL`).

---

#### Testing

*   **Framework:** Rust's built-in test framework (`#[test]`, `cargo test`).
*   **Integration Tests:** Placed in the `tests/` directory.
    *   Use `#[tokio::test]` for async test functions.
    *   API-dependent tests are marked with `#[ignore]` to prevent running them automatically during regular `cargo test` runs (run via `cargo test -- --ignored`).

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
│   │       ├── financials.rs # get_total_revenue, get_total_debt, etc.
│   │       └── audit.rs    # get_audit_outcome
│   ├── config.rs       # Configuration loading
│   ├── db/
│   │   ├── mod.rs      # Declares queries module
│   │   └── queries.rs    # Database interaction functions (sqlx)
│   ├── handlers/
│   │   ├── mod.rs      # Declares municipalities module
│   │   └── municipalities.rs # Actix request handlers (/api/...)
│   ├── models.rs       # Core application data structures (shared between layers)
│   └── errors.rs       # Application-level error types (distinct from api::ApiClientError)
├── target/             # Compiled output
└── tests/              # Integration/Unit tests (to be added)
```

##### API Endpoints

- **`/api/municipalities`**: Returns a GeoJSON `FeatureCollection` containing municipality boundaries and basic data suitable for map display. See [`docs/data-spec.md`](./data-spec.md#31-get-apimunicipalities) for payload details.
- **`/api/municipalities/{id}`**: Returns detailed financial data and score breakdown for a specific municipality ID. See [`docs/data-spec.md`](./data-spec.md#32-get-apimunicipalityid) for payload details.
- **`/api/health`**: Simple health check endpoint.

See the dedicated [`docs/data-spec.md`](./data-spec.md#2-database-schema-postgresql--postgis) for the complete database schema definition.

##### Data Flow

1.  Frontend request hits an Actix handler.
2.  Handler checks local cache (Postgres) for requested data (municipality, year).
3.  If data is fresh enough (based on `financial_data.created_at`), return cached data.
4.  If data is stale or missing, fetch necessary data from the Municipal Money API using the `reqwest` client.
5.  Process the API response (calculate score, structure data).
6.  Store/update the processed data in the Postgres cache (`municipalities`, `financial_data` tables).
7.  Return the processed data to the frontend, formatted according to the API payload specification in [`docs/data-spec.md`](./data-spec.md#3-api-payloads).

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

2.  **Total Debt ([`get_total_debt`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:60:0-94:1))**
    *   **Cube**: `financial_position_v2`
    *   **Method**: Use `fetch_incexp_aggregate` (or a similar aggregate call for `financial_position_v2` if implemented, otherwise requires a separate function) to get the `amount.sum` measure for `item.code`: `0500` (TOTAL LIABILITIES), filtering by `amount_type.code:AUDA`. *(Note: This function currently uses `fetch_generic_financial_data` which was removed. This section needs updating once debt fetching is refactored).*

3.  **Total Expenditure ([`get_total_expenditure`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:96:0-145:1))**
    *   **Cube**: `incexp_v2`
    *   **Method**: Use the `fetch_incexp_aggregate` function (as described for Revenue). Sum the `amount.sum` for the following `item.code`s observed in the response (example from CPT, 2023, AUDA):
        *   `3000`: Employee related costs - *(Added based on CPT 2023 data)*
        *   `3100`: Remuneration of councillors - *(Label updated based on CPT 2023 data)*
        *   `3200`: Debt impairment - *(Label updated based on CPT 2023 data)*
        *   `3300`: Depreciation and asset impairment - *(Label updated based on CPT 2023 data)*
        *   `3400`: Finance charges - *(Label updated based on CPT 2023 data)*
        *   `3500`: Bulk purchases - *(Label updated based on CPT 2023 data)*
        *   `3600`: Contracted services - *(Label updated based on CPT 2023 data)*
        *   `3700`: Transfers and grants - *(Label updated based on CPT 2023 data)*
        *   `3900`: Other expenditure - *(Added based on CPT 2023 data)*
        *   `4000`: Loss on disposal of PPE - *(Added based on CPT 2023 data)*
    *   *Note: This list is based on observed data for CPT/2023/AUDA and may differ for other municipalities/years. Codes like 3800, 4100-4300, 4600, 4700, 4900, 5200 from the previous list were not present or had zero amounts in the test data.*

4.  **Capital Expenditure ([`get_total_capital_expenditure`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/financials.rs:148:0-197:1))**
    *   **Cube**: `capital_v2`
    *   **Method**: Get the aggregate `amount.sum`.

5.  **Audit Outcome ([`get_audit_outcome`](cci:1://file:///c:/Users/kesha/CascadeProjects/seemycity/seemycity-backend/src/api/muni_money/audit.rs:10:0-58:1))**
    *   **Cube**: `audit_opinions`
    *   **Method**: Use the `client::fetch_audit_opinion_facts` method, which requests the `opinion.label` field. The response is parsed into the specific `types::AuditApiResponse` containing `types::AuditOpinionFact` structs. The `opinion_label` from the first fact (if any) is returned.

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

---

#### Model Descriptions

- `MapMunicipalityProperties`: Contains properties for each GeoJSON feature (`id`, `name`, `province`, `population`, `classification`, `score`). *(Updated to reflect current query)*
- `MunicipalityDetail`: Represents the full data for the single municipality view endpoint.
    - Includes fields from `MunicipalityDb`.
    - Includes a `financials` field containing an array of `FinancialYearData` objects (holding `year`, `score`, `revenue`, etc. for each available year).
    - Includes a `latest_score` field (likely derived from the most recent year in `financials`).
    - Includes an optional `score_breakdown` struct (calculation TBD).
- `FinancialYearData`: Represents financial data for a single year within the `MunicipalityDetail` struct (`year`, `score`, `revenue`, etc.). *(New struct)*

---

#### API Endpoints

*   **`GET /api/municipalities`**
    - Fetches GeoJSON FeatureCollection for the map view.
    - Handler: `get_municipalities_map_handler` (TBC, in `handlers.rs`).
    - Query: `get_data_for_map_view` (in `queries.rs`).
    - Returns a `geojson::FeatureCollection`.
*   **`GET /api/municipalities/{id}`**
    - Fetches detailed info for a single municipality (identified by `id`), including an array of all available historical financial data (`financials`).
    - Handler: `get_municipality_detail_handler` (TBC, in `handlers.rs`).
    - Query: `get_municipality_detail` (in `queries.rs`).
    - Returns a `MunicipalityDetail` struct (containing the `financials` array).
