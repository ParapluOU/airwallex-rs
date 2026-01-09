//! Batch Transfer models.
//!
//! Models for managing batch transfers (bulk payouts).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A batch transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTransfer {
    /// Batch transfer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Batch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Short reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_reference_id: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Transfer date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<String>,
    /// Total item count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i32>,
    /// Valid item count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_item_count: Option<i32>,
    /// Remarks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Funding details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<BatchFunding>,
    /// Quote summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_summary: Option<BatchQuoteSummary>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Batch funding details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFunding {
    /// Funding source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source_id: Option<String>,
    /// Deposit type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_type: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Failure reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Quote summary for a batch transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQuoteSummary {
    /// When the quote expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Last quoted timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_quoted_at: Option<String>,
    /// Individual quotes.
    #[serde(default)]
    pub quotes: Vec<BatchQuote>,
}

/// Individual quote within a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQuote {
    /// Source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// Transfer currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_currency: Option<String>,
    /// Amount payer pays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_payer_pays: Option<f64>,
    /// Amount beneficiary receives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_beneficiary_receives: Option<f64>,
    /// Client rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_rate: Option<f64>,
    /// Currency pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Fee amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// Fee currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
}

/// A batch transfer item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTransferItem {
    /// Item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Beneficiary ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    /// Source amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<f64>,
    /// Source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// Transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
    /// Transfer currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_currency: Option<String>,
    /// Transfer method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_method: Option<String>,
    /// Reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Validation errors.
    #[serde(default)]
    pub errors: Vec<Value>,
}

/// Request to create a batch transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchTransferRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Batch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Transfer date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<String>,
    /// Remarks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Funding source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<FundingSource>,
}

/// Funding source configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingSource {
    /// Linked account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Deposit type (DIRECT_DEBIT or FASTER_DIRECT_DEBIT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_type: Option<String>,
    /// Reference for bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl CreateBatchTransferRequest {
    /// Create a new request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            name: None,
            transfer_date: None,
            remarks: None,
            metadata: None,
            funding_source: None,
        }
    }

    /// Set batch name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set transfer date.
    pub fn transfer_date(mut self, date: impl Into<String>) -> Self {
        self.transfer_date = Some(date.into());
        self
    }

    /// Set remarks.
    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to add items to a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBatchItemsRequest {
    /// Items to add.
    pub items: Vec<BatchTransferItemRequest>,
}

/// A batch transfer item request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTransferItemRequest {
    /// Beneficiary ID.
    pub beneficiary_id: String,
    /// Source amount (mutually exclusive with transfer_amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<f64>,
    /// Source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// Transfer amount (mutually exclusive with source_amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
    /// Transfer currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_currency: Option<String>,
    /// Transfer method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_method: Option<String>,
    /// Reference for beneficiary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Reason for transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl BatchTransferItemRequest {
    /// Create a new item with source amount.
    pub fn with_source_amount(
        beneficiary_id: impl Into<String>,
        source_amount: f64,
        source_currency: impl Into<String>,
    ) -> Self {
        Self {
            beneficiary_id: beneficiary_id.into(),
            source_amount: Some(source_amount),
            source_currency: Some(source_currency.into()),
            transfer_amount: None,
            transfer_currency: None,
            transfer_method: None,
            reference: None,
            reason: None,
        }
    }

    /// Create a new item with transfer amount.
    pub fn with_transfer_amount(
        beneficiary_id: impl Into<String>,
        transfer_amount: f64,
        transfer_currency: impl Into<String>,
    ) -> Self {
        Self {
            beneficiary_id: beneficiary_id.into(),
            source_amount: None,
            source_currency: None,
            transfer_amount: Some(transfer_amount),
            transfer_currency: Some(transfer_currency.into()),
            transfer_method: None,
            reference: None,
            reason: None,
        }
    }

    /// Set transfer method.
    pub fn transfer_method(mut self, method: impl Into<String>) -> Self {
        self.transfer_method = Some(method.into());
        self
    }

    /// Set reference.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

/// Request to delete items from a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBatchItemsRequest {
    /// Item IDs to delete.
    pub item_ids: Vec<String>,
}

/// Parameters for listing batch transfers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListBatchTransfersParams {
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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

impl ListBatchTransfersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Response for listing batch transfers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBatchTransfersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of batch transfers.
    #[serde(default)]
    pub items: Vec<BatchTransfer>,
}

/// Parameters for listing batch transfer items.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListBatchItemsParams {
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListBatchItemsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Response for listing batch transfer items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBatchItemsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of items.
    #[serde(default)]
    pub items: Vec<BatchTransferItem>,
}
