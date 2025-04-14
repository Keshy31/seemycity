use actix_web::{get, web, HttpResponse};
use crate::api::muni_money::client::MunicipalMoneyClient;
use crate::api::muni_money::{audit, financials};
// use crate::utils::cache::Cache; // Commented out - Cache not yet implemented
use crate::db::queries::get_municipality_base_info_db;
use crate::db::queries::upsert_financial_data;
use crate::db::DbPool;
use crate::models::{MunicipalityDb, MunicipalityDetail, FinancialYearData, FinancialDataPoint};
use crate::errors::AppError;

#[get("/municipalities/{municipality_code}")]
async fn get_municipality_detail_handler(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    // cache: web::Data<Arc<Cache>>, // Commented out
    api_client: web::Data<MunicipalMoneyClient>,
) -> Result<HttpResponse, AppError> {
    let municipality_code = path.into_inner();
    // let cache_key = format!("municipality_detail_{}", municipality_code); // Commented out
    let current_year = 2023; // Example: Or get dynamically

    // 1. Check Cache (Placeholder - commented out)
    /*
    if let Some(cached_data) = cache.get::<MunicipalityDetail>(&cache_key).await {
        log::info!("Cache HIT for {}", cache_key);
        return Ok(HttpResponse::Ok().json(cached_data));
    }
    */
    log::info!("Cache MISS for {}. Fetching from DB and API.", municipality_code);

    // 2. Fetch Base Info (full details) from DB if cache miss
    let base_info_db: Option<MunicipalityDb> = get_municipality_base_info_db(&pool, &municipality_code)
        .await
        .map_err(|e| {
            log::error!("Database error fetching base info for {}: {:?}", municipality_code, e);
            e 
        })?;

    let base_info_db = match base_info_db {
        Some(info) => info,
        None => {
            log::warn!("Municipality base info not found for code: {}", municipality_code);
            return Err(AppError::NotFound(format!("Municipality {} not found", municipality_code)));
        }
    };

    // 3. Fetch Financial Data from API
    let total_revenue_res = financials::get_total_revenue(&api_client, &municipality_code, current_year).await;
    let total_expenditure_res = financials::get_total_expenditure(&api_client, &municipality_code, current_year).await;
    let total_debt_res = financials::get_total_debt(&api_client, &municipality_code, current_year).await;
    let capital_expenditure_res = financials::get_capital_expenditure(&api_client, &municipality_code, current_year).await;
    let audit_opinion_res = audit::get_audit_outcome(&api_client, &municipality_code, current_year).await;

    let total_revenue = total_revenue_res.map_err(AppError::ApiClientError)?;
    let total_expenditure = total_expenditure_res.map_err(AppError::ApiClientError)?;
    let total_debt = total_debt_res.map_err(AppError::ApiClientError)?;
    let capital_expenditure = capital_expenditure_res.map_err(AppError::ApiClientError)?;
    let audit_opinion = audit_opinion_res.map_err(AppError::ApiClientError)?;

    // 4. Upsert Financial Data to DB
    let financial_data_points = vec![
        FinancialDataPoint {
            municipality_code: municipality_code.clone(),
            year: current_year,
            metric_name: "total_revenue".to_string(),
            amount: total_revenue,
        },
        FinancialDataPoint {
            municipality_code: municipality_code.clone(),
            year: current_year,
            metric_name: "total_expenditure".to_string(),
            amount: total_expenditure,
        },
        FinancialDataPoint {
            municipality_code: municipality_code.clone(),
            year: current_year,
            metric_name: "total_debt".to_string(),
            amount: total_debt,
        },
        FinancialDataPoint {
            municipality_code: municipality_code.clone(),
            year: current_year,
            metric_name: "capital_expenditure".to_string(),
            amount: capital_expenditure,
        },
    ];

    if let Err(e) = upsert_financial_data(&pool, &financial_data_points).await {
        log::error!("Database error upserting financial data for {}: {:?}", municipality_code, e);
    }

    // 5. Construct Response (MunicipalityDetail)
    let financial_year_data = FinancialYearData {
        year: current_year,
        revenue: total_revenue,
        expenditure: total_expenditure,
        capital_expenditure: capital_expenditure,
        debt: total_debt,
        audit_outcome: audit_opinion,
        score: None,
    };

    let response_data = MunicipalityDetail {
        id: base_info_db.id,
        name: base_info_db.name,
        province: base_info_db.province,
        population: base_info_db.population,
        classification: base_info_db.classification,
        website: base_info_db.website,
        geometry: None,
        financials: vec![financial_year_data],
    };

    // 6. Update Cache (Placeholder - commented out)
    /*
    if let Err(e) = cache.set(&cache_key, &response_data, 3600).await {
        log::error!("Failed to cache data for {}: {}", cache_key, e);
    }
    */

    Ok(HttpResponse::Ok().json(response_data))
}