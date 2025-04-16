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
    pub debt: Option<Decimal>, // Total Liabilities (sum of items 0310-0500 in financial_position_v2 cube, based on current implementation)
    pub audit_outcome: Option<String>,
    // Scores
    pub overall_score: Option<Decimal>,
    pub financial_health_score: Option<Decimal>,
    pub infrastructure_score: Option<Decimal>,
    pub efficiency_score: Option<Decimal>,
    pub accountability_score: Option<Decimal>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialDataPoint {
    pub metric_code: String, // e.g., "revenue", "debt"
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, FromRow)]
pub struct FinancialYearData {
    #[sqlx(rename = "year")] // Map 'year' column from DB query result
    pub financial_year: i32, 
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>,
    // Scores
    pub overall_score: Option<Decimal>,
    pub financial_health_score: Option<Decimal>,
    pub infrastructure_score: Option<Decimal>,
    pub efficiency_score: Option<Decimal>,
    pub accountability_score: Option<Decimal>,
}

#### API Response Models (src/models.rs)

*(These structs define the shape of data returned by the API endpoints, e.g., `/api/municipality/{id}`)*

```rust
// src/models.rs (Simplified view)
use serde::{Serialize, Deserialize};
use sqlx::types::Decimal;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialYearData {
    pub year: i32,
    // Decimal fields serialized as Option<f64> or null
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>,
    pub overall_score: Option<Decimal>,
    pub financial_health_score: Option<Decimal>,
    pub infrastructure_score: Option<Decimal>,
    pub efficiency_score: Option<Decimal>,
    pub accountability_score: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MunicipalityDetail {
    pub id: String,
    pub name: String,
    pub province: String,
    // f32 serialized as Option<f64> or null
    pub population: Option<f32>,
    pub classification: Option<String>,
    pub website: Option<String>,
    pub financials: Vec<FinancialYearData>,
    pub geometry: Option<Value>, // GeoJSON
}
```

### 1.2. Frontend (Svelte/TypeScript)

#### `MunicipalityFeatureProperties` (Derived from API)

Properties expected within each GeoJSON feature for the map view.

```typescript
interface MunicipalityFeatureProperties {
    id: string; // e.g., "BUF"
    name: string;
    province: string;
    overall_score: number | null; // Latest available overall_score (0-100) from financial_data
    population: number | null; // Matches DB 'real', serialized as f64
    classification: string | null;
    // Add other properties needed for map popups or sidebar display
}
```

#### `MunicipalityDetail` (Derived from API)

Detailed information for the single municipality view and comparison view.

```typescript
// src/lib/types.ts
interface FinancialYearData {
    year: number; // Matches API 'year'
    revenue: number | null; // From financial_data (Decimal -> f64 | null)
    expenditure: number | null; // From financial_data (Decimal -> f64 | null)
    capital_expenditure: number | null; // From financial_data (Decimal -> f64 | null)
    debt: number | null; // Total Liabilities (Decimal -> f64 | null)
    audit_outcome: string | null; // From financial_data
    // Scores (Decimal -> f64 | null)
    overall_score: number | null;
    financial_health_score: number | null;
    infrastructure_score: number | null;
    efficiency_score: number | null;
    accountability_score: number | null;
}

interface MunicipalityDetail {
    id: string;
    name: string;
    province: string;
    population: number | null; // From municipalities (f32 -> f64 | null)
    classification: string | null; // From municipalities
    website: string | null; // From municipalities
    financials: FinancialYearData[]; // Array of financial data per year (from API)
    geometry?: any | null; // GeoJSON geometry value (from API)
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

-- Table to store financial data per municipality per year
CREATE TABLE financial_data (
    id uuid NOT NULL DEFAULT uuid_generate_v4(), -- Primary key
    municipality_id varchar NOT NULL,             -- Foreign key to municipalities.id
    year int NOT NULL,                            -- Financial year (e.g., 2023)
    revenue numeric NULL,                         -- Total Revenue
    expenditure numeric NULL,                     -- Total Expenditure
    capital_expenditure numeric NULL,             -- Capital Expenditure
    debt numeric NULL,                            -- Total Liabilities / Debt
    audit_outcome text NULL,                      -- Latest Audit Opinion Label
    -- Calculated Scores (stored after calculation)
    overall_score numeric NULL,                   -- Overall Financial Health Score (0-100)
    financial_health_score numeric NULL,          -- Component score (0-100)
    infrastructure_score numeric NULL,            -- Component score (0-100)
    efficiency_score numeric NULL,                -- Component score (0-100)
    accountability_score numeric NULL,            -- Component score (0-100)
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE (municipality_id, year),               -- Ensure only one record per muni per year
    FOREIGN KEY (municipality_id) REFERENCES municipalities(id)
);

-- Trigger to automatically update updated_at timestamp
CREATE TRIGGER set_timestamp_financial_data
BEFORE UPDATE ON financial_data
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Indexes
CREATE INDEX idx_financial_data_municipality_year ON financial_data(municipality_id, year);
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
                "classification": "Metro",
                "population": 5635127,
                "overall_score": 65.5 // Example latest overall score
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
            "financial_year": 2023,
            "revenue": 500000000.00,
            "expenditure": 480000000.00,
            "capital_expenditure": 80000000.00,
            "debt": 120000000.00,
            "audit_outcome": "Unqualified opinion with findings",
            "overall_score": 75.2,
            "financial_health_score": 80.0,
            "infrastructure_score": 70.0,
            "efficiency_score": 72.5,
            "accountability_score": 78.3
        },
        {
            "financial_year": 2022,
            "revenue": 480000000.00,
            // ... other fields for 2022
            "overall_score": 72.1
        }
        // ... more years
    ],
    "geometry": { /* ... */ } // Standard GeoJSON geometry object
    // "score_breakdown": { /* ... */ } // Future addition
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
    "population": 5635127, // From municipalities table (real -> f64)
    "classification": "Metro", // From municipalities table
    "website": "http://www.joburg.org.za", // From municipalities table
    "financials": [
        {
            "financial_year": 2023,
            "revenue": 7500000000.50, // numeric -> Option<Decimal> -> f64 | null
            "expenditure": 7200000000.25, // numeric -> Option<Decimal> -> f64 | null
            "capital_expenditure": 500000000.00, // numeric -> Option<Decimal> -> f64 | null
            "debt": 12000000000.00, // numeric -> Option<Decimal> -> f64 | null
            "audit_outcome": "Unqualified opinion",
            "overall_score": 85.50,
            "financial_health_score": 80.1,
            "infrastructure_score": 70.0,
            "efficiency_score": 72.3,
            "accountability_score": 85.0
        }
        // ... data for other years
    ],
    "geometry": { /* ... */ } // Standard GeoJSON geometry object
    // "score_breakdown": { /* ... */ } // Future addition
}
```

**Source:** Data joined from `municipalities` and `financial_data` tables for the specific `{id}`.

---
*(This document should be updated whenever data structures, schema, or API payloads change.)*
