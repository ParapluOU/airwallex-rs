//! Payers resource for the Airwallex API.
//!
//! Manage payer contacts for transfers.

use crate::client::Client;
use crate::error::Result;
use crate::models::payers::{
    CreatePayerRequest, ListPayersParams, ListPayersResponse, PayerContact, UpdatePayerRequest,
};

/// The Payers resource.
pub struct Payers<'a> {
    client: &'a Client,
}

impl<'a> Payers<'a> {
    /// Create a new Payers resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List payers.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/payers`
    pub async fn list(&self, params: ListPayersParams) -> Result<ListPayersResponse> {
        self.client
            .get_with_query("/api/v1/payers", &params)
            .await
    }

    /// Create a payer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/payers/create`
    pub async fn create(&self, request: CreatePayerRequest) -> Result<PayerContact> {
        self.client.post("/api/v1/payers/create", &request).await
    }

    /// Get a payer by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/payers/{id}`
    pub async fn get(&self, id: &str) -> Result<PayerContact> {
        self.client.get(&format!("/api/v1/payers/{}", id)).await
    }

    /// Update a payer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/payers/{id}/update`
    pub async fn update(&self, id: &str, request: UpdatePayerRequest) -> Result<PayerContact> {
        self.client
            .post(&format!("/api/v1/payers/{}/update", id), &request)
            .await
    }

    /// Delete a payer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/payers/{id}/delete`
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client
            .post_empty_no_response(&format!("/api/v1/payers/{}/delete", id))
            .await
    }

    /// Validate a payer request without creating it.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/payers/validate`
    pub async fn validate(&self, request: CreatePayerRequest) -> Result<()> {
        // Returns "OK" on success, we just discard it
        let _: String = self.client.post("/api/v1/payers/validate", &request).await?;
        Ok(())
    }
}
