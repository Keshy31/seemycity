use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono};
use uuid::Uuid;
use rust_decimal::Decimal;
use geojson::Geometry;

// --- Database Table Models ---

// Maps directly to the 'municipalities' table
// Corresponds to data-spec.md section 2
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MunicipalityDb {
    pub id: String, // Primary key, e.g., "BUF"
    pub name: String,
    pub province: String,
    // Ensure population is f32 to match DB 'real'
    pub population: Option<f32>, 
    pub classification: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub district_id: Option<String>,
    pub district_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Maps directly to the 'financial_data' table
// Corresponds to data-spec.md section 2
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FinancialDataDb {
    pub id: Uuid, // Primary key
    pub municipality_id: String, // Foreign key
    pub year: i32,
    pub revenue: Option<Decimal>,
    // Rename this field to match the DB column
    pub operational_expenditure: Option<Decimal>, 
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    // Make audit_outcome optional to match DB NULLable and query_as SELECT *
    pub audit_outcome: Option<String>,
    // Add the new score fields to match the DB table
    pub overall_score: Option<Decimal>,
    pub financial_health_score: Option<Decimal>,
    pub infrastructure_score: Option<Decimal>,
    pub efficiency_score: Option<Decimal>,
    pub accountability_score: Option<Decimal>,
    // Plausibility grade of the raw figures: "ok" | "suspect" | "unreliable".
    // None = not yet evaluated (backfilled by the healing pass).
    pub data_confidence: Option<String>,
    pub confidence_notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>, // Timestamp for cache management
    #[sqlx(default)] // Handle potential missing updated_at if not always set by upsert
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FinancialDataDb {
    /// True when the row carries at least one real metric or score. Rows that are
    /// all-NULL exist only as negative-cache markers and are not user-facing data.
    pub fn has_any_data(&self) -> bool {
        self.revenue.is_some()
            || self.operational_expenditure.is_some()
            || self.capital_expenditure.is_some()
            || self.debt.is_some()
            || self.audit_outcome.is_some()
            || self.overall_score.is_some()
    }
}

impl From<&FinancialDataDb> for FinancialYearData {
    fn from(row: &FinancialDataDb) -> Self {
        FinancialYearData {
            year: row.year,
            revenue: row.revenue,
            operational_expenditure: row.operational_expenditure,
            capital_expenditure: row.capital_expenditure,
            debt: row.debt,
            audit_outcome: row.audit_outcome.clone(),
            overall_score: row.overall_score,
            financial_health_score: row.financial_health_score,
            infrastructure_score: row.infrastructure_score,
            efficiency_score: row.efficiency_score,
            accountability_score: row.accountability_score,
            data_confidence: row.data_confidence.clone(),
            confidence_notes: row.confidence_notes.clone(),
        }
    }
}


// --- API Response / Query Result Models ---

// Data structure for the /api/municipalities map view properties
// Corresponds to data-spec.md section 3.1 properties
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapMunicipalityProperties {
    pub id: String,
    pub name: String,
    pub province: String,
    // Convert population to Option<f64> for JSON
    #[serde(serialize_with = "crate::utils::serialize_option_f32_as_f64")]
    pub population: Option<f32>,
    pub classification: Option<String>,
    // Canonical name across the API: matches the financial_data column and the
    // detail endpoint's field, and is what the map's data-driven styling reads.
    #[serde(rename = "overall_score")]
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub latest_score: Option<Decimal>,
}

// Data structure for individual financial year data within MunicipalityDetail
// Corresponds to data-spec.md section 3.2 financials array items
#[derive(Serialize, Deserialize, Debug, Clone, Default, FromRow)]
pub struct FinancialYearData {
    pub year: i32,
    // Use Option<f64> for JSON compatibility, convert from Decimal
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub revenue: Option<Decimal>,
    // Rename this field as well
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub operational_expenditure: Option<Decimal>, 
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub capital_expenditure: Option<Decimal>, // Added this field
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub debt: Option<Decimal>,
    // Make audit_outcome optional
    pub audit_outcome: Option<String>,
    // Add the new score fields
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub overall_score: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub financial_health_score: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub infrastructure_score: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub efficiency_score: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub accountability_score: Option<Decimal>,
    // "ok" | "suspect" | "unreliable" | null (not yet evaluated)
    pub data_confidence: Option<String>,
    pub confidence_notes: Option<String>,
}

// Detailed data structure for the /api/municipality/{id} view
// Corresponds to data-spec.md section 3.2
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MunicipalityDetail {
    pub id: String,
    pub name: String,
    pub province: String,
    // Convert population to Option<f64> for JSON
    #[serde(serialize_with = "crate::utils::serialize_option_f32_as_f64")]
    pub population: Option<f32>,
    pub classification: Option<String>,
    pub website: Option<String>,
    // Add other fields from municipalities table as needed (address, phone, district...)
    pub financials: Vec<FinancialYearData>,
    // pub score_breakdown: Option<serde_json::Value>, // Placeholder if needed later
    pub geometry: Option<serde_json::Value>, // Full geometry for single view
    // Potentially add overall latest update timestamp if useful
    // pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

// --- GeoJSON Structures for Map Summary ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapFeature {
    #[serde(rename = "type")]
    pub feature_type: String, // Should always be "Feature"
    pub geometry: Option<Geometry>, // Use geojson crate's Geometry type
    pub properties: MapMunicipalityProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapFeatureCollection {
    #[serde(rename = "type")]
    pub collection_type: String, // Should always be "FeatureCollection"
    pub features: Vec<MapFeature>,
}