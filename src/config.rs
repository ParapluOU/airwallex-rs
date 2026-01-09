//! Configuration for the Airwallex API client.

use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};

use crate::error::{Error, Result};

/// The API version to use for requests.
pub const DEFAULT_API_VERSION: &str = "2024-09-27";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default token refresh buffer (refresh token 5 minutes before expiry).
pub const DEFAULT_TOKEN_REFRESH_BUFFER: Duration = Duration::from_secs(300);

/// Environment (sandbox or production).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    /// Sandbox environment for testing.
    #[default]
    Sandbox,
    /// Production environment.
    Production,
}

impl Environment {
    /// Get the base URL for this environment.
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Sandbox => "https://api-demo.airwallex.com",
            Environment::Production => "https://api.airwallex.com",
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sandbox" | "demo" | "test" => Ok(Environment::Sandbox),
            "production" | "prod" | "live" => Ok(Environment::Production),
            _ => Err(Error::Config(format!(
                "Invalid environment '{}'. Expected 'sandbox' or 'production'.",
                s
            ))),
        }
    }
}

/// Configuration for the Airwallex client.
#[derive(Clone)]
pub struct Config {
    /// API client ID.
    pub(crate) client_id: String,
    /// API key (secret).
    pub(crate) api_key: SecretString,
    /// Environment (sandbox or production).
    pub(crate) environment: Environment,
    /// API version to use.
    pub(crate) api_version: String,
    /// Request timeout.
    pub(crate) timeout: Duration,
    /// Token refresh buffer.
    pub(crate) token_refresh_buffer: Duration,
    /// Optional account ID for connected account operations.
    pub(crate) on_behalf_of: Option<String>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("client_id", &self.client_id)
            .field("api_key", &"[REDACTED]")
            .field("environment", &self.environment)
            .field("api_version", &self.api_version)
            .field("timeout", &self.timeout)
            .field("token_refresh_buffer", &self.token_refresh_buffer)
            .field("on_behalf_of", &self.on_behalf_of)
            .finish()
    }
}

impl Config {
    /// Create a new configuration builder.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Load configuration from environment variables.
    ///
    /// Expected variables:
    /// - `AIRWALLEX_CLIENT_ID` - API client ID
    /// - `AIRWALLEX_API_KEY` - API key
    /// - `AIRWALLEX_ENVIRONMENT` - "sandbox" or "production" (default: "sandbox")
    pub fn from_env() -> Result<Self> {
        // Try to load .env file, but don't fail if it doesn't exist
        let _ = dotenvy::dotenv();

        let client_id = std::env::var("AIRWALLEX_CLIENT_ID")
            .map_err(|_| Error::Env("AIRWALLEX_CLIENT_ID not set".to_string()))?;

        let api_key = std::env::var("AIRWALLEX_API_KEY")
            .map_err(|_| Error::Env("AIRWALLEX_API_KEY not set".to_string()))?;

        let environment = std::env::var("AIRWALLEX_ENVIRONMENT")
            .unwrap_or_else(|_| "sandbox".to_string())
            .parse()?;

        Ok(Config::builder()
            .client_id(client_id)
            .api_key(api_key)
            .environment(environment)
            .build()?)
    }

    /// Get the base URL for the configured environment.
    pub fn base_url(&self) -> &str {
        self.environment.base_url()
    }

    /// Get the API key (for internal use only).
    pub(crate) fn api_key(&self) -> &str {
        self.api_key.expose_secret()
    }
}

/// Builder for creating a [`Config`].
#[derive(Default)]
pub struct ConfigBuilder {
    client_id: Option<String>,
    api_key: Option<SecretString>,
    environment: Environment,
    api_version: Option<String>,
    timeout: Option<Duration>,
    token_refresh_buffer: Option<Duration>,
    on_behalf_of: Option<String>,
}

impl ConfigBuilder {
    /// Set the API client ID.
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set the API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(SecretString::new(api_key.into().into()));
        self
    }

    /// Set the environment.
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Set the API version to use.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the token refresh buffer.
    pub fn token_refresh_buffer(mut self, buffer: Duration) -> Self {
        self.token_refresh_buffer = Some(buffer);
        self
    }

    /// Set the account ID for connected account operations.
    pub fn on_behalf_of(mut self, account_id: impl Into<String>) -> Self {
        self.on_behalf_of = Some(account_id.into());
        self
    }

    /// Build the configuration.
    pub fn build(self) -> Result<Config> {
        let client_id = self
            .client_id
            .ok_or_else(|| Error::Config("client_id is required".to_string()))?;

        let api_key = self
            .api_key
            .ok_or_else(|| Error::Config("api_key is required".to_string()))?;

        Ok(Config {
            client_id,
            api_key,
            environment: self.environment,
            api_version: self.api_version.unwrap_or_else(|| DEFAULT_API_VERSION.to_string()),
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            token_refresh_buffer: self.token_refresh_buffer.unwrap_or(DEFAULT_TOKEN_REFRESH_BUFFER),
            on_behalf_of: self.on_behalf_of,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_str() {
        assert_eq!("sandbox".parse::<Environment>().unwrap(), Environment::Sandbox);
        assert_eq!("production".parse::<Environment>().unwrap(), Environment::Production);
        assert_eq!("SANDBOX".parse::<Environment>().unwrap(), Environment::Sandbox);
        assert_eq!("prod".parse::<Environment>().unwrap(), Environment::Production);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .client_id("test_client")
            .api_key("test_key")
            .environment(Environment::Sandbox)
            .build()
            .unwrap();

        assert_eq!(config.client_id, "test_client");
        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.environment, Environment::Sandbox);
        assert_eq!(config.api_version, DEFAULT_API_VERSION);
    }

    #[test]
    fn test_config_builder_missing_required() {
        let result = Config::builder().build();
        assert!(result.is_err());
    }
}
