//! Conversion (FX) models.
//!
//! Models for managing foreign exchange conversions.

use serde::{Deserialize, Serialize};

/// A currency conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversion {
    /// Conversion ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Buy amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<f64>,
    /// Buy currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_currency: Option<String>,
    /// Sell amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<f64>,
    /// Sell currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_currency: Option<String>,
    /// Client rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_rate: Option<f64>,
    /// Currency pair (e.g., AUDUSD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Dealt currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealt_currency: Option<String>,
    /// Conversion date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_date: Option<String>,
    /// Settlement cutoff time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_cutoff_time: Option<String>,
    /// Short reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Reason for conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
}

/// Request to create a conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversionRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Sell currency.
    pub sell_currency: String,
    /// Buy currency.
    pub buy_currency: String,
    /// Sell amount (mutually exclusive with buy_amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<f64>,
    /// Buy amount (mutually exclusive with sell_amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<f64>,
    /// Conversion date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_date: Option<String>,
    /// Reason for conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Quote ID from a previous quote request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Termination currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_agreement: Option<bool>,
}

impl CreateConversionRequest {
    /// Create a conversion request with sell amount.
    pub fn sell(
        request_id: impl Into<String>,
        sell_currency: impl Into<String>,
        sell_amount: f64,
        buy_currency: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            sell_currency: sell_currency.into(),
            buy_currency: buy_currency.into(),
            sell_amount: Some(sell_amount),
            buy_amount: None,
            conversion_date: None,
            reason: None,
            quote_id: None,
            term_agreement: None,
        }
    }

    /// Create a conversion request with buy amount.
    pub fn buy(
        request_id: impl Into<String>,
        sell_currency: impl Into<String>,
        buy_currency: impl Into<String>,
        buy_amount: f64,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            sell_currency: sell_currency.into(),
            buy_currency: buy_currency.into(),
            sell_amount: None,
            buy_amount: Some(buy_amount),
            conversion_date: None,
            reason: None,
            quote_id: None,
            term_agreement: None,
        }
    }

    /// Set conversion date.
    pub fn conversion_date(mut self, date: impl Into<String>) -> Self {
        self.conversion_date = Some(date.into());
        self
    }

    /// Set reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Set quote ID.
    pub fn quote_id(mut self, id: impl Into<String>) -> Self {
        self.quote_id = Some(id.into());
        self
    }
}

/// Parameters for listing conversions.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListConversionsParams {
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by buy currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_currency: Option<String>,
    /// Filter by sell currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_currency: Option<String>,
    /// Start date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListConversionsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by buy currency.
    pub fn buy_currency(mut self, currency: impl Into<String>) -> Self {
        self.buy_currency = Some(currency.into());
        self
    }

    /// Filter by sell currency.
    pub fn sell_currency(mut self, currency: impl Into<String>) -> Self {
        self.sell_currency = Some(currency.into());
        self
    }

    /// Set page number.
    pub fn page_num(mut self, num: i32) -> Self {
        self.page_num = Some(num);
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}

/// Response for listing conversions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConversionsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of conversions.
    #[serde(default)]
    pub items: Vec<Conversion>,
}

/// A rate quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateQuote {
    /// Quote ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Quoted rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_rate: Option<f64>,
    /// Buy amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<f64>,
    /// Buy currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_currency: Option<String>,
    /// Sell amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<f64>,
    /// Sell currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_currency: Option<String>,
    /// Quote expiry time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

/// Request for a rate quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateQuoteRequest {
    /// Sell currency.
    pub sell_currency: String,
    /// Buy currency.
    pub buy_currency: String,
    /// Amount (in sell currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<f64>,
    /// Amount (in buy currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<f64>,
    /// Value date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_date: Option<String>,
}

impl RateQuoteRequest {
    /// Create a quote request.
    pub fn new(sell_currency: impl Into<String>, buy_currency: impl Into<String>) -> Self {
        Self {
            sell_currency: sell_currency.into(),
            buy_currency: buy_currency.into(),
            sell_amount: None,
            buy_amount: None,
            conversion_date: None,
        }
    }

    /// Set sell amount.
    pub fn sell_amount(mut self, amount: f64) -> Self {
        self.sell_amount = Some(amount);
        self
    }

    /// Set buy amount.
    pub fn buy_amount(mut self, amount: f64) -> Self {
        self.buy_amount = Some(amount);
        self
    }
}
