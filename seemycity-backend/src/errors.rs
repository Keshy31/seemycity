// src/errors.rs
use thiserror::Error;
use actix_web::{ResponseError, HttpResponse, http::StatusCode};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error), // Automatically converts sqlx::Error into AppError::SqlxError

    #[error("API Client Error: {0}")]
    ApiClientError(#[from] crate::api::muni_money::types::ApiClientError), // Used crate name

    #[error("GeoJSON Error: {0}")]
    GeoJsonError(#[from] geojson::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
    // Add other specific error types as needed
}

// Implement ResponseError for Actix Web integration
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ApiClientError(_) => StatusCode::INTERNAL_SERVER_ERROR, // Or maybe BAD_GATEWAY if appropriate
            AppError::GeoJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("Responding with error: {}", self); // Log the detailed error
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() })) // Return a generic error message
    }
}