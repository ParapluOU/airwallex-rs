//! Payment Intents resource for the Airwallex API.
//!
//! Manage payment intents for accepting payments.

use crate::client::Client;
use crate::error::Result;
use crate::models::payment_intents::{
    CancelPaymentIntentRequest, CapturePaymentIntentRequest, ConfirmPaymentIntentRequest,
    CreatePaymentIntentRequest, ListPaymentIntentsParams, ListPaymentIntentsResponse,
    PaymentIntent,
};

/// The Payment Intents resource.
pub struct PaymentIntents<'a> {
    client: &'a Client,
}

impl<'a> PaymentIntents<'a> {
    /// Create a new PaymentIntents resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List payment intents.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_intents`
    pub async fn list(
        &self,
        params: ListPaymentIntentsParams,
    ) -> Result<ListPaymentIntentsResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_intents", &params)
            .await
    }

    /// Create a payment intent.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_intents/create`
    pub async fn create(&self, request: CreatePaymentIntentRequest) -> Result<PaymentIntent> {
        self.client
            .post("/api/v1/pa/payment_intents/create", &request)
            .await
    }

    /// Get a payment intent by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_intents/{id}`
    pub async fn get(&self, id: &str) -> Result<PaymentIntent> {
        self.client
            .get(&format!("/api/v1/pa/payment_intents/{}", id))
            .await
    }

    /// Confirm a payment intent.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_intents/{id}/confirm`
    pub async fn confirm(
        &self,
        id: &str,
        request: ConfirmPaymentIntentRequest,
    ) -> Result<PaymentIntent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_intents/{}/confirm", id),
                &request,
            )
            .await
    }

    /// Capture a payment intent.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_intents/{id}/capture`
    pub async fn capture(
        &self,
        id: &str,
        request: CapturePaymentIntentRequest,
    ) -> Result<PaymentIntent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_intents/{}/capture", id),
                &request,
            )
            .await
    }

    /// Cancel a payment intent.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_intents/{id}/cancel`
    pub async fn cancel(
        &self,
        id: &str,
        request: CancelPaymentIntentRequest,
    ) -> Result<PaymentIntent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_intents/{}/cancel", id),
                &request,
            )
            .await
    }
}
