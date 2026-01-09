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
