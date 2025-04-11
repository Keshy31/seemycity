use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::{Decimal, chrono}};
use uuid::Uuid;
use serde_json::Value;

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

// Maps directly to the 'municipal_geometries' table (if you need it separately)
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MunicipalityGeometryDb {
    pub ogc_fid: i32, // Serial primary key
    pub municipality_id: String, // Foreign key
    // Changed to Value for better JSONB handling with sqlx
    pub geometry_geojson: Option<Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Maps directly to the 'financial_data' table
// Corresponds to data-spec.md section 2
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FinancialDataDb {
    pub id: Uuid, // Primary key
    pub municipality_id: String, // Foreign key
    #[sqlx(rename = "year")]
    pub financial_year: i32,
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    // Make audit_outcome optional to match DB NULLable and query_as SELECT *
    pub audit_outcome: Option<String>,
    pub score: Option<Decimal>,
    pub created_at: chrono::DateTime<chrono::Utc>, // Timestamp for cache management
    #[sqlx(default)] // Handle potential missing updated_at if not always set by upsert
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


// --- API Response / Query Result Models ---

// Basic info used in get_all_municipality_basic_info
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MunicipalityBasicInfo {
    pub id: String,
    pub name: String,
    pub province: String,
}

// Data structure for the /api/municipalities map view properties
// Corresponds to data-spec.md section 3.1 properties
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapMunicipalityProperties {
    #[serde(rename = "code")]
    pub id: String,
    pub name: String,
    pub province: String,
    // Convert population to Option<f64> for JSON
    #[serde(serialize_with = "crate::utils::serialize_option_f32_as_f64")]
    pub population: Option<f32>,
    pub classification: Option<String>,
    #[serde(rename = "financial_score")]
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub latest_score: Option<Decimal>, // Changed from Option<f64> to Option<Decimal>
}

// Data structure for individual financial year data within MunicipalityDetail
// Corresponds to data-spec.md section 3.2 financials array items
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialYearData {
    pub year: i32,
    // Use Option<f64> for JSON compatibility, convert from Decimal
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub revenue: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub expenditure: Option<Decimal>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub capital_expenditure: Option<Decimal>, // Added this field
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub debt: Option<Decimal>,
    // Make audit_outcome optional
    pub audit_outcome: Option<String>,
    #[serde(serialize_with = "crate::utils::serialize_option_decimal_as_f64")]
    pub score: Option<Decimal>,
    // Add other relevant fields from `financial_data` table if needed
}

// Detailed data structure for the /api/municipality/{id} view
// Corresponds to data-spec.md section 3.2
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MunicipalityDetail {
    #[serde(rename = "code")]
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


// Original struct - might be useful as a simplified API model if needed,
// but doesn't map directly to DB tables easily.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegacyMunicipality {
    pub code: String,          // e.g., "BUF"
    pub name: String,          // e.g., "Buffalo City Metropolitan Municipality"
    pub province: String,      // e.g., "Eastern Cape"
    pub financial_score: Option<f64>, // Score from 0.0 to 100.0
}