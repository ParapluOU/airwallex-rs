//! Conversion Amendments resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    AmendmentQuote, ConversionAmendment, CreateAmendmentQuoteRequest, CreateAmendmentRequest,
    ListAmendmentsParams, ListAmendmentsResponse,
};

/// Conversion Amendments resource for cancelling FX conversions.
#[derive(Debug)]
pub struct ConversionAmendments<'a> {
    client: &'a Client,
}

impl<'a> ConversionAmendments<'a> {
    /// Create a new Conversion Amendments resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create an amendment quote (get indicative charges for cancellation).
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/fx/conversion_amendments/quote`
    pub async fn quote(&self, request: &CreateAmendmentQuoteRequest) -> Result<AmendmentQuote> {
        self.client
            .post("/api/v1/fx/conversion_amendments/quote", request)
            .await
    }

    /// Create an amendment (cancel a conversion).
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/fx/conversion_amendments/create`
    pub async fn create(&self, request: &CreateAmendmentRequest) -> Result<ConversionAmendment> {
        self.client
            .post("/api/v1/fx/conversion_amendments/create", request)
            .await
    }

    /// List amendments for a conversion.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/fx/conversion_amendments`
    pub async fn list(&self, params: &ListAmendmentsParams) -> Result<ListAmendmentsResponse> {
        self.client
            .get_with_query("/api/v1/fx/conversion_amendments", params)
            .await
    }

    /// Get an amendment by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/fx/conversion_amendments/{conversion_amendment_id}`
    pub async fn get(&self, amendment_id: &str) -> Result<ConversionAmendment> {
        self.client
            .get(&format!("/api/v1/fx/conversion_amendments/{}", amendment_id))
            .await
    }
}
