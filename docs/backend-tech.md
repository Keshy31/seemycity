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

- **`/api/municipalities`**: Returns a GeoJSON `FeatureCollection` containing municipality boundaries and basic data suitable for map display. See [`docs/data-spec.md`](./data-spec.md#31-get-apimunicipalities) for payload details.
- **`/api/municipality/{id}`**: Returns detailed financial data and score breakdown for a specific municipality ID. See [`docs/data-spec.md`](./data-spec.md#32-get-apimunicipalityid) for payload details.
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
