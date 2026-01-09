//! Token management for Airwallex API authentication.

use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::error::{ApiErrorResponse, Error, Result};

/// Authentication token from the Airwallex API.
#[derive(Debug, Clone)]
pub struct Token {
    /// The bearer token value.
    value: SecretString,
    /// When the token expires.
    expires_at: DateTime<Utc>,
}

impl Token {
    /// Create a new token.
    pub fn new(value: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            value: SecretString::new(value.into()),
            expires_at,
        }
    }

    /// Get the token value for use in Authorization header.
    pub fn bearer_value(&self) -> String {
        format!("Bearer {}", self.value.expose_secret())
    }

    /// Check if the token is expired or will expire within the given buffer.
    pub fn is_expired_with_buffer(&self, buffer: Duration) -> bool {
        let buffer_chrono = chrono::Duration::from_std(buffer).unwrap_or(chrono::Duration::zero());
        Utc::now() + buffer_chrono >= self.expires_at
    }
}

/// Response from the authentication endpoint.
#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    /// The authentication token.
    pub token: String,
    /// Token expiration time.
    pub expires_at: DateTime<Utc>,
}

/// Manages authentication tokens with automatic refresh.
pub struct TokenManager {
    config: Config,
    http_client: reqwest::Client,
    token: Arc<RwLock<Option<Token>>>,
}

impl TokenManager {
    /// Create a new token manager.
    pub fn new(config: Config, http_client: reqwest::Client) -> Self {
        Self {
            config,
            http_client,
            token: Arc::new(RwLock::new(None)),
        }
    }

    /// Get a valid token, refreshing if necessary.
    pub async fn get_token(&self) -> Result<Token> {
        // Check if we have a valid token
        {
            let token_guard = self.token.read().await;
            if let Some(token) = token_guard.as_ref() {
                if !token.is_expired_with_buffer(self.config.token_refresh_buffer) {
                    return Ok(token.clone());
                }
            }
        }

        // Need to refresh - acquire write lock
        let mut token_guard = self.token.write().await;

        // Double-check after acquiring write lock (another task might have refreshed)
        if let Some(token) = token_guard.as_ref() {
            if !token.is_expired_with_buffer(self.config.token_refresh_buffer) {
                return Ok(token.clone());
            }
        }

        // Perform login
        let new_token = self.login().await?;
        *token_guard = Some(new_token.clone());
        Ok(new_token)
    }

    /// Perform login to get a new token.
    async fn login(&self) -> Result<Token> {
        let url = format!("{}/api/v1/authentication/login", self.config.base_url());

        let response = self
            .http_client
            .post(&url)
            .header("x-client-id", &self.config.client_id)
            .header("x-api-key", self.config.api_key())
            .header("Content-Type", "application/json")
            .header("Content-Length", "0")
            .body("")
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let login_response: LoginResponse = response.json().await?;
            Ok(Token::new(login_response.token, login_response.expires_at))
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            let error_body = response.text().await.unwrap_or_default();
            Err(Error::Authentication(format!(
                "Invalid credentials: {}",
                error_body
            )))
        } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            Err(Error::RateLimited { retry_after: None })
        } else {
            // Try to parse as API error
            let error_text = response.text().await.unwrap_or_default();
            match serde_json::from_str::<ApiErrorResponse>(&error_text) {
                Ok(api_error) => Err(Error::from_api_response(api_error)),
                Err(_) => Err(Error::Authentication(format!(
                    "Authentication failed with status {}: {}",
                    status, error_text
                ))),
            }
        }
    }

    /// Invalidate the current token, forcing a refresh on next request.
    pub async fn invalidate(&self) {
        let mut token_guard = self.token.write().await;
        *token_guard = None;
    }
}

impl std::fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenManager")
            .field("config", &self.config)
            .field("has_token", &self.token.try_read().map(|t| t.is_some()).unwrap_or(false))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_expiry() {
        let future = Utc::now() + chrono::Duration::hours(1);
        let token = Token::new("test".to_string(), future);

        // Should not be expired with 5 minute buffer
        assert!(!token.is_expired_with_buffer(Duration::from_secs(300)));

        // Should be expired if we check with 2 hour buffer
        assert!(token.is_expired_with_buffer(Duration::from_secs(7200)));
    }

    #[test]
    fn test_token_bearer_value() {
        let token = Token::new("abc123".to_string(), Utc::now());
        assert_eq!(token.bearer_value(), "Bearer abc123");
    }
}
