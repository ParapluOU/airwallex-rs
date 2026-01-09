//! Reference Data models.
//!
//! Models for retrieving reference data like supported currencies.

use serde::{Deserialize, Serialize};

/// Conversion currencies configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionCurrencies {
    /// List of buyable currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_currencies: Option<Vec<String>>,
    /// List of sellable currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_currencies: Option<Vec<String>>,
}

/// Supported currencies response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedCurrencies {
    /// Conversion currencies configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<ConversionCurrencies>,
}
