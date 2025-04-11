use actix_web::{get, web, Responder, HttpResponse, Result};
use crate::models::MunicipalityDetail;
use crate::errors::AppError;
use crate::db::{self, DbPool};
use serde_json::json;
use geojson::{FeatureCollection, Feature};

// Handler for the root path
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from SeeMyCity Backend! (via handlers module)")
}

// Handler for retrieving a list of municipalities.
// Returns data as a GeoJSON FeatureCollection.
#[get("/api/municipalities")]
pub async fn get_municipalities(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
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
            return Err(AppError::InternalServerError("Database error".to_string()));
        }
    }
    // --- End Test --- 

    // Fetch data from the database using the query function
    let features = db::queries::get_data_for_map_view(pool.get_ref()).await?;

    // Construct the GeoJSON FeatureCollection
    let feature_collection = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    Ok(HttpResponse::Ok().json(feature_collection))
}