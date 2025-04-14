// tests/muni_money_integration_test.rs

// We need to import items from our actual crate (seemycity_backend)
// Note: Rust implicitly creates a crate named after your package when running integration tests.
use seemycity_backend::api::{muni_money, MunicipalMoneyClient}; // Use the re-exported client
use dotenvy; // Need dotenvy for loading .env

// This test requires network access and hits the actual Municipal Money API
#[tokio::test]
#[ignore] // Mark as ignored by default. Run with: cargo test -- --ignored test_fetch_real_revenue
async fn test_fetch_real_revenue() { // Renamed back
    // Load .env variables - important for tests running outside `cargo run`
    dotenvy::dotenv().expect("Failed to load .env file for integration test");

    println!("Initializing client for integration test...");
    // Use the re-exported client path
    let client = MunicipalMoneyClient::new().expect("Failed to create client for test");

    let muni_code = "BUF"; // Buffalo City
    let year = 2022;       // Example year

    println!("Fetching total revenue for {} year {}...", muni_code, year);
    // Access financial functions through the muni_money module
    let result = muni_money::financials::get_total_revenue(&client, muni_code, year).await;

    println!("API call result: {:?}", result);
    // Original assertion
    assert!(result.is_ok(), "API call failed: {:?}", result.err());

    // Optional: Add more checks if you know the expected rough value
    if let Ok(revenue) = result {
        // Basic sanity check - revenue shouldn't typically be zero for a major metro/year.
        // It *could* be if the API returns no data for that specific item/amount_type combo.
        // assert!(revenue > 0.0, "Revenue was not positive, check API response");
        println!("Fetched revenue: {}", revenue);
    }
}

// Add more integration tests here for other functions (debt, expenditure, etc.)
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_fetch_real_debt
async fn test_fetch_real_debt() {
    dotenvy::dotenv().expect("Failed to load .env file for integration test");
    println!("Initializing client for integration test...");
    let client = MunicipalMoneyClient::new().expect("Failed to create client for test");
    let muni_code = "BUF";
    let year = 2022;
    println!("Fetching total debt for {} year {}...", muni_code, year);
    let result = muni_money::financials::get_total_debt(&client, muni_code, year).await;
    println!("API call result: {:?}", result);
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
     if let Ok(debt) = result {
        // Debt could be zero or positive
        println!("Fetched debt: {}", debt);
    }
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_fetch_real_expenditure
async fn test_fetch_real_expenditure() {
    dotenvy::dotenv().expect("Failed to load .env file for integration test");
    println!("Initializing client for integration test...");
    let client = MunicipalMoneyClient::new().expect("Failed to create client for test");
    let muni_code = "BUF";
    let year = 2022;
    println!("Fetching total expenditure for {} year {}...", muni_code, year);
    // Assuming the function exists in muni_money::financials
    let result = muni_money::financials::get_total_expenditure(&client, muni_code, year).await;
    println!("API call result: {:?}", result);
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
     if let Ok(expenditure) = result {
        println!("Fetched expenditure: {}", expenditure);
    }
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_fetch_real_capital_expenditure
async fn test_fetch_real_capital_expenditure() {
    dotenvy::dotenv().expect("Failed to load .env file for integration test");
    println!("Initializing client for integration test...");
    let client = MunicipalMoneyClient::new().expect("Failed to create client for test");
    let muni_code = "BUF";
    let year = 2022;
    println!("Fetching capital expenditure for {} year {}...", muni_code, year);
    // Assuming the function exists in muni_money::financials
    let result = muni_money::financials::get_capital_expenditure(&client, muni_code, year).await;
    println!("API call result: {:?}", result);
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
     if let Ok(cap_ex) = result {
        println!("Fetched capital expenditure: {}", cap_ex);
    }
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_fetch_real_audit_outcome
async fn test_fetch_real_audit_outcome() {
    dotenvy::dotenv().expect("Failed to load .env file for integration test");
    println!("Initializing client for integration test...");
    let client = MunicipalMoneyClient::new().expect("Failed to create client for test");
    let muni_code = "BUF";
    let year = 2022;
    println!("Fetching audit outcome for {} year {}...", muni_code, year);
    // Assuming the function exists in muni_money::audit
    let result = muni_money::audit::get_audit_outcome(&client, muni_code, year).await;
    
    // Print the raw result (Result<Option<String>, ApiClientError>)
    println!("get_audit_outcome result: {:?}", result);
    
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
    
     if let Ok(outcome_option) = result {
        // Print the extracted Option<String>
        println!("Fetched audit outcome Option: {:?}", outcome_option);
        // You could add a more specific assertion if you know the expected outcome
        // assert_eq!(outcome_option.as_deref(), Some("Unqualified - Emphasis of Matter items"));
    }
}