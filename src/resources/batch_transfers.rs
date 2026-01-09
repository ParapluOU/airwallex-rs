//! Batch Transfers resource for the Airwallex API.
//!
//! Manage batch transfers (bulk payouts).

use crate::client::Client;
use crate::error::Result;
use crate::models::batch_transfers::{
    AddBatchItemsRequest, BatchTransfer, CreateBatchTransferRequest, DeleteBatchItemsRequest,
    ListBatchItemsParams, ListBatchItemsResponse, ListBatchTransfersParams,
    ListBatchTransfersResponse,
};

/// The Batch Transfers resource.
pub struct BatchTransfers<'a> {
    client: &'a Client,
}

impl<'a> BatchTransfers<'a> {
    /// Create a new BatchTransfers resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a batch transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/create`
    pub async fn create(&self, request: CreateBatchTransferRequest) -> Result<BatchTransfer> {
        self.client
            .post("/api/v1/batch_transfers/create", &request)
            .await
    }

    /// List batch transfers.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/batch_transfers`
    pub async fn list(&self, params: ListBatchTransfersParams) -> Result<ListBatchTransfersResponse> {
        self.client
            .get_with_query("/api/v1/batch_transfers", &params)
            .await
    }

    /// Get a batch transfer by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/batch_transfers/{id}`
    pub async fn get(&self, id: &str) -> Result<BatchTransfer> {
        self.client
            .get(&format!("/api/v1/batch_transfers/{}", id))
            .await
    }

    /// Add items to a batch transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/{id}/add_items`
    pub async fn add_items(&self, id: &str, request: AddBatchItemsRequest) -> Result<BatchTransfer> {
        self.client
            .post(&format!("/api/v1/batch_transfers/{}/add_items", id), &request)
            .await
    }

    /// Delete items from a batch transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/{id}/delete_items`
    pub async fn delete_items(
        &self,
        id: &str,
        request: DeleteBatchItemsRequest,
    ) -> Result<BatchTransfer> {
        self.client
            .post(
                &format!("/api/v1/batch_transfers/{}/delete_items", id),
                &request,
            )
            .await
    }

    /// List items in a batch transfer.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/batch_transfers/{id}/items`
    pub async fn list_items(
        &self,
        id: &str,
        params: ListBatchItemsParams,
    ) -> Result<ListBatchItemsResponse> {
        self.client
            .get_with_query(&format!("/api/v1/batch_transfers/{}/items", id), &params)
            .await
    }

    /// Get a quote for a batch transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/{id}/quote`
    pub async fn quote(&self, id: &str) -> Result<BatchTransfer> {
        self.client
            .post_empty(&format!("/api/v1/batch_transfers/{}/quote", id))
            .await
    }

    /// Submit a batch transfer for processing.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/{id}/submit`
    pub async fn submit(&self, id: &str) -> Result<BatchTransfer> {
        self.client
            .post_empty(&format!("/api/v1/batch_transfers/{}/submit", id))
            .await
    }

    /// Delete a batch transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/batch_transfers/{id}/delete`
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client
            .post_empty_no_response(&format!("/api/v1/batch_transfers/{}/delete", id))
            .await
    }
}
