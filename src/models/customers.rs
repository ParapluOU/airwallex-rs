//! Customer models.
//!
//! Models for managing customers (for payment acceptance).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Merchant customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_customer_id: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CustomerAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Customer address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddress {
    /// Street address line 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
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

/// Request to create a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    /// Unique request ID.
    pub request_id: String,
    /// Merchant customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_customer_id: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CustomerAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl CreateCustomerRequest {
    /// Create a new request.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            merchant_customer_id: None,
            first_name: None,
            last_name: None,
            email: None,
            phone_number: None,
            business_name: None,
            address: None,
            additional_info: None,
            metadata: None,
        }
    }

    /// Set merchant customer ID.
    pub fn merchant_customer_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_customer_id = Some(id.into());
        self
    }

    /// Set first name.
    pub fn first_name(mut self, name: impl Into<String>) -> Self {
        self.first_name = Some(name.into());
        self
    }

    /// Set last name.
    pub fn last_name(mut self, name: impl Into<String>) -> Self {
        self.last_name = Some(name.into());
        self
    }

    /// Set email.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set phone number.
    pub fn phone_number(mut self, phone: impl Into<String>) -> Self {
        self.phone_number = Some(phone.into());
        self
    }

    /// Set business name.
    pub fn business_name(mut self, name: impl Into<String>) -> Self {
        self.business_name = Some(name.into());
        self
    }

    /// Set address.
    pub fn address(mut self, address: CustomerAddress) -> Self {
        self.address = Some(address);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to update a customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomerRequest {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CustomerAddress>,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Value>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl UpdateCustomerRequest {
    /// Create a new update request.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parameters for listing customers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCustomersParams {
    /// Filter by merchant customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_customer_id: Option<String>,
    /// Start date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_created_at: Option<String>,
    /// End date filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_created_at: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListCustomersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by merchant customer ID.
    pub fn merchant_customer_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_customer_id = Some(id.into());
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

/// Response for listing customers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomersResponse {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// List of customers.
    #[serde(default)]
    pub items: Vec<Customer>,
}

/// Client secret response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSecretResponse {
    /// Client secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Expiry time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
}
