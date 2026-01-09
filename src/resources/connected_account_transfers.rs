//! Connected Account Transfers resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    ConnectedAccountTransfer, CreateConnectedAccountTransferRequest,
    ListConnectedAccountTransfersParams, ListConnectedAccountTransfersResponse,
};

/// Connected Account Transfers resource for transferring funds between accounts.
#[derive(Debug)]
pub struct ConnectedAccountTransfers<'a> {
    client: &'a Client,
}

impl<'a> ConnectedAccountTransfers<'a> {
    /// Create a new Connected Account Transfers resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new connected account transfer.
    pub async fn create(
        &self,
        request: &CreateConnectedAccountTransferRequest,
    ) -> Result<ConnectedAccountTransfer> {
        self.client
            .post("/api/v1/connected_account_transfers/create", request)
            .await
    }

    /// List connected account transfers.
    pub async fn list(
        &self,
        params: &ListConnectedAccountTransfersParams,
    ) -> Result<ListConnectedAccountTransfersResponse> {
        self.client
            .get_with_query("/api/v1/connected_account_transfers", params)
            .await
    }

    /// Get a connected account transfer by ID.
    pub async fn get(&self, id: &str) -> Result<ConnectedAccountTransfer> {
        self.client
            .get(&format!("/api/v1/connected_account_transfers/{}", id))
            .await
    }
}
