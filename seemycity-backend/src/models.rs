use serde::{Serialize, Deserialize};

// Represents a South African municipality and its financial health score.
// Deriving Serialize and Deserialize allows us to easily convert this struct
// to and from JSON format (and potentially other formats later).
#[derive(Serialize, Deserialize, Debug, Clone)] // Added Debug and Clone for convenience
pub struct Municipality {
    pub code: String,          // e.g., "BUF"
    pub name: String,          // e.g., "Buffalo City Metropolitan Municipality"
    pub province: String,      // e.g., "Eastern Cape"
    // Option<f64> allows the score to be potentially missing (None)
    pub financial_score: Option<f64>, // Score from 0.0 to 100.0
}

// We can add more models here later, e.g., for detailed financial indicators
// or geographical boundary data.