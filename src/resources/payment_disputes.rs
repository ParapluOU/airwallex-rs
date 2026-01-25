//! Payment Disputes resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    AcceptDisputeRequest, ChallengeDisputeRequest, ListPaymentDisputesParams,
    ListPaymentDisputesResponse, PaymentDispute,
};

/// Payment Disputes resource for managing chargebacks and RFIs.
#[derive(Debug)]
pub struct PaymentDisputes<'a> {
    client: &'a Client,
}

impl<'a> PaymentDisputes<'a> {
    /// Create a new Payment Disputes resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List payment disputes.
    pub async fn list(
        &self,
        params: &ListPaymentDisputesParams,
    ) -> Result<ListPaymentDisputesResponse> {
        self.client
            .get_with_query("/api/v1/pa/payment_disputes", params)
            .await
    }

    /// Get a payment dispute by ID.
    pub async fn get(&self, id: &str) -> Result<PaymentDispute> {
        self.client
            .get(&format!("/api/v1/pa/payment_disputes/{}", id))
            .await
    }

    /// Accept a payment dispute.
    pub async fn accept(&self, id: &str, request: &AcceptDisputeRequest) -> Result<PaymentDispute> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_disputes/{}/accept", id),
                request,
            )
            .await
    }

    /// Challenge a payment dispute.
    pub async fn challenge(
        &self,
        id: &str,
        request: &ChallengeDisputeRequest,
    ) -> Result<PaymentDispute> {
        self.client
            .post(
                &format!("/api/v1/pa/payment_disputes/{}/challenge", id),
                request,
            )
            .await
    }
}
