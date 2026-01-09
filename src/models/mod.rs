//! Data models for the Airwallex API.

pub mod common;
pub mod balances;
pub mod global_accounts;
pub mod deposits;
pub mod beneficiaries;
pub mod transfers;
pub mod linked_accounts;
pub mod invoices;
pub mod payment_intents;
pub mod conversions;
pub mod customers;
pub mod refunds;

pub use common::*;
pub use balances::*;
pub use global_accounts::*;
pub use deposits::*;
pub use beneficiaries::*;
pub use transfers::*;
pub use linked_accounts::*;
pub use invoices::*;
pub use payment_intents::*;
pub use conversions::*;
pub use customers::*;
pub use refunds::*;
