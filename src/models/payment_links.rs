//! Payment Link models.
//!
//! Models for managing payment links (hosted checkout pages).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payment link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLink {
    /// Unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Title displayed on checkout page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payment URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the link is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Whether reusable (multiple payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    /// Status (UNPAID, PAID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Amount for fixed pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency for fixed pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Default currency for flexible pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    /// Supported currencies for flexible pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_currencies: Option<Vec<String>>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Merchant reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Expiry timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Collectable shopper info settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectable_shopper_info: Option<CollectableShopperInfo>,
    /// Successful payment count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment_intent_count: Option<i64>,
    /// Latest successful payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_successful_payment_intent_id: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Collectable shopper information settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectableShopperInfo {
    /// Require billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<bool>,
    /// Require shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<bool>,
    /// Require phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<bool>,
    /// Allow message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<bool>,
    /// Require reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<bool>,
}

/// Request to create a payment link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentLinkRequest {
    /// Title displayed on checkout page.
    pub title: String,
    /// Whether reusable.
    pub reusable: bool,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount for fixed pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency for fixed pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Default currency for flexible pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    /// Supported currencies for flexible pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_currencies: Option<Vec<String>>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Merchant reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Expiry timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Connected account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    /// Collectable shopper info settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectable_shopper_info: Option<CollectableShopperInfo>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreatePaymentLinkRequest {
    /// Create a fixed-price payment link.
    pub fn fixed(title: impl Into<String>, amount: f64, currency: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            reusable: false,
            description: None,
            amount: Some(amount),
            currency: Some(currency.into()),
            default_currency: None,
            supported_currencies: None,
            customer_id: None,
            reference: None,
            expires_at: None,
            connected_account_id: None,
            collectable_shopper_info: None,
            metadata: None,
        }
    }

    /// Create a flexible-price payment link.
    pub fn flexible(
        title: impl Into<String>,
        default_currency: impl Into<String>,
        supported_currencies: Vec<String>,
    ) -> Self {
        Self {
            title: title.into(),
            reusable: false,
            description: None,
            amount: None,
            currency: None,
            default_currency: Some(default_currency.into()),
            supported_currencies: Some(supported_currencies),
            customer_id: None,
            reference: None,
            expires_at: None,
            connected_account_id: None,
            collectable_shopper_info: None,
            metadata: None,
        }
    }

    /// Set as reusable (multiple payments).
    pub fn reusable(mut self, reusable: bool) -> Self {
        self.reusable = reusable;
        self
    }

    /// Set description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set customer ID.
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Set reference.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set expiry.
    pub fn expires_at(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    /// Set collectable shopper info.
    pub fn collectable_shopper_info(mut self, info: CollectableShopperInfo) -> Self {
        self.collectable_shopper_info = Some(info);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to update a payment link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePaymentLinkRequest {
    /// Title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Expiry timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl UpdatePaymentLinkRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set expiry.
    pub fn expires_at(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }
}

/// Request to notify shopper about payment link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyShopperRequest {
    /// Shopper email.
    pub email: String,
    /// Shopper name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NotifyShopperRequest {
    /// Create a new notify request.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: None,
        }
    }

    /// Set shopper name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Parameters for listing payment links.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPaymentLinksParams {
    /// Filter by active status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Filter by reusable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// From created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPaymentLinksParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by active status.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Filter by reusable.
    pub fn reusable(mut self, reusable: bool) -> Self {
        self.reusable = Some(reusable);
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
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

/// Response for listing payment links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentLinksResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment links.
    #[serde(default)]
    pub items: Vec<PaymentLink>,
}
