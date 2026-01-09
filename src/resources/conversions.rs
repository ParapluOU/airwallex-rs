//! Conversions resource for the Airwallex API.
//!
//! Manage foreign exchange conversions.

use crate::client::Client;
use crate::error::Result;
use crate::models::conversions::{
    Conversion, CreateConversionRequest, CreateQuoteRequest, FxRate, GetFxRateParams,
    ListConversionsParams, ListConversionsResponse, RateQuote,
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
    /// `GET /api/v1/fx/conversions`
    pub async fn list(&self, params: ListConversionsParams) -> Result<ListConversionsResponse> {
        self.client
            .get_with_query("/api/v1/fx/conversions", &params)
            .await
    }

    /// Create a conversion.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/fx/conversions/create`
    pub async fn create(&self, request: CreateConversionRequest) -> Result<Conversion> {
        self.client
            .post("/api/v1/fx/conversions/create", &request)
            .await
    }

    /// Get a conversion by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/fx/conversions/{conversion_id}`
    pub async fn get(&self, conversion_id: &str) -> Result<Conversion> {
        self.client
            .get(&format!("/api/v1/fx/conversions/{}", conversion_id))
            .await
    }

    /// Get current FX rate.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/fx/rates/current`
    pub async fn get_rate(&self, params: &GetFxRateParams) -> Result<FxRate> {
        self.client
            .get_with_query("/api/v1/fx/rates/current", params)
            .await
    }

    /// Create a rate quote with guaranteed rate for an agreed period.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/fx/quotes/create`
    pub async fn create_quote(&self, request: &CreateQuoteRequest) -> Result<RateQuote> {
        self.client.post("/api/v1/fx/quotes/create", request).await
    }

    /// Get a quote by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/fx/quotes/{quote_id}`
    pub async fn get_quote(&self, quote_id: &str) -> Result<RateQuote> {
        self.client
            .get(&format!("/api/v1/fx/quotes/{}", quote_id))
            .await
    }
}
