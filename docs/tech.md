### `tech.md` - Technical Specifications Overview

#### Overview
**Product Name**: Municipal Financial Dashboard
**Purpose**: The Municipal Financial Dashboard is a web application delivering a high-performance, engaging experience for exploring South African municipal financial health. This document provides a high-level overview of the technical architecture. For detailed specifications, refer to `frontend-tech.md` and `backend-tech.md`.

**Goals**:
- Fetch and process financial data efficiently from the Municipal Money API.
- Cache data in Postgres for speed and flexibility.
- Render an interactive, performant map UI with Svelte and MapLibre GL JS.
- Ensure scalability and maintainability for future growth.

**Date**: April 12, 2025.

---

#### Architecture

##### High-Level Overview
- **Backend**: Rust-based server (Actix Web, sqlx) handles API requests, data processing, and Postgres interactions. _Refactored for modularity and idiomatic Rust practices._
- **Database**: Postgres + PostGIS stores cached financial data and static municipality details/geometries.
- **Frontend**: SvelteKit + MapLibre GL JS delivers a static, reactive UI.
- **Deployment**: Fly.io hosts the full stack (Rust + Postgres + Svelte).

##### Data Flow
1. **External API**: Rust backend fetches audited actuals from the Municipal Money API (4 concurrent cube calls per municipality-year), with a 5-minute circuit breaker for upstream outages.
2. **Processing & Caching**: scores computed per the prd.md rubric (missing data → NULL, never 0), cached in Postgres with a 7-day TTL, negative caching, and lazy re-derivation when the formula changes. A background warmer keeps all municipalities scored (startup + daily).
3. **Internal API**: `/api/municipalities` (simplified GeoJSON summary, in-memory-cached 60s, gzipped) and `/api/municipalities/{id}` (detail with per-year financials).
4. **UI**: static SvelteKit SPA renders map, single, and comparison views; map colors read the design tokens at runtime.

_Current roadmap: `plan.md` Phase 8 (trust layer → scoring v2 → insight UI)._

---

#### Technology Choices Summary

*   **Backend**: Rust (Actix Web, sqlx, reqwest, serde)
*   **Frontend**: SvelteKit (TypeScript, SCSS, MapLibre GL JS, Iconify)
*   **Database**: PostgreSQL + PostGIS
*   **Deployment**: Fly.io

---

#### Detailed Specifications

For detailed technical information on each component, please see:

*   **[Frontend Technical Specifications](./frontend-tech.md)**
*   **[Backend Technical Specifications](./backend-tech.md)**

---

#### Data Sources Summary

1.  **Municipal Money API (National Treasury)**: Primary source for financial and audit data.
2.  **Stats SA (Population Data)**: Source for static population figures (TBD, likely CSV/JSON).
3.  **Municipal Demarcation Board (Boundaries)**: Source for static GeoJSON boundaries.

---

#### Assumptions & Risks Overview

*   **Assumptions**: API stability, GeoJSON validity, Fly.io free tier suitability.
*   **Risks**: API rate limits, large GeoJSON performance, database setup complexity. Mitigation strategies involve caching, optimization, and thorough testing.

---

#### Scalability & Security Overview

*   **Scalability**: Built with concurrent Rust backend and scalable hosting (Fly.io). Database scaling possible post-MVP.
*   **Security**: Focus on secure credential management (.env), potential rate limiting, and leveraging `sqlx` for query safety. No user authentication in MVP.