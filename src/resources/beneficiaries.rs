//! Beneficiaries resource for the Airwallex API.
//!
//! Manage payout beneficiaries (payment recipients).

use crate::client::Client;
use crate::error::Result;
use crate::models::beneficiaries::{
    Beneficiary, CreateBeneficiaryRequest, ListBeneficiariesParams, ListBeneficiariesResponse,
    UpdateBeneficiaryRequest, ValidateBeneficiaryRequest, ValidateBeneficiaryResponse,
    VerifyAccountRequest, VerifyAccountResponse,
};

/// The Beneficiaries resource.
pub struct Beneficiaries<'a> {
    client: &'a Client,
}

impl<'a> Beneficiaries<'a> {
    /// Create a new Beneficiaries resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List beneficiaries.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/beneficiaries`
    pub async fn list(&self, params: ListBeneficiariesParams) -> Result<ListBeneficiariesResponse> {
        self.client
            .get_with_query("/api/v1/beneficiaries", &params)
            .await
    }

    /// Create a beneficiary.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/beneficiaries/create`
    pub async fn create(&self, request: CreateBeneficiaryRequest) -> Result<Beneficiary> {
        self.client
            .post("/api/v1/beneficiaries/create", &request)
            .await
    }

    /// Get a beneficiary by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/beneficiaries/{beneficiary_id}`
    pub async fn get(&self, beneficiary_id: &str) -> Result<Beneficiary> {
        self.client
            .get(&format!("/api/v1/beneficiaries/{}", beneficiary_id))
            .await
    }

    /// Update a beneficiary.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/beneficiaries/update/{beneficiary_id}`
    pub async fn update(
        &self,
        beneficiary_id: &str,
        request: UpdateBeneficiaryRequest,
    ) -> Result<Beneficiary> {
        self.client
            .post(
                &format!("/api/v1/beneficiaries/update/{}", beneficiary_id),
                &request,
            )
            .await
    }

    /// Delete a beneficiary.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/beneficiaries/delete/{beneficiary_id}`
    pub async fn delete(&self, beneficiary_id: &str) -> Result<()> {
        self.client
            .post_empty_no_response(&format!("/api/v1/beneficiaries/delete/{}", beneficiary_id))
            .await
    }

    /// Validate beneficiary details.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/beneficiaries/validate`
    pub async fn validate(
        &self,
        request: ValidateBeneficiaryRequest,
    ) -> Result<ValidateBeneficiaryResponse> {
        self.client
            .post("/api/v1/beneficiaries/validate", &request)
            .await
    }

    /// Verify a beneficiary account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/beneficiaries/verify_account`
    pub async fn verify_account(
        &self,
        request: VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse> {
        self.client
            .post("/api/v1/beneficiaries/verify_account", &request)
            .await
    }
}
