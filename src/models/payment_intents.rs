//! Payment Intent models.
//!
//! Models for managing payment intents (the core of payment acceptance).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payment intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    /// Payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Payment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Payment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Captured amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_amount: Option<f64>,
    /// Merchant order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Descriptor shown to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Customer details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Value>,
    /// Invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Client secret for frontend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Latest payment attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_payment_attempt: Option<Value>,
    /// Next action required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<Value>,
    /// Cancellation reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Cancelled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Connected account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    /// Conversion quote ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_quote_id: Option<String>,
}

/// Request to create a payment intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntentRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Payment amount.
    pub amount: f64,
    /// Currency (3-letter ISO-4217).
    pub currency: String,
    /// Merchant order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Descriptor shown to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Order details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Value>,
    /// Payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Value>,
    /// Return URL after payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreatePaymentIntentRequest {
    /// Create a new request.
    pub fn new(request_id: impl Into<String>, amount: f64, currency: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            amount,
            currency: currency.into(),
            merchant_order_id: None,
            descriptor: None,
            customer_id: None,
            order: None,
            payment_method_options: None,
            return_url: None,
            metadata: None,
        }
    }

    /// Set merchant order ID.
    pub fn merchant_order_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_order_id = Some(id.into());
        self
    }

    /// Set descriptor.
    pub fn descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.descriptor = Some(descriptor.into());
        self
    }

    /// Set customer ID.
    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }

    /// Set return URL.
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to confirm a payment intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmPaymentIntentRequest {
    /// Payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Payment method ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Payment consent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_consent_id: Option<String>,
    /// Return URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Device data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_data: Option<Value>,
}

impl ConfirmPaymentIntentRequest {
    /// Create a new confirm request.
    pub fn new() -> Self {
        Self {
            payment_method: None,
            payment_method_id: None,
            payment_consent_id: None,
            return_url: None,
            device_data: None,
        }
    }

    /// Set payment method ID.
    pub fn payment_method_id(mut self, id: impl Into<String>) -> Self {
        self.payment_method_id = Some(id.into());
        self
    }

    /// Set payment consent ID.
    pub fn payment_consent_id(mut self, id: impl Into<String>) -> Self {
        self.payment_consent_id = Some(id.into());
        self
    }

    /// Set return URL.
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }
}

impl Default for ConfirmPaymentIntentRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to capture a payment intent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapturePaymentIntentRequest {
    /// Amount to capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

impl CapturePaymentIntentRequest {
    /// Create a new capture request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set amount to capture.
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
}

/// Request to cancel a payment intent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelPaymentIntentRequest {
    /// Cancellation reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
}

impl CancelPaymentIntentRequest {
    /// Create a new cancel request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set cancellation reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.cancellation_reason = Some(reason.into());
        self
    }
}

/// Parameters for listing payment intents.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPaymentIntentsParams {
    /// Filter by customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by merchant order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Start date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPaymentIntentsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by customer ID.
    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
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

/// Response for listing payment intents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentIntentsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment intents.
    #[serde(default)]
    pub items: Vec<PaymentIntent>,
}
