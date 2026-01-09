//! Issuing Authorizations resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    IssuingAuthorization, ListIssuingAuthorizationsParams, ListIssuingAuthorizationsResponse,
};

/// Issuing Authorizations resource for viewing card authorizations.
#[derive(Debug)]
pub struct IssuingAuthorizations<'a> {
    client: &'a Client,
}

impl<'a> IssuingAuthorizations<'a> {
    /// Create a new Issuing Authorizations resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List issuing authorizations.
    ///
    /// Defaults to a 30 day period unless both from_created_at and to_created_at are provided.
    pub async fn list(
        &self,
        params: &ListIssuingAuthorizationsParams,
    ) -> Result<ListIssuingAuthorizationsResponse> {
        self.client
            .get_with_query("/api/v1/issuing/authorizations", params)
            .await
    }

    /// Get an authorization by ID.
    pub async fn get(&self, id: &str) -> Result<IssuingAuthorization> {
        self.client
            .get(&format!("/api/v1/issuing/authorizations/{}", id))
            .await
    }
}
