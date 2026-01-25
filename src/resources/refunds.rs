//! Refunds resource for the Airwallex API.
//!
//! Manage refunds for payments.

use crate::client::Client;
use crate::error::Result;
use crate::models::refunds::{CreateRefundRequest, ListRefundsParams, ListRefundsResponse, Refund};

/// The Refunds resource.
pub struct Refunds<'a> {
    client: &'a Client,
}

impl<'a> Refunds<'a> {
    /// Create a new Refunds resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List refunds.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/refunds`
    pub async fn list(&self, params: ListRefundsParams) -> Result<ListRefundsResponse> {
        self.client
            .get_with_query("/api/v1/pa/refunds", &params)
            .await
    }

    /// Create a refund.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/refunds/create`
    pub async fn create(&self, request: CreateRefundRequest) -> Result<Refund> {
        self.client
            .post("/api/v1/pa/refunds/create", &request)
            .await
    }

    /// Get a refund by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/refunds/{id}`
    pub async fn get(&self, id: &str) -> Result<Refund> {
        self.client.get(&format!("/api/v1/pa/refunds/{}", id)).await
    }
}
