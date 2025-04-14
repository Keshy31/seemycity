// src/api/muni_money/mod.rs

// Declare the modules within this submodule
pub mod client;
pub mod incexp;
pub mod finpos;
pub mod capex;
pub mod financials;
pub mod types;
pub mod audit;

// Optional: Re-export key items for easier access within the muni_money module itself, if needed.
// pub use client::MunicipalMoneyClient;
// pub use types::{ApiClientError, FinancialFact, FactsApiResponse};
// pub use financials::*;