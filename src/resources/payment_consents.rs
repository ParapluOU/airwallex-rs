//! Payment Consents resource for the Airwallex API.
//!
//! Manage payment consents (recurring payments, subscriptions).

use crate::client::Client;
use crate::error::Result;
use crate::models::payment_consents::{
    CreatePaymentConsentRequest, DisablePaymentConsentRequest, ListPaymentConsentsParams,
    ListPaymentConsentsResponse, PaymentConsent, UpdatePaymentConsentRequest,
    VerifyPaymentConsentRequest,
};

/// The Payment Consents resource.
pub struct PaymentConsents<'a> {
    client: &'a Client,
}

impl<'a> PaymentConsents<'a> {
    /// Create a new PaymentConsents resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a payment consent.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_consents/create`
    pub async fn create(&self, request: CreatePaymentConsentRequest) -> Result<PaymentConsent> {
        self.client
            .post("/api/v1/pa/payment_consents/create", &request)
            .await
    }

    /// List payment consents.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_consents`
    pub async fn list(
        &self,
        params: ListPaymentConsentsParams,
    ) -> Result<ListPaymentConsentsResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_consents", &params)
            .await
    }

    /// Get a payment consent by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/payment_consents/{id}`
    pub async fn get(&self, id: &str) -> Result<PaymentConsent> {
        self.client
            .get(&format!("/api/v1/pa/payment_consents/{}", id))
            .await
    }

    /// Update a payment consent's metadata.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_consents/{id}/update`
    pub async fn update(
        &self,
        id: &str,
        request: UpdatePaymentConsentRequest,
    ) -> Result<PaymentConsent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_consents/{}/update", id),
                &request,
            )
            .await
    }

    /// Verify a payment consent with payment method details.
    ///
    /// After verification, the consent can be used for subsequent payments.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_consents/{id}/verify`
    pub async fn verify(
        &self,
        id: &str,
        request: VerifyPaymentConsentRequest,
    ) -> Result<PaymentConsent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_consents/{}/verify", id),
                &request,
            )
            .await
    }

    /// Disable a payment consent.
    ///
    /// After disabling, the consent cannot be used or updated.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/payment_consents/{id}/disable`
    pub async fn disable(
        &self,
        id: &str,
        request: DisablePaymentConsentRequest,
    ) -> Result<PaymentConsent> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_consents/{}/disable", id),
                &request,
            )
            .await
    }
}
