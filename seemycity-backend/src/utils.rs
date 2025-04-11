// src/utils.rs
use serde::{Serializer};
use sqlx::types::Decimal;
use std::str::FromStr;

/// Serializes Option<Decimal> into Option<f64> for JSON compatibility.
/// Returns null if Decimal cannot be converted to f64.
pub fn serialize_option_decimal_as_f64<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(decimal_val) => {
            // Attempt to convert Decimal to f64
            match f64::from_str(&decimal_val.to_string()) {
                Ok(float_val) => serializer.serialize_some(&float_val),
                Err(_) => {
                    // Log error or handle appropriately if conversion fails
                    // For now, serialize as null if conversion fails
                    serializer.serialize_none()
                }
            }
        }
        None => serializer.serialize_none(),
    }
}

/// Serializes Option<f32> into Option<f64> for JSON compatibility.
pub fn serialize_option_f32_as_f64<S>(value: &Option<f32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(f32_val) => serializer.serialize_some(&(f32_val.to_string().parse::<f64>().unwrap_or(f64::NAN))), // Convert f32 to f64
        None => serializer.serialize_none(),
    }
}

// You could add other utility functions here as needed.