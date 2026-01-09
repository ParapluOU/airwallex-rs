//! Issuing Cardholders resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    Cardholder, CreateCardholderRequest, ListCardholdersParams, ListCardholdersResponse,
    UpdateCardholderRequest,
};

/// Issuing Cardholders resource for managing Airwallex cardholders.
#[derive(Debug)]
pub struct IssuingCardholders<'a> {
    client: &'a Client,
}

impl<'a> IssuingCardholders<'a> {
    /// Create a new Issuing Cardholders resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new cardholder.
    pub async fn create(&self, request: &CreateCardholderRequest) -> Result<Cardholder> {
        self.client
            .post("/api/v1/issuing/cardholders/create", request)
            .await
    }

    /// List cardholders.
    pub async fn list(&self, params: &ListCardholdersParams) -> Result<ListCardholdersResponse> {
        self.client
            .get_with_query("/api/v1/issuing/cardholders", params)
            .await
    }

    /// Get a cardholder by ID.
    pub async fn get(&self, id: &str) -> Result<Cardholder> {
        self.client
            .get(&format!("/api/v1/issuing/cardholders/{}", id))
            .await
    }

    /// Update a cardholder.
    pub async fn update(&self, id: &str, request: &UpdateCardholderRequest) -> Result<Cardholder> {
        self.client
            .post(
                &format!("/api/v1/issuing/cardholders/{}/update", id),
                request,
            )
            .await
    }
}
