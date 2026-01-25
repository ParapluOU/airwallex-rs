#![allow(clippy::result_large_err)]
//! Example: Get current balances from Airwallex.
//!
//! This example demonstrates how to:
//! 1. Create a client from environment variables
//! 2. Authenticate with the Airwallex API
//! 3. Retrieve current balances
//!
//! Set the following environment variables:
//! - AIRWALLEX_CLIENT_ID
//! - AIRWALLEX_API_KEY
//! - AIRWALLEX_ENVIRONMENT (optional, defaults to "sandbox")

use airwallex_rs::Client;

#[tokio::main]
async fn main() -> airwallex_rs::Result<()> {
    // Initialize tracing for debug output
    tracing_subscriber::fmt::init();

    println!("Creating Airwallex client from environment...");
    let client = Client::from_env()?;

    println!("Fetching current balances...");
    let balances = client.balances().current().await?;

    println!("\nCurrent Balances:");
    println!("{:-<40}", "");

    if balances.items.is_empty() {
        println!("No balances found.");
    } else {
        for balance in &balances.items {
            println!(
                "{}: {} available, {} pending",
                balance.currency, balance.available_amount, balance.pending_amount
            );
        }
    }

    Ok(())
}
