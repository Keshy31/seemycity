// src/db/queries.rs
use sqlx::{PgPool};
use geojson;
use crate::models::{MunicipalityBasicInfo, MunicipalityDetail, FinancialDataDb, FinancialYearData, MapMunicipalityProperties, MunicipalityDb};
use crate::errors::AppError;
use uuid::Uuid;
// Added serde_json for parsing geometry string
use serde_json;

// --- Query Functions ---

// Function to get a simple list of all municipality IDs and names
pub async fn get_all_municipalities_basic(pool: &PgPool) -> Result<Vec<MunicipalityBasicInfo>, AppError> {
    log::info!("Fetching basic info for all municipalities");
    let municipalities = sqlx::query_as!(
        MunicipalityBasicInfo, // Target struct matching the SELECT list
        "SELECT id, name, province FROM municipalities ORDER BY name"
    )
    .fetch_all(pool)
    .await?;

    Ok(municipalities)
}

// Function to get data needed for the /api/municipalities map view
// Returns properties and geometry separately for easier construction of GeoJSON FeatureCollection in handler
pub async fn get_data_for_map_view(pool: &PgPool) -> Result<Vec<geojson::Feature>, AppError> {
    // Fetch municipalities and their latest scores
    let rows = sqlx::query!(
        r#"
        SELECT 
            m.id,
            m.name,
            m.province,
            m.population,
            m.classification,
            -- Fetch geometry as GeoJSON string
            ST_AsGeoJSON(g.geom) AS geometry_geojson, 
            fd.score AS latest_score
        FROM municipalities m
        LEFT JOIN municipal_geometries g ON m.id = g.munic_id
        LEFT JOIN (
            -- Subquery to get the latest financial data per municipality
            SELECT 
                municipality_id, 
                score,
                ROW_NUMBER() OVER(PARTITION BY municipality_id ORDER BY year DESC) as rn
            FROM financial_data
        ) fd ON m.id = fd.municipality_id AND fd.rn = 1
        WHERE g.geom IS NOT NULL -- Only include municipalities with actual geometry data
        ORDER BY m.name;
        "#
    )
    .fetch_all(pool)
    .await?;

    let features = rows.into_iter().filter_map(|row| {
        // Safely extract geometry string, skip if null
        let geometry_geojson_str = row.geometry_geojson?;
        // Parse the GeoJSON string into a serde_json::Value
        let geometry_value = match serde_json::from_str::<serde_json::Value>(&geometry_geojson_str) {
            Ok(val) => val,
            Err(e) => {
                log::error!("Error parsing geometry JSON string for {}: {}", row.id, e);
                return None; // Skip if JSON parsing fails
            }
        };
        // Convert serde_json::Value into geojson::Geometry
        let geojson_geometry = match geojson::Geometry::from_json_value(geometry_value) { // No clone needed
            Ok(geom) => geom,
            Err(e) => {
                log::error!("Error converting JSON Value to geojson::Geometry for {}: {}", row.id, e);
                return None; // Skip if geometry is invalid
            }
        };

        let muni_id_for_log = row.id.clone(); // Clone id for logging before move

        let properties = MapMunicipalityProperties {
            id: row.id,
            name: row.name,
            province: row.province,
            // population is now Option<f32>, will be serialized to f64 by serde attribute
            population: row.population, 
            classification: row.classification,
            // latest_score is Option<Decimal>, will be serialized to f64 by serde attribute
            latest_score: row.latest_score, 
        };

        // Try converting properties into a geojson::JsonObject
        let properties_json_object = match serde_json::to_value(properties) {
            Ok(serde_json::Value::Object(map)) => geojson::JsonObject::from(map),
            _ => {
                log::error!("Failed to convert properties to GeoJSON JsonObject for {}", muni_id_for_log); // Use cloned id
                return None; // Skip if properties conversion fails
            }
        };

        Some(geojson::Feature {
            bbox: None,
            geometry: Some(geojson_geometry),
            id: None, 
            properties: Some(properties_json_object),
            foreign_members: None,
        })
    }).collect::<Vec<_>>(); // Simpler collection, errors handled within filter_map

    Ok(features)
}

