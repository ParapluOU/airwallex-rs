//! # airwallex-rs
//!
//! A Rust client for the [Airwallex REST API](https://www.airwallex.com/docs/api).
//!
//! ## Features
//!
//! - Type-safe API client with async/await support
//! - Automatic token management and refresh
//! - Built-in retry logic for rate limits
//! - Support for all Airwallex API domains
//!
//! ## Quick Start
//!
//! ```no_run
//! use airwallex_rs::{Client, Config, Environment};
//!
//! #[tokio::main]
//! async fn main() -> airwallex_rs::Result<()> {
//!     // Create client from environment variables
//!     let client = Client::from_env()?;
//!
//!     // Get current balances
//!     let balances = client.balances().current().await?;
//!     for balance in &balances.items {
//!         println!("{}: {} available", balance.currency, balance.available_amount);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! The client can be configured via environment variables:
//!
//! - `AIRWALLEX_CLIENT_ID` - Your API client ID
//! - `AIRWALLEX_API_KEY` - Your API key
//! - `AIRWALLEX_ENVIRONMENT` - `sandbox` or `production` (default: `sandbox`)
//!
//! Or programmatically:
//!
//! ```no_run
//! use airwallex_rs::{Client, Config, Environment};
//!
//! # fn main() -> airwallex_rs::Result<()> {
//! let config = Config::builder()
//!     .client_id("your_client_id")
//!     .api_key("your_api_key")
//!     .environment(Environment::Sandbox)
//!     .build()?;
//!
//! let client = Client::new(config)?;
//! # Ok(())
//! # }
//! ```

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod resources;

// Re-export main types at crate root
pub use client::Client;
pub use config::{Config, ConfigBuilder, Environment};
pub use error::{Error, Result};
