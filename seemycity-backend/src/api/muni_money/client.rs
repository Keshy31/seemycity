// src/api/muni_money/client.rs
use super::types::{ApiClientError, FactsApiResponse}; // Import types from sibling module
use reqwest::Client;
use std::env;
use std::time::Duration;

const MUNI_MONEY_API_BASE_URL_ENV_VAR: &str = "MUNI_MONEY_API_BASE_URL";
const DEFAULT_MUNI_MONEY_API_BASE_URL: &str = "https://municipaldata.treasury.gov.za/api";
const DEFAULT_TIMEOUT_SECONDS: u64 = 30; // Timeout for API requests

/// Client for interacting with the Municipal Money API.
#[derive(Debug, Clone)]
pub struct MunicipalMoneyClient {
    client: Client,
    base_url: String,
}

impl MunicipalMoneyClient {
    /// Creates a new MunicipalMoneyClient with default settings.
    /// Reads the base URL from the MUNI_MONEY_API_BASE_URL environment variable,
    /// falling back to a default value if not set.
    pub fn new() -> Result<Self, ApiClientError> {
        let base_url = env::var(MUNI_MONEY_API_BASE_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_MUNI_MONEY_API_BASE_URL.to_string());

        log::info!("Initializing Municipal Money client with base URL: {}", base_url);

        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            .build()
            .map_err(ApiClientError::Request)?; // Map reqwest builder error

        Ok(Self { client, base_url })
    }

    /// Fetches generic financial data (facts) from a specified cube.
    ///
    /// # Arguments
    /// * `cube` - The name of the data cube (e.g., "incexp_v2", "financial_position_v2").
    /// * `municipality_code` - The demarcation code of the municipality (e.g., "BUF").
    /// * `year` - The financial year end year.
    /// * `cuts` - A slice of key-value pairs representing API 'cut' parameters (e.g., [("amount_type.code", "AUDA")]).
    ///
    /// # Returns
    /// A `Result` containing the parsed `FactsApiResponse` or an `ApiClientError`.
    pub async fn fetch_generic_financial_data(
        &self,
        cube: &str,
        municipality_code: &str,
        year: i32,
        cuts: &[(&str, &str)], // Use a slice for flexibility
    ) -> Result<FactsApiResponse, ApiClientError> {
        // Base cuts for municipality and year
        let base_cuts = format!(
            "demarcation.code:\"{}\"|financial_year_end.year:{}",
            municipality_code, year
        );

        // Combine base cuts with additional provided cuts
        let additional_cuts = cuts
            .iter()
            .map(|(k, v)| format!("{}:\"{}\"", k, v))
            .collect::<Vec<String>>()
            .join("|");

        let all_cuts = if additional_cuts.is_empty() {
            base_cuts
        } else {
            format!("{}|{}", base_cuts, additional_cuts)
        };

        // Construct URL - consider adding 'fields' and 'drilldowns' if needed
        let url = format!(
            "{}/cubes/{}/facts?cut={}", // Add &fields=... &drilldowns=... if used
            self.base_url, cube, all_cuts
        );

        log::debug!("Fetching URL: {}", url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("API request failed with status {}: {}", status, body);
            return Err(ApiClientError::ApiError { status, body });
        }

        let data: FactsApiResponse = response.json().await?;

        log::trace!("Received API response data: {:?}", data);

        Ok(data) // Return the parsed struct
    }
}

impl Default for MunicipalMoneyClient {
    /// Provides a default instance of the client. Panics if initialization fails.
    /// Useful for simpler setups or tests where failure is unexpected.
    fn default() -> Self {
        Self::new().expect("Failed to create default MunicipalMoneyClient")
    }
}