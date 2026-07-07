// src/db/municipalities.rs
use sqlx::PgPool;
use crate::models::{MunicipalityBasicInfo, MunicipalityDb, MapFeature, MapMunicipalityProperties};
use crate::errors::AppError;
use geojson;
use rust_decimal::Decimal;

// --- Municipality Query Functions ---

// Function to get a simple list of all municipality IDs and names
pub async fn get_all_municipalities_basic(pool: &PgPool) -> Result<Vec<MunicipalityBasicInfo>, AppError> {
    log::info!("Fetching basic info for all municipalities");
    let municipalities = sqlx::query_as!(
        MunicipalityBasicInfo, 
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
    .fetch_optional(pool) 
    .await?; 
    Ok(base_info)
}

// Fetches data required for the map's GeoJSON FeatureCollection
pub async fn get_municipalities_summary_for_map(pool: &PgPool, limit: Option<i64>) -> Result<Vec<MapFeature>, AppError> {
    log::info!("Fetching summary data for map view (limit: {:?})", limit);

    // Temporary struct to hold the raw query result
    #[derive(sqlx::FromRow, Debug)]
    struct MapQueryResult {
        id: String,
        name: String,
        province: String,
        population: Option<f32>,
        classification: Option<String>,
        latest_score: Option<Decimal>,
        geometry_geojson_str: Option<String>, 
    }

    // Use COALESCE for limit to handle None case cleanly in SQL
    let query_limit = limit.unwrap_or(i64::MAX); 

    // SQL query to fetch municipality info, geometry, and latest score
    let results = sqlx::query_as!(
        MapQueryResult,
        r#"
        WITH LatestScores AS (
            SELECT
                municipality_id,
                overall_score,
                ROW_NUMBER() OVER(PARTITION BY municipality_id ORDER BY year DESC) as rn
            FROM financial_data
            WHERE overall_score IS NOT NULL
        )
        SELECT
            m.id,
            m.name,
            m.province,
            m.population,
            m.classification,
            ls.overall_score as latest_score,
            -- Boundaries average ~5,500 points each (18 MB total raw). For a
            -- country-level choropleth, simplify to ~200 m tolerance and 5-decimal
            -- (~1 m) coordinates, cutting the payload by an order of magnitude.
            ST_AsGeoJSON(ST_SimplifyPreserveTopology(mg.geom, 0.002), 5)::TEXT as geometry_geojson_str
        FROM municipalities m
        LEFT JOIN municipal_geometries mg ON m.id = mg.munic_id
        LEFT JOIN LatestScores ls ON m.id = ls.municipality_id AND ls.rn = 1
        ORDER BY m.name
        LIMIT $1
        "#,
        query_limit
    )
    .fetch_all(pool)
    .await?;

    log::debug!("Fetched {} raw results from DB for map summary", results.len());

    // Process results into MapFeature vector
    let features: Vec<MapFeature> = results
        .into_iter()
        .filter_map(|row| {
            // Parse the geometry string
            let geometry = row.geometry_geojson_str.and_then(|geojson_str| {
                match geojson_str.parse::<geojson::GeoJson>() {
                    Ok(geojson::GeoJson::Geometry(geom)) => Some(geom),
                    Ok(_) => {
                        log::warn!("Parsed GeoJSON is not a Geometry for {}", row.id);
                        None
                    },
                    Err(e) => {
                        log::error!("Failed to parse GeoJSON geometry from DB for {}: {}", row.id, e);
                        None
                    }
                }
            });

            // If geometry parsing fails or is None, we might still want to include
            // the feature properties, or skip it. Skipping for now if geometry is essential.
            if geometry.is_none() {
                log::warn!("Skipping municipality {} due to missing or invalid geometry.", row.id);
                return None; 
            }

            let properties = MapMunicipalityProperties {
                id: row.id.clone(),
                name: row.name,
                province: row.province,
                population: row.population,
                classification: row.classification,
                latest_score: row.latest_score, 
            };

            Some(MapFeature {
                feature_type: "Feature".to_string(),
                geometry, 
                properties,
            })
        })
        .collect();

    log::info!("Successfully processed {} features for map summary.", features.len());
    Ok(features)
}
