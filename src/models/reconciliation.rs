//! Reconciliation models.
//!
//! Models for reconciliation data (treasury/balance transactions).

use serde::{Deserialize, Serialize};

/// Debit/Credit flag.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DebitCreditFlag {
    /// Debit.
    Debit,
    /// Credit.
    Credit,
}

/// A treasury balance entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryBalance {
    /// Entry ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Transaction type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub balance_type: Option<String>,
    /// Debit/credit flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_credit_flag: Option<DebitCreditFlag>,
    /// Opening balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<f64>,
    /// Closing balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_balance: Option<f64>,
    /// Client request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Reason/note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Reference to beneficiary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Beneficiary bank account name (for payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_account_name: Option<String>,
    /// Beneficiary bank account number (for payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_account_number: Option<String>,
    /// Remitter name (for deposits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remitter_name: Option<String>,
    /// Remitting bank (for deposits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remitting_bank: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Posted timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

/// Parameters for listing treasury balances.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTreasuryBalancesParams {
    /// Filter by client request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub balance_type: Option<String>,
    /// Start of posted_at range (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_posted_at: Option<String>,
    /// End of posted_at range (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_posted_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListTreasuryBalancesParams {
    /// Create new params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by client request ID.
    pub fn client_request_id(mut self, id: impl Into<String>) -> Self {
        self.client_request_id = Some(id.into());
        self
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Filter by type.
    pub fn balance_type(mut self, t: impl Into<String>) -> Self {
        self.balance_type = Some(t.into());
        self
    }

    /// Set start of posted_at range.
    pub fn from_posted_at(mut self, time: impl Into<String>) -> Self {
        self.from_posted_at = Some(time.into());
        self
    }

    /// Set end of posted_at range.
    pub fn to_posted_at(mut self, time: impl Into<String>) -> Self {
        self.to_posted_at = Some(time.into());
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

/// Response for listing treasury balances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTreasuryBalancesResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of balance entries.
    #[serde(default)]
    pub items: Vec<TreasuryBalance>,
}
