//! Account models for Scale (Connected Accounts).
//!
//! Models for managing connected accounts in Airwallex Scale.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An Airwallex account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Airwallex account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Platform identifier for the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Human-friendly account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Primary contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<AccountContact>,
    /// Account details (complex nested structure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Value>,
    /// Customer agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_agreements: Option<Value>,
    /// Next action required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<Value>,
    /// Requirements for account activation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Reactivation details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactivate_details: Option<Value>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// View link for hosted onboarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_link: Option<String>,
}

/// Account contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountContact {
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Mobile phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

/// Request to create a new account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    /// Platform identifier for the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Human-friendly account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Primary contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<AccountContact>,
    /// Account details (complex nested structure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Value>,
    /// Customer agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_agreements: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateAccountRequest {
    /// Create a new account request.
    pub fn new() -> Self {
        Self {
            identifier: None,
            nickname: None,
            primary_contact: None,
            account_details: None,
            customer_agreements: None,
            metadata: None,
        }
    }

    /// Set the platform identifier.
    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Set the account nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Set primary contact.
    pub fn primary_contact(mut self, email: impl Into<String>) -> Self {
        self.primary_contact = Some(AccountContact {
            email: Some(email.into()),
            mobile: None,
        });
        self
    }

    /// Set account details.
    pub fn account_details(mut self, details: Value) -> Self {
        self.account_details = Some(details);
        self
    }
}

impl Default for CreateAccountRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to update an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAccountRequest {
    /// Platform identifier for the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Human-friendly account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Primary contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<AccountContact>,
    /// Account details (complex nested structure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Value>,
    /// Customer agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_agreements: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl UpdateAccountRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Set account details.
    pub fn account_details(mut self, details: Value) -> Self {
        self.account_details = Some(details);
        self
    }
}

/// Parameters for listing accounts.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListAccountsParams {
    /// Filter by account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// Filter by email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Filter by identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Filter by metadata (key:value format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// From created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100, max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListAccountsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by account status.
    pub fn account_status(mut self, status: impl Into<String>) -> Self {
        self.account_status = Some(status.into());
        self
    }

    /// Filter by identifier.
    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
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

/// Response for listing accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccountsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of accounts.
    #[serde(default)]
    pub items: Vec<Account>,
}
