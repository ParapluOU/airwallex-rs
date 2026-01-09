//! Financial Transactions resource for the Airwallex API.
//!
//! View financial transactions that contribute to account balance.

use crate::client::Client;
use crate::error::Result;
use crate::models::financial_transactions::{
    ListFinancialTransactionsParams, ListFinancialTransactionsResponse,
};

/// The Financial Transactions resource.
pub struct FinancialTransactions<'a> {
    client: &'a Client,
}

impl<'a> FinancialTransactions<'a> {
    /// Create a new FinancialTransactions resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List financial transactions.
    ///
    /// Returns transactions that contributed to the Airwallex account balance.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/financial_transactions`
    pub async fn list(
        &self,
        params: ListFinancialTransactionsParams,
    ) -> Result<ListFinancialTransactionsResponse> {
        self.client
            .get_with_query("/api/v1/financial_transactions", &params)
            .await
    }
}
