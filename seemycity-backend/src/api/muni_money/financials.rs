//! Functions for fetching specific financial data points from the Municipal Money API.

use super::client::MunicipalMoneyClient;
use super::types::ApiClientError;

/// Fetches and sums revenue items for a given municipality and year.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_revenue(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    // Define the item codes for total revenue as per backend-tech.md
    const REVENUE_ITEM_CODES: &[&str] = &[
        "0200", "0300", "0400", "0500", "0600", "0800", "0900", "1000",
        "1100", "1200", "1300", "1400", "1500", "1600", "1700", "1800",
        "1900", "2000",
    ];

    // Define the cuts - prioritize Audited Actual ('AUDA')
    // TODO: Add fallback logic for 'ADJB', 'ORGB' if 'AUDA' returns NotFound
    let cuts = [("amount_type.code", "AUDA")];

    // Fetch facts for the incexp_v2 cube using the provided client
    let response = client
        .fetch_generic_financial_data("incexp_v2", municipality_code, year, &cuts)
        .await?;

    // Filter the results by item code and sum the amounts
    let total_revenue: f64 = response
        .data
        .iter()
        .filter_map(|fact| {
            if let Some(code) = &fact.item_code {
                if REVENUE_ITEM_CODES.contains(&code.as_str()) {
                    return Some(fact.amount.unwrap_or(0.0));
                }
            }
            None
        })
        .sum();

    log::debug!(
        "Calculated total revenue for {} year {}: {}",
        municipality_code, year, total_revenue
    );

    Ok(total_revenue)
}

/// Fetches the total liabilities (debt) for a given municipality and year.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_debt(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    const DEBT_ITEM_CODE: &str = "0500"; // TOTAL LIABILITIES

    // Define the cuts - prioritize Audited Actual ('AUDA')
    // TODO: Add fallback logic for 'ADJB', 'ORGB' if 'AUDA' returns NotFound
    let cuts = [("amount_type.code", "AUDA")];

    // Fetch facts for the financial_position_v2 cube using the provided client
    let response = client
        .fetch_generic_financial_data("financial_position_v2", municipality_code, year, &cuts)
        .await?;

    // Find the specific fact for the DEBT_ITEM_CODE
    let total_debt = response
        .data
        .iter()
        .find(|fact| fact.item_code.as_deref() == Some(DEBT_ITEM_CODE))
        .and_then(|fact| fact.amount)
        .unwrap_or(0.0);

    log::debug!(
        "Calculated total debt for {} year {}: {}",
        municipality_code, year, total_debt
    );

    Ok(total_debt)
}

/// Fetches and sums expenditure items for a given municipality and year.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    // Define the item codes for total expenditure
    const EXPENDITURE_ITEM_CODES: &[&str] = &[
        "3000", "3100", "3200", "3300", "3400", "3500", "3600", "3700",
        "3800", "3900", "4000", "4100", "4200", "4300", "4400", "4500",
        "4600", "4700", "4800", "4900",
    ];

    // Define the cuts - prioritize Audited Actual ('AUDA')
    // TODO: Add fallback logic for 'ADJB', 'ORGB' if 'AUDA' returns NotFound
    let cuts = [("amount_type.code", "AUDA")];

    // Fetch facts for the incexp_v2 cube using the provided client
    let response = client
        .fetch_generic_financial_data("incexp_v2", municipality_code, year, &cuts)
        .await?;

    // Filter the results by item code and sum the amounts
    let total_expenditure: f64 = response
        .data
        .iter()
        .filter_map(|fact| {
            if let Some(code) = &fact.item_code {
                if EXPENDITURE_ITEM_CODES.contains(&code.as_str()) {
                    return Some(fact.amount.unwrap_or(0.0));
                }
            }
            None
        })
        .sum();

    log::debug!(
        "Calculated total expenditure for {} year {}: {}",
        municipality_code, year, total_expenditure
    );

    Ok(total_expenditure)
}

/// Fetches the total capital expenditure for a given municipality and year.
/// Assumes the API returns an aggregated sum for the 'capital_v2' cube.
/// Prioritizes 'AUDA' amount type.
pub async fn get_capital_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    // Define the cuts - prioritize Audited Actual ('AUDA')
    // TODO: Add fallback logic for 'ADJB', 'ORGB' if 'AUDA' returns NotFound
    let cuts = [("amount_type.code", "AUDA")];

    // Fetch facts for the capital_v2 cube using the provided client
    let response = client
        .fetch_generic_financial_data("capital_v2", municipality_code, year, &cuts)
        .await?;

    // Expecting a single aggregated fact. Take the amount from the first fact.
    let capital_expenditure = response
        .data
        .first()
        .and_then(|fact| fact.amount)
        .unwrap_or(0.0);

    log::debug!(
        "Fetched capital expenditure for {} year {}: {}",
        municipality_code, year, capital_expenditure
    );

    Ok(capital_expenditure)
}