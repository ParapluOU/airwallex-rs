//! Payment Attempts models.
//!
//! Models for retrieving payment attempt information.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Status of a payment attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentAttemptStatus {
    /// The PaymentAttempt has been created.
    Received,
    /// Waiting for customer authentication.
    AuthenticationRedirected,
    /// Authorization request pending.
    PendingAuthorization,
    /// Successfully authorized.
    Authorized,
    /// Capture requested.
    CaptureRequested,
    /// Expired.
    Expired,
    /// Cancelled.
    Cancelled,
    /// Failed.
    Failed,
    /// Settlement received.
    Settled,
    /// Funds settled.
    Paid,
}

/// Failure code for a payment attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentAttemptFailureCode {
    /// Authentication failed.
    AuthenticationFailed,
    /// Capture failed.
    CaptureFailed,
    /// Authorization failed.
    AuthorizationFailed,
    /// Provider unavailable.
    ProviderUnavailable,
    /// System unavailable.
    SystemUnavailable,
    /// Fraud rejected.
    FraudRejected,
}

/// DCC (Dynamic Currency Conversion) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DccData {
    /// DCC amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// DCC currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Failure details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAttemptFailureDetails {
    /// Failure code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Failure message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Trace ID for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// Additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Authentication data (3DS, fraud, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationData {
    /// Authentication type (3ds, passkey, none).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// AVS result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_result: Option<String>,
    /// CVC result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_result: Option<String>,
    /// 3DS data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_data: Option<Value>,
    /// Fraud data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_data: Option<Value>,
    /// Passkey setup status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey_setup_status: Option<String>,
}

/// A payment attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAttempt {
    /// Unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentAttemptStatus>,
    /// Associated payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Associated payment consent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_consent_id: Option<String>,
    /// Merchant order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Captured amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_amount: Option<f64>,
    /// Refunded amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<f64>,
    /// Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    /// Acquirer reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer_reference_number: Option<String>,
    /// Failure code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<PaymentAttemptFailureCode>,
    /// Failure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<PaymentAttemptFailureDetails>,
    /// Authentication data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_data: Option<AuthenticationData>,
    /// DCC data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcc_data: Option<DccData>,
    /// Payment method (complex object - stored as Value for flexibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Value>,
    /// Payment method transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_transaction_id: Option<String>,
    /// Provider original response code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_original_response_code: Option<String>,
    /// Merchant advice code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_advice_code: Option<String>,
    /// Settlement channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_via: Option<String>,
    /// Creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Parameters for listing payment attempts.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPaymentAttemptsParams {
    /// Filter by payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start time of created_at range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End time of created_at range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starting from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size (default 10, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPaymentAttemptsParams {
    /// Create new params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by payment intent ID.
    pub fn payment_intent_id(mut self, id: impl Into<String>) -> Self {
        self.payment_intent_id = Some(id.into());
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

    /// Set start time.
    pub fn from_created_at(mut self, time: impl Into<String>) -> Self {
        self.from_created_at = Some(time.into());
        self
    }

    /// Set end time.
    pub fn to_created_at(mut self, time: impl Into<String>) -> Self {
        self.to_created_at = Some(time.into());
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

/// Response for listing payment attempts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentAttemptsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment attempts.
    #[serde(default)]
    pub items: Vec<PaymentAttempt>,
}
