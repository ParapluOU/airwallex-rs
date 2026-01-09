# airwallex-rs

Rust client for the [Airwallex REST API](https://www.airwallex.com/docs/api).

## Features

- Type-safe API client with async/await support
- Automatic token management and refresh
- Built-in retry logic with exponential backoff for rate limits
- Support for all Airwallex API domains

## Usage

```rust
use airwallex::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), airwallex::Error> {
    let client = Client::from_env()?;

    // Get current balances
    let balances = client.balances().current().await?;
    println!("Balances: {:?}", balances);

    Ok(())
}
```

## Configuration

Set environment variables:
- `AIRWALLEX_CLIENT_ID` - Your API client ID
- `AIRWALLEX_API_KEY` - Your API key
- `AIRWALLEX_ENVIRONMENT` - `sandbox` or `production` (default: `sandbox`)

Or configure programmatically:

```rust
let config = Config::builder()
    .client_id("your_client_id")
    .api_key("your_api_key")
    .environment(Environment::Sandbox)
    .build();

let client = Client::new(config);
```

## Implementation Progress

### Phase 1: Foundation
- [ ] Initialize Cargo project with dependencies
- [ ] Define configuration struct (base URL, API version, timeouts)
- [ ] Implement error types mapping Airwallex error codes
- [ ] Create base HTTP client with request/response handling
- [ ] Add authentication module (login endpoint, token storage, auto-refresh)
- [ ] Implement request builder with headers (Authorization, x-api-version, x-on-behalf-of)

### Phase 2: Core Resources

#### Balances
- [ ] GET /api/v1/balances/current
- [ ] GET /api/v1/balances/history

#### Global Accounts
- [ ] POST /api/v1/global_accounts/create
- [ ] GET /api/v1/global_accounts
- [ ] GET /api/v1/global_accounts/{id}
- [ ] POST /api/v1/global_accounts/{id}/update
- [ ] POST /api/v1/global_accounts/{id}/close
- [ ] POST /api/v1/global_accounts/{id}/generate_statement_letter
- [ ] GET /api/v1/global_accounts/{id}/transactions

#### Deposits
- [ ] POST /api/v1/deposits/create
- [ ] GET /api/v1/deposits
- [ ] GET /api/v1/deposits/{id}

#### Linked Accounts
- [ ] POST /api/v1/linked_accounts/create
- [ ] GET /api/v1/linked_accounts
- [ ] GET /api/v1/linked_accounts/{id}
- [ ] POST /api/v1/linked_accounts/{id}/confirm
- [ ] POST /api/v1/linked_accounts/{id}/suspend
- [ ] GET /api/v1/linked_accounts/{id}/balances

### Phase 3: Payouts

#### Beneficiaries
- [ ] POST /api/v1/beneficiaries/create
- [ ] GET /api/v1/beneficiaries
- [ ] GET /api/v1/beneficiaries/{id}
- [ ] POST /api/v1/beneficiaries/{id}/update
- [ ] POST /api/v1/beneficiaries/{id}/delete
- [ ] POST /api/v1/beneficiaries/validate
- [ ] POST /api/v1/beneficiary_api_schemas/generate
- [ ] POST /api/v1/beneficiary_form_schemas/generate

#### Transfers
- [ ] POST /api/v1/transfers/create
- [ ] POST /api/v1/transfers/validate
- [ ] GET /api/v1/transfers
- [ ] GET /api/v1/transfers/{id}
- [ ] POST /api/v1/transfers/{id}/cancel
- [ ] POST /api/v1/transfers/{id}/confirm_funding

#### Batch Transfers
- [ ] POST /api/v1/batch_transfers/create
- [ ] GET /api/v1/batch_transfers
- [ ] GET /api/v1/batch_transfers/{id}
- [ ] POST /api/v1/batch_transfers/{id}/add_items
- [ ] POST /api/v1/batch_transfers/{id}/delete_items
- [ ] GET /api/v1/batch_transfers/{id}/items
- [ ] POST /api/v1/batch_transfers/{id}/quote
- [ ] POST /api/v1/batch_transfers/{id}/submit
- [ ] POST /api/v1/batch_transfers/{id}/delete

#### Payers
- [ ] POST /api/v1/payers/create
- [ ] GET /api/v1/payers
- [ ] GET /api/v1/payers/{id}

### Phase 4: Transactional FX

#### Rates
- [ ] GET /api/v1/fx/rates/current

#### Quotes
- [ ] POST /api/v1/fx/quotes/create
- [ ] GET /api/v1/fx/quotes/{id}

#### Conversions
- [ ] POST /api/v1/fx/conversions/create
- [ ] GET /api/v1/fx/conversions
- [ ] GET /api/v1/fx/conversions/{id}

#### Conversion Amendments
- [ ] POST /api/v1/fx/conversion_amendments/quote
- [ ] POST /api/v1/fx/conversion_amendments/create
- [ ] GET /api/v1/fx/conversion_amendments
- [ ] GET /api/v1/fx/conversion_amendments/{id}

### Phase 5: Payment Acceptance

#### Customers
- [ ] POST /api/v1/pa/customers/create
- [ ] GET /api/v1/pa/customers
- [ ] GET /api/v1/pa/customers/{id}
- [ ] POST /api/v1/pa/customers/{id}/update
- [ ] POST /api/v1/pa/customers/{id}/generate_client_secret

#### Payment Intents
- [ ] POST /api/v1/pa/payment_intents/create
- [ ] GET /api/v1/pa/payment_intents
- [ ] GET /api/v1/pa/payment_intents/{id}
- [ ] POST /api/v1/pa/payment_intents/{id}/confirm
- [ ] POST /api/v1/pa/payment_intents/{id}/capture
- [ ] POST /api/v1/pa/payment_intents/{id}/cancel

#### Payment Methods
- [ ] POST /api/v1/pa/payment_methods/create
- [ ] GET /api/v1/pa/payment_methods
- [ ] GET /api/v1/pa/payment_methods/{id}
- [ ] POST /api/v1/pa/payment_methods/{id}/disable

#### Payment Consents
- [ ] POST /api/v1/pa/payment_consents/create
- [ ] GET /api/v1/pa/payment_consents
- [ ] GET /api/v1/pa/payment_consents/{id}
- [ ] POST /api/v1/pa/payment_consents/{id}/verify
- [ ] POST /api/v1/pa/payment_consents/{id}/disable

#### Payment Attempts
- [ ] GET /api/v1/pa/payment_attempts
- [ ] GET /api/v1/pa/payment_attempts/{id}

#### Refunds
- [ ] POST /api/v1/pa/refunds/create
- [ ] GET /api/v1/pa/refunds
- [ ] GET /api/v1/pa/refunds/{id}

#### Payment Disputes
- [ ] GET /api/v1/pa/payment_disputes
- [ ] GET /api/v1/pa/payment_disputes/{id}
- [ ] POST /api/v1/pa/payment_disputes/{id}/accept
- [ ] POST /api/v1/pa/payment_disputes/{id}/challenge

#### Payment Links
- [ ] POST /api/v1/pa/payment_links/create
- [ ] GET /api/v1/pa/payment_links
- [ ] GET /api/v1/pa/payment_links/{id}
- [ ] POST /api/v1/pa/payment_links/{id}/deactivate

#### Settlements
- [ ] GET /api/v1/pa/financial/settlements
- [ ] GET /api/v1/pa/financial/settlements/{id}
- [ ] GET /api/v1/pa/financial/settlements/{id}/report

#### Config
- [ ] GET /api/v1/pa/config/payment_method_types
- [ ] GET /api/v1/pa/config/banks

### Phase 6: Issuing

#### Cardholders
- [ ] POST /api/v1/issuing/cardholders/create
- [ ] GET /api/v1/issuing/cardholders
- [ ] GET /api/v1/issuing/cardholders/{id}
- [ ] POST /api/v1/issuing/cardholders/{id}/update

#### Cards
- [ ] POST /api/v1/issuing/cards/create
- [ ] GET /api/v1/issuing/cards
- [ ] GET /api/v1/issuing/cards/{id}
- [ ] POST /api/v1/issuing/cards/{id}/update
- [ ] POST /api/v1/issuing/cards/{id}/activate
- [ ] GET /api/v1/issuing/cards/{id}/details
- [ ] GET /api/v1/issuing/cards/{id}/limits

#### Transactions
- [ ] GET /api/v1/issuing/transactions
- [ ] GET /api/v1/issuing/transactions/{id}

#### Authorizations
- [ ] GET /api/v1/issuing/authorizations
- [ ] GET /api/v1/issuing/authorizations/{id}

#### Transaction Disputes
- [ ] POST /api/v1/issuing/transaction_disputes/create
- [ ] GET /api/v1/issuing/transaction_disputes
- [ ] GET /api/v1/issuing/transaction_disputes/{id}
- [ ] POST /api/v1/issuing/transaction_disputes/{id}/update
- [ ] POST /api/v1/issuing/transaction_disputes/{id}/submit
- [ ] POST /api/v1/issuing/transaction_disputes/{id}/cancel

#### Config
- [ ] GET /api/v1/issuing/config
- [ ] POST /api/v1/issuing/config/update

### Phase 7: Scale (Connected Accounts)

#### Accounts
- [ ] GET /api/v1/account
- [ ] POST /api/v1/accounts/create
- [ ] GET /api/v1/accounts
- [ ] GET /api/v1/accounts/{id}
- [ ] POST /api/v1/accounts/{id}/update

#### Account Capabilities
- [ ] POST /api/v1/account_capabilities/apply
- [ ] GET /api/v1/account_capabilities/{id}
- [ ] POST /api/v1/account_capabilities/{id}/enable
- [ ] GET /api/v1/account_capabilities/funding_limits

#### Connected Account Transfers
- [ ] POST /api/v1/connected_account_transfers/create
- [ ] GET /api/v1/connected_account_transfers
- [ ] GET /api/v1/connected_account_transfers/{id}

### Phase 8: Finance & Supporting Services

#### Financial Transactions
- [ ] GET /api/v1/financial_transactions

#### Reconciliation
- [ ] GET /api/v1/tc/balances

#### Reference Data
- [ ] GET /api/v1/reference/supported_currencies
- [ ] GET /api/v1/reference/banks

#### Webhooks
- [ ] Signature verification utility
- [ ] Event type definitions and deserialization

### Phase 9: Testing & Documentation
- [ ] Unit tests for all models (serialization/deserialization)
- [ ] Integration tests against sandbox environment
- [ ] Example applications
- [ ] API documentation with rustdoc

## License

MIT
