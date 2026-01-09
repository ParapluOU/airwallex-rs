//! Account Capabilities models.
//!
//! Models for managing account capabilities like funding limits and payment methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status of a capability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityStatus {
    /// Capability is enabled.
    Enabled,
    /// Capability is disabled.
    Disabled,
    /// Capability is pending approval.
    Pending,
}

/// Entity type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    /// Individual entity.
    Individual,
    /// Business entity.
    Business,
}

/// Funding limit type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FundingLimitType {
    /// Direct debit deposit.
    DirectDebitDeposit,
    /// Faster direct debit deposit.
    FasterDirectDebitDeposit,
}

/// Funding limit status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FundingLimitStatus {
    /// Limit is active.
    Active,
    /// Limit is requested.
    Requested,
}

/// Details about a capability's status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDetails {
    /// Reason codes for disabled capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
}

/// An account capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCapability {
    /// Unique ID of the capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Status of the capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CapabilityStatus>,
    /// Entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    /// Additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<CapabilityDetails>,
    /// Additional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Last update time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A funding limit request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingLimitRequest {
    /// Currency of the limit.
    pub currency: String,
    /// The limit amount.
    pub limit: f64,
    /// Type of limit.
    #[serde(rename = "type")]
    pub limit_type: FundingLimitType,
}

impl FundingLimitRequest {
    /// Create a new funding limit request.
    pub fn new(currency: impl Into<String>, limit: f64, limit_type: FundingLimitType) -> Self {
        Self {
            currency: currency.into(),
            limit,
            limit_type,
        }
    }
}

/// Request to apply for enhanced capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCapabilitiesRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Funding limit requests.
    pub funding_limits: Vec<FundingLimitRequest>,
}

impl ApplyCapabilitiesRequest {
    /// Create a new apply request.
    pub fn new(request_id: impl Into<String>, funding_limits: Vec<FundingLimitRequest>) -> Self {
        Self {
            request_id: request_id.into(),
            funding_limits,
        }
    }
}

/// Response from applying for capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCapabilitiesResponse {
    /// Result message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// A funding limit item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingLimit {
    /// Currency of the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The upper limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The requested limit (under evaluation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_limit: Option<f64>,
    /// Type of limit.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub limit_type: Option<FundingLimitType>,
    /// Status of the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FundingLimitStatus>,
    /// Available limits by scheme (e.g., SAME_DAY_ACH, BACS, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availables: Option<HashMap<String, f64>>,
    /// Settlement method or clearing system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_type: Option<String>,
    /// Effective timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    /// Last update time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Parameters for listing funding limits.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListFundingLimitsParams {
    /// Currency to display the limit in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Timestamp at which the limit becomes effective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    /// Settlement method or clearing system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_type: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListFundingLimitsParams {
    /// Create new params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set effective timestamp.
    pub fn effective_at(mut self, time: impl Into<String>) -> Self {
        self.effective_at = Some(time.into());
        self
    }

    /// Set mandate type.
    pub fn mandate_type(mut self, mandate_type: impl Into<String>) -> Self {
        self.mandate_type = Some(mandate_type.into());
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

/// Response for listing funding limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFundingLimitsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of funding limits.
    #[serde(default)]
    pub items: Vec<FundingLimit>,
}
