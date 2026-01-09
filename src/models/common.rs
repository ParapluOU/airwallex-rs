//! Common types shared across multiple API resources.

use serde::{Deserialize, Serialize};

/// ISO 4217 currency code (3 letters).
pub type Currency = String;

/// ISO 3166-2 country code (2 letters).
pub type CountryCode = String;

/// A monetary amount with currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Amount {
    /// The currency code (ISO 4217).
    pub currency: Currency,
    /// The amount value.
    pub value: f64,
}

impl Amount {
    /// Create a new amount.
    pub fn new(currency: impl Into<String>, value: f64) -> Self {
        Self {
            currency: currency.into(),
            value,
        }
    }
}

/// A physical or mailing address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Address {
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country code (ISO 3166-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Postal/ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// State or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
}

/// Pagination information for list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Cursor for the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
}

/// A paginated list response.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Whether there are more results.
    #[serde(default)]
    pub has_more: bool,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Cursor for the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
}

/// Common query parameters for list endpoints.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListParams {
    /// Page number (0-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Cursor-based pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

impl ListParams {
    /// Create new list parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number.
    pub fn page_num(mut self, page: i32) -> Self {
        self.page_num = Some(page);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Set the pagination cursor.
    pub fn page(mut self, cursor: impl Into<String>) -> Self {
        self.page = Some(cursor.into());
        self
    }
}
