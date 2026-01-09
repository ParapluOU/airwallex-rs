//! Payment Attempts resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{ListPaymentAttemptsParams, ListPaymentAttemptsResponse, PaymentAttempt};

/// Payment Attempts resource for retrieving payment attempt information.
#[derive(Debug)]
pub struct PaymentAttempts<'a> {
    client: &'a Client,
}

impl<'a> PaymentAttempts<'a> {
    /// Create a new Payment Attempts resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List payment attempts.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_attempts`
    pub async fn list(
        &self,
        params: &ListPaymentAttemptsParams,
    ) -> Result<ListPaymentAttemptsResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_attempts", params)
            .await
    }

    /// Get a payment attempt by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_attempts/{id}`
    pub async fn get(&self, attempt_id: &str) -> Result<PaymentAttempt> {
        self.client
            .get(&format!("/api/v1/pa/payment_attempts/{}", attempt_id))
            .await
    }
}
