//! The main Airwallex API client.

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::TokenManager;
use crate::config::Config;
use crate::error::{ApiErrorResponse, Error, Result};
use crate::resources;

/// The main Airwallex API client.
///
/// This client handles authentication, request building, and response parsing
/// for all Airwallex API operations.
///
/// # Example
///
/// ```no_run
/// use airwallex_rs::{Client, Config, Environment};
///
/// # async fn example() -> airwallex_rs::Result<()> {
/// // Create from environment variables
/// let client = Client::from_env()?;
///
/// // Or configure manually
/// let config = Config::builder()
///     .client_id("your_client_id")
///     .api_key("your_api_key")
///     .environment(Environment::Sandbox)
///     .build()?;
/// let client = Client::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
    token_manager: Arc<TokenManager>,
}

impl Client {
    /// Create a new client with the given configuration.
    pub fn new(config: Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(Error::Http)?;

        let token_manager = Arc::new(TokenManager::new(config.clone(), http_client.clone()));

        Ok(Self {
            config,
            http_client,
            token_manager,
        })
    }

    /// Create a new client from environment variables.
    ///
    /// See [`Config::from_env`] for the expected environment variables.
    pub fn from_env() -> Result<Self> {
        let config = Config::from_env()?;
        Self::new(config)
    }

    /// Get the base URL for the API.
    pub fn base_url(&self) -> &str {
        self.config.base_url()
    }

    /// Get the configured API version.
    pub fn api_version(&self) -> &str {
        &self.config.api_version
    }

