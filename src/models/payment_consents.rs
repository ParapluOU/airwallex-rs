//! Payment Consent models.
//!
//! Models for managing payment consents (recurring payments, subscriptions).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConsent {
    /// Unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Status (REQUIRES_PAYMENT_METHOD, REQUIRES_CUSTOMER_ACTION, PENDING, VERIFIED, DISABLED, PAUSED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Party to trigger subsequent payments (merchant, customer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_triggered_by: Option<String>,
    /// Merchant trigger reason (scheduled, unscheduled, installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trigger_reason: Option<String>,
    /// Client secret for browser/app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Connected account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    /// Payment method details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Next action required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,
    /// Terms of use for the consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<TermsOfUse>,
    /// Mandate information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Value>,
    /// Initial payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_payment_intent_id: Option<String>,
    /// Failure reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    /// Disable reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_reason: Option<String>,
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

/// Next action for a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction {
    /// Action type (redirect, redirect_iframe, notify_micro_deposits, retry_micro_debit).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// Redirect URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Redirect method (GET, POST).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Content type for POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Additional data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    /// Fallback URL for mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<String>,
    /// Email for micro deposit verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Number of micro deposits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_deposit_count: Option<i32>,
    /// Remaining verification attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_attempts: Option<i32>,
    /// QR code text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrcode: Option<String>,
    /// Stage of the request flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// Terms of use for a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsOfUse {
    /// Payment amount type (FIXED, VARIABLE).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_amount_type: Option<String>,
    /// Payment currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_currency: Option<String>,
    /// Fixed payment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_payment_amount: Option<f64>,
    /// First payment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_payment_amount: Option<f64>,
    /// Maximum payment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payment_amount: Option<f64>,
    /// Minimum payment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_payment_amount: Option<f64>,
    /// Start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Total billing cycles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_billing_cycles: Option<i32>,
    /// Billing cycle charge day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_charge_day: Option<i32>,
    /// Payment schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<PaymentSchedule>,
}

/// Payment schedule for a consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSchedule {
    /// Period count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// Period unit (DAY, WEEK, MONTH, YEAR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
}

/// Failure reason for a consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureReason {
    /// Failure code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Failure message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Failure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Request to create a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentConsentRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Customer ID.
    pub customer_id: String,
    /// Party to trigger subsequent payments (merchant, customer).
    pub next_triggered_by: String,
    /// Merchant trigger reason (scheduled, unscheduled, installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trigger_reason: Option<String>,
    /// Connected account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    /// Terms of use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<TermsOfUse>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreatePaymentConsentRequest {
    /// Create a merchant-initiated payment consent.
    pub fn merchant_initiated(
        request_id: impl Into<String>,
        customer_id: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            customer_id: customer_id.into(),
            next_triggered_by: "merchant".to_string(),
            merchant_trigger_reason: Some("unscheduled".to_string()),
            connected_account_id: None,
            terms_of_use: None,
            metadata: None,
        }
    }

    /// Create a customer-initiated payment consent.
    pub fn customer_initiated(
        request_id: impl Into<String>,
        customer_id: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            customer_id: customer_id.into(),
            next_triggered_by: "customer".to_string(),
            merchant_trigger_reason: None,
            connected_account_id: None,
            terms_of_use: None,
            metadata: None,
        }
    }

    /// Set merchant trigger reason.
    pub fn merchant_trigger_reason(mut self, reason: impl Into<String>) -> Self {
        self.merchant_trigger_reason = Some(reason.into());
        self
    }

    /// Set terms of use.
    pub fn terms_of_use(mut self, terms: TermsOfUse) -> Self {
        self.terms_of_use = Some(terms);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to verify a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyPaymentConsentRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Payment method details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Return URL after verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Verification options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_options: Option<Value>,
    /// Device data for risk assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_data: Option<Value>,
    /// Descriptor shown during verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    /// Risk control options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_control_options: Option<Value>,
}

impl VerifyPaymentConsentRequest {
    /// Create a new verify request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            payment_method: None,
            return_url: None,
            verification_options: None,
            device_data: None,
            descriptor: None,
            risk_control_options: None,
        }
    }

    /// Set payment method details.
    pub fn payment_method(mut self, payment_method: Value) -> Self {
        self.payment_method = Some(payment_method);
        self
    }

    /// Set return URL.
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }

    /// Set verification options.
    pub fn verification_options(mut self, options: Value) -> Self {
        self.verification_options = Some(options);
        self
    }
}

/// Request to update a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentConsentRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Metadata to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl UpdatePaymentConsentRequest {
    /// Create a new update request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            metadata: None,
        }
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to disable a payment consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisablePaymentConsentRequest {
    /// Unique request ID.
    pub request_id: String,
}

impl DisablePaymentConsentRequest {
    /// Create a new disable request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}

/// Parameters for listing payment consents.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPaymentConsentsParams {
    /// Customer ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Payment method ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Next triggered by filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_triggered_by: Option<String>,
    /// Merchant trigger reason filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trigger_reason: Option<String>,
    /// From created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// To created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starting from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPaymentConsentsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by customer ID.
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
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

/// Response for listing payment consents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentConsentsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment consents.
    #[serde(default)]
    pub items: Vec<PaymentConsent>,
}
