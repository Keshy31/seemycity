// src/api/muni_money/client.rs
use super::types::{ApiClientError, AuditApiResponse};
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
            .map_err(|e| {
                log::error!("Failed to build reqwest client: {}", e);
                ApiClientError::RequestError(e)
            })?;

        Ok(Self { client, base_url })
    }

    /// Fetches audit opinion facts for a specific municipality and year.
    ///
    /// # Arguments
    /// * `municipality_code` - The demarcation code of the municipality (e.g., "BUF").
    /// * `year` - The financial year end year.
    ///
    /// # Returns
    /// A `Result` containing the parsed `AuditApiResponse` or an `ApiClientError`.
    /// NOTE: This currently expects the API to return the old `AuditApiResponse` structure.
    /// This might need updating if the audit endpoint changes or if we want to standardize response parsing.
    pub async fn fetch_audit_opinion_facts(
        &self,
        municipality_code: &str,
        year: i32,
    ) -> Result<AuditApiResponse, ApiClientError> {
        const AUDIT_OPINION_CUBE: &str = "audit_opinions";
        // Define the specific fields we want from the audit opinions cube
        const AUDIT_DRILLDOWNS: &str = "demarcation.code|demarcation.label|opinion.code|opinion.label|financial_year_end.year";
        const AUDIT_AGGREGATES: &str = "amount.sum"; // Assuming we might still need a sum? Or just the labels?

        // Base cuts for municipality and year
        let cuts = format!(
            "demarcation.code:\"{}\"|financial_year_end.year:{}",
            municipality_code, year
        );

        // Construct URL - Using aggregate endpoint for potential future consistency?
        // Or should revert to /facts if that's more appropriate for this specific data?
        // Let's assume /aggregate for now, similar to incexp.
        let url = format!(
            "{}/cubes/{}/aggregate?drilldown={}&cut={}&aggregates={}",
            self.base_url, AUDIT_OPINION_CUBE, AUDIT_DRILLDOWNS, cuts, AUDIT_AGGREGATES
        );

        log::debug!("Fetching Audit Opinions URL: {}", url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
             let status = response.status();
             let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
             log::error!(
                "Audit Opinion API request failed with status {}: {}",
                status,
                body
            );
             return Err(ApiClientError::ApiError {
                status: status.as_u16(),
                body: Some(body),
            });
        }

        // Deserialize. IMPORTANT: Assumes AuditApiResponse structure matches the aggregate response format.
        // This might need adjustment based on the actual API response for the audit cube aggregate.
        // If the audit aggregate response is different, we might need a separate struct or parsing logic.
        let data: AuditApiResponse = response.json().await.map_err(ApiClientError::RequestError)?;

        log::trace!("Received Audit Opinion API response data: {:?}", data);

        Ok(data)
    }

    /// Returns a reference to the internal reqwest::Client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns a reference to the base_url string.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Default for MunicipalMoneyClient {
    /// Provides a default instance of the client. Panics if initialization fails.
    /// Useful for simpler setups or tests where failure is unexpected.
    fn default() -> Self {
        Self::new().expect("Failed to create default MunicipalMoneyClient")
    }
}