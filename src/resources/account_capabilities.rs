//! Account Capabilities resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    AccountCapability, ApplyCapabilitiesRequest, ApplyCapabilitiesResponse,
    ListFundingLimitsParams, ListFundingLimitsResponse,
};

/// Account Capabilities resource for managing capabilities and funding limits.
#[derive(Debug)]
pub struct AccountCapabilities<'a> {
    client: &'a Client,
}

impl<'a> AccountCapabilities<'a> {
    /// Create a new Account Capabilities resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Apply for enhanced capabilities (e.g., higher funding limits).
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/account_capabilities/apply`
    pub async fn apply(
        &self,
        request: &ApplyCapabilitiesRequest,
    ) -> Result<ApplyCapabilitiesResponse> {
        self.client
            .post("/api/v1/account_capabilities/apply", request)
            .await
    }

    /// Get the status of a capability by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/account_capabilities/{id}`
    pub async fn get(&self, capability_id: &str) -> Result<AccountCapability> {
        self.client
            .get(&format!("/api/v1/account_capabilities/{}", capability_id))
            .await
    }

    /// Enable a specific capability.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/account_capabilities/{id}/enable`
    pub async fn enable(&self, capability_id: &str) -> Result<AccountCapability> {
        self.client
            .post_empty(&format!(
                "/api/v1/account_capabilities/{}/enable",
                capability_id
            ))
            .await
    }

    /// Get funding limits.
    ///
    /// Query the available funding limit or Faster Direct Debit limit.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/account_capabilities/funding_limits`
    pub async fn funding_limits(
        &self,
        params: &ListFundingLimitsParams,
    ) -> Result<ListFundingLimitsResponse> {
        self.client
            .get_with_query("/api/v1/account_capabilities/funding_limits", params)
            .await
    }
}
