//! Models for the Balances API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::Currency;

/// Current balance for a currency.
#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    /// The currency code.
    pub currency: Currency,
    /// Available balance (can be used for payments).
    pub available_amount: f64,
    /// Pending balance (not yet settled).
    pub pending_amount: f64,
    /// Reserved balance (held for pending operations).
    #[serde(default)]
    pub reserved_amount: f64,
    /// Total balance.
    #[serde(default)]
    pub total_amount: f64,
    /// Prepayment amount.
    #[serde(default)]
    pub prepayment_amount: f64,
}

/// Response from GET /balances/current.
/// Note: The API returns a raw array, so we use a wrapper for convenience.
#[derive(Debug, Clone)]
pub struct CurrentBalancesResponse {
    /// List of balances by currency.
    pub items: Vec<Balance>,
}

impl CurrentBalancesResponse {
    /// Create from a vector of balances.
    pub fn new(items: Vec<Balance>) -> Self {
        Self { items }
    }
}

/// A single balance history entry.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceHistoryEntry {
    /// Unique ID for this transaction.
    pub id: String,
    /// The amount of the balance change.
    pub amount: f64,
    /// The currency.
    pub currency: Currency,
    /// Opening balance before this transaction.
    #[serde(default)]
    pub opening_balance: Option<f64>,
    /// Closing balance after this transaction.
    #[serde(default)]
    pub closing_balance: Option<f64>,
    /// Time when this transaction was posted.
    pub posted_at: DateTime<Utc>,
    /// Time when this transaction was created.
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    /// Type of transaction.
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Source type of the transaction.
    #[serde(default)]
    pub source_type: Option<String>,
    /// Source ID (e.g., transfer ID, deposit ID).
    #[serde(default)]
    pub source: Option<String>,
    /// Debit or credit indicator.
    #[serde(default)]
    pub debit_credit_flag: Option<String>,
    /// Client-provided request ID.
    #[serde(default)]
    pub request_id: Option<String>,
    /// Reference visible to beneficiary.
    #[serde(default)]
    pub reference: Option<String>,
    /// Internal note/reason.
    #[serde(default)]
    pub reason: Option<String>,
}

/// Response from GET /balances/history.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceHistoryResponse {
    /// List of balance history entries.
    pub items: Vec<BalanceHistoryEntry>,
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// Cursor for the next page.
    #[serde(default)]
    pub page_after: Option<String>,
    /// Cursor for the previous page.
    #[serde(default)]
    pub page_before: Option<String>,
}

/// Query parameters for balance history.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BalanceHistoryParams {
    /// Currency to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start of date range (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_post_at: Option<String>,
    /// End of date range (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_post_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Filter by request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl BalanceHistoryParams {
    /// Create new balance history parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Set the start of the date range.
    pub fn from_post_at(mut self, from: impl Into<String>) -> Self {
        self.from_post_at = Some(from.into());
        self
    }

    /// Set the end of the date range.
    pub fn to_post_at(mut self, to: impl Into<String>) -> Self {
        self.to_post_at = Some(to.into());
        self
    }

    /// Set the page number.
    pub fn page_num(mut self, page: i32) -> Self {
        self.page_num = Some(page);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Set the pagination cursor.
    pub fn page(mut self, cursor: impl Into<String>) -> Self {
        self.page = Some(cursor.into());
        self
    }
}
