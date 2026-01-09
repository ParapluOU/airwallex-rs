//! Reference Data resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::SupportedCurrencies;

/// Reference Data resource for retrieving reference information.
#[derive(Debug)]
pub struct ReferenceData<'a> {
    client: &'a Client,
}

impl<'a> ReferenceData<'a> {
    /// Create a new Reference Data resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get supported currencies for conversions.
    ///
    /// Returns the client-specific list of currencies that can be bought or sold.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/reference/supported_currencies`
    pub async fn supported_currencies(&self) -> Result<SupportedCurrencies> {
        self.client
            .get("/api/v1/reference/supported_currencies")
            .await
    }
}
