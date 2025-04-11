use actix_web::{get, web, Responder, HttpResponse};
use serde_json::json;
use crate::models::Municipality;
use crate::db::DbPool;

// Handler for the root path
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from SeeMyCity Backend! (via handlers module)")
}

// Handler for retrieving a list of municipalities.
// Returns data as a GeoJSON FeatureCollection.
#[get("/api/municipalities")]
pub async fn get_municipalities(pool: web::Data<DbPool>) -> impl Responder {
    // --- Test Database Connection --- 
    // Attempt a simple query to verify the pool works
    let db_result = sqlx::query("SELECT 1")
        .fetch_one(pool.get_ref())
        .await;

    match db_result {
        Ok(_) => println!("✅ Database connection test successful in handler."),
        Err(e) => {
            eprintln!("❌ Database connection test failed in handler: {}", e);
            // Return an internal server error if the DB connection fails
            return HttpResponse::InternalServerError().body("Database error");
        }
    }
    // --- End Test --- 

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
    // Use HttpResponse::Ok().json(...) for more control if needed, esp. with error handling
    HttpResponse::Ok().json(feature_collection)
}