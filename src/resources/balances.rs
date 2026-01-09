//! Balances resource for the Airwallex API.
//!
//! The Balances API allows you to retrieve your current and historical balances.

use crate::client::Client;
use crate::error::Result;
use crate::models::balances::{
    BalanceHistoryParams, BalanceHistoryResponse, CurrentBalancesResponse,
};

/// The Balances resource.
///
/// # Example
///
/// ```no_run
/// # async fn example(client: &airwallex_rs::Client) -> airwallex_rs::Result<()> {
/// // Get current balances
/// let balances = client.balances().current().await?;
/// for balance in &balances.items {
///     println!("{}: {} available", balance.currency, balance.available_amount);
/// }
///
/// // Get balance history for USD
/// use airwallex_rs::models::BalanceHistoryParams;
/// let params = BalanceHistoryParams::new().currency("USD");
/// let history = client.balances().history(params).await?;
/// # Ok(())
/// # }
/// ```
pub struct Balances<'a> {
    client: &'a Client,
}

impl<'a> Balances<'a> {
    /// Create a new Balances resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get current balances for all currencies.
    ///
    /// Returns available and pending balances for each currency in your account.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/balances/current`
    pub async fn current(&self) -> Result<CurrentBalancesResponse> {
        self.client.get("/api/v1/balances/current").await
    }

    /// Get balance history.
    ///
    /// Returns a list of balance changes based on the provided filters.
    /// The date range should be within 7 days when using `page_num` pagination.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/balances/history`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(client: &airwallex_rs::Client) -> airwallex_rs::Result<()> {
    /// use airwallex_rs::models::BalanceHistoryParams;
    ///
    /// let params = BalanceHistoryParams::new()
    ///     .currency("USD")
    ///     .page_size(50);
    ///
    /// let history = client.balances().history(params).await?;
    /// for entry in &history.items {
    ///     println!("{}: {} {}", entry.posted_at, entry.amount, entry.currency);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn history(&self, params: BalanceHistoryParams) -> Result<BalanceHistoryResponse> {
        self.client
            .get_with_query("/api/v1/balances/history", &params)
            .await
    }
}
