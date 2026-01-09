//! Payment Links resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    CreatePaymentLinkRequest, ListPaymentLinksParams, ListPaymentLinksResponse,
    NotifyShopperRequest, PaymentLink, UpdatePaymentLinkRequest,
};

/// Payment Links resource for managing hosted checkout pages.
#[derive(Debug)]
pub struct PaymentLinks<'a> {
    client: &'a Client,
}

impl<'a> PaymentLinks<'a> {
    /// Create a new Payment Links resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new payment link.
    pub async fn create(&self, request: &CreatePaymentLinkRequest) -> Result<PaymentLink> {
        self.client.post("/api/v1/pa/payment_links/create", request).await
    }

    /// List payment links.
    pub async fn list(&self, params: &ListPaymentLinksParams) -> Result<ListPaymentLinksResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_links", params)
            .await
    }

    /// Get a payment link by ID.
    pub async fn get(&self, id: &str) -> Result<PaymentLink> {
        self.client
            .get(&format!("/api/v1/pa/payment_links/{}", id))
            .await
    }

    /// Update a payment link.
    pub async fn update(&self, id: &str, request: &UpdatePaymentLinkRequest) -> Result<PaymentLink> {
        self.client
            .post(&format!("/api/v1/pa/payment_links/{}/update", id), request)
            .await
    }

    /// Activate a payment link.
    pub async fn activate(&self, id: &str) -> Result<PaymentLink> {
        self.client
            .post_empty(&format!("/api/v1/pa/payment_links/{}/activate", id))
            .await
    }

    /// Deactivate a payment link.
    pub async fn deactivate(&self, id: &str) -> Result<PaymentLink> {
        self.client
            .post_empty(&format!("/api/v1/pa/payment_links/{}/deactivate", id))
            .await
    }

    /// Send notification to shopper about payment link.
    pub async fn notify_shopper(&self, id: &str, request: &NotifyShopperRequest) -> Result<()> {
        self.client
            .post_no_response(&format!("/api/v1/pa/payment_links/{}/notify_shopper", id), request)
            .await
    }

    /// Delete a payment link.
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client
            .post_empty_no_response(&format!("/api/v1/pa/payment_links/{}/delete", id))
            .await
    }
}
