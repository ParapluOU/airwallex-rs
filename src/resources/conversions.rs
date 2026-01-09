//! Conversions resource for the Airwallex API.
//!
//! Manage foreign exchange conversions.

use crate::client::Client;
use crate::error::Result;
use crate::models::conversions::{
    Conversion, CreateConversionRequest, ListConversionsParams, ListConversionsResponse,
    RateQuote, RateQuoteRequest,
};

/// The Conversions resource.
pub struct Conversions<'a> {
    client: &'a Client,
}

impl<'a> Conversions<'a> {
    /// Create a new Conversions resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List conversions.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/conversions`
    pub async fn list(&self, params: ListConversionsParams) -> Result<ListConversionsResponse> {
        self.client
            .get_with_query("/api/v1/conversions", &params)
            .await
    }

    /// Create a conversion.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/conversions/create`
    pub async fn create(&self, request: CreateConversionRequest) -> Result<Conversion> {
        self.client
            .post("/api/v1/conversions/create", &request)
            .await
    }

    /// Get a conversion by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/conversions/{conversion_id}`
    pub async fn get(&self, conversion_id: &str) -> Result<Conversion> {
        self.client
            .get(&format!("/api/v1/conversions/{}", conversion_id))
            .await
    }

    /// Get a rate quote.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/rates/quote`
    pub async fn quote(&self, request: RateQuoteRequest) -> Result<RateQuote> {
        self.client.post("/api/v1/rates/quote", &request).await
    }
}
