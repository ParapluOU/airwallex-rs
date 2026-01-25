//! Invoices resource for the Airwallex API.
//!
//! Manage invoices for recurring payments.

use crate::client::Client;
use crate::error::Result;
use crate::models::invoices::{
    Invoice, InvoiceItem, InvoicePreviewRequest, InvoicePreviewResponse, ListInvoiceItemsResponse,
    ListInvoicesParams, ListInvoicesResponse,
};

/// The Invoices resource.
pub struct Invoices<'a> {
    client: &'a Client,
}

impl<'a> Invoices<'a> {
    /// Create a new Invoices resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List invoices.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/invoices`
    pub async fn list(&self, params: ListInvoicesParams) -> Result<ListInvoicesResponse> {
        self.client
            .get_with_query("/api/v1/invoices", &params)
            .await
    }

    /// Get an invoice by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/invoices/{id}`
    pub async fn get(&self, id: &str) -> Result<Invoice> {
        self.client.get(&format!("/api/v1/invoices/{}", id)).await
    }

    /// List items for an invoice.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/invoices/{id}/items`
    pub async fn list_items(&self, invoice_id: &str) -> Result<ListInvoiceItemsResponse> {
        self.client
            .get(&format!("/api/v1/invoices/{}/items", invoice_id))
            .await
    }

    /// Get an invoice item by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/invoices/{id}/items/{item_id}`
    pub async fn get_item(&self, invoice_id: &str, item_id: &str) -> Result<InvoiceItem> {
        self.client
            .get(&format!(
                "/api/v1/invoices/{}/items/{}",
                invoice_id, item_id
            ))
            .await
    }

    /// Preview an invoice.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/invoices/preview`
    pub async fn preview(&self, request: InvoicePreviewRequest) -> Result<InvoicePreviewResponse> {
        self.client.post("/api/v1/invoices/preview", &request).await
    }
}
