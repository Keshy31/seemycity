// src/db/financials.rs
use sqlx::PgPool;
use crate::models::FinancialDataDb;
use crate::errors::AppError;
use rust_decimal::Decimal; // For upsert function
use chrono::Utc; // For upsert and timestamp checks
use uuid::Uuid; // Import Uuid

// --- Financial Data Query Functions ---

// Inserts or updates a complete financial record for a municipality and year in the cache (DB)
pub async fn upsert_complete_financial_record(
    pool: &PgPool,
    municipality_id: &str,
    year: i32,
    revenue: Option<Decimal>,
    operational_expenditure: Option<Decimal>,
    capital_expenditure: Option<Decimal>,
    debt: Option<Decimal>,
    audit_outcome: Option<String>,
    overall_score: Option<Decimal>,
    financial_health_score: Option<Decimal>,
    infrastructure_score: Option<Decimal>,
    efficiency_score: Option<Decimal>,
    accountability_score: Option<Decimal>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let record_id = Uuid::new_v4(); // Generate a new UUID v4

    sqlx::query!(
        r#"
        INSERT INTO financial_data (
            id, municipality_id, year, revenue, operational_expenditure, capital_expenditure, debt, audit_outcome,
            overall_score, financial_health_score, infrastructure_score, efficiency_score, accountability_score, 
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        ON CONFLICT (municipality_id, year) DO UPDATE SET
            revenue = EXCLUDED.revenue,
            operational_expenditure = EXCLUDED.operational_expenditure,
            capital_expenditure = EXCLUDED.capital_expenditure,
            debt = EXCLUDED.debt,
            audit_outcome = EXCLUDED.audit_outcome,
            overall_score = EXCLUDED.overall_score,
            financial_health_score = EXCLUDED.financial_health_score,
            infrastructure_score = EXCLUDED.infrastructure_score,
            efficiency_score = EXCLUDED.efficiency_score,
            accountability_score = EXCLUDED.accountability_score,
            updated_at = EXCLUDED.updated_at
        "#,
        record_id, // Pass the generated UUID as the first parameter
        municipality_id,
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
        now, // created_at (only set on INSERT)
        now // updated_at (set on INSERT and UPDATE)
    )
    .execute(pool)
    .await?;

    log::info!("Successfully upserted financial record cache for {} year {}", municipality_id, year);
    Ok(())
}

// Fetches all financial-year rows (including cache timestamps) for a municipality.
// Rows with every metric NULL act as negative-cache entries and are filtered out
// of API responses by the handler.
pub async fn get_all_financial_years_db(pool: &PgPool, muni_id: &str) -> Result<Vec<FinancialDataDb>, AppError> {
    log::debug!("Fetching all financial years from DB for muni_id: {}", muni_id);
    let financials = sqlx::query_as!(
        FinancialDataDb,
        r#"
        SELECT
            id,
            municipality_id,
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
            created_at,
            updated_at
        FROM financial_data
        WHERE municipality_id = $1
        ORDER BY year DESC
        "#,
        muni_id
    )
    .fetch_all(pool)
    .await?;

    Ok(financials)
}
