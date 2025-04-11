// src/api/muni_money/types.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when interacting with the Municipal Money API client.
#[derive(Error, Debug)]
pub enum ApiClientError {
    /// Error during the HTTP request (e.g., network issue, invalid URL).
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Error parsing the JSON response from the API.
    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// The API returned an error status code (e.g., 4xx, 5xx).
    #[error("API returned error status {status}: {body}")]
    ApiError { status: reqwest::StatusCode, body: String },

    /// The API response was not in the expected format.
    #[error("Unexpected API response format: {0}")]
    UnexpectedFormat(String),

    /// A specific required field was missing from the API response.
    #[error("Missing required field: {0}")]
    MissingField(String),

    // TODO: Consider adding more specific errors, e.g., NotFound for specific items
}

/// Represents a single financial data point (fact) from the Municipal Money API.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct FinancialFact {
    // Basic financial value
    #[serde(rename = "amount.sum")]
    pub amount: Option<f64>, // Use f64 for financial figures

    // Dimensions (Examples - adapt based on actual API responses)
    #[serde(rename = "item.code")] // Add this for filtering/identification
    pub item_code: Option<String>,

    #[serde(rename = "amount_type.code")]
    pub amount_type: Option<String>,

    #[serde(rename = "financial_year_end.year")]
    pub year: Option<i32>,

    #[serde(rename = "demarcation.code")]
    pub municipality_code: Option<String>,
    // Add other relevant dimensions as needed (e.g., period, function.code)
}

/// Represents the overall structure of the /facts API response.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct FactsApiResponse {
    pub total_fact_count: usize,
    pub data: Vec<FinancialFact>,
    // Add other top-level fields if present in the response (e.g., 'page', 'total_pages')
}

// Potential future types related to the API:
// pub struct AuditOpinionFact { ... }
// pub struct AuditApiResponse { ... }