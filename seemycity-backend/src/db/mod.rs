// src/db/mod.rs

use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::config::Config; // Import the Config struct
use std::time::Duration;

// Type alias for the database pool for convenience
pub type DbPool = PgPool;

// Function to create and connect the database pool
pub async fn create_pool(config: &Config) -> Result<DbPool, sqlx::Error> {
    // Construct the database connection URL from the config components
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password, // Note: Use the password directly from config
        config.db_host,
        config.db_port,
        config.db_name
    );

    // Configure the pool options
    PgPoolOptions::new()
        .max_connections(10) // Example: Set max number of connections
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_url)
        .await // Connect to the database
}