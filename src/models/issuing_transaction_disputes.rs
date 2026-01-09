//! Issuing Transaction Disputes models.
//!
//! Models for creating and managing transaction disputes for card transactions.

use serde::{Deserialize, Serialize};

/// Reason for raising an issuing transaction dispute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssuingDisputeReason {
    /// Suspected fraud.
    SuspectedFraud,
    /// Unauthorized transaction.
    UnauthorizedTransaction,
    /// Duplicated transaction.
    DuplicatedTransaction,
    /// Paid by other means.
    PaidByOtherMeans,
    /// Goods/service not as described.
    GoodsServiceNotAsDescribed,
    /// Goods damaged.
    GoodsDamaged,
    /// Goods/service not received.
    GoodsServiceNotReceived,
    /// Refund unprocessed.
    RefundUnprocessed,
    /// Goods/service canceled.
    GoodsServiceCanceled,
    /// Recurring canceled.
    RecurringCanceled,
    /// Other reason.
    Other,
}

/// Status of an issuing transaction dispute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssuingDisputeStatus {
    /// Draft - not yet submitted.
    Draft,
    /// Submitted to Airwallex.
    Submitted,
    /// Rejected by Airwallex.
    Rejected,
    /// Canceled by customer.
    Canceled,
    /// In progress with card scheme.
    InProgress,
    /// Won the dispute.
    Won,
    /// Lost the dispute.
    Lost,
    /// Expired.
    Expired,
}

/// Detailed status of the issuing dispute life cycle between Airwallex and the card scheme.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssuingDisputeDetailedStatus {
    /// Dispute filed.
    DisputeFiled,
    /// Pre-arbitration received.
    PreArbReceived,
    /// Pre-arbitration declined by issuer.
    PreArbDeclinedByIssuer,
    /// Arbitration received.
    ArbitrationReceived,
    /// Chargeback declined.
    ChargebackDeclined,
    /// Pre-arbitration delivered.
    PreArbDelivered,
    /// Pre-arbitration declined by acquirer.
    PreArbDeclinedByAcquirer,
    /// Arbitration delivered.
    ArbDelivered,
    /// Won.
    Won,
    /// Lost.
    Lost,
}

/// Party who updated the issuing dispute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssuingDisputeUpdatedBy {
    /// Customer.
    Customer,
    /// Airwallex.
    Airwallex,
}

/// An entry in the dispute update history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeUpdateHistoryEntry {
    /// The list of file IDs containing evidence added to the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_files: Option<Vec<String>>,
    /// The note added to the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Time when the dispute was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The party who updated the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<IssuingDisputeUpdatedBy>,
}

/// An issuing transaction dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionDispute {
    /// The unique identifier for this dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Dispute amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The unique identifier for the disputed transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// The reason for raising the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<IssuingDisputeReason>,
    /// The latest status of the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuingDisputeStatus>,
    /// The detailed status of the dispute life cycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<IssuingDisputeDetailedStatus>,
    /// The latest notes added to the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A value added for internal reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The update history of the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_history: Option<Vec<DisputeUpdateHistoryEntry>>,
    /// The party who last updated the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<IssuingDisputeUpdatedBy>,
    /// Time when the dispute was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Time when the dispute was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Request to create a transaction dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssuingTransactionDisputeRequest {
    /// The unique identifier for the transaction to be disputed.
    pub transaction_id: String,
    /// The reason for raising the dispute.
    pub reason: IssuingDisputeReason,
    /// The amount to be disputed. If not specified, uses the billing amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The explanation on why the cardholder is disputing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A value for internal reference. Maximum 20 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// A list of file IDs containing evidence for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_files: Option<Vec<String>>,
}

impl CreateIssuingTransactionDisputeRequest {
    /// Create a new dispute request.
    pub fn new(transaction_id: impl Into<String>, reason: IssuingDisputeReason) -> Self {
        Self {
            transaction_id: transaction_id.into(),
            reason,
            amount: None,
            notes: None,
            reference: None,
            evidence_files: None,
        }
    }

    /// Set the dispute amount (for partial disputes).
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the notes/explanation.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Set the internal reference.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set the evidence file IDs.
    pub fn evidence_files(mut self, files: Vec<String>) -> Self {
        self.evidence_files = Some(files);
        self
    }
}

/// Request to update a transaction dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIssuingTransactionDisputeRequest {
    /// A unique request ID (for idempotency).
    pub request_id: String,
    /// The disputed amount. Can only be updated when status = DRAFT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The reason for raising the dispute. Can only be updated when status = DRAFT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<IssuingDisputeReason>,
    /// The explanation on why the cardholder is disputing the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A list of file IDs containing evidence for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_files: Option<Vec<String>>,
}

impl UpdateIssuingTransactionDisputeRequest {
    /// Create a new update request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            amount: None,
            reason: None,
            notes: None,
            evidence_files: None,
        }
    }

    /// Set the dispute amount.
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the dispute reason.
    pub fn reason(mut self, reason: IssuingDisputeReason) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Set the notes/explanation.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Set the evidence file IDs.
    pub fn evidence_files(mut self, files: Vec<String>) -> Self {
        self.evidence_files = Some(files);
        self
    }
}

/// Parameters for listing transaction disputes.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIssuingTransactionDisputesParams {
    /// Filter by transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuingDisputeStatus>,
    /// Filter by detailed status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<IssuingDisputeDetailedStatus>,
    /// Filter by reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<IssuingDisputeReason>,
    /// Filter by reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Filter by updated_by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<IssuingDisputeUpdatedBy>,
    /// Minimum created_at (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// Maximum created_at (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Minimum updated_at (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_updated_at: Option<String>,
    /// Maximum updated_at (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_updated_at: Option<String>,
    /// Page bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Number of disputes per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListIssuingTransactionDisputesParams {
    /// Create new params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by transaction ID.
    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: IssuingDisputeStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Filter by detailed status.
    pub fn detailed_status(mut self, status: IssuingDisputeDetailedStatus) -> Self {
        self.detailed_status = Some(status);
        self
    }

    /// Filter by reason.
    pub fn reason(mut self, reason: IssuingDisputeReason) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Filter by reference.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Filter by updated_by.
    pub fn updated_by(mut self, by: IssuingDisputeUpdatedBy) -> Self {
        self.updated_by = Some(by);
        self
    }

    /// Set minimum created_at.
    pub fn from_created_at(mut self, time: impl Into<String>) -> Self {
        self.from_created_at = Some(time.into());
        self
    }

    /// Set maximum created_at.
    pub fn to_created_at(mut self, time: impl Into<String>) -> Self {
        self.to_created_at = Some(time.into());
        self
    }

    /// Set minimum updated_at.
    pub fn from_updated_at(mut self, time: impl Into<String>) -> Self {
        self.from_updated_at = Some(time.into());
        self
    }

    /// Set maximum updated_at.
    pub fn to_updated_at(mut self, time: impl Into<String>) -> Self {
        self.to_updated_at = Some(time.into());
        self
    }

    /// Set page bookmark.
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}

/// Response for listing transaction disputes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIssuingTransactionDisputesResponse {
    /// List of disputes.
    #[serde(default)]
    pub items: Vec<IssuingTransactionDispute>,
    /// Pointer to the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Pointer to the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
}
