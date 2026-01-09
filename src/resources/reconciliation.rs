//! Reconciliation resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{ListTreasuryBalancesParams, ListTreasuryBalancesResponse};

/// Reconciliation resource for treasury/balance data.
#[derive(Debug)]
pub struct Reconciliation<'a> {
    client: &'a Client,
}

impl<'a> Reconciliation<'a> {
    /// Create a new Reconciliation resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get treasury balances.
    ///
    /// Query balances by client_request_id, currency, and/or posted_at date range.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/tc/balances`
    pub async fn balances(
        &self,
        params: &ListTreasuryBalancesParams,
    ) -> Result<ListTreasuryBalancesResponse> {
        self.client
            .get_with_query("/api/v1/tc/balances", params)
            .await
    }
}
