//! Global Accounts models.
//!
//! Models for managing global accounts that can receive funds via local clearing
//! or SWIFT systems.

use serde::{Deserialize, Serialize};

/// A global account in the list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAccount {
    /// Global account ID.
    pub id: String,
    /// Global account name.
    pub account_name: String,
    /// Global account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Routing code type (e.g., sort_code, ach, bsb).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_type: Option<String>,
    /// Routing code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_value: Option<String>,
    /// Financial institution branch code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// Supported clearing systems (e.g., ACH, SEPA, Faster Payments).
    pub clearing_systems: Vec<String>,
    /// Country code (2-letter ISO 3166-2).
    pub country_code: String,
    /// Currency (3-letter ISO-4217).
    pub currency: String,
    /// Financial institution name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Nickname of the global account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Payment methods (LOCAL, SWIFT).
    pub payment_methods: Vec<String>,
    /// Unique request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Status (ACTIVE, INACTIVE, CLOSED).
    pub status: String,
    /// Bank SWIFT code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    /// Email for Interac e-Transfer (Canada).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_email: Option<String>,
    /// Alternate account identifiers for specific clearing systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_account_identifiers: Option<AlternateAccountIdentifiers>,
}

/// A detailed global account (returned from create/get endpoints).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveGlobalAccount {
    /// Global account ID.
    pub id: String,
    /// Global account name.
    pub account_name: String,
    /// Global account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type (Checking, Saving, Current).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Supported payment and collection capabilities.
    pub capability: GlobalAccountCapability,
    /// Country code (2-letter ISO 3166-2).
    pub country_code: String,
    /// Currency (3-letter ISO-4217).
    pub currency: String,
    /// Target currency for deposit conversion (ID only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_conversion_currency: Option<String>,
    /// IBAN number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// Financial institution details.
    pub institution: Institution,
    /// Nickname of the global account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Email for Interac e-Transfer (Canada).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_email: Option<String>,
    /// Unique request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Routing information.
    pub routing_codes: Vec<RoutingCode>,
    /// Status (ACTIVE, INACTIVE).
    pub status: String,
    /// Bank SWIFT code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    /// Alternate account identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_account_identifiers: Option<AlternateAccountIdentifiers>,
}

/// Capabilities of a global account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAccountCapability {
    /// Supported clearing systems.
    pub clearing_systems: Vec<String>,
    /// Supported payment methods.
    pub payment_methods: Vec<String>,
    /// Whether Direct Debit payout is supported.
    pub support_direct_debit: bool,
}

/// Financial institution details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    /// Institution address line.
    pub address: String,
    /// Branch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// City/town.
    pub city: String,
    /// Institution name.
    pub name: String,
    /// Zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

/// Routing code information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingCode {
    /// Routing code type.
    #[serde(rename = "type")]
    pub routing_type: RoutingCodeType,
    /// Routing code value.
    pub value: String,
}

/// Types of routing codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoutingCodeType {
    Ach,
    Bsb,
    SortCode,
    BankCode,
    BranchCode,
    Fedwire,
    TransitNumber,
    InstitutionNumber,
}

/// Alternate account identifiers for specific clearing systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateAccountIdentifiers {
    /// Identifiers list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<AlternateIdentifier>>,
}

/// An alternate identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateIdentifier {
    /// Clearing system name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_system: Option<String>,
    /// Account identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<String>,
}

/// Request to create a global account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGlobalAccountRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Country code (2-letter ISO 3166-2).
    pub country_code: String,
    /// Currency (3-letter ISO-4217).
    pub currency: String,
    /// Payment methods (LOCAL, SWIFT).
    pub payment_methods: Vec<String>,
    /// Nickname for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Target currency for deposit conversion (ID only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_conversion_currency: Option<String>,
    /// Alternate account identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_account_identifiers: Option<AlternateAccountIdentifiers>,
}

impl CreateGlobalAccountRequest {
    /// Create a new request with required fields.
    pub fn new(
        request_id: impl Into<String>,
        country_code: impl Into<String>,
        currency: impl Into<String>,
        payment_methods: Vec<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            country_code: country_code.into(),
            currency: currency.into(),
            payment_methods,
            nick_name: None,
            deposit_conversion_currency: None,
            alternate_account_identifiers: None,
        }
    }

    /// Set the nickname.
    pub fn nick_name(mut self, nick_name: impl Into<String>) -> Self {
        self.nick_name = Some(nick_name.into());
        self
    }

    /// Set the deposit conversion currency.
    pub fn deposit_conversion_currency(mut self, currency: impl Into<String>) -> Self {
        self.deposit_conversion_currency = Some(currency.into());
        self
    }
}

/// Request to update a global account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGlobalAccountRequest {
    /// New nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
}

impl UpdateGlobalAccountRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the nickname.
    pub fn nick_name(mut self, nick_name: impl Into<String>) -> Self {
        self.nick_name = Some(nick_name.into());
        self
    }
}

/// Parameters for listing global accounts.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListGlobalAccountsParams {
    /// Filter by country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Filter by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Start date for created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End date for created_at filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Results per page (default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListGlobalAccountsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by country code.
    pub fn country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
        self
    }

    /// Filter by currency.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Filter by nickname.
    pub fn nick_name(mut self, name: impl Into<String>) -> Self {
        self.nick_name = Some(name.into());
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set page number.
    pub fn page_num(mut self, num: i32) -> Self {
        self.page_num = Some(num);
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}

/// Response for listing global accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGlobalAccountsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of global accounts.
    #[serde(default)]
    pub items: Vec<GlobalAccount>,
}

/// Parameters for listing transactions.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListTransactionsParams {
    /// Start date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_post_at: Option<String>,
    /// End date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_post_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListTransactionsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set page number.
    pub fn page_num(mut self, num: i32) -> Self {
        self.page_num = Some(num);
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}

/// A transaction on a global account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAccountTransaction {
    /// Transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Posted timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_at: Option<String>,
    /// Sender name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    /// Sender reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_reference: Option<String>,
}

/// Response for listing transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTransactionsResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of transactions.
    #[serde(default)]
    pub items: Vec<GlobalAccountTransaction>,
}

/// Request to generate a statement letter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateStatementLetterRequest {
    /// Request ID.
    pub request_id: String,
}

impl GenerateStatementLetterRequest {
    /// Create a new request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}

/// Response for statement letter generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementLetterResponse {
    /// URL to download the statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Expiry time for the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

// Direct Debit Mandate types for global accounts

/// A direct debit mandate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    /// Mandate ID.
    pub id: String,
    /// Global account ID.
    pub global_account_id: String,
    /// Mandate status.
    pub status: String,
    /// Debtor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_name: Option<String>,
    /// Debtor account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_account_number: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Request to create a mandate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMandateRequest {
    /// Request ID.
    pub request_id: String,
    /// Debtor name.
    pub debtor_name: String,
    /// Debtor account number.
    pub debtor_account_number: String,
    /// Debtor routing code type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_routing_type: Option<String>,
    /// Debtor routing code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debtor_routing_value: Option<String>,
}

impl CreateMandateRequest {
    /// Create a new request.
    pub fn new(
        request_id: impl Into<String>,
        debtor_name: impl Into<String>,
        debtor_account_number: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            debtor_name: debtor_name.into(),
            debtor_account_number: debtor_account_number.into(),
            debtor_routing_type: None,
            debtor_routing_value: None,
        }
    }
}

/// Response for listing mandates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMandatesResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of mandates.
    #[serde(default)]
    pub items: Vec<Mandate>,
}
