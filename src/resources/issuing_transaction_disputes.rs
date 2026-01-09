//! Issuing Transaction Disputes resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    CreateIssuingTransactionDisputeRequest, IssuingTransactionDispute,
    ListIssuingTransactionDisputesParams, ListIssuingTransactionDisputesResponse,
    UpdateIssuingTransactionDisputeRequest,
};

/// Issuing Transaction Disputes resource for managing card transaction disputes.
#[derive(Debug)]
pub struct IssuingTransactionDisputes<'a> {
    client: &'a Client,
}

impl<'a> IssuingTransactionDisputes<'a> {
    /// Create a new Issuing Transaction Disputes resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a transaction dispute in draft status.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/issuing/transaction_disputes/create`
    pub async fn create(
        &self,
        request: &CreateIssuingTransactionDisputeRequest,
    ) -> Result<IssuingTransactionDispute> {
        self.client
            .post("/api/v1/issuing/transaction_disputes/create", request)
            .await
    }

    /// List transaction disputes.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/issuing/transaction_disputes`
    pub async fn list(
        &self,
        params: &ListIssuingTransactionDisputesParams,
    ) -> Result<ListIssuingTransactionDisputesResponse> {
        self.client
            .get_with_query("/api/v1/issuing/transaction_disputes", params)
            .await
    }

    /// Get a transaction dispute by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/issuing/transaction_disputes/{id}`
    pub async fn get(&self, dispute_id: &str) -> Result<IssuingTransactionDispute> {
        self.client
            .get(&format!(
                "/api/v1/issuing/transaction_disputes/{}",
                dispute_id
            ))
            .await
    }

    /// Update a transaction dispute.
    ///
    /// In DRAFT status, all fields can be updated. Once submitted,
    /// only new evidence or explanation can be added.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/issuing/transaction_disputes/{id}/update`
    pub async fn update(
        &self,
        dispute_id: &str,
        request: &UpdateIssuingTransactionDisputeRequest,
    ) -> Result<IssuingTransactionDispute> {
        self.client
            .post(
                &format!(
                    "/api/v1/issuing/transaction_disputes/{}/update",
                    dispute_id
                ),
                request,
            )
            .await
    }

    /// Submit a dispute to Airwallex for review and processing.
    ///
    /// After submission, the status will change to SUBMITTED.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/issuing/transaction_disputes/{id}/submit`
    pub async fn submit(&self, dispute_id: &str) -> Result<IssuingTransactionDispute> {
        self.client
            .post_empty(&format!(
                "/api/v1/issuing/transaction_disputes/{}/submit",
                dispute_id
            ))
            .await
    }

    /// Cancel a submitted dispute.
    ///
    /// Can only be canceled when status = SUBMITTED. Once status = IN_PROGRESS,
    /// the dispute cannot be canceled.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/issuing/transaction_disputes/{id}/cancel`
    pub async fn cancel(&self, dispute_id: &str) -> Result<IssuingTransactionDispute> {
        self.client
            .post_empty(&format!(
                "/api/v1/issuing/transaction_disputes/{}/cancel",
                dispute_id
            ))
            .await
    }
}
