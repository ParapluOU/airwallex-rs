//! Issuing Transactions resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    IssuingTransaction, ListIssuingTransactionsParams, ListIssuingTransactionsResponse,
};

/// Issuing Transactions resource for viewing card transactions.
#[derive(Debug)]
pub struct IssuingTransactions<'a> {
    client: &'a Client,
}

impl<'a> IssuingTransactions<'a> {
    /// Create a new Issuing Transactions resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List issuing transactions.
    ///
    /// Defaults to a 30 day period unless both from_created_at and to_created_at are provided.
    pub async fn list(
        &self,
        params: &ListIssuingTransactionsParams,
    ) -> Result<ListIssuingTransactionsResponse> {
        self.client
            .get_with_query("/api/v1/issuing/transactions", params)
            .await
    }

    /// Get a transaction by ID.
    pub async fn get(&self, id: &str) -> Result<IssuingTransaction> {
        self.client
            .get(&format!("/api/v1/issuing/transactions/{}", id))
            .await
    }
}
