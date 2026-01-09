//! Global Accounts resource for the Airwallex API.
//!
//! Global accounts can be used to receive funds from payers via local clearing
//! or SWIFT systems.

use crate::client::Client;
use crate::error::Result;
use crate::models::global_accounts::{
    ActiveGlobalAccount, CreateGlobalAccountRequest, CreateMandateRequest,
    GenerateStatementLetterRequest, ListGlobalAccountsParams, ListGlobalAccountsResponse,
    ListMandatesResponse, ListTransactionsParams, ListTransactionsResponse, Mandate,
    StatementLetterResponse, UpdateGlobalAccountRequest,
};

/// The Global Accounts resource.
pub struct GlobalAccounts<'a> {
    client: &'a Client,
}

impl<'a> GlobalAccounts<'a> {
    /// Create a new GlobalAccounts resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List global accounts.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/global_accounts`
    pub async fn list(&self, params: ListGlobalAccountsParams) -> Result<ListGlobalAccountsResponse> {
        self.client
            .get_with_query("/api/v1/global_accounts", &params)
            .await
    }

    /// Create a global account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/create`
    pub async fn create(&self, request: CreateGlobalAccountRequest) -> Result<ActiveGlobalAccount> {
        self.client
            .post("/api/v1/global_accounts/create", &request)
            .await
    }

    /// Get a global account by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/global_accounts/{id}`
    pub async fn get(&self, id: &str) -> Result<ActiveGlobalAccount> {
        self.client
            .get(&format!("/api/v1/global_accounts/{}", id))
            .await
    }

    /// Update a global account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/update/{id}`
    pub async fn update(
        &self,
        id: &str,
        request: UpdateGlobalAccountRequest,
    ) -> Result<ActiveGlobalAccount> {
        self.client
            .post(&format!("/api/v1/global_accounts/update/{}", id), &request)
            .await
    }

    /// Close a global account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/{id}/close`
    pub async fn close(&self, id: &str) -> Result<ActiveGlobalAccount> {
        self.client
            .post_empty(&format!("/api/v1/global_accounts/{}/close", id))
            .await
    }

    /// List transactions for a global account.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/global_accounts/{id}/transactions`
    pub async fn transactions(
        &self,
        id: &str,
        params: ListTransactionsParams,
    ) -> Result<ListTransactionsResponse> {
        self.client
            .get_with_query(&format!("/api/v1/global_accounts/{}/transactions", id), &params)
            .await
    }

    /// Generate a statement letter for a global account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/{id}/generate_statement_letter`
    pub async fn generate_statement_letter(
        &self,
        id: &str,
        request: GenerateStatementLetterRequest,
    ) -> Result<StatementLetterResponse> {
        self.client
            .post(
                &format!("/api/v1/global_accounts/{}/generate_statement_letter", id),
                &request,
            )
            .await
    }

    /// List mandates for a global account.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/global_accounts/{global_account_id}/mandates`
    pub async fn list_mandates(&self, global_account_id: &str) -> Result<ListMandatesResponse> {
        self.client
            .get(&format!(
                "/api/v1/global_accounts/{}/mandates",
                global_account_id
            ))
            .await
    }

    /// Create a mandate for a global account.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/{global_account_id}/mandates`
    pub async fn create_mandate(
        &self,
        global_account_id: &str,
        request: CreateMandateRequest,
    ) -> Result<Mandate> {
        self.client
            .post(
                &format!("/api/v1/global_accounts/{}/mandates", global_account_id),
                &request,
            )
            .await
    }

    /// Get a mandate by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/global_accounts/{global_account_id}/mandates/{id}`
    pub async fn get_mandate(&self, global_account_id: &str, mandate_id: &str) -> Result<Mandate> {
        self.client
            .get(&format!(
                "/api/v1/global_accounts/{}/mandates/{}",
                global_account_id, mandate_id
            ))
            .await
    }

    /// Cancel a mandate.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/global_accounts/{global_account_id}/mandates/{id}/cancel`
    pub async fn cancel_mandate(
        &self,
        global_account_id: &str,
        mandate_id: &str,
    ) -> Result<Mandate> {
        self.client
            .post_empty(&format!(
                "/api/v1/global_accounts/{}/mandates/{}/cancel",
                global_account_id, mandate_id
            ))
            .await
    }
}
