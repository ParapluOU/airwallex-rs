//! Issuing Config resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{IssuingConfig, UpdateIssuingConfigRequest};

/// Issuing Config resource for managing issuing settings.
#[derive(Debug)]
pub struct IssuingConfigResource<'a> {
    client: &'a Client,
}

impl<'a> IssuingConfigResource<'a> {
    /// Create a new Issuing Config resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieve the issuing configuration.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/issuing/config`
    pub async fn get(&self) -> Result<IssuingConfig> {
        self.client.get("/api/v1/issuing/config").await
    }

    /// Update the issuing configuration.
    ///
    /// Currently only supports remote call config updates.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/issuing/config/update`
    pub async fn update(&self, request: &UpdateIssuingConfigRequest) -> Result<IssuingConfig> {
        self.client
            .post("/api/v1/issuing/config/update", request)
            .await
    }
}
