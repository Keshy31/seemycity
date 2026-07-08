// src/db/financials.rs
use sqlx::PgPool;
use crate::models::FinancialDataDb;
use crate::errors::AppError;
 // For upsert function
use chrono::Utc; // For upsert and timestamp checks
 // Import Uuid

// --- Financial Data Query Functions ---

// Inserts or updates a complete financial record for a municipality and year in
// the cache (DB). The row's id is used only on INSERT; an existing row keeps its
// own id, and updated_at is stamped fresh either way.
pub async fn upsert_complete_financial_record(
    pool: &PgPool,
    row: &FinancialDataDb,
) -> Result<(), AppError> {
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO financial_data (
            id, municipality_id, year, revenue, operational_expenditure, capital_expenditure, debt, audit_outcome,
            overall_score, financial_health_score, infrastructure_score, efficiency_score, accountability_score,
            data_confidence, confidence_notes,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
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
            data_confidence = EXCLUDED.data_confidence,
            confidence_notes = EXCLUDED.confidence_notes,
            updated_at = EXCLUDED.updated_at
        "#,
        row.id,
        row.municipality_id,
        row.year,
        row.revenue,
        row.operational_expenditure,
        row.capital_expenditure,
        row.debt,
        row.audit_outcome.as_deref(),
        row.overall_score,
        row.financial_health_score,
        row.infrastructure_score,
        row.efficiency_score,
        row.accountability_score,
        row.data_confidence.as_deref(),
        row.confidence_notes.as_deref(),
        now, // created_at (only set on INSERT)
        now  // updated_at (set on INSERT and UPDATE)
    )
    .execute(pool)
    .await?;

    log::info!(
        "Successfully upserted financial record cache for {} year {}",
        row.municipality_id, row.year
    );
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
            data_confidence,
            confidence_notes,
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
