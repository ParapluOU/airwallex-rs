//! Payment Methods resource for the Airwallex API.
//!
//! Manage payment methods (cards, direct debits, digital wallets).

use crate::client::Client;
use crate::error::Result;
use crate::models::payment_methods::{
    CreatePaymentMethodRequest, DisablePaymentMethodRequest, ListPaymentMethodsParams,
    ListPaymentMethodsResponse, PaymentMethod,
};

/// The Payment Methods resource.
pub struct PaymentMethods<'a> {
    client: &'a Client,
}

impl<'a> PaymentMethods<'a> {
    /// Create a new PaymentMethods resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a payment method.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_methods/create`
    pub async fn create(&self, request: CreatePaymentMethodRequest) -> Result<PaymentMethod> {
        self.client
            .post("/api/v1/pa/payment_methods/create", &request)
            .await
    }

    /// List payment methods.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_methods`
    pub async fn list(
        &self,
        params: ListPaymentMethodsParams,
    ) -> Result<ListPaymentMethodsResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_methods", &params)
            .await
    }

    /// Get a payment method by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_methods/{id}`
    pub async fn get(&self, id: &str) -> Result<PaymentMethod> {
        self.client
            .get(&format!("/api/v1/pa/payment_methods/{}", id))
            .await
    }

    /// Disable a payment method.
    ///
    /// Changes the payment method status to DISABLED.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_methods/{id}/disable`
    pub async fn disable(
        &self,
        id: &str,
        request: DisablePaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_methods/{}/disable", id),
                &request,
            )
            .await
    }
}
