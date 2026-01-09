//! Conversion Amendments models.
//!
//! Models for amending (cancelling) FX conversions.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A conversion amendment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionAmendment {
    /// Amendment ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amendment_id: Option<String>,
    /// Conversion ID being amended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Amendment type (CANCEL).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub amendment_type: Option<String>,
    /// Short reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Charges resulting from the amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<AmendmentCharge>>,
    /// Client-supplied metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A charge resulting from a conversion amendment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentCharge {
    /// Charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Charge currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Charge type (FEE or CREDIT).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// Currency pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Client rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_rate: Option<f64>,
    /// AWX rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awx_rate: Option<f64>,
}

/// Amendment quote response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentQuote {
    /// Conversion ID being amended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Amendment type (CANCEL).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub amendment_type: Option<String>,
    /// Short reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Indicative charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<AmendmentCharge>>,
    /// Client-supplied metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Request to create an amendment quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAmendmentQuoteRequest {
    /// Conversion ID to amend.
    pub conversion_id: String,
    /// Unique request ID.
    pub request_id: String,
    /// Amendment type (only CANCEL supported).
    #[serde(rename = "type")]
    pub amendment_type: String,
    /// Currency for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_currency: Option<String>,
    /// Client-supplied metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateAmendmentQuoteRequest {
    /// Create a cancellation quote request.
    pub fn cancel(conversion_id: impl Into<String>, request_id: impl Into<String>) -> Self {
        Self {
            conversion_id: conversion_id.into(),
            request_id: request_id.into(),
            amendment_type: "CANCEL".to_string(),
            charge_currency: None,
            metadata: None,
        }
    }

    /// Set charge currency.
    pub fn charge_currency(mut self, currency: impl Into<String>) -> Self {
        self.charge_currency = Some(currency.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to create an amendment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAmendmentRequest {
    /// Conversion ID to amend.
    pub conversion_id: String,
    /// Unique request ID.
    pub request_id: String,
    /// Amendment type (only CANCEL supported).
    #[serde(rename = "type")]
    pub amendment_type: String,
    /// Currency for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_currency: Option<String>,
    /// Client-supplied metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateAmendmentRequest {
    /// Create a cancellation request.
    pub fn cancel(conversion_id: impl Into<String>, request_id: impl Into<String>) -> Self {
        Self {
            conversion_id: conversion_id.into(),
            request_id: request_id.into(),
            amendment_type: "CANCEL".to_string(),
            charge_currency: None,
            metadata: None,
        }
    }

    /// Set charge currency.
    pub fn charge_currency(mut self, currency: impl Into<String>) -> Self {
        self.charge_currency = Some(currency.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Parameters for listing conversion amendments.
#[derive(Debug, Clone, Serialize)]
pub struct ListAmendmentsParams {
    /// Conversion ID (required).
    pub conversion_id: String,
}

impl ListAmendmentsParams {
    /// Create new parameters.
    pub fn new(conversion_id: impl Into<String>) -> Self {
        Self {
            conversion_id: conversion_id.into(),
        }
    }
}

/// Response for listing conversion amendments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAmendmentsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of amendments.
    #[serde(default)]
    pub items: Vec<ConversionAmendment>,
}
