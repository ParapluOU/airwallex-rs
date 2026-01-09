//! Deposits resource for the Airwallex API.
//!
//! Track incoming deposits to your global accounts.

use crate::client::Client;
use crate::error::Result;
use crate::models::deposits::{ListDepositsParams, ListDepositsResponse};

/// The Deposits resource.
pub struct Deposits<'a> {
    client: &'a Client,
}

impl<'a> Deposits<'a> {
    /// Create a new Deposits resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List deposits.
    ///
    /// Returns deposits received on your global accounts.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/deposits`
    pub async fn list(&self, params: ListDepositsParams) -> Result<ListDepositsResponse> {
        self.client
            .get_with_query("/api/v1/deposits", &params)
            .await
    }
}
