use actix_web::{get, web, HttpResponse};
use chrono::{Datelike, Duration, Utc};
use serde::Deserialize;
use crate::api::muni_money::audit::get_audit_outcome;
use crate::api::muni_money::client::MunicipalMoneyClient;
use crate::api::muni_money::financials::{
    get_capital_expenditure, get_revenue_and_expenditure, get_total_debt, IncexpFigures,
};
use crate::confidence::{evaluate as evaluate_confidence, ConfidenceInput};
use crate::db::financials::{get_all_financial_years_db, upsert_complete_financial_record};
use crate::db::municipalities::{
    get_all_municipality_populations, get_municipality_base_info_db,
    get_municipalities_summary_for_map,
};
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

    let mut rows =
        ensure_financials_fresh(&pool, &api_client, &upstream_health, &muni_code, population_opt)
            .await?;

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

/// Brings a municipality's financial-year rows up to date and returns them:
/// walks candidate years newest-first (audited actuals lag the calendar by
/// roughly a year) until one yields usable data, refreshing missing/expired
/// rows from the Treasury API; then re-derives scores for every cached row so
/// formula changes propagate without upstream calls.
///
/// Fresh cached rows are trusted as-is — including all-NULL negative-cache
/// rows. When the upstream circuit breaker is open, cached (even stale) data
/// is returned immediately. Used by both the detail handler and the
/// background cache warmer.
pub async fn ensure_financials_fresh(
    pool: &DbPool,
    api_client: &MunicipalMoneyClient,
    upstream_health: &UpstreamHealth,
    muni_code: &str,
    population_opt: Option<f32>,
) -> Result<Vec<FinancialDataDb>, AppError> {
    let mut rows = get_all_financial_years_db(pool, muni_code).await?;
    let now = Utc::now();

    // Walk until a year yields a *scorable* row (all four pillars), not merely
    // any data: the newest financial year often publishes figures months before
    // its audit opinion, and stopping there would leave the municipality
    // unscored while a complete prior year sits one step further back.
    let newest_candidate_year = now.year() - 1;
    for year in ((newest_candidate_year - YEAR_FALLBACK_DEPTH + 1)..=newest_candidate_year).rev() {
        let cached_fresh_has_score = rows
            .iter()
            .find(|r| r.year == year)
            .filter(|r| now - r.updated_at < Duration::days(CACHE_TTL_DAYS))
            .map(|r| r.overall_score.is_some());

        let has_score = match cached_fresh_has_score {
            Some(has_score) => has_score,
            None if !upstream_health.is_up() => {
                // Circuit open: serve whatever is cached (stale included) rather
                // than stalling the request on a known-degraded upstream.
                log::debug!("Muni: {muni_code}, upstream cooling down; skipping refresh of {year}");
                break;
            }
            None => {
                let prior = rows.iter().find(|r| r.year == year).cloned();
                match refresh_financial_year(pool, api_client, muni_code, year, population_opt, prior.as_ref())
                    .await
                {
                    Some(refreshed) => {
                        let has_score = refreshed.overall_score.is_some();
                        rows.retain(|r| r.year != year);
                        rows.push(refreshed);
                        has_score
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

        if has_score {
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
        // Confidence backfill from stored values for rows never evaluated.
        // A grade set at fetch time (which may reflect the revenue checksum)
        // is kept as-is.
        let (confidence, confidence_notes) = if row.data_confidence.is_none() {
            let grade = evaluate_confidence(&ConfidenceInput {
                revenue: row.revenue,
                operational_expenditure: row.operational_expenditure,
                capital_expenditure: row.capital_expenditure,
                debt: row.debt,
                population: population_opt.map(|p| p as u32),
                revenue_checksum: None,
            });
            (Some(grade.grade.to_string()), grade.notes)
        } else {
            (row.data_confidence.clone(), row.confidence_notes.clone())
        };
        let up_to_date = row.overall_score == breakdown.overall_score
            && row.financial_health_score == breakdown.financial_health_score
            && row.infrastructure_score == breakdown.infrastructure_score
            && row.efficiency_score == breakdown.efficiency_score
            && row.accountability_score == breakdown.accountability_score
            && row.data_confidence == confidence;
        if up_to_date {
            continue;
        }
        log::info!(
            "Muni: {}, healing year {} (older formula or missing confidence grade)",
            muni_code, row.year
        );
        row.overall_score = breakdown.overall_score;
        row.financial_health_score = breakdown.financial_health_score;
        row.infrastructure_score = breakdown.infrastructure_score;
        row.efficiency_score = breakdown.efficiency_score;
        row.accountability_score = breakdown.accountability_score;
        row.data_confidence = confidence;
        row.confidence_notes = confidence_notes;
        if let Err(e) = upsert_complete_financial_record(pool, row).await {
            log::error!("Muni: {}, failed to persist healed scores for {}: {}", muni_code, row.year, e);
        }
    }

    Ok(rows)
}

/// Warms the score cache for every municipality so the map is fully colored
/// without depending on detail-page traffic. Fresh rows are skipped by the
/// cache logic, so repeat runs are cheap; the run aborts early if the Treasury
/// API circuit breaker opens.
pub async fn warm_all_municipalities(
    pool: &DbPool,
    api_client: &MunicipalMoneyClient,
    upstream_health: &UpstreamHealth,
) {
    let munis = match get_all_municipality_populations(pool).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("Cache warmer: failed to list municipalities: {e}");
            return;
        }
    };

    log::info!("Cache warmer: checking {} municipalities", munis.len());
    let (mut scored, mut no_data) = (0u32, 0u32);
    for (id, population) in &munis {
        if !upstream_health.is_up() {
            log::warn!("Cache warmer: upstream circuit open, aborting run early");
            break;
        }
        match ensure_financials_fresh(pool, api_client, upstream_health, id, *population).await {
            Ok(rows) if rows.iter().any(|r| r.overall_score.is_some()) => scored += 1,
            Ok(_) => no_data += 1,
            Err(e) => log::error!("Cache warmer: {id} failed: {e}"),
        }
    }
    log::info!(
        "Cache warmer: done — {scored} municipalities scored, {no_data} without data (of {})",
        munis.len()
    );
}

/// Fetches all five metrics for one municipality-year from the Treasury API
/// (concurrently), recomputes scores, evaluates data confidence, and upserts
/// the result — NULLs included, so the row doubles as a negative-cache marker.
/// Individual fetch or upsert failures degrade to NULL fields rather than
/// failing the request.
///
/// Returns `None` when **every** upstream call failed at the transport level:
/// that means the Treasury API is unreachable, which must not be cached as
/// "this year has no data".
///
/// When the fetch succeeds but yields **no data at all** while `prior` holds
/// real data, the prior row is kept untouched: the Treasury API has been
/// observed returning empty-but-successful responses while degraded, and
/// stale-but-real beats fresh-but-empty.
async fn refresh_financial_year(
    pool: &DbPool,
    api_client: &MunicipalMoneyClient,
    muni_code: &str,
    year: i32,
    population: Option<f32>,
    prior: Option<&FinancialDataDb>,
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

    let IncexpFigures { revenue, operational_expenditure, revenue_checksum } = incexp_res
        .map_err(|e| log::error!("Muni: {muni_code}, Failed Revenue/Expenditure fetch for {year}: {e}"))
        .unwrap_or_default();
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

    // Empty-but-successful responses during upstream degradation must not
    // erase real cached data (observed 2026-07-07: 9 municipalities were
    // wrongly negative-cached this way, including eThekwini).
    let fetched_nothing = revenue.is_none()
        && operational_expenditure.is_none()
        && capital_expenditure.is_none()
        && debt.is_none()
        && audit_outcome.is_none();
    if fetched_nothing {
        if let Some(prior_row) = prior.filter(|p| p.has_any_data()) {
            log::warn!(
                "Muni: {muni_code}, year {year}: upstream returned no data but real data is cached — keeping the cached row (possible upstream degradation)"
            );
            return Some(prior_row.clone());
        }
    }

    let grade = evaluate_confidence(&ConfidenceInput {
        revenue,
        operational_expenditure,
        capital_expenditure,
        debt,
        population: population.map(|p| p as u32),
        revenue_checksum,
    });

    let now = Utc::now();
    let row = FinancialDataDb {
        id: Uuid::new_v4(), // in-memory only; an existing DB row keeps its own id
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
        data_confidence: Some(grade.grade.to_string()),
        confidence_notes: grade.notes,
        created_at: now,
        updated_at: now,
    };

    if let Err(e) = upsert_complete_financial_record(pool, &row).await {
        // Serve the fetched data anyway; the cache simply retries next request.
        log::error!("Muni: {muni_code}, Failed to upsert data for {year}: {e}");
    }

    Some(row)
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