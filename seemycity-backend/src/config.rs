// src/config.rs

use std::env;
use std::num::ParseIntError;

// Define a struct to hold our configuration values
#[derive(Debug, Clone)] // Add Clone trait
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    /// Address the HTTP server binds to. Defaults to 127.0.0.1 for local dev;
    /// containers must set SERVER_HOST=0.0.0.0.
    pub server_host: String,
    pub server_port: u16,
    /// Origins allowed by CORS, from the comma-separated CORS_ALLOWED_ORIGINS
    /// variable. Defaults to the Vite dev server.
    pub cors_allowed_origins: Vec<String>,
}

// Define a custom error type for configuration loading issues
#[derive(Debug)]
pub enum ConfigError {
    MissingVar(String),
    InvalidPort(ParseIntError),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingVar(var) => write!(f, "Missing environment variable: {}", var),
            ConfigError::InvalidPort(err) => write!(f, "Invalid database port: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {}

// Function to load configuration from environment variables
pub fn load_config() -> Result<Config, ConfigError> {
    let db_host = env::var("DB_HOST")
        .map_err(|_| ConfigError::MissingVar("DB_HOST".to_string()))?;
    let db_port_str = env::var("DB_PORT")
        .map_err(|_| ConfigError::MissingVar("DB_PORT".to_string()))?;
    let db_user = env::var("DB_USER")
        .map_err(|_| ConfigError::MissingVar("DB_USER".to_string()))?;
    let db_password = env::var("DB_PASSWORD")
        .map_err(|_| ConfigError::MissingVar("DB_PASSWORD".to_string()))?;
    let db_name = env::var("DB_NAME")
        .map_err(|_| ConfigError::MissingVar("DB_NAME".to_string()))?;

    // Parse the port string to u16
    let db_port = db_port_str.parse::<u16>()
        .map_err(ConfigError::InvalidPort)?;

    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .map_err(ConfigError::InvalidPort)?;
    let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(Config {
        db_host,
        db_port,
        db_user,
        db_password,
        db_name,
        server_host,
        server_port,
        cors_allowed_origins,
    })
}