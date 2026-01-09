//! Issuing Authorizations models.
//!
//! Models for card authorizations in the Airwallex Issuing system.

use serde::{Deserialize, Serialize};

use crate::models::issuing_transactions::{
    TransactionFeeDetail, TransactionMerchant, TransactionRiskDetails,
};

/// A card authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorization {
    /// Unique transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Card ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// Card nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_nickname: Option<String>,
    /// Masked card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_card_number: Option<String>,
    /// Authorization status (CLEARED, EXPIRED, FAILED, PENDING, REVERSED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<f64>,
    /// Transaction currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currency: Option<String>,
    /// Billing amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_amount: Option<f64>,
    /// Billing currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    /// Time authorization was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Authorization expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// Merchant information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<TransactionMerchant>,
    /// Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Retrieval reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_ref: Option<String>,
    /// Network transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_transaction_id: Option<String>,
    /// Lifecycle ID linking related transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_id: Option<String>,
    /// Digital wallet token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_wallet_token_id: Option<String>,
    /// Transaction ID that updated this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_transaction: Option<String>,
    /// Failure reason if status is FAILED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Fee details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_details: Option<Vec<TransactionFeeDetail>>,
    /// Risk details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_details: Option<TransactionRiskDetails>,
    /// Client data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_data: Option<String>,
    /// Acquiring institution identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquiring_institution_identifier: Option<String>,
}

/// Parameters for listing issuing authorizations.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListIssuingAuthorizationsParams {
    /// Filter by card ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// Filter by billing currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by lifecycle ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_id: Option<String>,
    /// Filter by retrieval reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_ref: Option<String>,
    /// Filter by digital wallet token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_wallet_token_id: Option<String>,
    /// From created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListIssuingAuthorizationsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by card ID.
    pub fn card_id(mut self, id: impl Into<String>) -> Self {
        self.card_id = Some(id.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by billing currency.
    pub fn billing_currency(mut self, currency: impl Into<String>) -> Self {
        self.billing_currency = Some(currency.into());
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

/// Response for listing issuing authorizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIssuingAuthorizationsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of authorizations.
    #[serde(default)]
    pub items: Vec<IssuingAuthorization>,
}
