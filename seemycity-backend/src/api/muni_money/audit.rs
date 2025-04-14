// src/api/muni_money/audit.rs

use super::client::MunicipalMoneyClient;
use super::types::{ApiClientError, AuditApiResponse, AuditOpinionFact}; 

/// Fetches the audit outcome for a specific municipality and year.
/// 
/// Returns the audit opinion string (e.g., "Unqualified", "Qualified") if available,
/// otherwise returns None.
pub async fn get_audit_outcome(
    client: &MunicipalMoneyClient,
    muni_code: &str,
    year: i32,
) -> Result<Option<String>, ApiClientError> {
    // Fetch audit facts using the specific client method
    let response: AuditApiResponse = client
        .fetch_audit_opinion_facts(muni_code, year)
        .await?;

    // The API should return zero or one fact for a specific muni/year combo in this cube.
    // We take the first fact if it exists.
    let first_fact: Option<&AuditOpinionFact> = response.cells.first();

    match first_fact {
        Some(fact) => Ok(Some(fact.opinion_label.clone())),
        None => Ok(None), // No data found for this muni/year
    }
}