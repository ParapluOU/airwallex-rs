//! Connected Account Transfers models.
//!
//! Models for transfers between connected accounts in the Scale product.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A connected account transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountTransfer {
    /// Unique ID for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency (3-letter ISO-4217 code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Destination Airwallex account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Reason for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Reference displayed to beneficiary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Unique request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Transfer status (NEW, SETTLED, PENDING, SUSPENDED, FAILED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Transfer fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// Short reference for support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Additional info including card payment data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Request to create a connected account transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectedAccountTransferRequest {
    /// Amount to transfer.
    pub amount: String,
    /// Currency (3-letter ISO-4217 code).
    pub currency: String,
    /// Destination Airwallex account ID.
    pub destination: String,
    /// Reason for the transfer.
    pub reason: String,
    /// Reference displayed to beneficiary (1-140 characters).
    pub reference: String,
    /// Unique request ID (1-50 characters).
    pub request_id: String,
    /// Additional info including card payment data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
}

impl CreateConnectedAccountTransferRequest {
    /// Create a new connected account transfer request.
    pub fn new(
        amount: impl Into<String>,
        currency: impl Into<String>,
        destination: impl Into<String>,
        reason: impl Into<String>,
        reference: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            destination: destination.into(),
            reason: reason.into(),
            reference: reference.into(),
            request_id: request_id.into(),
            additional_info: None,
        }
    }

    /// Set additional info.
    pub fn additional_info(mut self, info: Value) -> Self {
        self.additional_info = Some(info);
        self
    }
}

/// Parameters for listing connected account transfers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListConnectedAccountTransfersParams {
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by destination account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// From created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter (ISO8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 100, min 10, max 2000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListConnectedAccountTransfersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Filter by destination.
    pub fn destination(mut self, dest: impl Into<String>) -> Self {
        self.destination = Some(dest.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
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

/// Response for listing connected account transfers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConnectedAccountTransfersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of transfers.
    #[serde(default)]
    pub items: Vec<ConnectedAccountTransfer>,
}
