//! Linked Accounts models.
//!
//! Models for managing linked bank accounts used for direct debits.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A linked bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccount {
    /// Linked account ID.
    pub id: String,
    /// Account status.
    pub status: String,
    /// Account type (AU_BANK, US_BANK, GB_BANK, etc.).
    #[serde(rename = "type")]
    pub account_type: String,
    /// Supported currencies.
    pub supported_currencies: Vec<String>,
    /// Reason for current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Capabilities of the linked account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
    /// Next action required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<Value>,
    /// Failure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<Value>,
    /// Australia bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_bank: Option<Value>,
    /// US bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank: Option<Value>,
    /// GB bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank: Option<Value>,
    /// EU bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank: Option<Value>,
    /// HK bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hk_bank: Option<Value>,
    /// SG bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg_bank: Option<Value>,
    /// CA bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_bank: Option<Value>,
    /// NZ bank info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank: Option<Value>,
    /// Australia PayID info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_payid: Option<Value>,
}

/// Request to create a linked account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkedAccountRequest {
    /// Request ID.
    pub request_id: String,
    /// Account type.
    #[serde(rename = "type")]
    pub account_type: String,
    /// Account details (bank-specific).
    #[serde(flatten)]
    pub details: Value,
}

impl CreateLinkedAccountRequest {
    /// Create a new request.
    pub fn new(
        request_id: impl Into<String>,
        account_type: impl Into<String>,
        details: Value,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            account_type: account_type.into(),
            details,
        }
    }
}

/// Parameters for listing linked accounts.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLinkedAccountsParams {
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListLinkedAccountsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by type.
    pub fn account_type(mut self, account_type: impl Into<String>) -> Self {
        self.account_type = Some(account_type.into());
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

/// Response for listing linked accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLinkedAccountsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of linked accounts.
    #[serde(default)]
    pub items: Vec<LinkedAccount>,
}

/// Auth initiation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateAuthRequest {
    /// Linked account type.
    #[serde(rename = "type")]
    pub account_type: String,
    /// Redirect URL after auth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

/// Auth initiation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateAuthResponse {
    /// Auth URL to redirect user to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    /// Session token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

/// Complete auth request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteAuthRequest {
    /// Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Verify microdeposits request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMicrodepositsRequest {
    /// First amount.
    pub amount_1: f64,
    /// Second amount.
    pub amount_2: f64,
}

/// Linked account balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccountBalance {
    /// Available balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<f64>,
    /// Current balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Mandate information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccountMandate {
    /// Mandate ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Mandate status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}
