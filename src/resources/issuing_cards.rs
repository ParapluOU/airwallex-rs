//! Issuing Cards resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    CardLimits, CreateIssuingCardRequest, IssuingCard, IssuingCardDetails, ListCardsParams,
    ListCardsResponse, UpdateCardRequest,
};

/// Issuing Cards resource for managing Airwallex issued cards.
#[derive(Debug)]
pub struct IssuingCards<'a> {
    client: &'a Client,
}

impl<'a> IssuingCards<'a> {
    /// Create a new Issuing Cards resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new card.
    pub async fn create(&self, request: &CreateIssuingCardRequest) -> Result<IssuingCard> {
        self.client
            .post("/api/v1/issuing/cards/create", request)
            .await
    }

    /// List cards.
    pub async fn list(&self, params: &ListCardsParams) -> Result<ListCardsResponse> {
        self.client
            .get_with_query("/api/v1/issuing/cards", params)
            .await
    }

    /// Get a card by ID.
    pub async fn get(&self, id: &str) -> Result<IssuingCard> {
        self.client
            .get(&format!("/api/v1/issuing/cards/{}", id))
            .await
    }

    /// Update a card.
    pub async fn update(&self, id: &str, request: &UpdateCardRequest) -> Result<IssuingCard> {
        self.client
            .post(&format!("/api/v1/issuing/cards/{}/update", id), request)
            .await
    }

    /// Activate a physical card.
    pub async fn activate(&self, id: &str) -> Result<IssuingCard> {
        self.client
            .post_empty(&format!("/api/v1/issuing/cards/{}/activate", id))
            .await
    }

    /// Get sensitive card details (PAN, CVV, expiry).
    ///
    /// Only works for active virtual cards.
    pub async fn get_details(&self, id: &str) -> Result<IssuingCardDetails> {
        self.client
            .get(&format!("/api/v1/issuing/cards/{}/details", id))
            .await
    }

    /// Get remaining card limits.
    pub async fn get_limits(&self, id: &str) -> Result<CardLimits> {
        self.client
            .get(&format!("/api/v1/issuing/cards/{}/limits", id))
            .await
    }
}
