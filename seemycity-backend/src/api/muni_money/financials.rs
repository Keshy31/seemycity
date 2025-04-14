//! Functions for fetching specific financial data points from the Municipal Money API.

use super::client::MunicipalMoneyClient;
use super::types::ApiClientError;
use std::collections::HashSet;
use sqlx::types::Decimal;
use rust_decimal::prelude::FromPrimitive; 

/// Fetches the total operating and capital revenue for a municipality in a given year
/// by summing relevant items from the incexp_v2 aggregate endpoint.
/// It specifically targets audited figures ('AUDA').
pub async fn get_total_revenue(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> { 
    // Define the item codes for total revenue based on docs/backend-tech.md
    const REVENUE_ITEM_CODES: &[&str] = &[
        "0200", "0300", "0400", "0500", "0600", "0700", "0800", "0900", "1000", "1100",
        "1200", "1300", "1400", "1500", "1600", "1700", "1800", "1900", "2000", "2100",
        "2200", "2300", "2400", "2500",
    ];
    let revenue_codes_set: HashSet<&str> = REVENUE_ITEM_CODES.iter().cloned().collect();

    log::info!("Fetching all incexp items via aggregate for revenue calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_incexp_aggregate(municipality_code, year, "AUDA")
        .await?;

    let mut total_revenue = Decimal::ZERO; 
    let mut facts_found = false;

    for fact in response.cells.iter() {
        if revenue_codes_set.contains(fact.item_code.as_str()) {
            if let Some(amount_f64) = fact.amount {
                // Try converting f64 to Decimal
                if let Some(amount_decimal) = Decimal::from_f64(amount_f64) {
                    log::trace!(
                        "Revenue item: code={}, label='{}', amount={}",
                        fact.item_code,
                        fact.item_label,
                        amount_decimal 
                    );
                    total_revenue += amount_decimal;
                    facts_found = true;
                } else {
                    log::warn!("Could not convert revenue amount {} to Decimal for item {}", amount_f64, fact.item_code);
                }
            }
        }
    }

    if facts_found {
         log::info!(
            "Calculated Total Revenue for {} in {}: {}", 
            municipality_code, year, total_revenue
        );
        Ok(Some(total_revenue))
    } else {
         log::info!(
            "No valid revenue facts found for {} in {}",
            municipality_code, year
        );
        Ok(None)
    }
}

/// Fetches the total liabilities (debt) for a given municipality and year.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_debt(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> { 
    log::info!("Fetching all finpos items via aggregate for debt calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_finpos_aggregate(municipality_code, year, "AUDA")
        .await?;

    let mut total_debt = Decimal::ZERO; 
    let mut facts_found = false;

    for fact in response.cells.iter() {
         if let Ok(code) = fact.item_code.parse::<u32>() {
            if code >= 310 && code <= 500 { 
                 if let Some(amount_f64) = fact.amount {
                    if let Some(amount_decimal) = Decimal::from_f64(amount_f64) {
                        log::trace!(
                            "Debt item: code={}, label='{}', amount={}",
                            fact.item_code,
                            fact.item_label,
                            amount_decimal 
                        );
                        total_debt += amount_decimal;
                        facts_found = true;
                    } else {
                         log::warn!("Could not convert debt amount {} to Decimal for item {}", amount_f64, fact.item_code);
                    }
                 }
            }
        }
    }

     if facts_found {
         log::info!( 
            "Calculated total debt for {} year {}: {}", 
            municipality_code, year, total_debt
        );
        Ok(Some(total_debt))
    } else {
        log::info!(
            "No valid debt facts found for {} in {}",
            municipality_code, year
        );
        Ok(None)
    }
}

/// Fetches and sums expenditure items for a given municipality and year
/// by summing relevant items from the incexp_v2 aggregate endpoint.
/// Prioritizes 'AUDA' amount type.
pub async fn get_total_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> { 
    const EXPENDITURE_ITEM_CODES: &[&str] = &[
        "3000", "3100", "3200", "3300", "3400", "3500", "3600", "3700",
        "3800", "3900", "4000",
    ];
    let expenditure_codes_set: HashSet<&str> = EXPENDITURE_ITEM_CODES.iter().cloned().collect();

    log::info!("Fetching all incexp items via aggregate for expenditure calculation {} year {}", municipality_code, year);
    let response = client
        .fetch_incexp_aggregate(municipality_code, year, "AUDA")
        .await?;

    let mut total_expenditure = Decimal::ZERO; 
    let mut facts_found = false;

     for fact in response.cells.iter() {
        if expenditure_codes_set.contains(fact.item_code.as_str()) {
            if let Some(amount_f64) = fact.amount {
                if let Some(amount_decimal) = Decimal::from_f64(amount_f64) {
                     log::trace!(
                        "Expenditure item: code={}, label='{}', amount={}",
                        fact.item_code,
                        fact.item_label,
                        amount_decimal 
                    );
                    total_expenditure += amount_decimal;
                    facts_found = true;
                } else {
                     log::warn!("Could not convert expenditure amount {} to Decimal for item {}", amount_f64, fact.item_code);
                }
            }
        }
    }


    if facts_found {
         log::info!(
            "Calculated Total Expenditure for {} in {}: {}", 
            municipality_code, year, total_expenditure
        );
        Ok(Some(total_expenditure))
    } else {
        log::info!(
            "No valid expenditure facts found for {} in {}",
            municipality_code, year
        );
        Ok(None)
    }
}

/// Fetches the total capital expenditure for a given municipality and year.
/// Assumes the API returns an aggregated sum for the 'capital_v2' cube.
/// Prioritizes 'AUDA' amount type.
pub async fn get_capital_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> { 
    log::info!("Fetching all capital items via aggregate for capital expenditure calculation {} year {}", municipality_code, year);
    let response = client.fetch_capital_aggregate(municipality_code, year, "AUDA").await?;

    let mut capital_expenditure = Decimal::ZERO; 
    let mut facts_found = false;

    for fact in response.cells.iter() {
         if let Some(amount_f64) = fact.amount {
             if let Some(amount_decimal) = Decimal::from_f64(amount_f64) {
                log::trace!(
                    "Capital expenditure item: code={}, label='{}', amount={}",
                    fact.item_code,
                    fact.item_label,
                    amount_decimal 
                );
                capital_expenditure += amount_decimal;
                facts_found = true;
             } else {
                 log::warn!("Could not convert capital expenditure amount {} to Decimal for item {}", amount_f64, fact.item_code);
             }
         }
    }

     if facts_found {
         log::info!( 
            "Fetched capital expenditure for {} year {}: {}", 
            municipality_code, year, capital_expenditure
        );
        Ok(Some(capital_expenditure))
     } else {
         log::info!(
            "No valid capital expenditure facts found for {} in {}",
            municipality_code, year
        );
         Ok(None)
     }
}