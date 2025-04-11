use actix_web::{get, web, Responder, HttpResponse};
use serde_json::json;

// Import the Municipality struct from the models module
// We use `crate::` because models.rs is at the crate root level alongside main.rs
use crate::models::Municipality;

// Handler for the root path ("/").
// Mark the function as `pub` so it can be imported and used in `main.rs`.
#[get("/")]
pub async fn hello() -> impl Responder {
    // Use HttpResponse for a more explicit response
    HttpResponse::Ok().body("Hello from SeeMyCity Backend! (via handlers module)")
}

// Handler for retrieving a list of municipalities.
// Returns data as a GeoJSON FeatureCollection.
#[get("/api/municipalities")]
pub async fn get_municipalities() -> impl Responder {
    // Create some dummy data (can still use our Municipality struct for temporary storage)
    let municipalities_data = vec![
        Municipality {
            code: "BUF".to_string(),
            name: "Buffalo City Metropolitan Municipality".to_string(),
            province: "Eastern Cape".to_string(),
            financial_score: Some(65.5),
        },
        Municipality {
            code: "CPT".to_string(),
            name: "City of Cape Town Metropolitan Municipality".to_string(),
            province: "Western Cape".to_string(),
            financial_score: Some(88.2),
        },
        Municipality {
            code: "JHB".to_string(),
            name: "City of Johannesburg Metropolitan Municipality".to_string(),
            province: "Gauteng".to_string(),
            financial_score: None, // Example of missing score
        },
    ];

    // Convert the dummy data into GeoJSON Features
    let features: Vec<_> = municipalities_data
        .into_iter()
        .map(|muni| {
            json!({
                "type": "Feature",
                "geometry": null, // Placeholder for actual geometry later
                "properties": {
                    "id": muni.code,
                    "name": muni.name,
                    "province": muni.province,
                    // Use the financial_score, defaulting to null if None
                    "score": muni.financial_score
                }
            })
        })
        .collect();

    // Construct the final GeoJSON FeatureCollection
    let feature_collection = json!({
        "type": "FeatureCollection",
        "features": features
    });

    // Serialize the FeatureCollection into a JSON response.
    // web::Json automatically sets the correct Content-Type header.
    web::Json(feature_collection)
}