    /// Make a GET request to the API.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(reqwest::Method::GET, path, Option::<&()>::None)
            .await
    }

    /// Make a GET request with query parameters.
    pub async fn get_with_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        let token = self.token_manager.get_token().await?;
        let url = format!("{}{}", self.config.base_url(), path);

        let mut request = self
            .http_client
            .get(&url)
            .query(query)
            .header(AUTHORIZATION, token.bearer_value())
            .header("x-api-version", &self.config.api_version);

        if let Some(account_id) = &self.config.on_behalf_of {
            request = request.header("x-on-behalf-of", account_id);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request to the API.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request(reqwest::Method::POST, path, Some(body)).await
    }

    /// Make a POST request with an empty body.
    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let token = self.token_manager.get_token().await?;
        let url = format!("{}{}", self.config.base_url(), path);

        let mut request = self
            .http_client
            .post(&url)
            .header(AUTHORIZATION, token.bearer_value())
            .header(CONTENT_TYPE, "application/json")
            .header("x-api-version", &self.config.api_version)
            .header("Content-Length", "0")
            .body("");

        if let Some(account_id) = &self.config.on_behalf_of {
            request = request.header("x-on-behalf-of", account_id);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request with empty body and no response body.
    pub async fn post_empty_no_response(&self, path: &str) -> Result<()> {
        let token = self.token_manager.get_token().await?;
        let url = format!("{}{}", self.config.base_url(), path);

        let mut request = self
            .http_client
            .post(&url)
            .header(AUTHORIZATION, token.bearer_value())
            .header(CONTENT_TYPE, "application/json")
            .header("x-api-version", &self.config.api_version)
            .header("Content-Length", "0")
            .body("");

        if let Some(account_id) = &self.config.on_behalf_of {
            request = request.header("x-on-behalf-of", account_id);
        }

        let response = request.send().await?;
        self.handle_empty_response(response).await
    }

    /// Make a POST request without expecting a response body.
    pub async fn post_no_response<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let token = self.token_manager.get_token().await?;
        let url = format!("{}{}", self.config.base_url(), path);

        let mut request = self
            .http_client
            .post(&url)
            .header(AUTHORIZATION, token.bearer_value())
            .header(CONTENT_TYPE, "application/json")
            .header("x-api-version", &self.config.api_version)
            .json(body);

        if let Some(account_id) = &self.config.on_behalf_of {
            request = request.header("x-on-behalf-of", account_id);
        }

        let response = request.send().await?;
        self.handle_empty_response(response).await
    }

    /// Make an API request with the given method, path, and optional body.
    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let token = self.token_manager.get_token().await?;
        let url = format!("{}{}", self.config.base_url(), path);

        let mut request = self
            .http_client
            .request(method, &url)
            .header(AUTHORIZATION, token.bearer_value())
            .header("x-api-version", &self.config.api_version);

        if let Some(account_id) = &self.config.on_behalf_of {
            request = request.header("x-on-behalf-of", account_id);
        }

        if let Some(body) = body {
            request = request.header(CONTENT_TYPE, "application/json").json(body);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handle the API response, parsing success or error.
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.json().await?;
            Ok(body)
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Handle an API response that should have no body.
    async fn handle_empty_response(&self, response: reqwest::Response) -> Result<()> {
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Convert an error response into an Error.
    async fn handle_error_response<T>(
        &self,
        response: reqwest::Response,
        status: reqwest::StatusCode,
    ) -> Result<T> {
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);

            return Err(Error::RateLimited { retry_after });
        }

        if status == reqwest::StatusCode::UNAUTHORIZED {
            // Invalidate token and return auth error
            self.token_manager.invalidate().await;
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Authentication(format!(
                "Request unauthorized: {}",
                body
            )));
        }

        // Try to parse as API error
        let error_text = response.text().await.unwrap_or_default();
        match serde_json::from_str::<ApiErrorResponse>(&error_text) {
            Ok(api_error) => Err(Error::from_api_response(api_error)),
            Err(_) => Err(Error::Api {
                code: status.as_str().to_string(),
                message: error_text,
                trace_id: None,
                details: None,
            }),
        }
    }

    // =========================================================================
    // Resource accessors
    // =========================================================================

    /// Access the Balances resource.
    pub fn balances(&self) -> resources::Balances<'_> {
        resources::Balances::new(self)
    }

    /// Access the Global Accounts resource.
    pub fn global_accounts(&self) -> resources::GlobalAccounts<'_> {
        resources::GlobalAccounts::new(self)
    }

    /// Access the Deposits resource.
    pub fn deposits(&self) -> resources::Deposits<'_> {
        resources::Deposits::new(self)
    }

    /// Access the Beneficiaries resource.
    pub fn beneficiaries(&self) -> resources::Beneficiaries<'_> {
        resources::Beneficiaries::new(self)
    }

    /// Access the Transfers resource.
    pub fn transfers(&self) -> resources::Transfers<'_> {
        resources::Transfers::new(self)
    }

    /// Access the Linked Accounts resource.
    pub fn linked_accounts(&self) -> resources::LinkedAccounts<'_> {
        resources::LinkedAccounts::new(self)
    }

    /// Access the Invoices resource.
    pub fn invoices(&self) -> resources::Invoices<'_> {
        resources::Invoices::new(self)
    }

    /// Access the Payment Intents resource.
    pub fn payment_intents(&self) -> resources::PaymentIntents<'_> {
        resources::PaymentIntents::new(self)
    }

    /// Access the Conversions (FX) resource.
    pub fn conversions(&self) -> resources::Conversions<'_> {
        resources::Conversions::new(self)
    }

    /// Access the Customers resource.
    pub fn customers(&self) -> resources::Customers<'_> {
        resources::Customers::new(self)
    }

    /// Access the Refunds resource.
    pub fn refunds(&self) -> resources::Refunds<'_> {
        resources::Refunds::new(self)
    }

    /// Access the Payers resource.
    pub fn payers(&self) -> resources::Payers<'_> {
        resources::Payers::new(self)
    }

    /// Access the Batch Transfers resource.
    pub fn batch_transfers(&self) -> resources::BatchTransfers<'_> {
        resources::BatchTransfers::new(self)
    }

    /// Access the Payment Methods resource.
    pub fn payment_methods(&self) -> resources::PaymentMethods<'_> {
        resources::PaymentMethods::new(self)
    }

    /// Access the Payment Consents resource.
    pub fn payment_consents(&self) -> resources::PaymentConsents<'_> {
        resources::PaymentConsents::new(self)
    }

    /// Access the Financial Transactions resource.
    pub fn financial_transactions(&self) -> resources::FinancialTransactions<'_> {
        resources::FinancialTransactions::new(self)
    }

    /// Access the Payment Links resource.
    pub fn payment_links(&self) -> resources::PaymentLinks<'_> {
        resources::PaymentLinks::new(self)
    }

    /// Access the Payment Disputes resource.
    pub fn payment_disputes(&self) -> resources::PaymentDisputes<'_> {
        resources::PaymentDisputes::new(self)
    }

    /// Access the Settlements resource.
    pub fn settlements(&self) -> resources::Settlements<'_> {
        resources::Settlements::new(self)
    }

    /// Access the Accounts resource (Scale/Connected Accounts).
    pub fn accounts(&self) -> resources::Accounts<'_> {
        resources::Accounts::new(self)
    }

    /// Access the Issuing Cards resource.
    pub fn issuing_cards(&self) -> resources::IssuingCards<'_> {
        resources::IssuingCards::new(self)
    }

    /// Access the Issuing Cardholders resource.
    pub fn issuing_cardholders(&self) -> resources::IssuingCardholders<'_> {
        resources::IssuingCardholders::new(self)
    }

    /// Access the Issuing Transactions resource.
    pub fn issuing_transactions(&self) -> resources::IssuingTransactions<'_> {
        resources::IssuingTransactions::new(self)
    }

    /// Access the Issuing Authorizations resource.
    pub fn issuing_authorizations(&self) -> resources::IssuingAuthorizations<'_> {
        resources::IssuingAuthorizations::new(self)
    }

    /// Access the Issuing Transaction Disputes resource.
    pub fn issuing_transaction_disputes(&self) -> resources::IssuingTransactionDisputes<'_> {
        resources::IssuingTransactionDisputes::new(self)
    }

    /// Access the Issuing Config resource.
    pub fn issuing_config(&self) -> resources::IssuingConfigResource<'_> {
        resources::IssuingConfigResource::new(self)
    }

    /// Access the Account Capabilities resource.
    pub fn account_capabilities(&self) -> resources::AccountCapabilities<'_> {
        resources::AccountCapabilities::new(self)
    }

    /// Access the Payment Attempts resource.
    pub fn payment_attempts(&self) -> resources::PaymentAttempts<'_> {
        resources::PaymentAttempts::new(self)
    }

    /// Access the Connected Account Transfers resource.
    pub fn connected_account_transfers(&self) -> resources::ConnectedAccountTransfers<'_> {
        resources::ConnectedAccountTransfers::new(self)
    }

    /// Access the Conversion Amendments resource.
    pub fn conversion_amendments(&self) -> resources::ConversionAmendments<'_> {
        resources::ConversionAmendments::new(self)
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            http_client: self.http_client.clone(),
            token_manager: Arc::clone(&self.token_manager),
        }
    }
}
