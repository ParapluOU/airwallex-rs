//! Payment Acceptance Config models.
//!
//! Models for retrieving payment method and bank configuration.

use serde::{Deserialize, Serialize};

/// Bank resources (logos, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankResources {
    /// URL of the bank logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

/// A bank available for a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    /// The code name of the bank (pass to confirm PaymentIntent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The bank name for display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Bank resources (logos).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<BankResources>,
}

/// A payment method type configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodTypeConfig {
    /// The type of payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the payment method is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Transaction mode (oneoff, recurring).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_mode: Option<String>,
    /// Supported flows (qrcode, mobile_web, mobile_app).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<String>>,
    /// Supported transaction currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currencies: Option<Vec<String>>,
}

/// Parameters for listing payment method types.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPaymentMethodTypesParams {
    /// Filter by active status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Filter by supported country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Filter by transaction currency (required if country_code is given).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currency: Option<String>,
    /// Filter by transaction mode (oneoff, recurring).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_mode: Option<String>,
    /// Page number (starting from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPaymentMethodTypesParams {
    /// Create new params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by active status.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Filter by country code.
    pub fn country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
        self
    }

    /// Filter by transaction currency.
    pub fn transaction_currency(mut self, currency: impl Into<String>) -> Self {
        self.transaction_currency = Some(currency.into());
        self
    }

    /// Filter by transaction mode.
    pub fn transaction_mode(mut self, mode: impl Into<String>) -> Self {
        self.transaction_mode = Some(mode.into());
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

/// Response for listing payment method types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentMethodTypesResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment method types.
    #[serde(default)]
    pub items: Vec<PaymentMethodTypeConfig>,
}

/// Parameters for listing banks.
#[derive(Debug, Clone, Serialize)]
pub struct ListBanksParams {
    /// The payment method type (required).
    pub payment_method_type: String,
    /// Country code to filter banks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Page number (starting from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListBanksParams {
    /// Create new params with required payment method type.
    pub fn new(payment_method_type: impl Into<String>) -> Self {
        Self {
            payment_method_type: payment_method_type.into(),
            country_code: None,
            page_num: None,
            page_size: None,
        }
    }

    /// Set country code.
    pub fn country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
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

/// Response for listing banks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBanksResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of banks.
    #[serde(default)]
    pub items: Vec<Bank>,
}
