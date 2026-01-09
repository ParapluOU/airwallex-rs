//! Beneficiary models.
//!
//! Models for managing payout beneficiaries (recipients of payments).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A beneficiary (payment recipient).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Beneficiary ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Beneficiary type (PERSONAL or COMPANY).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_type: Option<String>,
    /// Company name (for COMPANY type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// First name (for PERSONAL type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (for PERSONAL type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Bank details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_details: Option<BeneficiaryBankDetails>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BeneficiaryAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Digital wallet details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_wallet: Option<Value>,
    /// When the beneficiary was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the beneficiary was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Beneficiary bank details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryBankDetails {
    /// Account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// Account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_currency: Option<String>,
    /// Bank country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_country_code: Option<String>,
    /// Bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// SWIFT/BIC code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    /// IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// Local clearing system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_clearing_system: Option<String>,
    /// Account routing type 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_type1: Option<String>,
    /// Account routing value 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_value1: Option<String>,
    /// Account routing type 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_type2: Option<String>,
    /// Account routing value 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_routing_value2: Option<String>,
}

/// Beneficiary address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryAddress {
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State/province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postcode/ZIP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// Country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// Request to create a beneficiary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBeneficiaryRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Beneficiary type (PERSONAL or COMPANY).
    #[serde(rename = "type")]
    pub beneficiary_type: String,
    /// Company name (for COMPANY type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// First name (for PERSONAL type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (for PERSONAL type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Bank details.
    pub bank_details: BeneficiaryBankDetails,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BeneficiaryAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
}

impl CreateBeneficiaryRequest {
    /// Create a request for a personal beneficiary.
    pub fn personal(
        request_id: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        bank_details: BeneficiaryBankDetails,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            beneficiary_type: "PERSONAL".to_string(),
            company_name: None,
            first_name: Some(first_name.into()),
            last_name: Some(last_name.into()),
            entity_type: None,
            date_of_birth: None,
            bank_details,
            address: None,
            additional_info: None,
            payment_methods: None,
        }
    }

    /// Create a request for a company beneficiary.
    pub fn company(
        request_id: impl Into<String>,
        company_name: impl Into<String>,
        bank_details: BeneficiaryBankDetails,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            beneficiary_type: "COMPANY".to_string(),
            company_name: Some(company_name.into()),
            first_name: None,
            last_name: None,
            entity_type: None,
            date_of_birth: None,
            bank_details,
            address: None,
            additional_info: None,
            payment_methods: None,
        }
    }

    /// Set the address.
    pub fn address(mut self, address: BeneficiaryAddress) -> Self {
        self.address = Some(address);
        self
    }

    /// Set payment methods.
    pub fn payment_methods(mut self, methods: Vec<String>) -> Self {
        self.payment_methods = Some(methods);
        self
    }
}

/// Request to update a beneficiary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBeneficiaryRequest {
    /// Beneficiary type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_type: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Bank details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_details: Option<BeneficiaryBankDetails>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BeneficiaryAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
}

impl UpdateBeneficiaryRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Request to validate a beneficiary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBeneficiaryRequest {
    /// Beneficiary type.
    #[serde(rename = "type")]
    pub beneficiary_type: String,
    /// Bank details.
    pub bank_details: BeneficiaryBankDetails,
    /// Payment method.
    pub payment_method: String,
}

/// Response for beneficiary validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBeneficiaryResponse {
    /// Whether the beneficiary is valid.
    #[serde(default)]
    pub valid: bool,
    /// Validation errors.
    #[serde(default)]
    pub errors: Vec<ValidationError>,
}

/// A validation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Field with the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Request to verify a beneficiary account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountRequest {
    /// Account number.
    pub account_number: String,
    /// Bank country code.
    pub bank_country_code: String,
    /// Routing type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<String>,
    /// Routing value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_value: Option<String>,
}

/// Response for account verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountResponse {
    /// Verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Account holder name (if verified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

/// Parameters for listing beneficiaries.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListBeneficiariesParams {
    /// Filter by bank country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_country_code: Option<String>,
    /// Filter by account currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_currency: Option<String>,
    /// Filter by entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListBeneficiariesParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by bank country code.
    pub fn bank_country_code(mut self, code: impl Into<String>) -> Self {
        self.bank_country_code = Some(code.into());
        self
    }

    /// Filter by currency.
    pub fn account_currency(mut self, currency: impl Into<String>) -> Self {
        self.account_currency = Some(currency.into());
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

/// Response for listing beneficiaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBeneficiariesResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of beneficiaries.
    #[serde(default)]
    pub items: Vec<Beneficiary>,
}
