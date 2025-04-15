// src/db/financials.rs
use sqlx::PgPool;
use crate::models::{FinancialDataDb, FinancialYearData}; // Add necessary models
use crate::errors::AppError;
use rust_decimal::Decimal; // For upsert function
use chrono::Utc; // For upsert and timestamp checks
use uuid::Uuid; // Ensure Uuid is imported

// --- Financial Data Query Functions ---

// Checks if financial data for a specific municipality and year exists in the cache (DB)
pub async fn get_cached_financials(pool: &PgPool, muni_id: &str, year: i32) -> Result<Option<FinancialDataDb>, AppError> {
    log::debug!("Checking DB cache for {} year {}", muni_id, year);
    let result = sqlx::query_as!(
        FinancialDataDb,
        // Explicitly list columns for safety and clarity
        r#"
        SELECT
            id,
            municipality_id,
            year AS financial_year,
            revenue AS "revenue?",
            expenditure AS "expenditure?",
            capital_expenditure AS "capital_expenditure?",
            debt AS "debt?",
            audit_outcome AS "audit_outcome?",
            score AS "score?",
            created_at,
            updated_at
        FROM financial_data
        WHERE municipality_id = $1 AND year = $2
        "#,
        muni_id,
        year
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

// Finds the latest year for which financial data is cached for a municipality
pub async fn get_latest_cached_year(pool: &PgPool, muni_id: &str) -> Result<Option<i32>, AppError> {
    let result = sqlx::query!(
        "SELECT MAX(year) as max_year FROM financial_data WHERE municipality_id = $1",
        muni_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.and_then(|record| record.max_year))
}


// Inserts or updates a complete financial record for a municipality and year in the cache (DB)
pub async fn upsert_complete_financial_record(
    pool: &PgPool,
    muni_id: &str,
    year: i32,
    revenue: Option<Decimal>,
    expenditure: Option<Decimal>,
    capital_expenditure: Option<Decimal>,
    debt: Option<Decimal>,
    audit_outcome: Option<String>,
    score: Option<Decimal>,
) -> Result<(), AppError> {
    let now = Utc::now();
    let record_id = Uuid::new_v4(); // Generate UUID explicitly

    sqlx::query!(
        r#"
        INSERT INTO financial_data (
            id, municipality_id, year, revenue, expenditure, capital_expenditure, debt, audit_outcome, score, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (municipality_id, year) DO UPDATE SET
            revenue = EXCLUDED.revenue,
            expenditure = EXCLUDED.expenditure,
            capital_expenditure = EXCLUDED.capital_expenditure,
            debt = EXCLUDED.debt,
            audit_outcome = EXCLUDED.audit_outcome,
            score = EXCLUDED.score,
            updated_at = $11
        "#,
        record_id,           // $1
        muni_id,             // $2
        year,                // $3
        revenue,             // $4
        expenditure,         // $5
        capital_expenditure, // $6
        debt,                // $7
        audit_outcome,       // $8
        score,               // $9
        now,                 // $10 created_at (for INSERT)
        now                  // $11 updated_at (for INSERT and UPDATE)
    )
    .execute(pool)
    .await?;

    log::info!("Successfully upserted financial record cache for {} year {}", muni_id, year);
    Ok(())
}

// Helper function used by get_municipality_detail_db_only
// Fetches all financial years data directly from the DB for a municipality.
pub async fn get_all_financial_years_db(pool: &PgPool, muni_id: &str) -> Result<Vec<FinancialYearData>, AppError> {
    let financials_db: Vec<FinancialDataDb> = sqlx::query_as!(
        FinancialDataDb,
        r#"
        SELECT
            id, municipality_id, year AS financial_year,
            revenue AS "revenue?", expenditure AS "expenditure?",
            capital_expenditure AS "capital_expenditure?", debt AS "debt?",
            audit_outcome AS "audit_outcome?", score AS "score?",
            created_at, updated_at
        FROM financial_data
        WHERE municipality_id = $1
        ORDER BY year DESC
        "#,
        muni_id
    )
    .fetch_all(pool)
    .await?;

    // Convert FinancialDataDb to FinancialYearData for the API response model
    let financials: Vec<FinancialYearData> = financials_db.into_iter().map(|db_data| FinancialYearData {
        year: db_data.financial_year,
        revenue: db_data.revenue,
        expenditure: db_data.expenditure,
        capital_expenditure: db_data.capital_expenditure,
        debt: db_data.debt,
        audit_outcome: db_data.audit_outcome,
        score: db_data.score,
    }).collect();

    Ok(financials)
}
