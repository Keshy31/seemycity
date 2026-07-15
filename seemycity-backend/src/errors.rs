// src/errors.rs
use thiserror::Error;
use actix_web::{ResponseError, HttpResponse, http::StatusCode};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error), // Automatically converts sqlx::Error into AppError::SqlxError

    #[error("API Client Error: {0}")]
    ApiClientError(#[from] crate::api::muni_money::types::ApiClientError), // Used crate name

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String), // Add BadRequest variant

    #[error("Internal server error: {0}")]
    InternalError(String),
    // Add other specific error types as needed
}

// Implement ResponseError for Actix Web integration
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Upstream Treasury API failures are not our server's fault
            AppError::ApiClientError(_) => StatusCode::BAD_GATEWAY,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("Responding with error: {}", self); // Log the detailed error server-side

        // NotFound/BadRequest messages are written for clients; everything else
        // carries internal detail (SQL text, upstream bodies) that must not leak.
        let client_message = match self {
            AppError::NotFound(msg) => msg.clone(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::ApiClientError(_) => "The upstream data source is unavailable.".to_string(),
            _ => "An internal error occurred.".to_string(),
        };

        HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": client_message }))
    }
}