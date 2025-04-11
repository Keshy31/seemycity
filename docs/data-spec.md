# SeeMyCity Data Specification

This document defines the core data structures, database schema, and API payloads used throughout the SeeMyCity application. It serves as the single source of truth for data definitions.

## 1. Core Data Structures

### 1.1. Backend (Rust)

#### Municipality Struct (src/models.rs)

*(These structs map directly to database tables)*

```rust
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::{Decimal, chrono}};
use uuid::Uuid;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MunicipalityDb {
    pub id: String, // Primary key, e.g., "BUF"
    pub name: String,
    pub province: String,
    pub population: Option<f32>, // Matches DB 'real'
    pub classification: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub district_id: Option<String>,
    pub district_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MunicipalityGeometryDb {
    pub ogc_fid: i32, // Serial primary key
    // Note: DB uses 'geometry' type, requires special handling (e.g., ST_AsGeoJSON in query or postgis crate)
    pub geom: Option<postgis::geojson::Geometry>, // If using postgis crate
    pub munic_id: String, // Foreign key (Matches DB column name)
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FinancialDataDb {
    pub id: Uuid, // Primary key
    pub municipality_id: String, // Foreign key
    #[sqlx(rename = "year")] // Maps DB 'year' to this field
    pub financial_year: i32,
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>,
    pub score: Option<Decimal>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

*(API response models like `MunicipalityBasicInfo`, `MapMunicipalityProperties`, `FinancialYearData`, `MunicipalityDetail` are also defined in `src/models.rs` but omitted here for brevity, see Section 3)*

### 1.2. Frontend (Svelte/TypeScript)

#### `MunicipalityFeatureProperties` (Derived from API)

Properties expected within each GeoJSON feature for the map view.

```typescript
interface MunicipalityFeatureProperties {
    id: string; // e.g., "BUF"
    name: string;
    province: string;
    latest_score: number | null; // Latest available score (0-100)
    population: number | null; // Matches DB 'real', serialized as f64
    classification: string | null;
    // Add other properties needed for map popups or sidebar display
}
```

#### `MunicipalityDetails` (Derived from API)

Detailed information for the single municipality view.

```typescript
interface MunicipalityDetails {
    id: string;
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
    capital_expenditure: number | null;
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    CONSTRAINT municipalities_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE municipalities IS 'Stores static details for South African municipalities.';

-- Table to store municipality geometry
CREATE TABLE municipal_geometries (
    ogc_fid serial4 NOT NULL,               -- Primary key
    munic_id varchar NOT NULL,             -- Foreign key to municipalities.id (Matches DB)
    geom public.geometry(geometry, 4326) NULL, -- Actual geometry type from DB
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    CONSTRAINT municipal_geometries_pkey PRIMARY KEY (ogc_fid),
    CONSTRAINT municipal_geometries_munic_id_fkey FOREIGN KEY (munic_id) REFERENCES public.municipalities(id) ON DELETE CASCADE -- Matches DB
);
CREATE INDEX municipal_geometries_geom_geom_idx ON public.municipal_geometries USING gist (geom); -- Matches DB index
CREATE INDEX municipal_geometries_munic_id_idx ON public.municipal_geometries USING btree (munic_id); -- Matches DB index
COMMENT ON TABLE municipal_geometries IS 'Stores geographic boundaries for South African municipalities.';

-- Table to store cached financial data and scores
CREATE TABLE financial_data (
    id uuid NOT NULL DEFAULT gen_random_uuid(), -- Primary key
    municipality_id varchar NOT NULL,            -- Foreign key to municipalities.id
    year int4 NOT NULL,                          -- Financial year (Matches DB column name)
    revenue numeric NULL,
    expenditure numeric NULL,
    capital_expenditure numeric NULL,            -- Added field
    debt numeric NULL,
    audit_outcome text NULL,
    score numeric(5, 2) NULL,                  -- Calculated financial health score (0.00 - 100.00)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc', now()) NOT NULL,
    CONSTRAINT financial_data_pkey PRIMARY KEY (id),
    CONSTRAINT financial_data_unique UNIQUE (municipality_id, year), -- Matches DB unique constraint
    CONSTRAINT financial_data_municipalities_fk FOREIGN KEY (municipality_id) REFERENCES public.municipalities(id) -- Matches DB FK
);
CREATE INDEX financial_data_municipality_id_year_idx ON public.financial_data USING btree (municipality_id, year); -- Matches DB index
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
            }, // Note: Actual geometry comes from 'geom' column, likely needs conversion to GeoJSON in backend
            "properties": {
                "id": "JHB01", // municipality.id
                "name": "Johannesburg Metro (Mock)", // municipality.name
                "province": "GP", // municipality.province
                "population": 5635127, // municipality.population
                "classification": "Metro", // municipality.classification
                "score": 85.50 // latest financial_data.score (or derived)
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
    "id": "CPT",
    "name": "City of Cape Town Metropolitan Municipality",
    "province": "Western Cape",
    "population": 4600000.0,
    "classification": "Metro",
    "website": "https://www.capetown.gov.za",
    // ... other fields from municipalities table
    "financials": [
        {
            "year": 2022,
            "revenue": 50000000000.0,
            "expenditure": 48000000000.0,
            "capital_expenditure": 5000000000.0,
            "debt": 15000000000.0,
            "audit_outcome": "Unqualified",
            "score": 85.2
        }
        // ... more years
    ],
    "score_breakdown": {} // TBD: Define structure for score components
}
```

**Source:** Data joined from `municipalities` and `financial_data` tables for the specific `municipality_id`.

### 3.3. `GET /api/municipalities/{id}`

Returns detailed information for a single municipality identified by `{munic_id}` (which corresponds to `municipalities.id`).

**Structure:**

```json
{
    "id": "JHB01",
    "name": "Johannesburg Metro (Mock)",
    "province": "GP",
    "population": 5635127, // From municipalities table
    "classification": "Metro", // From municipalities table
    "website": "https://joburg.org.za", // From municipalities table

    // Array of financial data, ordered by year (e.g., descending)
    "financials": [
        {
            "year": 2023, 
            "score": 85.50, // Score for this specific year (numeric(5,2))
            "revenue": 70000000000,
            "expenditure": 68000000000,
            "capital_expenditure": 10000000000,
            "debt": 30000000000,
            "audit_outcome": "Qualified"
        },
        {
            "year": 2022,
            "score": 82.10, 
            "revenue": 68000000000,
            "expenditure": 65000000000,
            "capital_expenditure": 9000000000,
            "debt": 28000000000,
            "audit_outcome": "Qualified"
        }
        // ... more years
    ],
    // Top-level score for easy access (e.g., latest year's score)
    "latest_score": 85.50, // Matches score for 2023
    // Score breakdown based on the latest year's data (TBD Implementation)
    "score_breakdown": { 
        "financial_health": 25.5, 
        "infrastructure": 20.0, 
        "efficiency": 18.0, 
    }
}
```

**Source:** Data joined from `municipalities` and `financial_data` tables for the specific `{id}`.

---
*(This document should be updated whenever data structures, schema, or API payloads change.)*
