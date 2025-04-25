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
1. **External API**: Rust backend fetches data from Municipal Money API.
2. **Processing & Caching**: Rust normalizes data (incl. current debt aggregation logic: sum of items 0310-0500), calculates scores, and caches in Postgres.
3. **Internal API**: Rust serves processed data to Svelte frontend via REST endpoints (`/api/municipalities` for GeoJSON map summary, `/api/municipalities/{id}` for detailed data).
4. **UI**: Svelte frontend renders map, single, and comparison views.

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