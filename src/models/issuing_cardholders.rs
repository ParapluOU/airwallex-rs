//! Issuing Cardholders models.
//!
//! Models for managing cardholders in the Airwallex Issuing system.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A cardholder in the Airwallex Issuing system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cardholder {
    /// Unique identifier for cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<String>,
    /// Email address of the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Individual information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CardholderIndividual>,
    /// Mobile number of the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    /// Postal address for card delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<CardholderAddress>,
    /// Status of the cardholder (PENDING, READY, DISABLED, INCOMPLETE, DELETED).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type of cardholder (INDIVIDUAL or DELEGATE).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub cardholder_type: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Individual information for a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardholderIndividual {
    /// Residential address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CardholderAddress>,
    /// Date of birth in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Name information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CardholderName>,
    /// Nationality (2-letter ISO country code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Identification documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<CardholderIdentification>,
    /// Employer information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employers: Option<Vec<CardholderEmployer>>,
    /// Express consent obtained (must be "yes").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_consent_obtained: Option<String>,
    /// Canada only: Cardholder agreement terms consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_agreement_terms_consent_obtained: Option<String>,
    /// Canada only: Paperless notification consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paperless_notification_consent_obtained: Option<String>,
    /// Canada only: Privacy policy terms consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_terms_consent_obtained: Option<String>,
    /// Tax identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
}

/// Address for a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardholderAddress {
    /// City of address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postcode or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// State or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Name information for a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardholderName {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Identification document for a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardholderIdentification {
    /// ISO country code of identification document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Document number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Type of identification (PASSPORT, DRIVERS_LICENSE, ID_CARD).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    /// Expiry date in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// State (required for AU driver's license).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Gender (M/F, for UK driver's license).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// File ID for front of document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_front_file_id: Option<String>,
    /// File ID for back of document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_back_file_id: Option<String>,
}

/// Employer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardholderEmployer {
    /// Business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Business identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_identifiers: Option<Vec<BusinessIdentifier>>,
}

/// Business identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessIdentifier {
    /// Country code (2-letter ISO).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Registration number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Type (BRN, EIN, SSN, VAT).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<String>,
}

/// Request to create a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardholderRequest {
    /// Email address of the cardholder.
    pub email: String,
    /// Individual information.
    pub individual: CreateCardholderIndividual,
    /// Type of cardholder (INDIVIDUAL or DELEGATE).
    #[serde(rename = "type")]
    pub cardholder_type: String,
    /// Mobile number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    /// Postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<CardholderAddress>,
}

/// Individual info for creating a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardholderIndividual {
    /// Residential address (required).
    pub address: CardholderAddress,
    /// Date of birth in YYYY-MM-DD format (required).
    pub date_of_birth: String,
    /// Name information (required).
    pub name: CardholderName,
    /// Express consent obtained - must be "yes".
    pub express_consent_obtained: String,
    /// Nationality (2-letter ISO country code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Identification documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<CardholderIdentification>,
    /// Employer information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employers: Option<Vec<CardholderEmployer>>,
    /// Canada only: Cardholder agreement terms consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_agreement_terms_consent_obtained: Option<String>,
    /// Canada only: Paperless notification consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paperless_notification_consent_obtained: Option<String>,
    /// Canada only: Privacy policy terms consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_terms_consent_obtained: Option<String>,
    /// Tax identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
}

impl CreateCardholderRequest {
    /// Create a new cardholder request for an individual.
    pub fn individual(
        email: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        date_of_birth: impl Into<String>,
        address: CardholderAddress,
    ) -> Self {
        Self {
            email: email.into(),
            cardholder_type: "INDIVIDUAL".to_string(),
            individual: CreateCardholderIndividual {
                address,
                date_of_birth: date_of_birth.into(),
                name: CardholderName {
                    first_name: Some(first_name.into()),
                    last_name: Some(last_name.into()),
                    middle_name: None,
                    title: None,
                },
                express_consent_obtained: "yes".to_string(),
                nationality: None,
                identification: None,
                employers: None,
                cardholder_agreement_terms_consent_obtained: None,
                paperless_notification_consent_obtained: None,
                privacy_policy_terms_consent_obtained: None,
                tax_identification_number: None,
            },
            mobile_number: None,
            postal_address: None,
        }
    }

    /// Create a delegate cardholder request.
    pub fn delegate(
        email: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        date_of_birth: impl Into<String>,
        address: CardholderAddress,
    ) -> Self {
        let mut req = Self::individual(email, first_name, last_name, date_of_birth, address);
        req.cardholder_type = "DELEGATE".to_string();
        req
    }

    /// Set mobile number.
    pub fn mobile_number(mut self, number: impl Into<String>) -> Self {
        self.mobile_number = Some(number.into());
        self
    }

    /// Set postal address for card delivery.
    pub fn postal_address(mut self, address: CardholderAddress) -> Self {
        self.postal_address = Some(address);
        self
    }
}

impl CardholderAddress {
    /// Create a new address.
    pub fn new(
        line1: impl Into<String>,
        city: impl Into<String>,
        postcode: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        Self {
            line1: Some(line1.into()),
            city: Some(city.into()),
            postcode: Some(postcode.into()),
            country: Some(country.into()),
            line2: None,
            state: None,
        }
    }

    /// Set address line 2.
    pub fn line2(mut self, line2: impl Into<String>) -> Self {
        self.line2 = Some(line2.into());
        self
    }

    /// Set state or region.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }
}

/// Request to update a cardholder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardholderRequest {
    /// Individual information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Value>,
    /// Mobile number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    /// Postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<CardholderAddress>,
    /// Type of cardholder.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub cardholder_type: Option<String>,
}

impl UpdateCardholderRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set mobile number.
    pub fn mobile_number(mut self, number: impl Into<String>) -> Self {
        self.mobile_number = Some(number.into());
        self
    }

    /// Set postal address.
    pub fn postal_address(mut self, address: CardholderAddress) -> Self {
        self.postal_address = Some(address);
        self
    }
}

/// Parameters for listing cardholders.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCardholdersParams {
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Page number (starts from 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListCardholdersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by email.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
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

/// Response for listing cardholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCardholdersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of cardholders.
    #[serde(default)]
    pub items: Vec<Cardholder>,
}
