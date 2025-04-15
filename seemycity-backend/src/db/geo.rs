// src/db/geo.rs
use sqlx::PgPool;
use crate::models::MapMunicipalityProperties; // Use the specific model for map properties
use crate::errors::AppError;
use geojson; // For Feature struct
use rust_decimal::Decimal; // Needed by MapMunicipalityProperties potentially via query macro
use serde_json; // For converting properties struct

// --- Geospatial Query Functions ---

// Function to get data needed for the /api/municipalities map view
pub async fn get_data_for_map_view(pool: &PgPool) -> Result<Vec<geojson::Feature>, AppError> {
    // Fetch municipalities, geometry, and their latest scores
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
            -- Fetch the latest score for each municipality
            (
                SELECT fd.score
                FROM financial_data fd
                WHERE fd.municipality_id = m.id
                ORDER BY fd.year DESC
                LIMIT 1
            ) AS "latest_score?: Option<Decimal>" -- Ensure type matches MapMunicipalityProperties and handle NULL
        FROM
            municipalities m
        LEFT JOIN
            municipal_geometries g ON m.id = g.munic_id -- Use munic_id from geometries table
        "#
    )
    .fetch_all(pool)
    .await?;

    // Process rows into GeoJSON Features
    let features = rows.into_iter().filter_map(|row| {
        // Extract geometry first, handling potential errors
        let geojson_geometry: geojson::Geometry = match row.geometry_geojson {
            Some(geojson_str) => match geojson_str.parse::<geojson::GeoJson>() {
                Ok(geojson::GeoJson::Geometry(geom)) => geom,
                _ => {
                    log::error!("Failed to parse GeoJSON geometry for municipality ID: {:?}", row.id);
                    return None; // Skip if geometry parsing fails
                }
            },
            None => {
                 log::warn!("Missing geometry for municipality ID: {:?}", row.id);
                 // Skip features without geometry for map view
                 return None;
            }
        };

        // Prepare properties - Ensure types match MapMunicipalityProperties
        let properties = MapMunicipalityProperties {
            id: row.id.expect("Database query unexpectedly returned null ID for municipality").clone(), // Convert Option<String> from query to String, panic if None
            name: row.name.unwrap_or_default(), // Convert Option<String> from query to String for struct
            province: row.province.unwrap_or_default(), // Convert Option<String> from query to String for struct
            population: row.population, // Expects Option<f32>, assign directly
            classification: row.classification, // Expects Option<String>, assign directly
            latest_score: row.latest_score.flatten(), // Flatten Option<Option<Decimal>> to Option<Decimal>
        };

        // Use cloned id for logging, avoids moving row.id
        let muni_id_for_log = properties.id.clone();

        // Convert properties struct to serde_json::Value, then to JsonObject
        let properties_json_value = match serde_json::to_value(properties) {
             Ok(val) => val,
             Err(e) => {
                log::error!("Failed to serialize properties to JSON Value for municipality ID: {:?}: {}", muni_id_for_log, e);
                return None; // Skip if serialization fails
             }
        };

        let properties_json_object = match properties_json_value {
            serde_json::Value::Object(map) => map,
            _ => {
                log::error!("Failed to convert properties to GeoJSON JsonObject for municipality ID: {:?}", muni_id_for_log); // Use cloned id
                return None; // Skip if properties conversion fails
            }
        };

        Some(geojson::Feature {
            bbox: None,
            geometry: Some(geojson_geometry),
            id: None, // GeoJSON Feature ID, often null or based on a property
            properties: Some(properties_json_object),
            foreign_members: None,
        })
    }).collect::<Vec<_>>();

    Ok(features)
}