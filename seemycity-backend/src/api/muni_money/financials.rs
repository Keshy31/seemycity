//! Functions for fetching specific financial data points from the Municipal Money API.

use super::client::MunicipalMoneyClient;
use super::types::{ApiClientError, FinancialItemFact};
use std::ops::RangeInclusive;
use sqlx::types::Decimal;
use rust_decimal::prelude::FromPrimitive;

// incexp_v2 item ranges on the cube's mSCOA basis, validated 2026-07-07
// against Cape Town's audited AFS FY2024 (note 37.4.1 reconciliation:
// mSCOA expenditure 58.45bn vs our sum 58.67bn, +0.4%; mSCOA revenue
// 61.47bn vs our sum 61.84bn, +0.6%) and against the rollup identity
// below across 8 sample municipalities. See docs/backend-tech.md.
//
// Operating revenue: property rates, service charges, transfers (2200),
// fines, gains, etc. Excludes capital transfers (4600/4700) and all
// below-the-line items (4900+).
const REVENUE_ITEM_RANGE: RangeInclusive<u32> = 200..=2800;
// Operating expenditure: payroll (3100), bulk purchases, depreciation,
// contracted services, operational cost (4100), losses (4300), etc.
const EXPENDITURE_ITEM_RANGE: RangeInclusive<u32> = 3000..=4300;
// Item 2900 "Other expenditure" is in fact a TOTAL-REVENUE ROLLUP
// (|2900 - Σrevenue| = 0.00% for every municipality tested) mislabeled in
// the Treasury cube. It sits between the two ranges and must never be
// summed into either.
const REVENUE_ROLLUP_ITEM: u32 = 2900;

/// Sums the amounts of cells whose numeric item code falls in `range`.
/// Returns `None` when no matching fact carried an amount — "no data",
/// as distinct from a legitimate sum of zero.
fn sum_item_range(
    cells: &[FinancialItemFact],
    range: &RangeInclusive<u32>,
    what: &str,
) -> Option<Decimal> {
    let mut total = Decimal::ZERO;
    let mut facts_found = false;

    for fact in cells {
        let Ok(code) = fact.item_code.parse::<u32>() else {
            continue;
        };
        if code == REVENUE_ROLLUP_ITEM || !range.contains(&code) {
            continue;
        }
        if let Some(amount_f64) = fact.amount {
            if let Some(amount_decimal) = Decimal::from_f64(amount_f64) {
                log::trace!(
                    "{} item: code={}, label='{}', amount={}",
                    what, fact.item_code, fact.item_label, amount_decimal
                );
                total += amount_decimal;
                facts_found = true;
            } else {
                log::warn!(
                    "Could not convert {} amount {} to Decimal for item {}",
                    what, amount_f64, fact.item_code
                );
            }
        }
    }

    facts_found.then_some(total)
}

/// Figures extracted from one incexp_v2 aggregate response.
#[derive(Debug, Clone, Default)]
pub struct IncexpFigures {
    pub revenue: Option<Decimal>,
    pub operational_expenditure: Option<Decimal>,
    /// The cube's own total-revenue rollup (item 2900), used as a checksum
    /// against `revenue` by the data-confidence layer.
    pub revenue_checksum: Option<Decimal>,
}

/// Fetches total revenue and total operational expenditure together from a
/// single incexp_v2 aggregate call ('AUDA' figures). Both metrics live in the
/// same cube, so fetching them separately would download the identical
/// response twice.
pub async fn get_revenue_and_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<IncexpFigures, ApiClientError> {
    log::info!(
        "Fetching incexp aggregate for revenue + expenditure, {} year {}",
        municipality_code, year
    );
    let response = client
        .fetch_incexp_aggregate(municipality_code, year, "AUDA")
        .await?;

    let revenue = sum_item_range(&response.cells, &REVENUE_ITEM_RANGE, "revenue");
    let expenditure = sum_item_range(&response.cells, &EXPENDITURE_ITEM_RANGE, "expenditure");
    let revenue_checksum = response
        .cells
        .iter()
        .find(|c| c.item_code.parse::<u32>() == Ok(REVENUE_ROLLUP_ITEM))
        .and_then(|c| c.amount)
        .and_then(Decimal::from_f64);
    log::info!(
        "Incexp results for {} in {}: revenue={:?}, expenditure={:?}",
        municipality_code, year, revenue, expenditure
    );
    Ok(IncexpFigures { revenue, operational_expenditure: expenditure, revenue_checksum })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn fact(code: &str, amount: Option<f64>) -> FinancialItemFact {
        FinancialItemFact {
            demarcation_code: "TST".to_string(),
            demarcation_label: "Test".to_string(),
            item_code: code.to_string(),
            item_label: format!("Item {code}"),
            amount,
        }
    }

    #[test]
    fn revenue_range_includes_gains_and_transfers_excludes_rollup_and_capital() {
        let cells = vec![
            fact("1800", Some(100.0)), // property rates — in
            fact("2200", Some(50.0)),  // operational transfers — in
            fact("2700", Some(25.0)),  // other gains — in
            fact("2900", Some(999.0)), // total-revenue rollup — OUT
            fact("3100", Some(70.0)),  // payroll — expenditure, out
            fact("4600", Some(40.0)),  // capital transfers — out
        ];
        assert_eq!(
            sum_item_range(&cells, &REVENUE_ITEM_RANGE, "revenue"),
            Some(dec!(175.0))
        );
    }

    #[test]
    fn expenditure_range_includes_opcost_and_losses_excludes_rollup() {
        let cells = vec![
            fact("2900", Some(999.0)), // rollup — OUT even though numerically < 3000
            fact("3100", Some(60.0)),  // payroll — in
            fact("4100", Some(30.0)),  // operational cost — in (missed pre-2026 fix)
            fact("4300", Some(10.0)),  // other losses — in
            fact("4600", Some(40.0)),  // capital transfers — out
            fact("4900", Some(5.0)),   // income tax — out
        ];
        assert_eq!(
            sum_item_range(&cells, &EXPENDITURE_ITEM_RANGE, "expenditure"),
            Some(dec!(100.0))
        );
    }

    #[test]
    fn no_matching_facts_is_none_not_zero() {
        let cells = vec![fact("2900", Some(999.0)), fact("bogus", Some(1.0))];
        assert_eq!(sum_item_range(&cells, &REVENUE_ITEM_RANGE, "revenue"), None);
        assert_eq!(sum_item_range(&cells, &EXPENDITURE_ITEM_RANGE, "expenditure"), None);
    }
}

/// Fetches the total operating and capital revenue for a municipality in a given year.
/// Thin wrapper over [`get_revenue_and_expenditure`]; prefer that function when
/// you need both metrics.
pub async fn get_total_revenue(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> {
    Ok(get_revenue_and_expenditure(client, municipality_code, year).await?.revenue)
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

/// Fetches total operational expenditure for a given municipality and year.
/// Thin wrapper over [`get_revenue_and_expenditure`]; prefer that function when
/// you need both metrics.
pub async fn get_total_expenditure(
    client: &MunicipalMoneyClient,
    municipality_code: &str,
    year: i32,
) -> Result<Option<Decimal>, ApiClientError> {
    Ok(get_revenue_and_expenditure(client, municipality_code, year)
        .await?
        .operational_expenditure)
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