# airwallex-rs

Rust client for the [Airwallex REST API](https://www.airwallex.com/docs/api).

> ### Looking for IT services?
> <img src="https://fromulo.com/codesociety.png" align="left" width="80" alt="CodeSociety">
>
> **[CodeSociety](https://codesocietyhub.com/)** is our consulting & contracting arm — specializing in
> **IT architecture**, **XML authoring systems**, **FontoXML integration**, and **TerminusDB consulting**.
> We build structured content platforms and data solutions that power digital publishing.
>
> **[Let's talk! &#8594;](https://codesocietyhub.com/contact.html)**

## Features

- Type-safe API client with async/await support
- Automatic token management and refresh
- Built-in retry logic with exponential backoff for rate limits
- Webhook signature verification (standard and remote authorization)
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
- `AIRWALLEX_LOGIN_AS` or `AIRWALLEX_SANDBOX_LOGIN_AS` - (Optional) Account ID for scoped API keys with multi-account access

Or configure programmatically:

```rust
use airwallex_rs::{Client, Config, Environment};

let config = Config::builder()
    .client_id("your_client_id")
    .api_key("your_api_key")
    .environment(Environment::Sandbox)
    // For scoped API keys with multi-account access:
    // .login_as("your_account_id")
    .build()?;

let client = Client::new(config)?;
```

## Webhook Verification

Verify webhook signatures to ensure events are from Airwallex:

```rust
use airwallex_rs::webhooks;

// Standard webhooks (x-timestamp + x-signature headers)
let secret = "whsec_your_webhook_secret";
let timestamp = "1357872222592";  // x-timestamp header
let signature = "abc123...";       // x-signature header
let payload = r#"{"name":"payment_intent.succeeded",...}"#;

if webhooks::verify_signature(secret, timestamp, payload, signature).is_ok() {
    // Signature is valid, process the webhook
    let event = webhooks::RawWebhookEvent::from_payload(payload)?;
    println!("Event: {}", event.name);
}

// Remote authorization webhooks for Issuing (x-nonce + x-signature headers)
let shared_secret = "your_shared_secret";
let nonce = "1650458086181.oIS519+CsXhPOM8X";  // x-nonce header
let signature = "3qBE5ZmGis7ZlTEwiGRJxgH8jgAsHoSoqM6VSp6paCM=";  // x-signature header

if webhooks::verify_remote_auth_signature(shared_secret, nonce, signature).is_ok() {
    // Process remote authorization request
}
```

## Implemented Resources (147 endpoints)

### Core Resources
- **Balances** - Current and history
- **Global Accounts** - Full CRUD, transactions, mandates, statement letters
- **Deposits** - Create, list, get
- **Linked Accounts** - Full CRUD, auth flows, microdeposit verification, mandates

### Payouts & Contacts
- **Beneficiaries** - Full CRUD, validation, account verification
- **Transfers** - Create, list, get, cancel, confirm funding
- **Batch Transfers** - Full lifecycle: create, list, get, add/delete items, quote, submit, delete
- **Payers** - Create, list, get

### Transactional FX
- **Conversions** - Create, list, get with rate quotes
- **Conversion Amendments** - Quote, create, list, get

### Payment Acceptance
- **Customers** - Full CRUD, client secret generation
- **Payment Intents** - Create, list, get, confirm, capture, cancel
- **Payment Methods** - Create, list, get, disable
- **Payment Consents** - Create, list, get, verify, disable
- **Payment Attempts** - List, get
- **Refunds** - Create, list, get
- **Payment Disputes** - List, get, accept, challenge
- **Payment Links** - Create, list, get, activate, deactivate, send, update
- **Settlements** - List, get, report
- **Payment Config** - Payment method types, banks

### Issuing
- **Issuing Cards** - Create, list, get, update, activate, details, limits
- **Issuing Cardholders** - Create, list, get, update
- **Issuing Transactions** - List, get
- **Issuing Authorizations** - List, get
- **Issuing Transaction Disputes** - Create, list, get, update, submit, cancel
- **Issuing Config** - Get, update

### Scale / Connected Accounts
- **Accounts** - Create, list, get, update, get own account
- **Account Capabilities** - Apply, get, enable, funding limits
- **Connected Account Transfers** - Create, list, get

### Finance & Reconciliation
- **Financial Transactions** - List
- **Reconciliation** - Treasury balances
- **Invoices** - List, get, items, preview

### Supporting Services
- **Reference Data** - Supported currencies

## API Coverage Summary

| Domain | Status |
|--------|--------|
| Foundation (Auth, Config, Errors) | ✅ Complete |
| Core Resources (Balances, Global Accounts, Deposits, Linked Accounts) | ✅ Complete |
| Payouts (Beneficiaries, Transfers, Batch Transfers, Payers) | ✅ Complete |
| Transactional FX (Conversions, Amendments) | ✅ Complete |
| Payment Acceptance (14 resources) | ✅ Complete |
| Issuing (6 resources) | ✅ Complete |
| Scale / Connected Accounts (3 resources) | ✅ Complete |
| Finance & Reconciliation | ✅ Complete |
| Webhook Verification | ✅ Complete |

## License

MIT
