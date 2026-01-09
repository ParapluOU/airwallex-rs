//! Financial Transaction models.
//!
//! Models for viewing financial transactions that affect account balance.

use serde::{Deserialize, Serialize};

/// A financial transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialTransaction {
    /// Transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gross amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Net amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<f64>,
    /// Fee amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// Currency (3-letter ISO-4217).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Status (PENDING, SETTLED, CANCELLED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Source type (PAYOUT, CONVERSION, DEPOSIT, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// Source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// Transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Batch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Funding source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source_id: Option<String>,
    /// Client rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_rate: Option<f64>,
    /// Currency pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Settled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,
    /// Estimated settled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_settled_at: Option<String>,
}

/// Parameters for listing financial transactions.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListFinancialTransactionsParams {
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by status (PENDING, SETTLED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by batch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Filter by source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// From created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starting from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListFinancialTransactionsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by batch ID.
    pub fn batch_id(mut self, batch_id: impl Into<String>) -> Self {
        self.batch_id = Some(batch_id.into());
        self
    }

    /// Filter by source ID.
    pub fn source_id(mut self, source_id: impl Into<String>) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    /// Set from created_at filter.
    pub fn from_created_at(mut self, from: impl Into<String>) -> Self {
        self.from_created_at = Some(from.into());
        self
    }

    /// Set to created_at filter.
    pub fn to_created_at(mut self, to: impl Into<String>) -> Self {
        self.to_created_at = Some(to.into());
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

/// Response for listing financial transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFinancialTransactionsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of financial transactions.
    #[serde(default)]
    pub items: Vec<FinancialTransaction>,
}
