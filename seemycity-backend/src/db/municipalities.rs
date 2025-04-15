// src/db/municipalities.rs
use sqlx::PgPool;
use crate::models::{MunicipalityBasicInfo, MunicipalityDb, MunicipalityDetail, FinancialYearData}; // Add necessary models
use crate::errors::AppError;
use serde_json; // For geometry handling in detail function
use geojson; // For geometry handling

// --- Municipality Query Functions ---

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

// Function to get just the base MunicipalityDb info for a single municipality
// Used by the detail handler before checking cache/API
pub async fn get_municipality_base_info_db(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDb>, AppError> {
    log::debug!("Fetching base DB info for municipality: {}", muni_id);
    let base_info = sqlx::query_as!(
        MunicipalityDb,
        "SELECT id, name, province, district_id, district_name, address, phone, population, classification, website, created_at, updated_at FROM municipalities WHERE id = $1",
        muni_id
    )
    .fetch_optional(pool) // Use fetch_optional to return Option<MunicipalityDb>
    .await?; // Use ? for automatic conversion via From<sqlx::Error>
    Ok(base_info)
}

// Function to get detailed info for a single municipality (fetches everything from DB)
// Note: This is kept separate from the handler logic.
// The handler implements the cache/API fetching logic using base_info + financials cache.
pub async fn get_municipality_detail_db_only(pool: &PgPool, muni_id: &str) -> Result<Option<MunicipalityDetail>, AppError> {
    // Fetch base municipality info
    let base_info = get_municipality_base_info_db(pool, muni_id).await?;

    // If no base info, return None
    if base_info.is_none() {
        log::warn!("Base info not found for {} in get_municipality_detail_db_only", muni_id);
        return Ok(None);
    }
    let base_info_unwrapped = base_info.unwrap(); // Safe unwrap

    // Fetch geometry
    // Note: Consider moving geometry fetching to db/geo.rs if preferred
    let geometry_row = sqlx::query!(
        // Fetch geometry as GeoJSON string
        "SELECT ST_AsGeoJSON(geom) AS geometry_geojson FROM municipal_geometries WHERE munic_id = $1",
        muni_id
    )
    .fetch_optional(pool)
    .await?;

    let geometry: Option<geojson::Geometry> = geometry_row.and_then(|row| {
        row.geometry_geojson.and_then(|geojson_str| {
            match geojson_str.parse::<geojson::GeoJson>() {
                Ok(geojson::GeoJson::Geometry(geom)) => Some(geom),
                _ => {
                    log::error!("Failed to parse GeoJSON geometry from DB for {}", muni_id);
                    None
                }
            }
        })
    });

    // Fetch all associated financial data from the database
    // Note: Uses a function assumed to be in db/financials.rs
    let financials: Vec<FinancialYearData> = crate::db::financials::get_all_financial_years_db(pool, muni_id).await?;


    // Construct the final MunicipalityDetail struct
    let detail = MunicipalityDetail {
        id: base_info_unwrapped.id,
        name: base_info_unwrapped.name,
        province: base_info_unwrapped.province,
        population: base_info_unwrapped.population,
        classification: base_info_unwrapped.classification,
        website: base_info_unwrapped.website,
        financials,
        // Convert geojson::Geometry to serde_json::Value for the API model
        geometry: geometry.map(|g| serde_json::to_value(g).ok()).flatten(),
    };

    Ok(Some(detail))
}
