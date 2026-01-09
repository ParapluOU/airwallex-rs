//! Settlement models.
//!
//! Models for payment acceptance settlements.

use serde::{Deserialize, Serialize};

/// A settlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    /// Batch ID of the settlement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Settlement amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Fee of the settlement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// Status (PENDING, SETTLED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Estimated settled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_settled_at: Option<String>,
    /// Actual settled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,
}

/// Settlement report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementReport {
    /// Batch ID of the settlement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URL of the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Parameters for listing settlements.
#[derive(Debug, Clone, Serialize)]
pub struct ListSettlementsParams {
    /// Currency of the settlement (required).
    pub currency: String,
    /// Status of the settlement (required): PENDING or SETTLED.
    pub status: String,
    /// Start date of settled_at (required).
    pub from_settled_at: String,
    /// End date of settled_at (required).
    pub to_settled_at: String,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListSettlementsParams {
    /// Create new parameters with required fields.
    pub fn new(
        currency: impl Into<String>,
        status: impl Into<String>,
        from_settled_at: impl Into<String>,
        to_settled_at: impl Into<String>,
    ) -> Self {
        Self {
            currency: currency.into(),
            status: status.into(),
            from_settled_at: from_settled_at.into(),
            to_settled_at: to_settled_at.into(),
            page_num: None,
            page_size: None,
        }
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

/// Parameters for getting a settlement report.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSettlementReportParams {
    /// File format (csv or excel).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    /// Version of the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetSettlementReportParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set file format.
    pub fn file_format(mut self, format: impl Into<String>) -> Self {
        self.file_format = Some(format.into());
        self
    }

    /// Set version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

/// Response for listing settlements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSettlementsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of settlements.
    #[serde(default)]
    pub items: Vec<Settlement>,
}
