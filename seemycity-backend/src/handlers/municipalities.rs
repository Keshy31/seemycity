use actix_web::{get, web, HttpResponse};
use chrono::{Datelike, Duration, Utc};
use serde::Deserialize;
use crate::api::muni_money::audit::get_audit_outcome;
use crate::api::muni_money::client::MunicipalMoneyClient;
use crate::api::muni_money::financials::{
    get_capital_expenditure, get_revenue_and_expenditure, get_total_debt,
};
use crate::db::financials::{get_all_financial_years_db, upsert_complete_financial_record};
use crate::db::municipalities::{get_municipality_base_info_db, get_municipalities_summary_for_map};
use crate::errors::AppError;
use crate::models::{FinancialDataDb, FinancialYearData, MunicipalityDetail, MapFeatureCollection};
use crate::scoring::{calculate_financial_score, ScoreBreakdown, ScoringInput};
use sqlx::PgPool as DbPool;
use uuid::Uuid;

/// How long a cached financial_data row (including an all-NULL negative-cache row)
/// is trusted before the Treasury API is consulted again. Municipal figures change
/// at most quarterly, so a week keeps us fresh without hammering the upstream.
const CACHE_TTL_DAYS: i64 = 7;

/// How many financial years to walk back looking for usable data. Audited actuals
/// lag the calendar year by roughly one year, and some municipalities publish later.
const YEAR_FALLBACK_DEPTH: i32 = 3;

/// The map payload is ~1 MB of mostly-static geometry that is expensive to pull
/// and simplify per request. Serve a cached copy for this long; scores changing
/// through the detail endpoint appear on the map within this window.
const MAP_CACHE_TTL_SECS: u64 = 60;

/// After a refresh round where every upstream call failed at the transport level,
/// skip the Treasury API for this long and serve cached data only. Prevents a
/// degraded upstream from stalling every cold request on timeouts.
const UPSTREAM_COOLDOWN_SECS: u64 = 300;

/// Circuit breaker for the Treasury API. Shared across workers.
#[derive(Default)]
pub struct UpstreamHealth {
    down_until: std::sync::RwLock<Option<std::time::Instant>>,
}

impl UpstreamHealth {
    fn is_up(&self) -> bool {
        match self.down_until.read() {
            Ok(guard) => guard.map_or(true, |t| std::time::Instant::now() >= t),
            Err(_) => true,
        }
    }

    fn mark_down(&self) {
        if let Ok(mut guard) = self.down_until.write() {
            *guard = Some(std::time::Instant::now() + std::time::Duration::from_secs(UPSTREAM_COOLDOWN_SECS));
        }
        log::warn!(
            "Treasury API unreachable — skipping upstream fetches for {}s, serving cached data only",
            UPSTREAM_COOLDOWN_SECS
        );
    }
}

/// In-memory cache for the full (no-limit) map FeatureCollection response body.
#[derive(Default)]
pub struct MapResponseCache {
    inner: std::sync::RwLock<Option<(std::time::Instant, String)>>,
}

impl MapResponseCache {
    fn get_fresh(&self) -> Option<String> {
        let guard = self.inner.read().ok()?;
        let (created, body) = guard.as_ref()?;
        (created.elapsed().as_secs() < MAP_CACHE_TTL_SECS).then(|| body.clone())
    }

    fn store(&self, body: String) {
        if let Ok(mut guard) = self.inner.write() {
            *guard = Some((std::time::Instant::now(), body));
        }
    }
}

