//! Deposit models.
//!
//! Models for tracking incoming deposits to global accounts.

use serde::{Deserialize, Serialize};

/// A deposit received on a global account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Unique deposit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_id: Option<String>,
    /// Amount deposited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency (3-letter ISO-4217).
    pub currency: String,
    /// Global account ID that received the deposit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_account_id: Option<String>,
    /// Statement reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_ref: Option<String>,
    /// When the deposit was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Parameters for listing deposits.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDepositsParams {
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by global account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_account_id: Option<String>,
    /// Start date for created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End date for created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListDepositsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Filter by global account ID.
    pub fn global_account_id(mut self, id: impl Into<String>) -> Self {
        self.global_account_id = Some(id.into());
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

/// Response for listing deposits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDepositsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of deposits.
    #[serde(default)]
    pub items: Vec<Deposit>,
}
