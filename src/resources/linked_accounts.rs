//! Linked Accounts resource for the Airwallex API.
//!
//! Manage linked bank accounts for direct debits.

use crate::client::Client;
use crate::error::Result;
use crate::models::linked_accounts::{
    CompleteAuthRequest, CreateLinkedAccountRequest, InitiateAuthRequest, InitiateAuthResponse,
    LinkedAccount, LinkedAccountBalance, LinkedAccountMandate, ListLinkedAccountsParams,
    ListLinkedAccountsResponse, VerifyMicrodepositsRequest,
};

/// The Linked Accounts resource.
pub struct LinkedAccounts<'a> {
    client: &'a Client,
}

impl<'a> LinkedAccounts<'a> {
    /// Create a new LinkedAccounts resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List linked accounts.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/linked_accounts`
    pub async fn list(
        &self,
        params: ListLinkedAccountsParams,
    ) -> Result<ListLinkedAccountsResponse> {
        self.client
            .get_with_query("/api/v1/linked_accounts", &params)
            .await
    }

    /// Create a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/create`
    pub async fn create(&self, request: CreateLinkedAccountRequest) -> Result<LinkedAccount> {
        self.client
            .post("/api/v1/linked_accounts/create", &request)
            .await
    }

    /// Get a linked account by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/linked_accounts/{id}`
    pub async fn get(&self, id: &str) -> Result<LinkedAccount> {
        self.client
            .get(&format!("/api/v1/linked_accounts/{}", id))
            .await
    }

    /// Delete a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/delete`
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client
            .post_empty_no_response(&format!("/api/v1/linked_accounts/{}/delete", id))
            .await
    }

    /// Suspend a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/suspend`
    pub async fn suspend(&self, id: &str) -> Result<LinkedAccount> {
        self.client
            .post_empty(&format!("/api/v1/linked_accounts/{}/suspend", id))
            .await
    }

    /// Confirm a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/confirm`
    pub async fn confirm(&self, id: &str) -> Result<LinkedAccount> {
        self.client
            .post_empty(&format!("/api/v1/linked_accounts/{}/confirm", id))
            .await
    }

    /// Initiate authentication for linked accounts.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/auth`
    pub async fn initiate_auth(
        &self,
        request: InitiateAuthRequest,
    ) -> Result<InitiateAuthResponse> {
        self.client
            .post("/api/v1/linked_accounts/auth", &request)
            .await
    }

    /// Initiate authentication for a specific linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/auth`
    pub async fn initiate_account_auth(
        &self,
        id: &str,
        request: InitiateAuthRequest,
    ) -> Result<InitiateAuthResponse> {
        self.client
            .post(&format!("/api/v1/linked_accounts/{}/auth", id), &request)
            .await
    }

    /// Complete authentication for a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/complete_auth`
    pub async fn complete_auth(
        &self,
        id: &str,
        request: CompleteAuthRequest,
    ) -> Result<LinkedAccount> {
        self.client
            .post(
                &format!("/api/v1/linked_accounts/{}/complete_auth", id),
                &request,
            )
            .await
    }

    /// Get balances for a linked account.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/linked_accounts/{id}/balances`
    pub async fn balances(&self, id: &str) -> Result<Vec<LinkedAccountBalance>> {
        self.client
            .get(&format!("/api/v1/linked_accounts/{}/balances", id))
            .await
    }

    /// Get mandate for a linked account.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/linked_accounts/{id}/mandate`
    pub async fn mandate(&self, id: &str) -> Result<LinkedAccountMandate> {
        self.client
            .get(&format!("/api/v1/linked_accounts/{}/mandate", id))
            .await
    }

    /// Verify microdeposits for a linked account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/linked_accounts/{id}/verify_microdeposits`
    pub async fn verify_microdeposits(
        &self,
        id: &str,
        request: VerifyMicrodepositsRequest,
    ) -> Result<LinkedAccount> {
        self.client
            .post(
                &format!("/api/v1/linked_accounts/{}/verify_microdeposits", id),
                &request,
            )
            .await
    }
}
