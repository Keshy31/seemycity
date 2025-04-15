// src/lib.rs

// Declare the modules publicly so they form the library's public API
// and can be accessed by the binary crate (main.rs) or other consumers.
pub mod api;
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod utils;
pub mod scoring;

// Re-export key items for convenience
pub use api::{ApiClientError, MunicipalMoneyClient};
pub use db::DbPool; // Assuming DbPool is defined in src/db.rs
pub use config::Config; // Re-export Config struct