// Replace the existing function with this one:
// Handler to get details for a single municipality by ID
pub async fn get_municipality_detail_handler(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    api_client: web::Data<MunicipalMoneyClient>,
    upstream_health: web::Data<UpstreamHealth>,
) -> Result<HttpResponse, AppError> {
    let muni_id_str = path.into_inner();
    log::info!("START: Handling request for /api/municipalities/{}", muni_id_str);

    // Fetch base municipality info
    let base_info = get_municipality_base_info_db(&pool, &muni_id_str).await?;
    let base_info_unwrapped = base_info.ok_or_else(|| {
        log::warn!("Municipality base info not found for ID: {}", muni_id_str);
        AppError::NotFound(format!("Municipality with ID {} not found", muni_id_str))
    })?;
    let muni_code = base_info_unwrapped.id.clone();
    let population_opt = base_info_unwrapped.population;

    let mut rows = get_all_financial_years_db(&pool, &muni_id_str).await?;
    let now = Utc::now();

    // Walk candidate years newest-first until one yields usable data. Audited
    // actuals lag roughly a year behind the calendar, so start at year - 1.
    // A fresh cached row is trusted as-is — even an all-NULL row, which serves
    // as a negative cache so absent upstream data doesn't trigger an API round
    // on every request. Missing or expired rows get a full refresh (all five
    // metrics), so previously incomplete years heal automatically.
    let newest_candidate_year = now.year() - 1;
    for year in ((newest_candidate_year - YEAR_FALLBACK_DEPTH + 1)..=newest_candidate_year).rev() {
        let cached_fresh_has_data = rows
            .iter()
            .find(|r| r.year == year)
            .filter(|r| now - r.updated_at < Duration::days(CACHE_TTL_DAYS))
            .map(|r| r.has_any_data());

        let has_data = match cached_fresh_has_data {
            Some(has_data) => has_data,
            None if !upstream_health.is_up() => {
                // Circuit open: serve whatever is cached (stale included) rather
                // than stalling the request on a known-degraded upstream.
                log::debug!("Muni: {muni_code}, upstream cooling down; skipping refresh of {year}");
                break;
            }
            None => {
                match refresh_financial_year(&pool, &api_client, &muni_code, year, population_opt).await {
                    Some(refreshed) => {
                        let has_data = refreshed.has_any_data();
                        rows.retain(|r| r.year != year);
                        rows.push(refreshed);
                        has_data
                    }
                    None => {
                        // Every upstream call failed at the transport level. Not
                        // the same as "no data" — nothing is persisted, and we
                        // stop trying older years against a dead upstream.
                        upstream_health.mark_down();
                        break;
                    }
                }
            }
        };

        if has_data {
            break;
        }
    }

    // Scores are pure derivations of the stored raw metrics, so recompute them for
    // every cached row and heal any that disagree with the current formula. This
    // propagates scoring-rubric changes to historical years (and to the map, which
    // reads persisted scores) lazily, without any Treasury API calls.
    for row in rows.iter_mut() {
        if !row.has_any_data() {
            continue;
        }
        let breakdown = calculate_financial_score(&ScoringInput {
            revenue: row.revenue,
            operational_expenditure: row.operational_expenditure,
            capital_expenditure: row.capital_expenditure,
            debt: row.debt,
            audit_outcome: row.audit_outcome.clone(),
            population: population_opt.map(|p| p as u32),
        });
        let up_to_date = row.overall_score == breakdown.overall_score
            && row.financial_health_score == breakdown.financial_health_score
            && row.infrastructure_score == breakdown.infrastructure_score
            && row.efficiency_score == breakdown.efficiency_score
            && row.accountability_score == breakdown.accountability_score;
        if up_to_date {
            continue;
        }
        log::info!(
            "Muni: {}, healing year {} scores computed under an older formula",
            muni_code, row.year
        );
        row.overall_score = breakdown.overall_score;
        row.financial_health_score = breakdown.financial_health_score;
        row.infrastructure_score = breakdown.infrastructure_score;
        row.efficiency_score = breakdown.efficiency_score;
        row.accountability_score = breakdown.accountability_score;
        if let Err(e) = upsert_complete_financial_record(
            &pool,
            &muni_code,
            row.year,
            row.revenue,
            row.operational_expenditure,
            row.capital_expenditure,
            row.debt,
            row.audit_outcome.clone(),
            row.overall_score,
            row.financial_health_score,
            row.infrastructure_score,
            row.efficiency_score,
            row.accountability_score,
        )
        .await
        {
            log::error!("Muni: {}, failed to persist healed scores for {}: {}", muni_code, row.year, e);
        }
    }

    // All-NULL rows are cache internals, not user data; newest year first.
    rows.sort_by(|a, b| b.year.cmp(&a.year));
    let financials: Vec<FinancialYearData> = rows
        .iter()
        .filter(|r| r.has_any_data())
        .map(FinancialYearData::from)
        .collect();

    // Geometry is intentionally omitted here: the detail view renders no map, and
    // boundary polygons average ~90 KB each. The map endpoint serves geometry.
    let response = MunicipalityDetail {
        id: base_info_unwrapped.id,
        name: base_info_unwrapped.name,
        province: base_info_unwrapped.province,
        population: base_info_unwrapped.population,
        classification: base_info_unwrapped.classification,
        website: base_info_unwrapped.website,
        financials,
        geometry: None,
    };

    log::info!("END: Handling request for /api/municipalities/{}", muni_id_str);
    Ok(HttpResponse::Ok().json(response))
}

