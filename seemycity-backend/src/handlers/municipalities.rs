// Add these imports at the top of the file if they are missing
use actix_web::{get, web, HttpResponse}; // Import actix_web components
use serde::Deserialize; // Import Deserialize
use crate::api::muni_money::audit::get_audit_outcome; // Correct audit import path
use crate::api::muni_money::client::MunicipalMoneyClient;
use crate::api::muni_money::financials::{ // Correct financials import path
    get_capital_expenditure, get_total_debt, get_total_expenditure, get_total_revenue,
};
use crate::db::financials::{get_all_financial_years_db, upsert_complete_financial_record}; // Import DB functions
use crate::db::municipalities::{get_municipality_base_info_db, get_municipalities_summary_for_map}; // <-- Add new DB function
use crate::errors::AppError; // Import custom error type
use crate::models::{FinancialYearData, MunicipalityDetail, MapFeatureCollection}; // <-- Add MapFeatureCollection
use crate::scoring::{calculate_financial_score, ScoringInput};
use sqlx::PgPool as DbPool;
use tokio; // Import tokio

// Replace the existing function with this one:
// Handler to get details for a single municipality by ID
pub async fn get_municipality_detail_handler(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    api_client: web::Data<MunicipalMoneyClient>,
) -> Result<HttpResponse, AppError> {
    let muni_id_str = path.into_inner();
    log::info!("START: Handling request for /api/municipality/{}", muni_id_str); // Updated log message path

    // Fetch base municipality info
    let base_info = get_municipality_base_info_db(&pool, &muni_id_str).await?;
    let base_info_unwrapped = base_info.ok_or_else(|| {
        log::warn!("Municipality base info not found for ID: {}", muni_id_str);
        AppError::NotFound(format!("Municipality with ID {} not found", muni_id_str))
    })?;
    let muni_code = base_info_unwrapped.id.clone();
    let population_opt = base_info_unwrapped.population;

    // Determine the financial year to fetch/calculate for
    let fetch_year = 2023; // Hardcode to 2023 for now
    log::debug!("Muni: {}, Determined fetch year: {}", muni_id_str, fetch_year);

    // Fetch existing financial records from DB
    let financial_data_vec = get_all_financial_years_db(&pool, &muni_id_str).await?;

    // Find data for the target year, or start with a default struct
    let mut financial_data = financial_data_vec
        .into_iter()
        .find(|fd| fd.year == fetch_year) // Corrected field name: financial_year -> year
        .unwrap_or_else(|| {
            log::warn!(
                "Muni: {}, No financial data found in DB for year {}. Starting with default.",
                muni_id_str,
                fetch_year
            );
            FinancialYearData {
                year: fetch_year, // Corrected field name: financial_year -> year
                ..Default::default()
            }
        });

    // --- Concurrently Fetch Missing Data from API ---
    let (revenue_res, expenditure_res, capex_res, debt_res, audit_res) = tokio::join!(
        async {
            if financial_data.revenue.is_none() {
                log::debug!("Muni: {}, Fetching Revenue for {}", muni_id_str, fetch_year);
                get_total_revenue(&api_client, &muni_code, fetch_year).await
            } else {
                Ok(financial_data.revenue) // Use existing value
            }
        },
        async {
            if financial_data.expenditure.is_none() {
                log::debug!("Muni: {}, Fetching Expenditure for {}", muni_id_str, fetch_year);
                get_total_expenditure(&api_client, &muni_code, fetch_year).await
            } else {
                Ok(financial_data.expenditure)
            }
        },
        async {
            if financial_data.capital_expenditure.is_none() {
                log::debug!("Muni: {}, Fetching Capex for {}", muni_id_str, fetch_year);
                get_capital_expenditure(&api_client, &muni_code, fetch_year).await
            } else {
                Ok(financial_data.capital_expenditure)
            }
        },
        async {
            if financial_data.debt.is_none() {
                log::debug!("Muni: {}, Fetching Debt for {}", muni_id_str, fetch_year);
                get_total_debt(&api_client, &muni_code, fetch_year).await
            } else {
                Ok(financial_data.debt)
            }
        },
        async {
            if financial_data.audit_outcome.is_none() {
                log::debug!("Muni: {}, Fetching Audit Outcome for {}", muni_id_str, fetch_year);
                get_audit_outcome(&api_client, &muni_code, fetch_year).await
            } else {
                Ok(financial_data.audit_outcome.clone()) // Clone Option<String>
            }
        }
    );

    // --- Update Financial Data with Fetched Results ---
    // Log errors from API calls but proceed; scoring might still be possible partially
    financial_data.revenue = revenue_res.map_err(|e| log::error!("Muni: {}, Failed Revenue fetch: {}", muni_id_str, e)).ok().flatten();
    financial_data.expenditure = expenditure_res.map_err(|e| log::error!("Muni: {}, Failed Expenditure fetch: {}", muni_id_str, e)).ok().flatten();
    financial_data.capital_expenditure = capex_res.map_err(|e| log::error!("Muni: {}, Failed Capex fetch: {}", muni_id_str, e)).ok().flatten();
    financial_data.debt = debt_res.map_err(|e| log::error!("Muni: {}, Failed Debt fetch: {}", muni_id_str, e)).ok().flatten();
    financial_data.audit_outcome = audit_res.map_err(|e| log::error!("Muni: {}, Failed Audit fetch: {}", muni_id_str, e)).ok().flatten();

    // --- Calculate Scores ---
    log::debug!("Muni: {}, Calculating scores for year {}", muni_id_str, fetch_year);
    let scoring_input = ScoringInput {
        revenue: financial_data.revenue,
        expenditure: financial_data.expenditure,
        capital_expenditure: financial_data.capital_expenditure,
        debt: financial_data.debt,
        audit_outcome: financial_data.audit_outcome.clone(),
        population: population_opt.map(|p| p as u32), // Cast f32 to u32
    };

    // Calculate scores using the (potentially updated) financial data
    if let Some(score_breakdown) = calculate_financial_score(&scoring_input) {
        log::debug!("Muni: {}, Scores calculated: {:?}", muni_id_str, score_breakdown);
        financial_data.overall_score = Some(score_breakdown.overall_score);
        financial_data.financial_health_score = Some(score_breakdown.financial_health_score);
        financial_data.infrastructure_score = Some(score_breakdown.infrastructure_score);
        financial_data.efficiency_score = Some(score_breakdown.efficiency_score);
        financial_data.accountability_score = Some(score_breakdown.accountability_score);
    } else {
        log::warn!("Muni: {}, Scoring calculation failed. Scores set to None.", muni_id_str);
        // Ensure all scores are None if calculation fails
        financial_data.overall_score = None;
        financial_data.financial_health_score = None;
        financial_data.infrastructure_score = None;
        financial_data.efficiency_score = None;
        financial_data.accountability_score = None;
    }

    // --- Upsert Data and Scores to DB ---
    log::debug!("Muni: {}, Upserting financial data for year {}", muni_id_str, fetch_year);
    match upsert_complete_financial_record(
        &pool,
        &muni_code, // Use the cloned muni_code
        fetch_year,
        financial_data.revenue,
        financial_data.expenditure,
        financial_data.capital_expenditure,
        financial_data.debt,
        financial_data.audit_outcome.clone(), // Clone Option<String> again
        financial_data.overall_score,
        financial_data.financial_health_score,
        financial_data.infrastructure_score,
        financial_data.efficiency_score,
        financial_data.accountability_score,
    )
    .await
    {
        Ok(_) => log::debug!("Muni: {}, Successfully upserted data for {}", muni_id_str, fetch_year),
        Err(e) => {
            // Log DB error but don't fail the request; return potentially stale data
            log::error!("Muni: {}, Failed to upsert data for {}: {}", muni_id_str, fetch_year, e);
        }
    }

    // --- Prepare and Return Response ---
    // Currently returns only the data for the `fetch_year`.
    // Fetch geometry separately if/when needed for the detail view.
    let geometry = None; // Placeholder
    let response = MunicipalityDetail {
        id: base_info_unwrapped.id,
        name: base_info_unwrapped.name,
        province: base_info_unwrapped.province,
        population: base_info_unwrapped.population,
        classification: base_info_unwrapped.classification,
        website: base_info_unwrapped.website,
        financials: vec![financial_data], // Return the potentially updated data for the year
        geometry,
    };

    log::info!("END: Handling request for /api/municipalities/{}", muni_id_str);
    Ok(HttpResponse::Ok().json(response))
}

// --- Handler for fetching municipality list/summary (GeoJSON) ---

// Define query parameters for the list endpoint
#[derive(Deserialize, Debug)]
pub struct ListQuery { 
    limit: Option<i64>, // Optional limit parameter
}

// GET /api/municipalities
#[get("/api/municipalities")]
pub async fn get_municipalities_list_handler(
    pool: web::Data<DbPool>,
    query: web::Query<ListQuery>, // Extract query parameters
) -> Result<HttpResponse, AppError> {
    let limit = query.limit;
    log::info!("START: Handling request for /api/municipalities with limit: {:?}", limit);

    // Fetch the features using the new DB function
    let map_features = get_municipalities_summary_for_map(&pool, limit).await?;

    // Construct the FeatureCollection
    let feature_collection = MapFeatureCollection {
        collection_type: "FeatureCollection".to_string(),
        features: map_features,
    };

    log::info!("END: Returning {} features for /api/municipalities", feature_collection.features.len());
    Ok(HttpResponse::Ok().json(feature_collection))
}