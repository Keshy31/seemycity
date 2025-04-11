// src/db/queries.rs
use sqlx::{types::Decimal, PgPool};
use crate::models::{MunicipalityBasicInfo, MapMunicipalityProperties, MunicipalityDetail, FinancialDataDb, FinancialYearData}; // Import necessary models
use crate::errors::AppError;
use serde_json::json;

// --- Query Functions ---

// Function to get a simple list of all municipality IDs and names
pub async fn get_all_municipality_basic_info(pool: &PgPool) -> Result<Vec<MunicipalityBasicInfo>, AppError> {
    log::info!("Fetching basic info for all municipalities");
    let municipalities = sqlx::query_as!(
        MunicipalityBasicInfo, // Target struct matching the SELECT list
        "SELECT id, name FROM municipalities ORDER BY name"
    )
    .fetch_all(pool)
    .await?; // Automatically converts sqlx::Error into AppError::SqlxError via From trait

    Ok(municipalities)
}

// Placeholder for function to get data needed for the /api/municipalities map view
// Returns properties and geometry separately for easier construction of GeoJSON FeatureCollection in handler
/*
pub async fn get_data_for_map_view(pool: &PgPool) -> Result<Vec<(MapMunicipalityProperties, serde_json::Value)>, AppError> {
    log::info!("Fetching data for map view");
    // TODO: Implement SQL query joining municipalities, financial_data (for latest score),
    // and municipal_geometries (using ST_AsGeoJSON for geometry)
    // Need to handle NULL scores and potentially NULL geometries.
    // Need to convert Decimal score to f64 for MapMunicipalityProperties.
    
    // Example structure (needs actual SQL and mapping):
    // let results = sqlx::query!(
    //     r#"
    //     SELECT 
    //         m.id, 
    //         m.name, 
    //         m.province, 
    //         latest_financials.score as latest_score, -- Assuming score is Decimal?
    //         ST_AsGeoJSON(g.geom) as geometry_geojson -- Query geometry as GeoJSON string
    //     FROM municipalities m
    //     LEFT JOIN (
    //         SELECT DISTINCT ON (municipality_id) 
    //             municipality_id, score, year
    //         FROM financial_data
    //         ORDER BY municipality_id, year DESC
    //     ) latest_financials ON m.id = latest_financials.municipality_id
    //     LEFT JOIN municipal_geometries g ON m.id = g.munic_id
    //     ORDER BY m.name
    //     "#
    // )
    // .fetch_all(pool)
    // .await?;
    //
    // let map_data = results.into_iter().map(|row| {
    //     let properties = MapMunicipalityProperties {
    //         id: row.id,
    //         name: row.name,
    //         province: row.province,
    //         latest_score: row.latest_score.and_then(|score: Decimal| score.to_string().parse::<f64>().ok()), // Convert Decimal to Option<f64>
    //     };
    //     let geometry = match row.geometry_geojson {
    //         Some(geojson_str) => serde_json::from_str(&geojson_str).unwrap_or(json!(null)), // Parse GeoJSON string
    //         None => json!(null), // Handle null geometry
    //     };
    //     (properties, geometry)
    // }).collect();
    //
    // Ok(map_data)

    Ok(vec![]) // Placeholder
}
*/

