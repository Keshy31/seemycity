# SeeMyCity Data Specification

This document defines the core data structures, database schema, and API payloads used throughout the SeeMyCity application. It serves as the single source of truth for data definitions.

## 1. Core Data Structures

### 1.1. Backend (Rust)

#### Municipality Struct (src/models.rs)

Represents core municipality data used internally and potentially in API responses.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Municipality {
    pub code: String,          // e.g., "BUF", Unique identifier
    pub name: String,          // e.g., "Buffalo City Metropolitan Municipality"
    pub province: String,      // e.g., "Eastern Cape"
    pub financial_score: Option<f64>, // Calculated score (0.0 - 100.0), if available
    // Add other fields mapped from DB or external API as needed
    // pub population: Option<f64>,
    // pub classification: Option<String>,
    // pub district_id: Option<String>,
    // pub district_name: Option<String>,
}
```

*(Add other backend structs here as they are defined, e.g., `FinancialRecord`, `GeometryDetail`)*

### 1.2. Frontend (Svelte/TypeScript)

#### `MunicipalityFeatureProperties` (Derived from API)

Properties expected within each GeoJSON feature for the map view.

```typescript
interface MunicipalityFeatureProperties {
  code: string;
  name: string;
  province: string;
  financial_score: number | null; // Or use `undefined` if preferred over `null`
  // Add other properties needed for map popups or sidebar display
}
```

#### `MunicipalityDetails` (Derived from API)

Detailed information for the single municipality view.

```typescript
interface MunicipalityDetails {
  code: string;
  name: string;
  province: string;
  population: number | null;
  classification: string | null;
  website: string | null;
  // ... other fields from municipalities table
  financials: FinancialYearData[]; // Array of financial data per year
  score_breakdown: any; // TBD: Define structure for score components
}

interface FinancialYearData {
  year: number;
  revenue: number | null;
  expenditure: number | null;
  debt: number | null;
  audit_outcome: string | null;
  score: number | null;
  // ... other relevant fields from `financial_data` table
}
```

*(Adjust frontend types based on actual API responses and component needs)*


## 2. Database Schema (PostgreSQL + PostGIS)

Defines the structure of the data stored in the application database.

```sql
-- Enable PostGIS extension if not already enabled
CREATE EXTENSION IF NOT EXISTS postgis;

-- Table to store municipality details
CREATE TABLE municipalities (
	id varchar NOT NULL,          -- Corresponds to `Municipality.code`, PRIMARY KEY
	"name" text NOT NULL,         -- Corresponds to `Municipality.name`
	province text NOT NULL,       -- Corresponds to `Municipality.province`
	population real NULL,
	classification text NULL,
	address text NULL,
	website text NULL,
	phone text NULL,
	district_id varchar NULL,
	district_name text NULL,
	CONSTRAINT municipalities_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE municipalities IS 'Stores static details for South African municipalities.';

-- Table to store municipality geometry
CREATE TABLE municipal_geometries (
	ogc_fid serial4 NOT NULL,
	geom public.geometry(geometry, 4326) NULL, -- GeoJSON geometry stored here
	munic_id varchar NOT NULL,                  -- Foreign key to `municipalities.id`
	CONSTRAINT municipal_geometries_pkey PRIMARY KEY (ogc_fid),
    CONSTRAINT municipal_geometries_municipalities_fk FOREIGN KEY (munic_id) REFERENCES public.municipalities(id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX municipal_geometries_geom_geom_idx ON public.municipal_geometries USING gist (geom);
CREATE INDEX municipal_geometries_munic_id_idx ON public.municipal_geometries USING btree (munic_id);
COMMENT ON TABLE municipal_geometries IS 'Stores geographic boundaries for South African municipalities.';

-- Table to store cached financial data and scores
CREATE TABLE financial_data (
	id uuid NOT NULL DEFAULT gen_random_uuid(), -- Internal primary key
	municipality_id varchar NULL,                -- Foreign key to `municipalities.id`
	"year" int4 NOT NULL,
	revenue numeric NULL,
	expenditure numeric NULL,
	capital_expenditure numeric NULL,
	debt numeric NULL,
	audit_outcome text NULL,
	score numeric NULL,                          -- Corresponds to `Municipality.financial_score`
	created_at timestamptz DEFAULT now() NULL,   -- Timestamp for cache management
	CONSTRAINT financial_data_pkey PRIMARY KEY (id),
    CONSTRAINT financial_data_municipalities_fk FOREIGN KEY (municipality_id) REFERENCES public.municipalities(id) ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT financial_data_municipality_year_unique UNIQUE (municipality_id, "year") -- Ensure one record per muni per year
);
CREATE INDEX financial_data_municipality_id_year_idx ON public.financial_data USING btree (municipality_id, year);
COMMENT ON TABLE financial_data IS 'Stores cached financial metrics and calculated scores for municipalities, linked by municipality_id and year.';
```

## 3. API Payloads

Defines the structure of data exchanged between the frontend and backend.

### 3.1. `GET /api/municipalities`

Returns a GeoJSON `FeatureCollection` suitable for map display.

**Structure:**

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point", // Or Polygon, MultiPolygon, etc. - Sourced from `municipal_geometries`
        "coordinates": [longitude, latitude] // Or nested arrays for polygons
      },
      "properties": {
        // Matches Frontend `MunicipalityFeatureProperties`
        "code": "string",
        "name": "string",
        "province": "string",
        "financial_score": "number | null"
      }
    }
    // ... more features
  ]
}
```

**Source:** Data aggregated from `municipalities`, `municipal_geometries`, and the latest relevant `financial_data` tables.

### 3.2. `GET /api/municipality/{id}`

Returns detailed information for a single municipality identified by `{id}` (which corresponds to `municipalities.id`).

**Structure:**

```json
{
  // Matches Frontend `MunicipalityDetails`
  "code": "string",
  "name": "string",
  "province": "string",
  "population": "number | null",
  "classification": "string | null",
  "website": "string | null",
  // ... other fields from municipalities table
  "financials": [
    // Matches Frontend `FinancialYearData`
    {
      "year": "number",
      "revenue": "number | null",
      "expenditure": "number | null",
      "debt": "number | null",
      "audit_outcome": "string | null",
      "score": "number | null"
      // ... other relevant fields from `financial_data` table
    }
    // ... more years
  ],
  "score_breakdown": {} // TBD: Define structure for score components
}
```

**Source:** Data joined from `municipalities` and `financial_data` tables for the specific `municipality_id`.

---
*(This document should be updated whenever data structures, schema, or API payloads change.)*
