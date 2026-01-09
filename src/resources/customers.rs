//! Customers resource for the Airwallex API.
//!
//! Manage customers for payment acceptance.

use crate::client::Client;
use crate::error::Result;
use crate::models::customers::{
    ClientSecretResponse, CreateCustomerRequest, Customer, ListCustomersParams,
    ListCustomersResponse, UpdateCustomerRequest,
};

/// The Customers resource.
pub struct Customers<'a> {
    client: &'a Client,
}

impl<'a> Customers<'a> {
    /// Create a new Customers resource.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List customers.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/customers`
    pub async fn list(&self, params: ListCustomersParams) -> Result<ListCustomersResponse> {
        self.client
            .get_with_query("/api/v1/pa/customers", &params)
            .await
    }

    /// Create a customer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/customers/create`
    pub async fn create(&self, request: CreateCustomerRequest) -> Result<Customer> {
        self.client
            .post("/api/v1/pa/customers/create", &request)
            .await
    }

    /// Get a customer by ID.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/customers/{id}`
    pub async fn get(&self, id: &str) -> Result<Customer> {
        self.client
            .get(&format!("/api/v1/pa/customers/{}", id))
            .await
    }

    /// Update a customer.
    ///
    /// # API Reference
    ///
    /// `POST /api/v1/pa/customers/{id}/update`
    pub async fn update(&self, id: &str, request: UpdateCustomerRequest) -> Result<Customer> {
        self.client
            .post(&format!("/api/v1/pa/customers/{}/update", id), &request)
            .await
    }

    /// Generate a client secret for a customer.
    ///
    /// # API Reference
    ///
    /// `GET /api/v1/pa/customers/{id}/generate_client_secret`
    pub async fn generate_client_secret(&self, id: &str) -> Result<ClientSecretResponse> {
        self.client
            .get(&format!(
                "/api/v1/pa/customers/{}/generate_client_secret",
                id
            ))
            .await
    }
}
