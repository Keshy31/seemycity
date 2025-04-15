use actix_web::{get, web, HttpResponse};
use crate::api::muni_money::client::MunicipalMoneyClient;
use crate::db::municipalities::{get_municipality_base_info_db, get_all_municipalities_basic};
use crate::db::financials::{get_latest_cached_year, get_cached_financials, upsert_complete_financial_record};
use crate::db::geo::get_data_for_map_view;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{MunicipalityDetail, FinancialYearData};
use geojson::FeatureCollection;
use chrono::{Utc, Duration};
use crate::api::muni_money::financials::{get_total_revenue, get_total_expenditure, get_capital_expenditure, get_total_debt};
use crate::api::muni_money::audit::get_audit_outcome;
use serde::Deserialize;
use crate::scoring::{self, ScoringInput};

// Cache TTL: ~91 days (quarterly) in seconds
// 91 days * 24 hours/day * 60 minutes/hour * 60 seconds/minute = 7,862,400 seconds
const CACHE_TTL_SECONDS: i64 = 7_862_400;

#[get("/api/municipalities/{id}")]
async fn get_municipality_detail_handler(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    api_client: web::Data<MunicipalMoneyClient>,
) -> Result<HttpResponse, AppError> {
    let muni_id = path.into_inner();
    log::info!("Fetching details for municipality: {}", muni_id);

    let base_info = get_municipality_base_info_db(&pool, &muni_id).await?;
    if base_info.is_none() {
        log::warn!("Municipality with ID '{}' not found in database.", muni_id);
        return Ok(HttpResponse::NotFound().json(serde_json::json!({ "error": "Municipality not found" })));
    }
    let base_info_unwrapped = base_info.unwrap(); // Safe to unwrap here

    let mut financial_data_to_use: Option<FinancialYearData> = None;
    // Determine the latest cached year, but we might not use its value if we fetch fresh
    // Renamed to avoid potential confusion if cache is hit but stale
    let latest_cached_year_opt = get_latest_cached_year(&pool, &muni_id).await?;

    if let Some(latest_year) = latest_cached_year_opt {
         match get_cached_financials(&pool, &muni_id, latest_year).await? {
             Some(cached_data) => {
                 let now = Utc::now();
                 let cache_age = now.signed_duration_since(cached_data.updated_at);
                 if cache_age < Duration::seconds(CACHE_TTL_SECONDS) {
                     log::info!("Cache hit for {} year {}", muni_id, latest_year);
                     financial_data_to_use = Some(FinancialYearData {
                         year: cached_data.financial_year,
                         revenue: cached_data.revenue,
                         expenditure: cached_data.expenditure,
                         capital_expenditure: cached_data.capital_expenditure,
                         debt: cached_data.debt,
                         audit_outcome: cached_data.audit_outcome,
                         score: cached_data.score, // Use the score from cache
                     });
                 } else {
                    log::info!("Cache stale for {} year {}. Fetching fresh data.", muni_id, latest_year);
                 }
             }
             None => {
                 log::warn!("Latest cached year {} reported for {}, but failed to retrieve data. Will fetch fresh data.", latest_year, muni_id);
             }
         }
     } else {
         log::info!("Cache miss for {}. Fetching fresh data.", muni_id);
     }


    if financial_data_to_use.is_none() {
        // If cache miss or stale, always fetch for a specific recent year (e.g., 2023)
        // TODO: Make the fetch year dynamic (e.g., based on current date or an env variable)
        let fetch_year = 2023;
        log::info!("Fetching fresh data from Municipal Money API for {} year {}", muni_id, fetch_year);

        let revenue_future = get_total_revenue(&api_client, &muni_id, fetch_year);
        let expenditure_future = get_total_expenditure(&api_client, &muni_id, fetch_year);
        let capital_future = get_capital_expenditure(&api_client, &muni_id, fetch_year);
        let debt_future = get_total_debt(&api_client, &muni_id, fetch_year);
        let audit_future = get_audit_outcome(&api_client, &muni_id, fetch_year);

        let (revenue_res, expenditure_res, capital_res, debt_res, audit_res) =
            tokio::join!(revenue_future, expenditure_future, capital_future, debt_future, audit_future);

        // Log detailed errors if any API call failed
        let revenue_api = revenue_res.map_err(|e| log::error!("Failed to get revenue: {}", e)).ok().flatten();
        let expenditure_api = expenditure_res.map_err(|e| log::error!("Failed to get expenditure: {}", e)).ok().flatten();
        let capital_expenditure_api = capital_res.map_err(|e| log::error!("Failed to get capex: {}", e)).ok().flatten();
        let debt_api = debt_res.map_err(|e| log::error!("Failed to get debt: {}", e)).ok().flatten();
        let latest_audit_opinion = audit_res.map_err(|e| log::error!("Failed to get audit outcome: {}", e)).ok().flatten();

        // --- Calculate Score ---
        let scoring_input = ScoringInput {
            revenue: revenue_api,
            expenditure: expenditure_api,
            capital_expenditure: capital_expenditure_api,
            debt: debt_api,
            audit_outcome: latest_audit_opinion.clone(), // Clone the Option<String>
             population: base_info_unwrapped.population, // Get population from base info
        };

        let calculated_score = scoring::calculate_financial_score(&scoring_input);
        log::info!("Calculated score for {}: {:?}", muni_id, calculated_score);

        // --- Upsert Data ---
        log::debug!("Upserting fresh API data (including score) into cache for {} year {}", muni_id, fetch_year);
        upsert_complete_financial_record(
            &pool,
            &muni_id,
            fetch_year,
            revenue_api,
            expenditure_api,
            capital_expenditure_api,
            debt_api,
            latest_audit_opinion.clone(), // Clone again for upsert
            calculated_score,            // Pass the calculated score
        ).await?;

        // --- Prepare Response ---
        financial_data_to_use = Some(FinancialYearData {
            year: fetch_year,
            revenue: revenue_api,
            expenditure: expenditure_api,
            capital_expenditure: capital_expenditure_api,
            debt: debt_api,
            audit_outcome: latest_audit_opinion,
            score: calculated_score, // Use calculated score in response
        });
    }

    let response_data = MunicipalityDetail {
        id: base_info_unwrapped.id,
        name: base_info_unwrapped.name,
        province: base_info_unwrapped.province,
        population: base_info_unwrapped.population,
        classification: base_info_unwrapped.classification,
        website: base_info_unwrapped.website,
        // Wrap the single year's data (or empty vec) correctly
        financials: financial_data_to_use.map_or(vec![], |data| vec![data]),
        geometry: None, // Geometry is typically fetched separately for map view
    };

    Ok(HttpResponse::Ok().json(response_data))
}

// Struct to capture optional query parameters for the map view
#[derive(Debug, Deserialize)]
pub struct MapParams {
    limit: Option<i64>,
}

#[get("/api/municipalities")]
pub async fn get_municipalities_map_handler(
    pool: web::Data<DbPool>,
    params: web::Query<MapParams>,
) -> Result<HttpResponse, AppError> {
    let limit = params.limit;
    log::info!("Fetching map data for municipalities (limit: {:?})", limit);

    let features = get_data_for_map_view(&pool, limit).await?;
    log::info!("Fetched {} features for map view", features.len());

    let feature_collection = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    Ok(HttpResponse::Ok().json(feature_collection))
}

#[get("/api/municipalities/list")]
pub async fn get_municipalities_list_handler(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    log::info!("Handling request for /api/municipalities/list");
    let municipalities_list = get_all_municipalities_basic(&pool).await?;
    Ok(HttpResponse::Ok().json(municipalities_list))
}