// Function to get detailed info for a single municipality
// Needs to fetch base details and all associated financial years
pub async fn get_municipality_detail(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDetail>, AppError> {
    // Fetch base municipality info
    let base_info = sqlx::query_as!(
        MunicipalityDb, 
        "SELECT id, name, province, district_id, district_name, address, phone, population, classification, website, created_at, updated_at FROM municipalities WHERE id = $1", 
        muni_id
    )
    .fetch_one(pool)
    .await?;

    // Fetch geometry
    let geometry_row = sqlx::query!(
        // Fetch geometry as GeoJSON string
        "SELECT ST_AsGeoJSON(geom) AS geometry_geojson FROM municipal_geometries WHERE munic_id = $1", 
        muni_id
    )
    .fetch_optional(pool)
    .await?;
    // Convert geometry string to geojson::Geometry
    let geometry = geometry_row
        .and_then(|row| row.geometry_geojson) // Option<String>
        .and_then(|geojson_str: String| { // Parse the string
            match serde_json::from_str::<serde_json::Value>(&geojson_str) {
                Ok(val) => Some(val),
                Err(e) => {
                    log::error!("Error parsing geometry JSON string for {}: {}", muni_id, e);
                    None
                }
            }
        })
        .and_then(|json_value| { // Convert to geojson::Geometry
            match geojson::Geometry::from_json_value(json_value) {
                Ok(geom) => Some(geom),
                Err(e) => {
                    log::error!("Error converting JSON Value to geojson::Geometry for {}: {}", muni_id, e);
                    None
                }
            }
        });

    // Fetch financial data history
    // Explicitly list columns and alias 'year' to 'financial_year'
    let financial_rows = sqlx::query_as!(
        FinancialDataDb,
        r#"
        SELECT 
            id, 
            municipality_id, 
            year AS financial_year, 
            revenue AS "revenue?", 
            expenditure AS "expenditure?", 
            capital_expenditure AS "capital_expenditure?", 
            debt AS "debt?", 
            audit_outcome AS "audit_outcome?", 
            score AS "score?", 
            created_at, 
            updated_at
        FROM financial_data 
        WHERE municipality_id = $1 
        ORDER BY financial_year DESC
        "#,
        muni_id
    )
    .fetch_all(pool)
    .await?;

    // Convert FinancialDataDb to FinancialYearData (handles type conversion if needed)
    let financials: Vec<FinancialYearData> = financial_rows.into_iter().map(|db_data| FinancialYearData {
        year: db_data.financial_year,
        revenue: db_data.revenue,
        expenditure: db_data.expenditure,
        capital_expenditure: db_data.capital_expenditure, 
        debt: db_data.debt,
        audit_outcome: db_data.audit_outcome,
        score: db_data.score,
    }).collect();

    // Construct the final MunicipalityDetail struct
    let detail = MunicipalityDetail {
        id: base_info.id,
        name: base_info.name,
        province: base_info.province,
        // Convert population f32 -> f64 for JSON via serde attribute
        population: base_info.population, 
        classification: base_info.classification,
        website: base_info.website,
        financials,
        // Convert geojson::Geometry to serde_json::Value for the API model
        geometry: geometry.map(|g| serde_json::to_value(g).ok()).flatten(),
    };

    Ok(Some(detail))
}

// Checks if financial data for a specific municipality and year exists
pub async fn get_cached_financials(pool: &PgPool, muni_id: &str, year: i32) -> Result<Option<FinancialDataDb>, AppError> {
    let result = sqlx::query_as!(
        FinancialDataDb,
        // Also explicitly list columns here for consistency and safety
        r#"
        SELECT 
            id, 
            municipality_id, 
            year AS financial_year, 
            revenue AS "revenue?", 
            expenditure AS "expenditure?", 
            capital_expenditure AS "capital_expenditure?", 
            debt AS "debt?", 
            audit_outcome AS "audit_outcome?", 
            score AS "score?", 
            created_at, 
            updated_at
        FROM financial_data 
        WHERE municipality_id = $1 AND year = $2
        "#,
        muni_id,
        year
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

// Inserts or updates financial data for a municipality
// Now relies on DB trigger for updated_at
pub async fn upsert_financial_data(pool: &PgPool, data: FinancialDataDb) -> Result<Uuid, AppError> {
    // NOTE: created_at is handled by DEFAULT NOW() in the DB
    // NOTE: updated_at is handled by the DB trigger
    let result = sqlx::query!(
        r#"
        INSERT INTO financial_data (
            id, municipality_id, year, revenue, expenditure, 
            capital_expenditure, debt, audit_outcome, score 
            -- created_at is DEFAULT NOW(), updated_at handled by trigger
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (municipality_id, year) 
        DO UPDATE SET
            revenue = EXCLUDED.revenue,
            expenditure = EXCLUDED.expenditure,
            capital_expenditure = EXCLUDED.capital_expenditure,
            debt = EXCLUDED.debt,
            audit_outcome = EXCLUDED.audit_outcome,
            score = EXCLUDED.score
            -- updated_at is handled by trigger, no need to set here
        RETURNING id
        "#,
        data.id, // uuid
        data.municipality_id,
        data.financial_year,
        data.revenue, // Option<Decimal>
        data.expenditure, // Option<Decimal>
        data.capital_expenditure, // Option<Decimal>
        data.debt, // Option<Decimal>
        data.audit_outcome, // Option<String>
        data.score // Option<Decimal>
    )
    .fetch_one(pool) // Use fetch_one as RETURNING id guarantees one row
    .await?;
    Ok(result.id)
}