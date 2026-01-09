//! Error types for the Airwallex API client.

use std::time::Duration;

use serde::Deserialize;

/// The main error type for the Airwallex client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API error returned by Airwallex.
    #[error("API error [{code}]: {message}")]
    Api {
        /// Error code from the API (e.g., "invalid_argument", "not_found").
        code: String,
        /// Human-readable error message.
        message: String,
        /// Trace ID for debugging with Airwallex support.
        trace_id: Option<String>,
        /// Additional error details.
        details: Option<serde_json::Value>,
    },

    /// Rate limit exceeded (HTTP 429).
    #[error("Rate limit exceeded")]
    RateLimited {
        /// Suggested retry delay if provided by the API.
        retry_after: Option<Duration>,
    },

    /// Authentication failed.
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Request validation failed.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Resource not found (HTTP 404).
    #[error("Resource not found")]
    NotFound,

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Environment variable error.
    #[error("Environment error: {0}")]
    Env(String),
}

/// API error response structure from Airwallex.
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    /// Error code.
    pub code: String,
    /// Error message.
    pub message: String,
    /// Trace ID for debugging.
    #[serde(default)]
    pub trace_id: Option<String>,
    /// Additional details.
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

impl Error {
    /// Create an API error from a response body.
    pub fn from_api_response(response: ApiErrorResponse) -> Self {
        Error::Api {
            code: response.code,
            message: response.message,
            trace_id: response.trace_id,
            details: response.details,
        }
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::RateLimited { .. } => true,
            Error::Http(e) => e.is_timeout() || e.is_connect(),
            _ => false,
        }
    }

    /// Get the suggested retry delay for rate limited errors.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Error::RateLimited { retry_after } => *retry_after,
            _ => None,
        }
    }
}

/// Result type alias for Airwallex operations.
pub type Result<T> = std::result::Result<T, Error>;
