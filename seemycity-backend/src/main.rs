use actix_web::{App, HttpServer, web, middleware::Logger, http};
use dotenvy::dotenv; // To load .env file
use seemycity_backend::db; // Import db module (which contains create_pool and queries)
use seemycity_backend::config; // Import config module
use seemycity_backend::api::muni_money::client::MunicipalMoneyClient; // Import API Client
use seemycity_backend::handlers::municipalities::{ // Import handlers
    get_municipality_detail_handler,
    get_municipalities_list_handler, // Import the new handler
};
use std::sync::Arc; // Import Arc if needed for Cache later, good practice
use actix_cors::Cors; // Import CORS

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load environment variables from .env file
    dotenv().ok();
    log::info!("Loaded .env file using dotenvy");

    // Load configuration
    let config = match config::load_config() { // Use imported config module
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    let config_arc = Arc::new(config); // Cloneable config for client

    // Create database connection pool
    log::info!("Connecting to database...");
    let pool = match db::create_pool(&config_arc).await { // Use create_pool from library
        Ok(pool) => {
            log::info!("Successfully connected to the database!");
            pool
        }
        Err(err) => {
            log::error!("Failed to connect to the database: {}", err);
            std::process::exit(1); // Exit if connection fails
        }
    };

    // Create Municipal Money API Client instance
    let api_client = match MunicipalMoneyClient::new() { 
        Ok(client) => client,
        Err(e) => {
            log::error!("Failed to create Municipal Money API client: {}", e);
            std::process::exit(1);
        }
    };

    let server_port: u16 = 4000; 
    log::info!("Starting HTTP server at http://127.0.0.1:{}", server_port); 

    // Start Actix Web server
    HttpServer::new(move || {
        // Define CORS settings
        let cors = Cors::default()
              .allowed_origin("http://localhost:5173") // Allow frontend dev origin
              // In production, you might want to restrict this further or use allowed_origin_fn
              // .allowed_origin("YOUR_PRODUCTION_FRONTEND_URL") 
              .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
              .supports_credentials()
              .max_age(3600);

        App::new()
            .wrap(Logger::default()) // Add logger middleware
            .wrap(cors) // Add CORS middleware
            .app_data(web::Data::new(pool.clone())) // Share the pool
            .app_data(web::Data::new(api_client.clone())) // Share the API client
            // Explicitly register the detail route
            .route("/api/municipalities/{id}", web::get().to(get_municipality_detail_handler))
             // Keep using .service() for the list handler as its path is defined by its macro
            .service(get_municipalities_list_handler)
    })
    .bind(("127.0.0.1", server_port))? 
    .run()
    .await
}
