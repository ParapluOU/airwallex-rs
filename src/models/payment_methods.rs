//! Payment Method models.
//!
//! Models for managing payment methods (cards, direct debits, digital wallets).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The customer this payment method belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Type of payment method (card, applepay, googlepay, ach_direct_debit, etc.).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// Status (CREATED, DISABLED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Card details (if type is card).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,
    /// Apple Pay details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applepay: Option<Value>,
    /// Google Pay details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub googlepay: Option<Value>,
    /// ACH Direct Debit details (US).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_direct_debit: Option<Value>,
    /// BACS Direct Debit details (UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_direct_debit: Option<Value>,
    /// BECS Direct Debit details (Australia).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub becs_direct_debit: Option<Value>,
    /// SEPA Direct Debit details (EU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_direct_debit: Option<Value>,
    /// EFT Direct Debit details (Canada).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eft_direct_debit: Option<Value>,
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

/// Card details for a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    /// Card brand (visa, mastercard, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Last 4 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// BIN (Bank Identification Number).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    /// Card fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Expiry month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,
    /// Expiry year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Card type (credit, debit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// Number type (PAN, EXTERNAL_NETWORK_TOKEN, AIRWALLEX_NETWORK_TOKEN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    /// Whether the card is commercial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_commercial: Option<bool>,
    /// Issuer country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_country_code: Option<String>,
    /// Issuer name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_name: Option<String>,
    /// Billing details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingDetails>,
    /// Lifecycle ID (for MasterCard).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_id: Option<String>,
}

/// Billing details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BillingAddress>,
}

/// Billing address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    /// Street.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postcode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// Country code (ISO 3166-1 alpha-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// Request to create a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Customer ID this payment method belongs to.
    pub customer_id: String,
    /// Type of payment method (card, applepay, googlepay).
    #[serde(rename = "type")]
    pub payment_type: String,
    /// Card details (for card type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCardRequest>,
    /// Apple Pay details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applepay: Option<Value>,
    /// Google Pay details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub googlepay: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Card details for creating a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardRequest {
    /// Card number.
    pub number: String,
    /// Expiry month (2 digits).
    pub expiry_month: String,
    /// Expiry year (4 digits).
    pub expiry_year: String,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// CVC/CVV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Number type (PAN, EXTERNAL_NETWORK_TOKEN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    /// Billing details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingDetails>,
}

impl CreatePaymentMethodRequest {
    /// Create a new card payment method request.
    pub fn card(
        request_id: impl Into<String>,
        customer_id: impl Into<String>,
        card: CreateCardRequest,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            customer_id: customer_id.into(),
            payment_type: "card".to_string(),
            card: Some(card),
            applepay: None,
            googlepay: None,
            metadata: None,
        }
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl CreateCardRequest {
    /// Create a new card request.
    pub fn new(
        number: impl Into<String>,
        expiry_month: impl Into<String>,
        expiry_year: impl Into<String>,
    ) -> Self {
        Self {
            number: number.into(),
            expiry_month: expiry_month.into(),
            expiry_year: expiry_year.into(),
            name: None,
            cvc: None,
            number_type: None,
            billing: None,
        }
    }

    /// Set cardholder name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set CVC.
    pub fn cvc(mut self, cvc: impl Into<String>) -> Self {
        self.cvc = Some(cvc.into());
        self
    }

    /// Set billing details.
    pub fn billing(mut self, billing: BillingDetails) -> Self {
        self.billing = Some(billing);
        self
    }
}

/// Request to disable a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisablePaymentMethodRequest {
    /// Unique request ID.
    pub request_id: String,
}

impl DisablePaymentMethodRequest {
    /// Create a new disable request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}

/// Parameters for listing payment methods.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPaymentMethodsParams {
    /// Customer ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Status filter (CREATED, DISABLED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type filter (card, ach_direct_debit, etc.).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
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

impl ListPaymentMethodsParams {
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

    /// Filter by type.
    pub fn payment_type(mut self, payment_type: impl Into<String>) -> Self {
        self.payment_type = Some(payment_type.into());
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

/// Response for listing payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentMethodsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payment methods.
    #[serde(default)]
    pub items: Vec<PaymentMethod>,
}
