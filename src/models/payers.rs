//! Payer models.
//!
//! Models for managing payers (contacts for transfers).

use serde::{Deserialize, Serialize};

/// A payer contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayerContact {
    /// Unique ID for this payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Nickname of the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Payer details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<Payer>,
}

/// Payer details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payer {
    /// Entity type (PERSONAL or COMPANY).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// First name (for personal payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (for personal payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name (for company payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Date of birth (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Address details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PayerAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<PayerAdditionalInfo>,
}

/// Payer address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayerAddress {
    /// Country code (2-letter ISO 3166-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State/province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
}

impl PayerAddress {
    /// Create a new address with required country code.
    pub fn new(country_code: impl Into<String>) -> Self {
        Self {
            country_code: Some(country_code.into()),
            city: None,
            state: None,
            postcode: None,
            street_address: None,
        }
    }

    /// Set city.
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Set state.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Set postcode.
    pub fn postcode(mut self, postcode: impl Into<String>) -> Self {
        self.postcode = Some(postcode.into());
        self
    }

    /// Set street address.
    pub fn street_address(mut self, address: impl Into<String>) -> Self {
        self.street_address = Some(address.into());
        self
    }
}

/// Payer additional info.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayerAdditionalInfo {
    /// Business registration number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_number: Option<String>,
    /// Business registration type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_type: Option<String>,
    /// External ID in your system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Personal email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_email: Option<String>,
    /// Personal ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_id_number: Option<String>,
}

/// Request to create a payer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayerRequest {
    /// Payer details.
    pub payer: CreatePayerDetails,
    /// Nickname for the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

/// Payer details for create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayerDetails {
    /// Entity type (PERSONAL or COMPANY) - required.
    pub entity_type: String,
    /// Address - required.
    pub address: PayerAddress,
    /// First name (for personal payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (for personal payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name (for company payers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Date of birth (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<PayerAdditionalInfo>,
}

impl CreatePayerRequest {
    /// Create a new personal payer request.
    pub fn personal(
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        address: PayerAddress,
    ) -> Self {
        Self {
            payer: CreatePayerDetails {
                entity_type: "PERSONAL".to_string(),
                address,
                first_name: Some(first_name.into()),
                last_name: Some(last_name.into()),
                company_name: None,
                date_of_birth: None,
                additional_info: None,
            },
            nickname: None,
        }
    }

    /// Create a new company payer request.
    pub fn company(company_name: impl Into<String>, address: PayerAddress) -> Self {
        Self {
            payer: CreatePayerDetails {
                entity_type: "COMPANY".to_string(),
                address,
                first_name: None,
                last_name: None,
                company_name: Some(company_name.into()),
                date_of_birth: None,
                additional_info: None,
            },
            nickname: None,
        }
    }

    /// Set nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Set date of birth.
    pub fn date_of_birth(mut self, dob: impl Into<String>) -> Self {
        self.payer.date_of_birth = Some(dob.into());
        self
    }

    /// Set additional info.
    pub fn additional_info(mut self, info: PayerAdditionalInfo) -> Self {
        self.payer.additional_info = Some(info);
        self
    }
}

/// Request to update a payer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayerRequest {
    /// Payer details.
    pub payer: CreatePayerDetails,
    /// Nickname for the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

/// Parameters for listing payers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPayersParams {
    /// Filter by entity type (PERSONAL or COMPANY).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Filter by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Start date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// End date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListPayersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by entity type.
    pub fn entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }

    /// Filter by name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Filter by nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nick_name = Some(nickname.into());
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

/// Response for listing payers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPayersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of payers.
    #[serde(default)]
    pub items: Vec<PayerContact>,
}
