// src/db/queries.rs
use sqlx::{Arguments, PgPool};
use geojson;
use crate::models::{MunicipalityBasicInfo, MunicipalityDetail, FinancialDataDb, FinancialYearData, MapMunicipalityProperties, MunicipalityDb, FinancialDataPoint};
use crate::errors::AppError;
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

// Function to get just the base MunicipalityDb info for a single municipality
// Extracted from get_municipality_detail
pub async fn get_municipality_base_info_db(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDb>, AppError> {
    log::debug!("Fetching base DB info for municipality: {}", muni_id);
    let base_info = sqlx::query_as!(
        MunicipalityDb, 
        "SELECT id, name, province, district_id, district_name, address, phone, population, classification, website, created_at, updated_at FROM municipalities WHERE id = $1", 
        muni_id
    )
    .fetch_optional(pool) // Use fetch_optional to return Option<MunicipalityDb>
    .await
    ?; // Use ? for automatic conversion via From<sqlx::Error>
    Ok(base_info)
}

// Function to get detailed info for a single municipality
// Needs to fetch base details and all associated financial years
pub async fn get_municipality_detail(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDetail>, AppError> {
    // Fetch base municipality info
    let base_info = get_municipality_base_info_db(pool, muni_id).await?;

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
        id: base_info.as_ref().unwrap().id.clone(), // Clone String
        name: base_info.as_ref().unwrap().name.clone(), // Clone String
        province: base_info.as_ref().unwrap().province.clone(), // Clone String
        // Convert population f32 -> f64 for JSON via serde attribute
        population: base_info.as_ref().unwrap().population, 
        classification: base_info.as_ref().unwrap().classification.clone(), // Clone Option<String>
        website: base_info.as_ref().unwrap().website.clone(), // Clone Option<String>
        financials,
        // Convert geojson::Geometry to serde_json::Value for the API model
        geometry: geometry.map(|g| serde_json::to_value(g).ok()).flatten(),
    };

    // Wrap the detail struct in Option, consistent with fetch_optional possibility (though fetch_one was used before)
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

// Inserts or updates financial data for a municipality based on individual metrics
// Accepts a slice of FinancialDataPoint structs
pub async fn upsert_financial_data(pool: &PgPool, data_points: &[FinancialDataPoint]) -> Result<(), AppError> {
    // Ensure there's data to process
    if data_points.is_empty() {
        log::warn!("upsert_financial_data called with empty data points slice");
        return Ok(());
    }

    // Assuming all points are for the same municipality and year
    let muni_code = &data_points[0].municipality_code;
    let year = data_points[0].year;

    log::debug!("Upserting {} financial data points for {} year {}", data_points.len(), muni_code, year);

    // Use a transaction for atomicity
    let mut tx = pool.begin().await?; // Use ? for automatic conversion

    // Prepare the base INSERT ... ON CONFLICT statement
    // We target the unique constraint (municipality_id, year)
    // We will dynamically add SET clauses for each metric provided
    let mut set_clauses = Vec::new();
    let mut arguments = sqlx::postgres::PgArguments::default();
    let mut placeholder_index = 3; // Start placeholders at $3
    // Bind municipality_id and year first
    arguments.add(muni_code); // Use trait method
    arguments.add(year); // Use trait method

    for point in data_points {
        // Ensure consistency (should ideally be checked earlier)
        if point.municipality_code != *muni_code || point.year != year {
            log::error!("Inconsistent municipality code or year in upsert_financial_data batch");
            // Consider returning an error or skipping the point
            continue; 
        }

        // Map metric_name to column name and add SET clause if amount is Some
        let column_name = match point.metric_name.as_str() {
            "total_revenue" => Some("revenue"),
            "total_expenditure" => Some("expenditure"),
            "capital_expenditure" => Some("capital_expenditure"),
            "total_debt" => Some("debt"),
            // Add mappings for other metrics like 'audit_outcome' (String) or 'score' (Decimal) if needed
            _ => {
                log::warn!("Unknown metric name in upsert_financial_data: {}", point.metric_name);
                None
            }
        };

        if let Some(col) = column_name {
            if let Some(amount) = point.amount {
                 // Use manually tracked placeholder index
                set_clauses.push(format!("{} = ${}", col, placeholder_index));
                arguments.add(amount); // Use trait method
                placeholder_index += 1;
            } else {
                // Optionally set to NULL if amount is None
                // set_clauses.push(format!("{} = NULL", col));
            }
        }
    }

    // Only proceed if there are actual updates to make
    if set_clauses.is_empty() {
        log::debug!("No valid metric data points found to upsert for {} year {}", muni_code, year);
        // We still need to commit or rollback the transaction started
        tx.commit().await?; // Use ? for automatic conversion
        return Ok(());
    }

    let query_str = format!(
        "INSERT INTO financial_data (municipality_id, year, {})
         VALUES ($1, $2, {})
         ON CONFLICT (municipality_id, year)
         DO UPDATE SET {}
        ",
        set_clauses.iter().map(|s| s.split('=').next().unwrap_or("").trim()).collect::<Vec<&str>>().join(", "), // Column names for INSERT
        (3..placeholder_index).map(|i| format!("${}", i)).collect::<Vec<String>>().join(", "), // Placeholders for INSERT values using counter
        set_clauses.join(", ") // SET clauses for UPDATE
    );

    // Execute the dynamic query within the transaction
    sqlx::query_with(&query_str, arguments)
        .execute(&mut *tx) // Use &mut *tx to borrow mutable reference
        .await?; // Use ? for automatic conversion

    // Commit the transaction
    tx.commit().await?; // Use ? for automatic conversion

    log::info!("Successfully upserted financial data for {} year {}", muni_code, year);
    Ok(())
}