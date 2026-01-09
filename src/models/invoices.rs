//! Invoice models.
//!
//! Models for managing invoices for recurring payments.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Invoice status (SENT, PAID, PAYMENT_FAILED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Currency (3-letter ISO-4217).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Associated payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Billing period start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start_at: Option<String>,
    /// Billing period end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end_at: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Paid timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    /// Last payment attempt timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_attempt_at: Option<String>,
    /// Next payment attempt timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt_at: Option<String>,
    /// Number of past payment attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_payment_attempt_count: Option<i32>,
    /// Remaining payment attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_payment_attempt_count: Option<i32>,
}

/// An invoice item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    /// Item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Unit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<f64>,
    /// Period start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start_at: Option<String>,
    /// Period end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end_at: Option<String>,
    /// Price ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    /// Subscription item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item_id: Option<String>,
}

/// Parameters for listing invoices.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListInvoicesParams {
    /// Filter by customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Filter by subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListInvoicesParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by customer ID.
    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }

    /// Filter by subscription ID.
    pub fn subscription_id(mut self, id: impl Into<String>) -> Self {
        self.subscription_id = Some(id.into());
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

/// Response for listing invoices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvoicesResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of invoices.
    #[serde(default)]
    pub items: Vec<Invoice>,
}

/// Response for listing invoice items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvoiceItemsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of items.
    #[serde(default)]
    pub items: Vec<InvoiceItem>,
}

/// Request for invoice preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewRequest {
    /// Customer ID.
    pub customer_id: String,
    /// Currency.
    pub currency: String,
    /// Items to preview.
    pub items: Vec<InvoicePreviewItem>,
}

/// An item in an invoice preview request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewItem {
    /// Price ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    /// Quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

/// Invoice preview response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewResponse {
    /// Total amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// Subtotal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Line items.
    #[serde(default)]
    pub items: Vec<Value>,
}
