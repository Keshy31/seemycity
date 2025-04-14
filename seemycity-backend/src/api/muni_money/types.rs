// src/api/muni_money/types.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when interacting with the Municipal Money API client.
#[derive(Error, Debug)]
pub enum ApiClientError {
    /// Error during the HTTP request (e.g., network issue, invalid URL).
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Error parsing the JSON response from the API.
    #[error("Failed to parse JSON response: {0}")]
    ParseError(#[from] serde_json::Error),

    /// The API returned an error status code (e.g., 4xx, 5xx).
    #[error("API request failed with status {status}: {body:?}")]
    ApiError {
        status: u16,
        body: Option<String>,
    },

    /// Data field is unexpectedly empty.
    #[error("Data field is unexpectedly empty")]
    NoData,

    /// A specific required field was missing from the API response.
    #[error("Required field missing in API response: {0}")]
    MissingField(String),

    /// The API response was not in the expected format.
    #[error("Unexpected data format in API response: {0}")]
    UnexpectedDataFormat(String),

    /// Invalid parameters provided.
    #[error("Invalid parameters provided: {0}")]
    InvalidParameters(String),
}

/// Represents a single fact from the audit_opinions cube.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AuditOpinionFact {
    #[serde(rename = "demarcation.code")]
    pub demarcation_code: String,
    #[serde(rename = "demarcation.label")]
    pub demarcation_label: String,
    #[serde(rename = "opinion.label")]
    pub opinion_label: String,
    #[serde(rename = "financial_year_end.year")]
    pub financial_year: i32,
    #[serde(rename = "opinion.code")]
    pub opinion_code: String,
    #[serde(rename = "amount.sum")]
    pub amount: Option<f64>,
}

/// Represents a single financial item entry from the aggregate endpoint.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FinancialItemFact {
    #[serde(rename = "demarcation.code")]
    pub demarcation_code: String,
    #[serde(rename = "demarcation.label")]
    pub demarcation_label: String,
    #[serde(rename = "item.code")]
    pub item_code: String,
    #[serde(rename = "item.label")]
    pub item_label: String,
    #[serde(rename = "amount.sum")]
    pub amount: Option<f64>,
}

/// Represents the overall structure of the /facts API response.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FactsApiResponse<T> {
    pub total_cell_count: u32,
    pub cells: Vec<T>,
}

/// Represents the overall structure of the /facts API response for the audit_opinions cube.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AuditApiResponse {
    pub total_cell_count: u32,
    pub cells: Vec<AuditOpinionFact>,
}

/// Struct for financial summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FinancialSummary {
    pub year: i32,
    pub municipality_code: String,
    pub total_revenue: Option<f64>,
    pub total_expenditure: Option<f64>,
}