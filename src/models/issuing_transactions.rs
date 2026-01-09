//! Issuing Transactions models.
//!
//! Models for card transactions in the Airwallex Issuing system.

use serde::{Deserialize, Serialize};

/// A card transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingTransaction {
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
    /// Transaction type (AUTHORIZATION, CLEARING, REFUND, REVERSAL, ORIGINAL_CREDIT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Transaction status (APPROVED, CLEARED, EXPIRED, FAILED, PENDING, REVERSED).
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
    /// Transaction date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<String>,
    /// Posted date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
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
    /// Matched authorization IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_authorizations: Option<Vec<String>>,
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

/// Merchant information for a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMerchant {
    /// Merchant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Merchant category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    /// Merchant city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Merchant country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Merchant state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Merchant postcode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// Merchant identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Additional merchant info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_merchant_info: Option<AdditionalMerchantInfo>,
}

/// Additional merchant information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalMerchantInfo {
    /// Merchant category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category: Option<String>,
    /// Merchant sub category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_sub_category: Option<String>,
    /// Merchant full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_full_name: Option<String>,
    /// Merchant logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_logo_url: Option<String>,
}

/// Transaction fee detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFeeDetail {
    /// Fee amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Fee currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Fee type (UNKNOWN, ATM_ACCESS_FEE).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
}

/// Transaction risk details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRiskDetails {
    /// Risk actions performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_actions_performed: Option<Vec<String>>,
    /// Risk factors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<Vec<String>>,
    /// 3D Secure outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_dsecure_outcome: Option<String>,
}

/// Parameters for listing issuing transactions.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListIssuingTransactionsParams {
    /// Filter by card ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// Filter by billing currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    /// Filter by transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
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

impl ListIssuingTransactionsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by card ID.
    pub fn card_id(mut self, id: impl Into<String>) -> Self {
        self.card_id = Some(id.into());
        self
    }

    /// Filter by transaction type.
    pub fn transaction_type(mut self, t: impl Into<String>) -> Self {
        self.transaction_type = Some(t.into());
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

/// Response for listing issuing transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIssuingTransactionsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of transactions.
    #[serde(default)]
    pub items: Vec<IssuingTransaction>,
}
