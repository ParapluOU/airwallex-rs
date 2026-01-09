//! Payment Dispute models.
//!
//! Models for managing payment disputes (chargebacks, RFIs).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A payment dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDispute {
    /// PaymentDispute ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// PaymentDispute amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// PaymentDispute currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// PaymentDispute stage (RFI, PRE_CHARGEBACK, CHARGEBACK, PRE_ARBITRATION, ARBITRATION).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// PaymentDispute status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// PaymentDispute mode (ALLOCATION, COLLABORATION).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// PaymentDispute reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<DisputeReason>,
    /// PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// PaymentAttempt ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_attempt_id: Option<String>,
    /// Customer ID of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Customer name of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// Order ID of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Payment method type of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Card brand of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Acquirer reference number of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer_reference_number: Option<String>,
    /// Transaction type (PAYMENT, REFUND).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Due date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    /// Issuer comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_comment: Option<String>,
    /// Issuer document file IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_documents: Option<Vec<String>>,
    /// Accept details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_details: Option<Vec<AcceptDetail>>,
    /// Challenge details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_details: Option<Vec<ChallengeDetail>>,
    /// Refunds of original payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<DisputeRefund>>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Dispute reason.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeReason {
    /// Reason type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub reason_type: Option<String>,
    /// Reason code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_code: Option<String>,
    /// Reason description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Accept detail for a dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptDetail {
    /// Stage when accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// Accept reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Accept description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Accepted by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_by: Option<String>,
    /// Accepted at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    /// Refund details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<AcceptRefund>,
}

/// Refund when accepting a dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptRefund {
    /// Refund amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Refund reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Challenge detail for a dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeDetail {
    /// Stage when challenged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// Challenge reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Challenged by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenged_by: Option<String>,
    /// Challenged at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenged_at: Option<String>,
    /// Product type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// Refund refusal reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_reason: Option<String>,
    /// Customer info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_info: Option<Value>,
    /// Delivery info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_info: Option<Value>,
    /// Order info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Value>,
    /// Seller info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_info: Option<Value>,
    /// Supporting documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_documents: Option<Value>,
}

/// Refund associated with a dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeRefund {
    /// Refund ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Acquirer reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer_reference_number: Option<String>,
}

/// Request to accept a payment dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptDisputeRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Accept reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Accept description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User ID accepting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_by: Option<String>,
    /// Refund details for RFI stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<AcceptRefund>,
}

impl AcceptDisputeRequest {
    /// Create a new accept dispute request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            reason: None,
            description: None,
            accepted_by: None,
            refund: None,
        }
    }

    /// Set the accept reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Set the accept description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Request to challenge a payment dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeDisputeRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Challenge reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Product type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// Refund refusal reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_reason: Option<String>,
    /// Challenged by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenged_by: Option<String>,
    /// Customer info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_info: Option<Value>,
    /// Delivery info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_info: Option<Value>,
    /// Order info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Value>,
    /// Seller info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_info: Option<Value>,
    /// Supporting documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_documents: Option<Value>,
}

impl ChallengeDisputeRequest {
    /// Create a new challenge dispute request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            reason: None,
            product_type: None,
            product_description: None,
            refund_refusal_reason: None,
            challenged_by: None,
            customer_info: None,
            delivery_info: None,
            order_info: None,
            seller_info: None,
            supporting_documents: None,
        }
    }

    /// Set the challenge reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Set the product type.
    pub fn product_type(mut self, product_type: impl Into<String>) -> Self {
        self.product_type = Some(product_type.into());
        self
    }

    /// Set supporting documents.
    pub fn supporting_documents(mut self, documents: Value) -> Self {
        self.supporting_documents = Some(documents);
        self
    }
}

/// Parameters for listing payment disputes.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPaymentDisputesParams {
    /// Filter by stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by reason code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// Filter by payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    /// Filter by payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Filter by customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Filter by customer name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// Filter by merchant order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_id: Option<String>,
    /// Filter by transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// From due_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_due_at: Option<String>,
    /// To due_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_due_at: Option<String>,
    /// From updated_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_updated_at: Option<String>,
    /// To updated_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_updated_at: Option<String>,
    /// Page cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListPaymentDisputesParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by stage.
    pub fn stage(mut self, stage: impl Into<String>) -> Self {
        self.stage = Some(stage.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set page size.
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
}

/// Response for listing payment disputes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentDisputesResponse {
    /// List of payment disputes.
    #[serde(default)]
    pub items: Vec<PaymentDispute>,
    /// Cursor for next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Cursor for previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
}
