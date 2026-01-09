//! Payment Acceptance Config resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    ListBanksParams, ListBanksResponse, ListPaymentMethodTypesParams,
    ListPaymentMethodTypesResponse,
};

/// Payment Config resource for retrieving payment method and bank configuration.
#[derive(Debug)]
pub struct PaymentConfig<'a> {
    client: &'a Client,
}

impl<'a> PaymentConfig<'a> {
    /// Create a new Payment Config resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List available payment method types.
    ///
    /// Retrieve configuration for available payment method types with different
    /// transaction modes (oneoff, recurring).
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/config/payment_method_types`
    pub async fn payment_method_types(
        &self,
        params: &ListPaymentMethodTypesParams,
    ) -> Result<ListPaymentMethodTypesResponse> {
        self.client
            .get_with_query("/api/v1/pa/config/payment_method_types", params)
            .await
    }

    /// List available banks for a payment method type.
    ///
    /// Some payment method types (e.g., online_banking) require the bank_name
    /// to be filled when confirming a PaymentIntent.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/config/banks`
    pub async fn banks(&self, params: &ListBanksParams) -> Result<ListBanksResponse> {
        self.client
            .get_with_query("/api/v1/pa/config/banks", params)
            .await
    }
}