// Function to get detailed info for a single municipality
// Needs to fetch base details and all associated financial years
pub async fn get_municipality_details(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDetail>, AppError> {
    log::info!("Fetching details for municipality ID: {}", muni_id);
    
    // 1. Fetch base municipality details (including geometry)
    // Note: Using query! macro here for potentially more complex selection/joins than query_as!
    let base_details = sqlx::query!(
        r#"
        SELECT 
            m.id, m.name, m.province, m.population, m.classification, m.website, 
            -- Include other necessary fields from municipalities table (address, phone, district_id, district_name)
            ST_AsGeoJSON(g.geom) as geometry_geojson 
        FROM municipalities m 
        LEFT JOIN municipal_geometries g ON m.id = g.munic_id 
        WHERE m.id = $1
        "#,
        muni_id
    )
    .fetch_optional(pool)
    .await?;

    if base_details.is_none() {
        log::warn!("Municipality with ID {} not found.", muni_id);
        return Ok(None); // Municipality not found
    }
    let base = base_details.unwrap();

    // 2. Fetch all financial data rows for this municipality, ordered by year
    let financial_rows = sqlx::query_as!(
        FinancialDataDb,
        "SELECT * FROM financial_data WHERE municipality_id = $1 ORDER BY year DESC",
        muni_id
    )
    .fetch_all(pool)
    .await?;

    // 3. Combine into MunicipalityDetail struct
    let financials: Vec<FinancialYearData> = financial_rows.into_iter().map(|db_data| {
        // Convert Decimal fields to Option<f64> for JSON compatibility
        FinancialYearData {
            year: db_data.financial_year,
            revenue: db_data.revenue
                .and_then(|d| Decimal::to_string(&d).parse().ok()),
            expenditure: db_data.expenditure
                .and_then(|d| Decimal::to_string(&d).parse().ok()),
            debt: db_data.debt
                .and_then(|d| Decimal::to_string(&d).parse().ok()),
            audit_outcome: db_data.audit_outcome,
            score: db_data.score
                .and_then(|d| Decimal::to_string(&d).parse().ok()),
            // Add other relevant fields from FinancialDataDb if needed in FinancialYearData
        }
    }).collect();
    
    // Parse the GeoJSON string into serde_json::Value
    // Match on a reference to avoid moving the Option's content
    let geometry = match &base.geometry_geojson { 
        // geojson_str here is &String, from_str takes &str, which works
        Some(geojson_str) => serde_json::from_str(geojson_str).unwrap_or(json!(null)),
        None => json!(null),
    };
    
    // Construct the final MunicipalityDetail
    let detail = MunicipalityDetail {
        id: base.id, // From the query! result
        name: base.name,
        province: base.province,
        // Note: Population is already Option<f64> from MunicipalityDb
        // If it was fetched as Option<i64> or similar, convert here.
        population: base.population, 
        classification: base.classification,
        website: base.website,
        // Add other fields fetched from base_details if needed
        financials,
        geometry,
    };
    Ok(Some(detail))

    // Ok(None) // Placeholder
}

// Function for saving/updating financial data (using corrected FinancialDataDb)
pub async fn upsert_financial_data(pool: &PgPool, data: &FinancialDataDb) -> Result<(), AppError> {
    log::info!("Upserting financial data for municipality {} year {}", data.municipality_id, data.financial_year);
    
    // Generate a new UUID if the input data doesn't have one (e.g., for new records)
    // If data is coming from an API that provides its own stable ID, adjust accordingly.
    // For now, assume we generate the UUID upon insertion.
    let record_id = data.id; // Use the UUID provided in the data struct
    
    // Use sqlx::query! referencing fields from the corrected FinancialDataDb struct
    let result = sqlx::query!(
        r#"
        INSERT INTO financial_data (
            id, municipality_id, year, revenue, expenditure, 
            capital_expenditure, debt, audit_outcome, score, 
            created_at, updated_at -- Set timestamps on insert
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
        ON CONFLICT (municipality_id, year) -- Define the conflict target
        DO UPDATE SET
            revenue = EXCLUDED.revenue, -- Use EXCLUDED to reference values from the attempted INSERT
            expenditure = EXCLUDED.expenditure,
            capital_expenditure = EXCLUDED.capital_expenditure,
            debt = EXCLUDED.debt,
            audit_outcome = EXCLUDED.audit_outcome,
            score = EXCLUDED.score,
            updated_at = NOW() -- Update timestamp on update
        -- Optional: WHERE clause to only update if data actually changed, though often simpler to just update
        -- WHERE financial_data.revenue IS DISTINCT FROM EXCLUDED.revenue OR ... 
        "#,
        record_id, // Use the generated or provided UUID
        data.municipality_id,
        data.financial_year,
        data.revenue, // These are Option<Decimal>, sqlx handles them
        data.expenditure,
        data.capital_expenditure,
        data.debt,
        data.audit_outcome, // Option<String>
        data.score // Option<Decimal>
    )
    .execute(pool)
    .await?;
    
    if result.rows_affected() > 0 {
        log::info!("Successfully upserted financial data for {} year {}. Rows affected: {}", data.municipality_id, data.financial_year, result.rows_affected());
    } else {
        log::warn!("Upsert for {} year {} did not affect any rows. Data might be identical.", data.municipality_id, data.financial_year);
    }
    Ok(())

    // Ok(()) // Placeholder
}

// Placeholder for checking cached financial data for a specific year
/*
pub async fn get_cached_financials(pool: &PgPool, muni_id: &str, year: i32) -> Result<Option<FinancialDataDb>, AppError> {
    log::info!("Checking cache for municipality {} year {}", muni_id, year);
    // Use sqlx::query_as! with the corrected FinancialDataDb struct
    let data = sqlx::query_as!( 
        FinancialDataDb,
        "SELECT * FROM financial_data WHERE municipality_id = $1 AND year = $2",
        muni_id,
        year
    )
    .fetch_optional(pool)
    .await?;
    Ok(data)

    // Ok(None) // Placeholder
}
*/ // Ensure this closing comment tag exists and is placed correctly