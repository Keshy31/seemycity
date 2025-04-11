mod errors;
mod models;
mod db;
// mod api_client; // Keep commented until needed
// mod handlers;    // Keep commented until needed

use actix_web::{App, HttpServer, web, middleware::Logger};
use sqlx::PgPool;
use std::env; // To read environment variables
use dotenvy::dotenv; // To load .env file

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load environment variables from .env file
    dotenv().ok(); 
    log::info!("Loaded .env file using dotenvy");

    // Get Database URL from environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    

    
    // Create database connection pool
    log::info!("Connecting to database...");
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            log::info!("Successfully connected to the database!");
            pool
        }
        Err(err) => {
            log::error!("Failed to connect to the database: {}", err);
            std::process::exit(1); // Exit if connection fails
        }
    };

    // --- Temporary DB Test --- 
    log::info!("Attempting to fetch basic municipality info...");
    match db::queries::get_all_municipalities_basic(&pool).await {
        Ok(municipalities) => {
            log::info!("✅ Successfully fetched {} municipalities basic info.", municipalities.len());
            // Log the first few municipalities as an example
            for muni in municipalities.iter().take(5) {
                log::info!("- ID: {}, Name: {}", muni.id, muni.name);
            }
            if municipalities.len() > 5 {
                log::info!("...and {} more.", municipalities.len() - 5);
            }
        }
        Err(e) => {
            log::error!("Error fetching municipalities: {}", e);
        }
    }
    // --- End Temporary DB Test ---

    log::info!("Starting HTTP server at http://127.0.0.1:8080");

    // Start Actix Web server (minimal setup for now)
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Add logger middleware
            .app_data(web::Data::new(pool.clone())) // Share the pool
            // .configure(handlers::config) // Add handlers later
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
