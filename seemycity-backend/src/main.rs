use actix_web::{ App, HttpServer, web };
use dotenvy;

// Declare modules corresponding to our file structure
pub mod api;
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;

// Import necessary items
use crate::handlers::{hello, get_municipalities};
use crate::config::load_config;
use crate::db::{create_pool, DbPool};

// The main function is the entry point of the application.
// The #[actix_web::main] macro sets up the Tokio async runtime needed by Actix.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();

    // Load configuration
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create database connection pool
    let pool = match create_pool(&config).await {
        Ok(p) => {
            println!("✅ Database pool created successfully.");
            p
        }
        Err(e) => {
            eprintln!("❌ Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    // Print a message indicating the server is starting
    println!("🚀 Server starting on http://127.0.0.1:4000");

    // HttpServer::new creates a new application instance.
    // The closure passed to `new` configures the application, defining routes and middleware.
    // We use `.app_data()` to share the database pool (wrapped in web::Data) with all handlers.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool
            .service(hello)
            .service(get_municipalities)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
