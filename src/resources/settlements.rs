//! Settlements resource.

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    GetSettlementReportParams, ListSettlementsParams, ListSettlementsResponse, Settlement,
    SettlementReport,
};

/// Settlements resource for payment acceptance settlements.
#[derive(Debug)]
pub struct Settlements<'a> {
    client: &'a Client,
}

impl<'a> Settlements<'a> {
    /// Create a new Settlements resource.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List settlements.
    pub async fn list(&self, params: &ListSettlementsParams) -> Result<ListSettlementsResponse> {
        self.client
            .get_with_query("/api/v1/pa/financial/settlements", params)
            .await
    }

    /// Get a settlement by ID.
    pub async fn get(&self, id: &str) -> Result<Settlement> {
        self.client
            .get(&format!("/api/v1/pa/financial/settlements/{}", id))
            .await
    }

    /// Get a settlement report.
    pub async fn get_report(
        &self,
        id: &str,
        params: &GetSettlementReportParams,
    ) -> Result<SettlementReport> {
        self.client
            .get_with_query(
                &format!("/api/v1/pa/financial/settlements/{}/report", id),
                params,
            )
            .await
    }
}
