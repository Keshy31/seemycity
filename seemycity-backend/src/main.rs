use actix_web::{App, HttpServer, web, middleware::Logger};
use dotenvy::dotenv; // To load .env file
use seemycity_backend::db; // Import db module (which contains create_pool and queries)

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load environment variables from .env file
    dotenv().ok(); 
    log::info!("Loaded .env file using dotenvy");

    // Load configuration
    let config = match seemycity_backend::config::load_config() { // Call the load_config function
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create database connection pool
    log::info!("Connecting to database...");
    let pool = match db::create_pool(&config).await { // Use create_pool from library
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
    match db::queries::get_all_municipalities_basic(&pool).await { // Use function from library
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
            // Consider specific error handling if AppError provides more context
        }
    }
    // --- End Temporary DB Test ---

    log::info!("Starting HTTP server at http://127.0.0.1:4000");

    // Start Actix Web server (minimal setup for now)
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Add logger middleware
            .app_data(web::Data::new(pool.clone())) // Share the pool
            // .configure(handlers::config) // Add handlers later
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
