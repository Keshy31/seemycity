//! Functions for fetching specific financial data points from the Municipal Money API.

use super::client::MunicipalMoneyClient;
use super::types::{ApiClientError, FinancialItemFact};
use std::collections::HashSet;

/// Fetches the total operating and capital revenue for a municipality in a given year
/// by summing relevant items from the incexp_v2 aggregate endpoint.
/// It specifically targets audited figures ('AUDA').
pub async fn get_total_revenue(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    // Define the item codes for total revenue based on docs/backend-tech.md
    const REVENUE_ITEM_CODES: &[&str] = &[
        "0200", "0300", "0400", "0500", "0600", "0700", "0800", "0900", "1000", "1100",
        "1200", "1300", "1400", "1500", "1600", "1700", "1800", "1900", "2000", "2100",
        "2200", "2300", "2400", "2500",
    ];
    // Use a HashSet for efficient lookup
    let revenue_codes_set: HashSet<&str> = REVENUE_ITEM_CODES.iter().cloned().collect();

    // Fetch all items using the new aggregate function
    log::info!("Fetching all incexp items via aggregate for revenue calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_incexp_aggregate(municipality_code, year, "AUDA") // Target audited figures
        .await?;

    // Filter the results by item code and sum the amounts
    let total_revenue: f64 = response
        .cells // Use 'cells' field from FactsApiResponse<FinancialItemFact>
        .iter()
        .filter_map(|fact: &FinancialItemFact| { // Explicit type annotation
            if revenue_codes_set.contains(fact.item_code.as_str()) {
                let amount = fact.amount.unwrap_or(0.0);
                 log::trace!( // Use trace for item-level details
                    "Revenue item: code={}, label='{}', amount={}",
                    fact.item_code,
                    fact.item_label,
                    amount
                );
                Some(amount)
            } else {
                None
            }
        })
        .sum();

    log::info!(
        "Calculated Total Revenue for {} in {}: {:.2}",
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
    // Fetch all items using the new aggregate function
    log::info!("Fetching all finpos items via aggregate for debt calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_finpos_aggregate(municipality_code, year, "AUDA") // Target audited figures
        .await?;

    // Sum all items where item.code is in the range 0310 to 0500 (inclusive)
    let total_debt: f64 = response
        .cells
        .iter()
        .filter_map(|fact: &FinancialItemFact| {
            if let Ok(code) = fact.item_code.parse::<u32>() {
                if code >= 310 && code <= 500 {
                    let amount = fact.amount.unwrap_or(0.0);
                    log::trace!(
                        "Debt item: code={}, label='{}', amount={}",
                        fact.item_code,
                        fact.item_label,
                        amount
                    );
                    return Some(amount);
                }
            }
            None
        })
        .sum();

    log::debug!(
        "Calculated total debt for {} year {}: {}",
        municipality_code, year, total_debt
    );

    Ok(total_debt)
}

/// Fetches and sums expenditure items for a given municipality and year
/// by summing relevant items from the incexp_v2 aggregate endpoint.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<f64, ApiClientError> {
    // Define the item codes for total expenditure based on docs/backend-tech.md
    const EXPENDITURE_ITEM_CODES: &[&str] = &[
        "3000", "3100", "3200", "3300", "3400", "3500", "3600", "3700",
        "3800", "3900", "4000",
        // Note: Codes like 4100, 4200, etc. from API response are currently excluded based on backend-tech.md
    ];
     // Use a HashSet for efficient lookup
    let expenditure_codes_set: HashSet<&str> = EXPENDITURE_ITEM_CODES.iter().cloned().collect();

    // Fetch all items using the new aggregate function
    log::info!("Fetching all incexp items via aggregate for expenditure calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_incexp_aggregate(municipality_code, year, "AUDA") // Target audited figures
        .await?;

    // Filter the results by item code and sum the amounts
    let total_expenditure: f64 = response
        .cells // Use 'cells' field from FactsApiResponse<FinancialItemFact>
        .iter()
        .filter_map(|fact: &FinancialItemFact| { // Explicit type annotation
            if expenditure_codes_set.contains(fact.item_code.as_str()) {
                 let amount = fact.amount.unwrap_or(0.0);
                 log::trace!( // Use trace for item-level details
                    "Expenditure item: code={}, label='{}', amount={}",
                    fact.item_code,
                    fact.item_label,
                    amount
                );
                Some(amount)
            } else {
                None
            }
        })
        .sum();

    log::info!(
        "Calculated Total Expenditure for {} in {}: {:.2}",
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
    // Fetch all items using the new aggregate function
    log::info!("Fetching all capital items via aggregate for capital expenditure calculation {} year {}", municipality_code, year);
    let response = client.fetch_capital_aggregate(municipality_code, year, "AUDA").await?;

    // Sum the amounts
    let capital_expenditure: f64 = response
        .cells // Use 'cells' field from FactsApiResponse<FinancialItemFact>
        .iter()
        .filter_map(|fact: &FinancialItemFact| { // Explicit type annotation
            let amount = fact.amount.unwrap_or(0.0);
             log::trace!( // Use trace for item-level details
                "Capital expenditure item: code={}, label='{}', amount={}",
                fact.item_code,
                fact.item_label,
                amount
            );
            Some(amount)
        })
        .sum();

    log::debug!(
        "Fetched capital expenditure for {} year {}: {}",
        municipality_code, year, capital_expenditure
    );

    Ok(capital_expenditure)
}