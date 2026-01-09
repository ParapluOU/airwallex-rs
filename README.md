# airwallex-rs

Rust client for the [Airwallex REST API](https://www.airwallex.com/docs/api).

**API Schema**: [schema.json](./schema.json) (371 endpoints across 94 resource groups)

## Features

- Type-safe API client with async/await support
- Automatic token management and refresh
- Built-in retry logic with exponential backoff for rate limits
- Support for all Airwallex API domains

## Usage

```rust
use airwallex_rs::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), airwallex_rs::Error> {
    let client = Client::from_env()?;

    // Get current balances
    let balances = client.balances().current().await?;
    println!("Balances: {:?}", balances);

    Ok(())
}
```

## Configuration

Set environment variables:
- `AIRWALLEX_CLIENT_ID` or `AIRWALLEX_SANDBOX_CLIENT_ID` - Your API client ID
- `AIRWALLEX_API_KEY` or `AIRWALLEX_SANDBOX_API_KEY` - Your API key
- `AIRWALLEX_ENVIRONMENT` - `sandbox` or `production` (default: `sandbox`)

Or configure programmatically:

```rust
use airwallex_rs::{Client, Config, Environment};

let config = Config::builder()
    .client_id("your_client_id")
    .api_key("your_api_key")
    .environment(Environment::Sandbox)
    .build()?;

let client = Client::new(config)?;
```

## Implementation Progress

### Phase 1: Foundation
- [x] Initialize Cargo project with dependencies
- [x] Define configuration struct (base URL, API version, timeouts)
- [x] Implement error types mapping Airwallex error codes
- [x] Create base HTTP client with request/response handling
- [x] Add authentication module (login endpoint, token storage, auto-refresh)
- [x] Implement request builder with headers (Authorization, x-api-version, x-on-behalf-of)

### Phase 2: Core Resources (22 endpoints)

#### Balances (2)
- [x] GET /api/v1/balances/current
- [ ] GET /api/v1/balances/history

#### Global Accounts (11)
- [ ] POST /api/v1/global_accounts/create
- [ ] GET /api/v1/global_accounts
- [ ] GET /api/v1/global_accounts/{id}
- [ ] POST /api/v1/global_accounts/{id}/update
- [ ] POST /api/v1/global_accounts/{id}/close
- [ ] POST /api/v1/global_accounts/{id}/generate_statement_letter
- [ ] GET /api/v1/global_accounts/{id}/transactions
- [ ] + 4 more

#### Deposits (1)
- [ ] GET /api/v1/deposits/{id}

#### Linked Accounts (13)
- [ ] POST /api/v1/linked_accounts/create
- [ ] GET /api/v1/linked_accounts
- [ ] GET /api/v1/linked_accounts/{id}
- [ ] POST /api/v1/linked_accounts/{id}/confirm
- [ ] POST /api/v1/linked_accounts/{id}/suspend
- [ ] GET /api/v1/linked_accounts/{id}/balances
- [ ] + 7 more

#### Direct Debits (3)
- [ ] All endpoints

#### Config (3)
- [ ] All endpoints

### Phase 3: Payouts & Contacts (34 endpoints)

#### Beneficiaries (7)
- [ ] POST /api/v1/beneficiaries/create
- [ ] GET /api/v1/beneficiaries
- [ ] GET /api/v1/beneficiaries/{id}
- [ ] POST /api/v1/beneficiaries/{id}/update
- [ ] POST /api/v1/beneficiaries/{id}/delete
- [ ] POST /api/v1/beneficiaries/validate
- [ ] + schema endpoints

#### Payers (6)
- [ ] All endpoints

#### Transfers (8)
- [ ] POST /api/v1/transfers/create
- [ ] POST /api/v1/transfers/validate
- [ ] GET /api/v1/transfers
- [ ] GET /api/v1/transfers/{id}
- [ ] POST /api/v1/transfers/{id}/cancel
- [ ] POST /api/v1/transfers/{id}/confirm_funding
- [ ] + 2 more

#### Batch Transfers (9)
- [ ] POST /api/v1/batch_transfers/create
- [ ] GET /api/v1/batch_transfers
- [ ] GET /api/v1/batch_transfers/{id}
- [ ] POST /api/v1/batch_transfers/{id}/add_items
- [ ] POST /api/v1/batch_transfers/{id}/delete_items
- [ ] GET /api/v1/batch_transfers/{id}/items
- [ ] POST /api/v1/batch_transfers/{id}/quote
- [ ] POST /api/v1/batch_transfers/{id}/submit
- [ ] POST /api/v1/batch_transfers/{id}/delete

#### Wallet Transfers (3)
- [ ] All endpoints

### Phase 4: Transactional FX (12 endpoints)

#### Rates (1)
- [ ] GET /api/v1/fx/rates/current

#### Quotes (2)
- [ ] POST /api/v1/fx/quotes/create
- [ ] GET /api/v1/fx/quotes/{id}

#### Conversions (3)
- [ ] POST /api/v1/fx/conversions/create
- [ ] GET /api/v1/fx/conversions
- [ ] GET /api/v1/fx/conversions/{id}

#### Conversion Amendments (4)
- [ ] POST /api/v1/fx/conversion_amendments/quote
- [ ] POST /api/v1/fx/conversion_amendments/create
- [ ] GET /api/v1/fx/conversion_amendments
- [ ] GET /api/v1/fx/conversion_amendments/{id}

#### LockFX & MarketFX
- [ ] Additional FX product endpoints

### Phase 5: Payment Acceptance (70+ endpoints)

#### Customers (5)
- [ ] POST /api/v1/pa/customers/create
- [ ] GET /api/v1/pa/customers
- [ ] GET /api/v1/pa/customers/{id}
- [ ] POST /api/v1/pa/customers/{id}/update
- [ ] POST /api/v1/pa/customers/{id}/generate_client_secret

#### Payment Intents (9)
- [ ] POST /api/v1/pa/payment_intents/create
- [ ] GET /api/v1/pa/payment_intents
- [ ] GET /api/v1/pa/payment_intents/{id}
- [ ] POST /api/v1/pa/payment_intents/{id}/confirm
- [ ] POST /api/v1/pa/payment_intents/{id}/capture
- [ ] POST /api/v1/pa/payment_intents/{id}/cancel
- [ ] + 3 more

#### Payment Methods (5)
- [ ] All endpoints

#### Payment Consents (7)
- [ ] All endpoints

#### Payment Attempts (2)
- [ ] GET /api/v1/pa/payment_attempts
- [ ] GET /api/v1/pa/payment_attempts/{id}

#### Refunds (3)
- [ ] POST /api/v1/pa/refunds/create
- [ ] GET /api/v1/pa/refunds
- [ ] GET /api/v1/pa/refunds/{id}

#### Payment Disputes (5)
- [ ] All endpoints

#### Payment Links (7)
- [ ] All endpoints

#### Terminals (9)
- [ ] All endpoints (physical POS)

#### Conversion Quotes (2)
- [ ] All endpoints

#### Customs Declarations (4)
- [ ] All endpoints

#### Funds Splits (4)
- [ ] All endpoints

#### Funds Split Reversals (3)
- [ ] All endpoints

#### Config (7)
- [ ] GET /api/v1/pa/config/payment_method_types
- [ ] GET /api/v1/pa/config/banks
- [ ] + 5 more

#### Settlement Records (1)
- [ ] All endpoints

#### Reference Data (1)
- [ ] Card bin lookups

### Phase 6: Issuing (33 endpoints)

#### Cardholders (5)
- [ ] POST /api/v1/issuing/cardholders/create
- [ ] GET /api/v1/issuing/cardholders
- [ ] GET /api/v1/issuing/cardholders/{id}
- [ ] POST /api/v1/issuing/cardholders/{id}/update
- [ ] + 1 more

#### Cards (7)
- [ ] POST /api/v1/issuing/cards/create
- [ ] GET /api/v1/issuing/cards
- [ ] GET /api/v1/issuing/cards/{id}
- [ ] POST /api/v1/issuing/cards/{id}/update
- [ ] POST /api/v1/issuing/cards/{id}/activate
- [ ] GET /api/v1/issuing/cards/{id}/details
- [ ] GET /api/v1/issuing/cards/{id}/limits

#### Transactions (2)
- [ ] GET /api/v1/issuing/transactions
- [ ] GET /api/v1/issuing/transactions/{id}

#### Transaction Lifecycles & Events
- [ ] Transaction lifecycle endpoints
- [ ] Transaction lifecycle events endpoints

#### Authorizations (2)
- [ ] GET /api/v1/issuing/authorizations
- [ ] GET /api/v1/issuing/authorizations/{id}

#### Transaction Disputes (6)
- [ ] All endpoints

#### Digital Wallet Tokens (2)
- [ ] All endpoints

#### Merchant Brands (2)
- [ ] All endpoints

#### Config (2)
- [ ] GET /api/v1/issuing/config
- [ ] POST /api/v1/issuing/config/update

### Phase 7: Scale / Connected Accounts (50+ endpoints)

#### Accounts (12)
- [ ] GET /api/v1/account
- [ ] POST /api/v1/accounts/create
- [ ] GET /api/v1/accounts
- [ ] GET /api/v1/accounts/{id}
- [ ] POST /api/v1/accounts/{id}/update
- [ ] + 7 more

#### Account Capabilities (4)
- [ ] All endpoints

#### Connected Account Transfers (3)
- [ ] POST /api/v1/connected_account_transfers/create
- [ ] GET /api/v1/connected_account_transfers
- [ ] GET /api/v1/connected_account_transfers/{id}

#### Charges (3)
- [ ] All endpoints

#### PSP Settlement Intents (6)
- [ ] All endpoints

#### PSP Settlement Splits (5)
- [ ] All endpoints

#### PSP Settlement Deposits (1)
- [ ] All endpoints

#### Platform Liquidity Programs (7)
- [ ] All endpoints

#### Platform Reports (2)
- [ ] All endpoints

#### Hosted Flows (3)
- [ ] All endpoints

#### Invitation Links (2)
- [ ] All endpoints

#### Embedded Components (1)
- [ ] All endpoints

### Phase 8: Billing (21 endpoints)

#### Products (4)
- [ ] All endpoints

#### Prices (4)
- [ ] All endpoints

#### Invoices (5)
- [ ] All endpoints

#### Subscriptions (7)
- [ ] All endpoints

### Phase 9: Spend Management (19 endpoints)

#### Vendors (4)
- [ ] All endpoints

#### Bills (3)
- [ ] All endpoints

#### Purchase Orders (4)
- [ ] All endpoints

#### Card Expenses (3)
- [ ] All endpoints

#### Reimbursements (5)
- [ ] All endpoints

### Phase 10: Risk & Compliance (11 endpoints)

#### Fraud Feedback (1)
- [ ] POST /api/v1/risk/issuing/fraud_feedback

#### Request for Information (RFI) (3)
- [ ] All endpoints

#### Subsellers (3)
- [ ] All endpoints

#### Watchlist (4)
- [ ] All endpoints

### Phase 11: Partner & Lending (20+ endpoints)

#### Partner Connections (2)
- [ ] All endpoints

#### Partner Transfers (3)
- [ ] All endpoints

#### Factor Managed Accounts (4)
- [ ] All endpoints

#### Loans (4)
- [ ] All endpoints

#### Loan Transactions (3)
- [ ] All endpoints

#### Borrower APIs (4)
- [ ] All endpoints

#### Lender APIs (1)
- [ ] All endpoints

### Phase 12: Finance & Reconciliation (15 endpoints)

#### Financial Transactions (2)
- [ ] GET /api/v1/financial_transactions
- [ ] + 1 more

#### Financial Reports (4)
- [ ] All endpoints

#### Settlement (3)
- [ ] All endpoints

#### Reconciliation (4)
- [ ] All endpoints

#### Reserves (2)
- [ ] All endpoints

### Phase 13: Supporting Services (30+ endpoints)

#### Authentication (6)
- [ ] POST /api/v1/authentication/login (implemented)
- [ ] OAuth2 endpoints (3)
- [ ] Partner API Access (1)
- [ ] Embedded Components auth (1)

#### Reference Data (7)
- [ ] GET /api/v1/reference/supported_currencies
- [ ] GET /api/v1/reference/banks
- [ ] + 5 more

#### Webhooks (OAuth Apps) (7)
- [ ] All endpoints

#### File Service (2)
- [ ] All endpoints

#### Developer Management (1)
- [ ] All endpoints

#### Confirmation Letter (1)
- [ ] All endpoints

#### Connected Stores (2)
- [ ] All endpoints

#### Auto Payments (7)
- [ ] All endpoints

### Phase 14: Simulation (Sandbox Only) (26 endpoints)

#### Accounts (3)
- [ ] All endpoints

#### Deposits (4)
- [ ] All endpoints

#### Issuing (6)
- [ ] All endpoints

#### Linked Accounts (4)
- [ ] All endpoints

#### Payment Acceptance (4)
- [ ] All endpoints

#### Payouts (1)
- [ ] All endpoints

#### Transfers (1)
- [ ] All endpoints

#### RFI (3)
- [ ] All endpoints

### Phase 15: Testing & Documentation
- [ ] Unit tests for all models (serialization/deserialization)
- [ ] Integration tests against sandbox environment
- [ ] Example applications
- [ ] API documentation with rustdoc

## API Coverage Summary

| Domain | Endpoints | Status |
|--------|-----------|--------|
| Foundation | N/A | Complete |
| Core Resources | 22 | Partial |
| Payouts & Contacts | 34 | Not started |
| Transactional FX | 12 | Not started |
| Payment Acceptance | 70+ | Not started |
| Issuing | 33 | Not started |
| Scale | 50+ | Not started |
| Billing | 21 | Not started |
| Spend Management | 19 | Not started |
| Risk & Compliance | 11 | Not started |
| Partner & Lending | 20+ | Not started |
| Finance & Reconciliation | 15 | Not started |
| Supporting Services | 30+ | Not started |
| Simulation (Sandbox) | 26 | Not started |
| **Total** | **371** | **~1%** |

## License

MIT
