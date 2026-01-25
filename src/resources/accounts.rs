//! Accounts resource for Scale (Connected Accounts).

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    Account, CreateAccountRequest, ListAccountsParams, ListAccountsResponse, UpdateAccountRequest,
};

/// Accounts resource for managing connected accounts (Scale).
#[derive(Debug)]
pub struct Accounts<'a> {
    client: &'a Client,
}

impl<'a> Accounts<'a> {
    /// Create a new Accounts resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get your own Airwallex account details.
    pub async fn get_own(&self) -> Result<Account> {
        self.client.get("/api/v1/account").await
    }

    /// Create a new connected account.
    pub async fn create(&self, request: &CreateAccountRequest) -> Result<Account> {
        self.client.post("/api/v1/accounts/create", request).await
    }

    /// List connected accounts.
    pub async fn list(&self, params: &ListAccountsParams) -> Result<ListAccountsResponse> {
        self.client.get_with_query("/api/v1/accounts", params).await
    }

    /// Get a connected account by ID.
    pub async fn get(&self, id: &str) -> Result<Account> {
        self.client.get(&format!("/api/v1/accounts/{}", id)).await
    }

    /// Update a connected account.
    pub async fn update(&self, id: &str, request: &UpdateAccountRequest) -> Result<Account> {
        self.client
            .post(&format!("/api/v1/accounts/{}/update", id), request)
            .await
    }
}
