// src/db/mod.rs

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::config::Config; // Import the Config struct

// Type alias for the database pool for convenience
pub type DbPool = PgPool;

// Declare the new modules
pub mod municipalities;
pub mod financials;
pub mod geo;

// Function to create the database connection pool
pub async fn create_pool(config: &Config) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10) // Example: Set max connections
        .connect_with(sqlx::postgres::PgConnectOptions::new()
            .host(&config.db_host)
            .port(config.db_port)
            .username(&config.db_user)
            .password(&config.db_password)
            .database(&config.db_name))
        .await
}