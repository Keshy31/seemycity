use actix_web::{App, HttpServer, web, middleware::{Compress, Logger}, http};
use dotenvy::dotenv; // To load .env file
use seemycity_backend::db; // Import db module (which contains create_pool and queries)
use seemycity_backend::config; // Import config module
use seemycity_backend::api::muni_money::client::MunicipalMoneyClient; // Import API Client
use seemycity_backend::handlers::municipalities::{ // Import handlers
    get_municipality_detail_handler,
    get_municipalities_list_handler, // Import the new handler
    MapResponseCache,
    UpstreamHealth,
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

    let server_host = config_arc.server_host.clone();
    let server_port = config_arc.server_port;
    log::info!("Starting HTTP server at http://{}:{}", server_host, server_port);

    // Shared across workers so the map payload is built once per TTL, not per worker
    let map_cache = web::Data::new(MapResponseCache::default());
    // Circuit breaker for the Treasury API, shared across workers
    let upstream_health = web::Data::new(UpstreamHealth::default());

    // Start Actix Web server
    let cors_origins = config_arc.cors_allowed_origins.clone();
    HttpServer::new(move || {
        // Origins come from CORS_ALLOWED_ORIGINS (comma-separated)
        let mut cors = Cors::default()
              .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
              .max_age(3600);
        for origin in &cors_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(Logger::default()) // Add logger middleware
            .wrap(Compress::default()) // gzip/brotli — GeoJSON compresses ~5-10x
            .wrap(cors) // Add CORS middleware
            .app_data(web::Data::new(pool.clone())) // Share the pool
            .app_data(web::Data::new(api_client.clone())) // Share the API client
            .app_data(map_cache.clone()) // Shared map response cache
            .app_data(upstream_health.clone()) // Treasury API circuit breaker
            // Explicitly register the detail route
            .route("/api/municipalities/{id}", web::get().to(get_municipality_detail_handler))
             // Keep using .service() for the list handler as its path is defined by its macro
            .service(get_municipalities_list_handler)
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