/// Fetches all five metrics for one municipality-year from the Treasury API
/// (concurrently), recomputes scores, and upserts the result — NULLs included,
/// so the row doubles as a negative-cache marker. Individual fetch or upsert
/// failures degrade to NULL fields rather than failing the request.
///
/// Returns `None` when **every** upstream call failed at the transport level:
/// that means the Treasury API is unreachable, which must not be cached as
/// "this year has no data".
async fn refresh_financial_year(
    pool: &DbPool,
    api_client: &MunicipalMoneyClient,
    muni_code: &str,
    year: i32,
    population: Option<f32>,
) -> Option<FinancialDataDb> {
    log::info!("Muni: {}, refreshing financial data for {} from Treasury API", muni_code, year);

    // Revenue and opex share one incexp cube fetch; capex, debt, and audit each
    // have their own cube. Four concurrent upstream calls in total.
    let (incexp_res, capex_res, debt_res, audit_res) = tokio::join!(
        get_revenue_and_expenditure(api_client, muni_code, year),
        get_capital_expenditure(api_client, muni_code, year),
        get_total_debt(api_client, muni_code, year),
        get_audit_outcome(api_client, muni_code, year),
    );

    if incexp_res.is_err() && capex_res.is_err() && debt_res.is_err() && audit_res.is_err() {
        log::error!("Muni: {muni_code}, all Treasury API calls failed for {year}; upstream unreachable");
        return None;
    }

    let (revenue, operational_expenditure) = incexp_res
        .map_err(|e| log::error!("Muni: {muni_code}, Failed Revenue/Expenditure fetch for {year}: {e}"))
        .unwrap_or((None, None));
    let capital_expenditure = capex_res
        .map_err(|e| log::error!("Muni: {muni_code}, Failed Capex fetch for {year}: {e}"))
        .ok()
        .flatten();
    let debt = debt_res
        .map_err(|e| log::error!("Muni: {muni_code}, Failed Debt fetch for {year}: {e}"))
        .ok()
        .flatten();
    let audit_outcome = audit_res
        .map_err(|e| log::error!("Muni: {muni_code}, Failed Audit fetch for {year}: {e}"))
        .ok()
        .flatten();

    let scoring_input = ScoringInput {
        revenue,
        operational_expenditure,
        capital_expenditure,
        debt,
        audit_outcome: audit_outcome.clone(),
        population: population.map(|p| p as u32),
    };
    let ScoreBreakdown {
        overall_score,
        financial_health_score,
        infrastructure_score,
        efficiency_score,
        accountability_score,
    } = calculate_financial_score(&scoring_input);

    if let Err(e) = upsert_complete_financial_record(
        pool,
        muni_code,
        year,
        revenue,
        operational_expenditure,
        capital_expenditure,
        debt,
        audit_outcome.clone(),
        overall_score,
        financial_health_score,
        infrastructure_score,
        efficiency_score,
        accountability_score,
    )
    .await
    {
        // Serve the fetched data anyway; the cache simply retries next request.
        log::error!("Muni: {muni_code}, Failed to upsert data for {year}: {e}");
    }

    let now = Utc::now();
    Some(FinancialDataDb {
        id: Uuid::new_v4(), // in-memory only; the DB row keeps its own id
        municipality_id: muni_code.to_string(),
        year,
        revenue,
        operational_expenditure,
        capital_expenditure,
        debt,
        audit_outcome,
        overall_score,
        financial_health_score,
        infrastructure_score,
        efficiency_score,
        accountability_score,
        created_at: now,
        updated_at: now,
    })
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
    cache: web::Data<MapResponseCache>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit;
    if let Some(l) = limit {
        if l <= 0 {
            return Err(AppError::BadRequest(format!("limit must be positive, got {l}")));
        }
    }
    log::info!("START: Handling request for /api/municipalities with limit: {:?}", limit);

    // The unlimited payload (the map's landing request) is served from memory.
    if limit.is_none() {
        if let Some(body) = cache.get_fresh() {
            log::debug!("Serving /api/municipalities from in-memory cache");
            return Ok(geojson_response(body));
        }
    }

    let map_features = get_municipalities_summary_for_map(&pool, limit).await?;
    let feature_collection = MapFeatureCollection {
        collection_type: "FeatureCollection".to_string(),
        features: map_features,
    };

    let body = serde_json::to_string(&feature_collection)
        .map_err(|e| AppError::InternalError(format!("Failed to serialize map payload: {e}")))?;
    if limit.is_none() {
        cache.store(body.clone());
    }

    log::info!("END: Returning {} features for /api/municipalities", feature_collection.features.len());
    Ok(geojson_response(body))
}

fn geojson_response(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header((actix_web::http::header::CACHE_CONTROL, "public, max-age=60"))
        .body(body)
}