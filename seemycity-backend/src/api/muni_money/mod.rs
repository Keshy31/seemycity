// src/api/muni_money/mod.rs

// Declare the modules within this submodule
pub mod client;
pub mod financials;
pub mod types;
pub mod audit; // Declare audit even though it's empty for now

// Optional: Re-export key items for easier access within the muni_money module itself, if needed.
// pub use client::MunicipalMoneyClient;
// pub use types::{ApiClientError, FinancialFact, FactsApiResponse};
// pub use financials::*;