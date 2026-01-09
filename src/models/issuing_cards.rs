//! Issuing Cards models.
//!
//! Models for managing Airwallex issued cards.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An issued card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingCard {
    /// Unique card identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// Masked card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// Card brand (e.g., VISA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Card status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status: Option<String>,
    /// Current card version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_version: Option<i32>,
    /// Form factor (PHYSICAL or VIRTUAL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// Whether card is personalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personalized: Option<bool>,
    /// Cardholder ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<String>,
    /// Additional cardholder IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_cardholder_ids: Option<Vec<String>>,
    /// Card nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Authorization controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<Value>,
    /// Alert settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_settings: Option<Value>,
    /// Delivery details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_details: Option<Value>,
    /// All card versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_card_versions: Option<Vec<Value>>,
    /// Client data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_data: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Created by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Sensitive card details (PAN, CVV, expiry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingCardDetails {
    /// Full card number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// CVV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv: Option<String>,
    /// Expiry month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<i32>,
    /// Expiry year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<i32>,
    /// Name on card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_on_card: Option<String>,
}

/// Card remaining limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLimits {
    /// Currency of transaction limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Remaining limits for intervals.
    #[serde(default)]
    pub limits: Vec<CardLimit>,
    /// Cash withdrawal limits.
    #[serde(default)]
    pub cash_withdrawal_limits: Vec<CardLimit>,
}

/// A single card limit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLimit {
    /// Transaction limit interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Transaction limit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Remaining amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
}

/// Request to create an issuing card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssuingCardRequest {
    /// Cardholder ID.
    pub cardholder_id: String,
    /// Form factor (PHYSICAL or VIRTUAL).
    pub form_factor: String,
    /// Whether personalized.
    pub is_personalized: bool,
    /// Creator name.
    pub created_by: String,
    /// Authorization controls.
    pub authorization_controls: Value,
    /// Card brand (default VISA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Card nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Activate on issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_on_issue: Option<bool>,
    /// Postal address for physical cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<Value>,
    /// Program details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<Value>,
    /// Alert settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_settings: Option<Value>,
    /// Client data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_data: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateIssuingCardRequest {
    /// Create a new issuing card request.
    pub fn new(
        cardholder_id: impl Into<String>,
        form_factor: impl Into<String>,
        is_personalized: bool,
        created_by: impl Into<String>,
        authorization_controls: Value,
    ) -> Self {
        Self {
            cardholder_id: cardholder_id.into(),
            form_factor: form_factor.into(),
            is_personalized,
            created_by: created_by.into(),
            authorization_controls,
            brand: None,
            nick_name: None,
            activate_on_issue: None,
            postal_address: None,
            program: None,
            alert_settings: None,
            client_data: None,
            metadata: None,
        }
    }

    /// Set card nickname.
    pub fn nick_name(mut self, name: impl Into<String>) -> Self {
        self.nick_name = Some(name.into());
        self
    }

    /// Set activate on issue.
    pub fn activate_on_issue(mut self, activate: bool) -> Self {
        self.activate_on_issue = Some(activate);
        self
    }
}

/// Request to update a card.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardRequest {
    /// Card status (INACTIVE, ACTIVE, CLOSED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status: Option<String>,
    /// Card nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Cardholder ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<String>,
    /// Authorization controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<Value>,
    /// Purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl UpdateCardRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set card status.
    pub fn card_status(mut self, status: impl Into<String>) -> Self {
        self.card_status = Some(status.into());
        self
    }

    /// Set card nickname.
    pub fn nick_name(mut self, name: impl Into<String>) -> Self {
        self.nick_name = Some(name.into());
        self
    }
}

/// Parameters for listing cards.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCardsParams {
    /// Filter by card status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status: Option<String>,
    /// Filter by cardholder ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<String>,
    /// Filter by nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// From created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// From updated_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_updated_at: Option<String>,
    /// To updated_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_updated_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListCardsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by card status.
    pub fn card_status(mut self, status: impl Into<String>) -> Self {
        self.card_status = Some(status.into());
        self
    }

    /// Filter by cardholder ID.
    pub fn cardholder_id(mut self, id: impl Into<String>) -> Self {
        self.cardholder_id = Some(id.into());
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

/// Response for listing cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCardsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of cards.
    #[serde(default)]
    pub items: Vec<IssuingCard>,
}
