//! Issuing Config models.
//!
//! Models for managing issuing configuration settings.

use serde::{Deserialize, Serialize};

/// Transaction scope types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionScope {
    /// All transactions.
    AllTransactions,
    /// Online transactions.
    OnlineTransaction,
    /// Contactless transactions.
    ContactlessTransaction,
    /// Contact chip transactions.
    ContactChipTransaction,
    /// Magstripe transactions.
    Magstripe,
    /// Cash withdrawal transactions.
    CashWithdrawal,
    /// Bill payment transactions.
    BillPayment,
    /// Account funding transactions.
    AccountFunding,
}

/// Usage scope types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsageScope {
    /// All (domestic and international).
    All,
    /// International only.
    International,
    /// Domestic only.
    Domestic,
}

/// Remote auth default action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteAuthDefaultAction {
    /// Authorize the transaction.
    Authorized,
    /// Decline the transaction.
    Declined,
}

/// Remote provisioning default action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteProvisioningDefaultAction {
    /// Unknown.
    Unknown,
    /// Conditional approval.
    ConditionalApproval,
    /// Generic decline.
    GenericDecline,
}

/// Blocked transaction usage setting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTransactionUsage {
    /// Transaction scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_scope: Option<TransactionScope>,
    /// Usage scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_scope: Option<UsageScope>,
}

/// Remote auth settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuthSettings {
    /// Whether remote auth is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The URL for the remote auth endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Default action when remote auth fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RemoteAuthDefaultAction>,
    /// Shared secret (only returned when initializing or requesting new secret).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// Creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Remote call method configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCallMethod {
    /// Name of the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path for the remote call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Remote call configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCallConfig {
    /// Base URL for remote calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Shared secret (only returned when initializing or requesting new secret).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// Individual method configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<RemoteCallMethod>>,
}

/// Remote provisioning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteProvisioningConfig {
    /// Whether remote provisioning is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Default action when remote provisioning fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RemoteProvisioningDefaultAction>,
}

/// Per-transaction limit setting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerTransactionLimit {
    /// Currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Default limit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f64>,
    /// Maximum limit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

/// Spending limit settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingLimitSettings {
    /// Per-transaction limits for supported currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_transaction_limits: Option<Vec<PerTransactionLimit>>,
}

/// Issuing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingConfig {
    /// Blocked transaction usage settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_transaction_usages: Option<Vec<BlockedTransactionUsage>>,
    /// Remote auth settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_auth_settings: Option<RemoteAuthSettings>,
    /// Remote call configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_call_config: Option<RemoteCallConfig>,
    /// Remote provisioning configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_provisioning_config: Option<RemoteProvisioningConfig>,
    /// Spending limit settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limit_settings: Option<SpendingLimitSettings>,
}

/// Remote auth update request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteAuthUpdate {
    /// Whether remote auth is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The URL for the remote auth endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Default action when remote auth fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    /// Whether to create a new shared secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_shared_secret: Option<bool>,
}

/// Remote call config update request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteCallConfigUpdate {
    /// Base URL for remote calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Shared secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// Individual method configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<RemoteCallMethod>>,
}

/// Remote provisioning config update request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteProvisioningConfigUpdate {
    /// Whether remote provisioning is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Default action when remote provisioning fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<RemoteProvisioningDefaultAction>,
}

/// Request to update issuing configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateIssuingConfigRequest {
    /// Remote auth update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_auth: Option<RemoteAuthUpdate>,
    /// Remote call config update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_call_config: Option<RemoteCallConfigUpdate>,
    /// Remote provisioning config update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_provisioning_config: Option<RemoteProvisioningConfigUpdate>,
}

impl UpdateIssuingConfigRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set remote auth configuration.
    pub fn remote_auth(mut self, config: RemoteAuthUpdate) -> Self {
        self.remote_auth = Some(config);
        self
    }

    /// Set remote call configuration.
    pub fn remote_call_config(mut self, config: RemoteCallConfigUpdate) -> Self {
        self.remote_call_config = Some(config);
        self
    }

    /// Set remote provisioning configuration.
    pub fn remote_provisioning_config(mut self, config: RemoteProvisioningConfigUpdate) -> Self {
        self.remote_provisioning_config = Some(config);
        self
    }
}

impl RemoteAuthUpdate {
    /// Create a new remote auth update.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable remote auth.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Set the remote auth URL.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the default action.
    pub fn default_action(mut self, action: impl Into<String>) -> Self {
        self.default_action = Some(action.into());
        self
    }

    /// Request a new shared secret.
    pub fn new_shared_secret(mut self, request_new: bool) -> Self {
        self.new_shared_secret = Some(request_new);
        self
    }
}
