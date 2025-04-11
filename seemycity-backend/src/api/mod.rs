// src/api/mod.rs

// Declare the new submodule
pub mod muni_money;

// Re-export key items from the submodule for easier use in other parts of the application
pub use muni_money::client::MunicipalMoneyClient;
pub use muni_money::types::ApiClientError;
// Potentially re-export financial functions if they are called directly from handlers,
// but it's often cleaner to call them via methods on a client instance or a dedicated service layer.
// pub use muni_money::financials::*;