//! Transfers resource for the Airwallex API.
//!
//! Manage payout transfers (sending payments to beneficiaries).

use crate::client::Client;
use crate::error::Result;
use crate::models::transfers::{
    CreateTransferRequest, ListTransfersParams, ListTransfersResponse, Transfer,
};

/// The Transfers resource.
pub struct Transfers<'a> {
    client: &'a Client,
}

impl<'a> Transfers<'a> {
    /// Create a new Transfers resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List transfers.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/transfers`
    pub async fn list(&self, params: ListTransfersParams) -> Result<ListTransfersResponse> {
        self.client
            .get_with_query("/api/v1/transfers", &params)
            .await
    }

    /// Create a transfer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/transfers/create`
    pub async fn create(&self, request: CreateTransferRequest) -> Result<Transfer> {
        self.client.post("/api/v1/transfers/create", &request).await
    }

    /// Get a transfer by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/transfers/{id}`
    pub async fn get(&self, id: &str) -> Result<Transfer> {
        self.client.get(&format!("/api/v1/transfers/{}", id)).await
    }
}
