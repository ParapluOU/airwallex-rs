//! Refund models.
//!
//! Models for managing refunds on payments.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    /// Refund ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Payment attempt ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_attempt_id: Option<String>,
    /// Refund amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Refund status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Reason for refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Request to create a refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Payment intent ID to refund.
    pub payment_intent_id: String,
    /// Amount to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Payment attempt ID to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_attempt_id: Option<String>,
    /// Reason for refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateRefundRequest {
    /// Create a new refund request.
    pub fn new(request_id: impl Into<String>, payment_intent_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            payment_intent_id: payment_intent_id.into(),
            amount: None,
            payment_attempt_id: None,
            reason: None,
            metadata: None,
        }
    }

    /// Set refund amount (for partial refund).
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set payment attempt ID.
    pub fn payment_attempt_id(mut self, id: impl Into<String>) -> Self {
        self.payment_attempt_id = Some(id.into());
        self
    }

    /// Set reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Parameters for listing refunds.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListRefundsParams {
    /// Filter by payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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

impl ListRefundsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by payment intent ID.
    pub fn payment_intent_id(mut self, id: impl Into<String>) -> Self {
        self.payment_intent_id = Some(id.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
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

/// Response for listing refunds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRefundsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of refunds.
    #[serde(default)]
    pub items: Vec<Refund>,
}
