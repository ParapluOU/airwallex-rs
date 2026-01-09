//! Transfer models.
//!
//! Models for managing payout transfers (sending payments to beneficiaries).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payout transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    /// Transfer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Transfer status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Short reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Source amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<f64>,
    /// Source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// Target amount (amount to beneficiary before fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<f64>,
    /// Target currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_currency: Option<String>,
    /// Amount beneficiary receives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_beneficiary_receives: Option<f64>,
    /// Fee amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// Fee currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
    /// Who pays the fee (PAYER or BENEFICIARY).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_paid_by: Option<String>,
    /// Payment method (LOCAL or SWIFT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Reference shown to beneficiary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Reason for transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Beneficiary ID (if using saved beneficiary).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    /// Beneficiary details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<Value>,
    /// Swift charge option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_charge_option: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Completion date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// Payout failure reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_failure_reason: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Request to create a transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Source currency.
    pub source_currency: String,
    /// Source amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<f64>,
    /// Target currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_currency: Option<String>,
    /// Target amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<f64>,
    /// Who pays the fee.
    pub fee_paid_by: String,
    /// Payment method.
    pub payment_method: String,
    /// Reference shown to beneficiary (1-140 chars).
    pub reference: String,
    /// Reason for transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Beneficiary ID (for saved beneficiary).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    /// Beneficiary details (for inline beneficiary).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<Value>,
    /// Swift charge option (OUR, SHA, BEN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_charge_option: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateTransferRequest {
    /// Create a new transfer request with a saved beneficiary.
    pub fn with_beneficiary_id(
        request_id: impl Into<String>,
        beneficiary_id: impl Into<String>,
        source_currency: impl Into<String>,
        source_amount: f64,
        payment_method: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            source_currency: source_currency.into(),
            source_amount: Some(source_amount),
            target_currency: None,
            target_amount: None,
            fee_paid_by: "PAYER".to_string(),
            payment_method: payment_method.into(),
            reference: reference.into(),
            reason: None,
            beneficiary_id: Some(beneficiary_id.into()),
            beneficiary: None,
            swift_charge_option: None,
            metadata: None,
        }
    }

    /// Create a new transfer request with inline beneficiary.
    pub fn with_beneficiary(
        request_id: impl Into<String>,
        beneficiary: Value,
        source_currency: impl Into<String>,
        source_amount: f64,
        payment_method: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            source_currency: source_currency.into(),
            source_amount: Some(source_amount),
            target_currency: None,
            target_amount: None,
            fee_paid_by: "PAYER".to_string(),
            payment_method: payment_method.into(),
            reference: reference.into(),
            reason: None,
            beneficiary_id: None,
            beneficiary: Some(beneficiary),
            swift_charge_option: None,
            metadata: None,
        }
    }

    /// Set who pays the fee.
    pub fn fee_paid_by(mut self, payer: impl Into<String>) -> Self {
        self.fee_paid_by = payer.into();
        self
    }

    /// Set the reason for transfer.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Set the target currency and amount.
    pub fn target(mut self, currency: impl Into<String>, amount: f64) -> Self {
        self.target_currency = Some(currency.into());
        self.target_amount = Some(amount);
        self
    }

    /// Set Swift charge option.
    pub fn swift_charge_option(mut self, option: impl Into<String>) -> Self {
        self.swift_charge_option = Some(option.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Parameters for listing transfers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListTransfersParams {
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// Filter by beneficiary ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    /// Filter by payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
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

impl ListTransfersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by source currency.
    pub fn source_currency(mut self, currency: impl Into<String>) -> Self {
        self.source_currency = Some(currency.into());
        self
    }

    /// Filter by beneficiary ID.
    pub fn beneficiary_id(mut self, id: impl Into<String>) -> Self {
        self.beneficiary_id = Some(id.into());
        self
    }

    /// Filter by payment method.
    pub fn payment_method(mut self, method: impl Into<String>) -> Self {
        self.payment_method = Some(method.into());
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

/// Response for listing transfers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTransfersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of transfers.
    #[serde(default)]
    pub items: Vec<Transfer>,
}
