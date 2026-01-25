//! Integration tests against the Airwallex sandbox API.
//!
//! These tests require valid sandbox credentials in environment variables:
//! - AIRWALLEX_SANDBOX_CLIENT_ID or AIRWALLEX_CLIENT_ID
//! - AIRWALLEX_SANDBOX_API_KEY or AIRWALLEX_API_KEY
//! - AIRWALLEX_SANDBOX_LOGIN_AS or AIRWALLEX_LOGIN_AS (for scoped API keys with multi-account access)
//!
//! Run with: cargo test --test integration
//!
//! NOTE: If your scoped API key has access to multiple accounts or both organization-level
//! and account-level resources, you MUST set AIRWALLEX_SANDBOX_LOGIN_AS to your account ID.
//! Without this, the token defaults to organization-level access and account-level APIs
//! (like balances, customers, etc.) will fail with "Insufficient permissions".
//!
//! NOTE: Many tests require specific API key permissions in the Airwallex dashboard.
//! Tests will be skipped if the required permissions are not available.
//!
//! To enable all tests, ensure your API key has these scopes:
//! - balances:read
//! - global_accounts:read, global_accounts:write
//! - beneficiaries:read, beneficiaries:write
//! - transfers:read, transfers:write
//! - conversions:read, conversions:write
//! - customers:read, customers:write
//! - payment_intents:read, payment_intents:write
//! - refunds:read, refunds:write
//! - invoices:read
//! - linked_accounts:read
//! - deposits:read

use airwallex_rs::{
    Client, Error,
    models::{
        BalanceHistoryParams, CreateCustomerRequest, GetFxRateParams, ListAccountsParams,
        ListAmendmentsParams, ListBanksParams, ListBatchTransfersParams, ListBeneficiariesParams,
        ListCardholdersParams, ListCardsParams, ListConnectedAccountTransfersParams,
        ListConversionsParams, ListCustomersParams, ListDepositsParams,
        ListFinancialTransactionsParams, ListFundingLimitsParams, ListGlobalAccountsParams,
        ListInvoicesParams, ListIssuingAuthorizationsParams, ListIssuingTransactionDisputesParams,
        ListIssuingTransactionsParams, ListLinkedAccountsParams, ListPayersParams,
        ListPaymentAttemptsParams, ListPaymentConsentsParams, ListPaymentDisputesParams,
        ListPaymentIntentsParams, ListPaymentLinksParams, ListPaymentMethodTypesParams,
        ListPaymentMethodsParams, ListRefundsParams, ListSettlementsParams, ListTransfersParams,
        ListTreasuryBalancesParams,
    },
};

fn get_client() -> Client {
    dotenvy::dotenv().ok();
    Client::from_env().expect("Failed to create client from environment - check AIRWALLEX_SANDBOX_CLIENT_ID and AIRWALLEX_SANDBOX_API_KEY")
}

/// Check if an error is a permissions error (vs auth failure or other errors)
fn is_permission_error(e: &Error) -> bool {
    let err_str = format!("{:?}", e);
    err_str.contains("Insufficient permissions") || err_str.contains("unauthorized")
}

/// Check if an error is a real auth failure
fn is_auth_failure(e: &Error) -> bool {
    let err_str = format!("{:?}", e);
    err_str.contains("credentials_invalid") || err_str.contains("credentials_expired")
}

// ============================================================================
// Authentication - This test MUST pass
// ============================================================================

#[tokio::test]
async fn test_authentication_works() {
    let client = get_client();
    // Try to make any authenticated request
    let result = client.balances().current().await;

    match result {
        Ok(_) => {
            // Great, we have full access
        }
        Err(ref e) if is_permission_error(e) => {
            // Permission error means auth worked, just lacking scope
        }
        Err(ref e) if is_auth_failure(e) => {
            panic!(
                "Authentication failed - check your API credentials: {:?}",
                e
            );
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Balances
// ============================================================================

#[tokio::test]
async fn test_balances_current() {
    let client = get_client();
    let result = client.balances().current().await;

    match result {
        Ok(balances) => {
            println!("SUCCESS: Got {} balance entries", balances.items.len());
            for balance in &balances.items {
                println!(
                    "  {}: available={}",
                    balance.currency, balance.available_amount
                );
            }
            // Verify response structure
            assert!(balances.items.iter().all(|b| !b.currency.is_empty()));
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: balances:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_balances_history() {
    let client = get_client();
    let params = BalanceHistoryParams::new().page_size(10);
    let result = client.balances().history(params).await;

    match result {
        Ok(history) => {
            println!("SUCCESS: Got {} history entries", history.items.len());
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: balances:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Global Accounts
// ============================================================================

#[tokio::test]
async fn test_global_accounts_list() {
    let client = get_client();
    let params = ListGlobalAccountsParams::new().page_size(10);
    let result = client.global_accounts().list(params).await;

    match result {
        Ok(accounts) => {
            println!("SUCCESS: Got {} global accounts", accounts.items.len());
            for account in &accounts.items {
                println!(
                    "  {}: {} {} ({})",
                    account.id, account.currency, account.country_code, account.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: global_accounts:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Beneficiaries
// ============================================================================

#[tokio::test]
async fn test_beneficiaries_list() {
    let client = get_client();
    let params = ListBeneficiariesParams::new().page_size(10);
    let result = client.beneficiaries().list(params).await;

    match result {
        Ok(beneficiaries) => {
            println!("SUCCESS: Got {} beneficiaries", beneficiaries.items.len());
            for ben in &beneficiaries.items {
                println!("  {:?}: {:?} {:?}", ben.id, ben.first_name, ben.last_name);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: beneficiaries:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Transfers
// ============================================================================

#[tokio::test]
async fn test_transfers_list() {
    let client = get_client();
    let params = ListTransfersParams::new().page_size(10);
    let result = client.transfers().list(params).await;

    match result {
        Ok(transfers) => {
            println!("SUCCESS: Got {} transfers", transfers.items.len());
            for transfer in &transfers.items {
                println!(
                    "  {:?}: {:?} {:?} -> {:?}",
                    transfer.id, transfer.source_amount, transfer.source_currency, transfer.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: transfers:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Conversions
// ============================================================================

#[tokio::test]
async fn test_conversions_list() {
    let client = get_client();
    let params = ListConversionsParams::new().page_size(10);
    let result = client.conversions().list(params).await;

    match result {
        Ok(conversions) => {
            println!("SUCCESS: Got {} conversions", conversions.items.len());
            for conv in &conversions.items {
                println!(
                    "  {:?}: {:?} {:?} -> {:?} {:?}",
                    conv.conversion_id,
                    conv.sell_amount,
                    conv.sell_currency,
                    conv.buy_amount,
                    conv.buy_currency
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: conversions:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Customers (Payment Acceptance)
// ============================================================================

#[tokio::test]
async fn test_customers_list() {
    let client = get_client();
    let params = ListCustomersParams::new().page_size(10);
    let result = client.customers().list(params).await;

    match result {
        Ok(customers) => {
            println!("SUCCESS: Got {} customers", customers.items.len());
            for cust in &customers.items {
                println!("  {:?}: {:?} {:?}", cust.id, cust.first_name, cust.email);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: customers:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_customer_create_and_get() {
    let client = get_client();

    let request_id = format!("test-{}", uuid::Uuid::new_v4());
    let merchant_customer_id = format!("merchant-{}", uuid::Uuid::new_v4());
    let request = CreateCustomerRequest::new(&request_id)
        .merchant_customer_id(&merchant_customer_id)
        .first_name("Test")
        .last_name("Customer")
        .email("test@example.com");

    let result = client.customers().create(request).await;

    match result {
        Ok(customer) => {
            println!("SUCCESS: Created customer: {:?}", customer.id);
            assert!(customer.id.is_some());
            assert_eq!(customer.first_name.as_deref(), Some("Test"));
            assert_eq!(customer.last_name.as_deref(), Some("Customer"));

            // Now fetch it
            if let Some(id) = &customer.id {
                let get_result = client.customers().get(id).await;
                match get_result {
                    Ok(fetched) => {
                        assert_eq!(fetched.id, customer.id);
                        println!("SUCCESS: Fetched customer matches created customer");
                    }
                    Err(e) => panic!("Failed to fetch created customer: {:?}", e),
                }
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: customers:write permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Intents
// ============================================================================

#[tokio::test]
async fn test_payment_intents_list() {
    let client = get_client();
    let params = ListPaymentIntentsParams::new().page_size(10);
    let result = client.payment_intents().list(params).await;

    match result {
        Ok(intents) => {
            println!("SUCCESS: Got {} payment intents", intents.items.len());
            for intent in &intents.items {
                println!(
                    "  {:?}: {:?} {:?} ({})",
                    intent.id,
                    intent.amount,
                    intent.currency,
                    intent.status.as_deref().unwrap_or("unknown")
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_intents:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Invoices
// ============================================================================

#[tokio::test]
async fn test_invoices_list() {
    let client = get_client();
    let params = ListInvoicesParams::new().page_size(10);
    let result = client.invoices().list(params).await;

    match result {
        Ok(invoices) => {
            println!("SUCCESS: Got {} invoices", invoices.items.len());
            for invoice in &invoices.items {
                println!(
                    "  {:?}: {:?} {:?}",
                    invoice.id, invoice.currency, invoice.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: invoices:read permission not available");
        }
        Err(e) => {
            // Special case: this endpoint has API version requirements
            let err_str = format!("{:?}", e);
            if err_str.contains("API version") {
                println!("SKIPPED: invoices endpoint requires different API version");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Refunds
// ============================================================================

#[tokio::test]
async fn test_refunds_list() {
    let client = get_client();
    let params = ListRefundsParams::new().page_size(10);
    let result = client.refunds().list(params).await;

    match result {
        Ok(refunds) => {
            println!("SUCCESS: Got {} refunds", refunds.items.len());
            for refund in &refunds.items {
                println!("  {:?}: {:?} {:?}", refund.id, refund.amount, refund.status);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: refunds:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Deposits
// ============================================================================

#[tokio::test]
async fn test_deposits_list() {
    let client = get_client();
    let params = ListDepositsParams::new().page_num(0).page_size(10);
    let result = client.deposits().list(params).await;

    match result {
        Ok(deposits) => {
            println!("SUCCESS: Got {} deposits", deposits.items.len());
            for deposit in &deposits.items {
                println!(
                    "  {:?}: {:?} {}",
                    deposit.deposit_id, deposit.amount, deposit.currency
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: deposits:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Linked Accounts
// ============================================================================

#[tokio::test]
async fn test_linked_accounts_list() {
    let client = get_client();
    let params = ListLinkedAccountsParams::new().page_size(10);
    let result = client.linked_accounts().list(params).await;

    match result {
        Ok(accounts) => {
            println!("SUCCESS: Got {} linked accounts", accounts.items.len());
            for account in &accounts.items {
                println!(
                    "  {}: {} ({})",
                    account.id, account.account_type, account.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: linked_accounts:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payers
// ============================================================================

#[tokio::test]
async fn test_payers_list() {
    let client = get_client();
    let params = ListPayersParams::new().page_size(10);
    let result = client.payers().list(params).await;

    match result {
        Ok(payers) => {
            println!("SUCCESS: Got {} payers", payers.items.len());
            for payer_contact in &payers.items {
                let entity_type = payer_contact
                    .payer
                    .as_ref()
                    .and_then(|p| p.entity_type.as_deref());
                println!(
                    "  {:?}: {:?} ({:?})",
                    payer_contact.id, payer_contact.nickname, entity_type
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payers:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Batch Transfers
// ============================================================================

#[tokio::test]
async fn test_batch_transfers_list() {
    let client = get_client();
    let params = ListBatchTransfersParams::new().page_size(10);
    let result = client.batch_transfers().list(params).await;

    match result {
        Ok(batches) => {
            println!("SUCCESS: Got {} batch transfers", batches.items.len());
            for batch in &batches.items {
                println!("  {:?}: {:?} ({:?})", batch.id, batch.name, batch.status);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: batch_transfers:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Methods
// ============================================================================

#[tokio::test]
async fn test_payment_methods_list() {
    let client = get_client();
    let params = ListPaymentMethodsParams::new().page_size(10);
    let result = client.payment_methods().list(params).await;

    match result {
        Ok(methods) => {
            println!("SUCCESS: Got {} payment methods", methods.items.len());
            for method in &methods.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    method.id, method.payment_type, method.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_methods:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Consents
// ============================================================================

#[tokio::test]
async fn test_payment_consents_list() {
    let client = get_client();
    let params = ListPaymentConsentsParams::new().page_size(10);
    let result = client.payment_consents().list(params).await;

    match result {
        Ok(consents) => {
            println!("SUCCESS: Got {} payment consents", consents.items.len());
            for consent in &consents.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    consent.id, consent.next_triggered_by, consent.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_consents:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Financial Transactions
// ============================================================================

#[tokio::test]
async fn test_financial_transactions_list() {
    let client = get_client();
    let params = ListFinancialTransactionsParams::new().page_size(10);
    let result = client.financial_transactions().list(params).await;

    match result {
        Ok(transactions) => {
            println!(
                "SUCCESS: Got {} financial transactions",
                transactions.items.len()
            );
            for txn in &transactions.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    txn.id, txn.amount, txn.currency, txn.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: financial_transactions:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Links
// ============================================================================

#[tokio::test]
async fn test_payment_links_list() {
    let client = get_client();
    let params = ListPaymentLinksParams::new().page_size(10);
    let result = client.payment_links().list(&params).await;

    match result {
        Ok(links) => {
            println!("SUCCESS: Got {} payment links", links.items.len());
            for link in &links.items {
                println!("  {:?}: {:?} ({:?})", link.id, link.title, link.status);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_links:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Disputes
// ============================================================================

#[tokio::test]
async fn test_payment_disputes_list() {
    let client = get_client();
    let params = ListPaymentDisputesParams::new().size(10);
    let result = client.payment_disputes().list(&params).await;

    match result {
        Ok(disputes) => {
            println!("SUCCESS: Got {} payment disputes", disputes.items.len());
            for dispute in &disputes.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    dispute.id, dispute.amount, dispute.currency, dispute.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_disputes:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Accounts (Scale/Connected Accounts)
// ============================================================================

#[tokio::test]
async fn test_accounts_get_own() {
    let client = get_client();
    let result = client.accounts().get_own().await;

    match result {
        Ok(account) => {
            println!("SUCCESS: Got own account: {:?}", account.id);
            println!("  Nickname: {:?}", account.nickname);
            println!("  Status: {:?}", account.status);
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: accounts:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_accounts_list() {
    let client = get_client();
    let params = ListAccountsParams::new().page_size(10);
    let result = client.accounts().list(&params).await;

    match result {
        Ok(accounts) => {
            println!("SUCCESS: Got {} connected accounts", accounts.items.len());
            for account in &accounts.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    account.id, account.nickname, account.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: accounts:read permission not available (or not a platform)");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Issuing Cards
// ============================================================================

#[tokio::test]
async fn test_issuing_cards_list() {
    let client = get_client();
    let params = ListCardsParams::new().page_size(10);
    let result = client.issuing_cards().list(&params).await;

    match result {
        Ok(cards) => {
            println!("SUCCESS: Got {} issuing cards", cards.items.len());
            for card in &cards.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    card.card_id, card.nick_name, card.card_status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_cards:read permission not available");
        }
        Err(e) => {
            // Issuing may not be enabled for all accounts
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Issuing Cardholders
// ============================================================================

#[tokio::test]
async fn test_issuing_cardholders_list() {
    let client = get_client();
    let params = ListCardholdersParams::new().page_size(10);
    let result = client.issuing_cardholders().list(&params).await;

    match result {
        Ok(cardholders) => {
            println!(
                "SUCCESS: Got {} issuing cardholders",
                cardholders.items.len()
            );
            for cardholder in &cardholders.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    cardholder.cardholder_id, cardholder.email, cardholder.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_cardholders:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Issuing Transactions
// ============================================================================

#[tokio::test]
async fn test_issuing_transactions_list() {
    let client = get_client();
    let params = ListIssuingTransactionsParams::new().page_size(10);
    let result = client.issuing_transactions().list(&params).await;

    match result {
        Ok(transactions) => {
            println!(
                "SUCCESS: Got {} issuing transactions",
                transactions.items.len()
            );
            for txn in &transactions.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    txn.transaction_id,
                    txn.transaction_amount,
                    txn.transaction_currency,
                    txn.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_transactions:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Issuing Authorizations
// ============================================================================

#[tokio::test]
async fn test_issuing_authorizations_list() {
    let client = get_client();
    let params = ListIssuingAuthorizationsParams::new().page_size(10);
    let result = client.issuing_authorizations().list(&params).await;

    match result {
        Ok(authorizations) => {
            println!(
                "SUCCESS: Got {} issuing authorizations",
                authorizations.items.len()
            );
            for auth in &authorizations.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    auth.transaction_id,
                    auth.transaction_amount,
                    auth.transaction_currency,
                    auth.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_authorizations:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Connected Account Transfers
// ============================================================================

#[tokio::test]
async fn test_connected_account_transfers_list() {
    let client = get_client();
    let params = ListConnectedAccountTransfersParams::new()
        .page_num(0)
        .page_size(10);
    let result = client.connected_account_transfers().list(&params).await;

    match result {
        Ok(transfers) => {
            println!(
                "SUCCESS: Got {} connected account transfers",
                transfers.items.len()
            );
            for transfer in &transfers.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    transfer.id, transfer.amount, transfer.currency, transfer.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: connected_account_transfers:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled")
                || err_str.contains("forbidden")
                || err_str.contains("Scale")
            {
                println!("SKIPPED: Scale/Connected Accounts feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// FX Rates
// ============================================================================

#[tokio::test]
async fn test_fx_get_rate() {
    let client = get_client();
    let params = GetFxRateParams::new("USD", "EUR").buy_amount(1000.0);
    let result = client.conversions().get_rate(&params).await;

    match result {
        Ok(rate) => {
            println!("SUCCESS: Got FX rate");
            println!("  Rate: {:?}", rate.rate);
            println!("  Currency pair: {:?}", rate.currency_pair);
            println!("  Conversion date: {:?}", rate.conversion_date);
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: fx:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Payment Attempts
// ============================================================================

#[tokio::test]
async fn test_payment_attempts_list() {
    let client = get_client();
    let params = ListPaymentAttemptsParams::new().page_size(10);
    let result = client.payment_attempts().list(&params).await;

    match result {
        Ok(attempts) => {
            println!("SUCCESS: Got {} payment attempts", attempts.items.len());
            for attempt in &attempts.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    attempt.id, attempt.amount, attempt.currency, attempt.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_attempts:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Settlements
// ============================================================================

#[tokio::test]
async fn test_settlements_list() {
    let client = get_client();
    // Settlements require date range in ISO date format (YYYY-MM-DD)
    let params =
        ListSettlementsParams::new("USD", "SETTLED", "2024-01-01", "2025-12-31").page_size(10);
    let result = client.settlements().list(&params).await;

    match result {
        Ok(settlements) => {
            println!("SUCCESS: Got {} settlements", settlements.items.len());
            for settlement in &settlements.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    settlement.id, settlement.amount, settlement.currency, settlement.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: settlements:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Issuing Transaction Disputes
// ============================================================================

#[tokio::test]
async fn test_issuing_transaction_disputes_list() {
    let client = get_client();
    let params = ListIssuingTransactionDisputesParams::new().page_size(10);
    let result = client.issuing_transaction_disputes().list(&params).await;

    match result {
        Ok(disputes) => {
            println!(
                "SUCCESS: Got {} issuing transaction disputes",
                disputes.items.len()
            );
            for dispute in &disputes.items {
                println!(
                    "  {:?}: {:?} ({:?})",
                    dispute.id, dispute.amount, dispute.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_transaction_disputes:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Reference Data
// ============================================================================

#[tokio::test]
async fn test_reference_data_currencies() {
    let client = get_client();
    let result = client.reference_data().supported_currencies().await;

    match result {
        Ok(currencies) => {
            println!("SUCCESS: Got supported currencies");
            if let Some(conversion) = &currencies.conversion {
                println!("  Buy currencies: {:?}", conversion.buy_currencies);
                println!("  Sell currencies: {:?}", conversion.sell_currencies);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: reference_data:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Issuing Config
// ============================================================================

#[tokio::test]
async fn test_issuing_config_get() {
    let client = get_client();
    let result = client.issuing_config().get().await;

    match result {
        Ok(config) => {
            println!("SUCCESS: Got issuing config");
            println!(
                "  Remote auth settings: {:?}",
                config.remote_auth_settings.is_some()
            );
            println!(
                "  Spending limit settings: {:?}",
                config.spending_limit_settings.is_some()
            );
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: issuing_config:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("forbidden") {
                println!("SKIPPED: Issuing feature not enabled for this account");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Payment Config
// ============================================================================

#[tokio::test]
async fn test_payment_config_method_types() {
    let client = get_client();
    let params = ListPaymentMethodTypesParams::new().page_size(10);
    let result = client.payment_config().payment_method_types(&params).await;

    match result {
        Ok(types) => {
            println!("SUCCESS: Got {} payment method types", types.items.len());
            for pmt in &types.items {
                println!("  {:?}: active={:?}", pmt.name, pmt.active);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_config:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_payment_config_banks() {
    let client = get_client();
    // Banks endpoint requires payment_method_type and country_code for bank_transfer/online_banking
    let params = ListBanksParams::new("online_banking").country_code("NL");
    let result = client.payment_config().banks(&params).await;

    match result {
        Ok(banks) => {
            println!("SUCCESS: Got {} banks", banks.items.len());
            for bank in &banks.items {
                println!("  {:?}: {:?}", bank.bank_name, bank.display_name);
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: payment_config:read permission not available");
        }
        Err(e) => {
            // Banks endpoint may not return data for all payment method types/countries
            let err_str = format!("{:?}", e);
            if err_str.contains("not supported")
                || err_str.contains("invalid")
                || err_str.contains("not available")
            {
                println!("SKIPPED: online_banking for NL may not be available");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Reconciliation (Treasury Balances)
// ============================================================================

#[tokio::test]
async fn test_reconciliation_balances() {
    let client = get_client();
    let params = ListTreasuryBalancesParams::new()
        .currency("USD")
        .page_size(10);
    let result = client.reconciliation().balances(&params).await;

    match result {
        Ok(balances) => {
            println!(
                "SUCCESS: Got {} treasury balance entries",
                balances.items.len()
            );
            for balance in &balances.items {
                println!(
                    "  {:?}: {:?} {:?} ({:?})",
                    balance.id, balance.amount, balance.currency, balance.balance_type
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: reconciliation:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

// ============================================================================
// Account Capabilities
// ============================================================================

#[tokio::test]
async fn test_account_capabilities_funding_limits() {
    let client = get_client();
    let params = ListFundingLimitsParams::new();
    let result = client.account_capabilities().funding_limits(&params).await;

    match result {
        Ok(limits) => {
            println!("SUCCESS: Got {} funding limits", limits.items.len());
            for limit in &limits.items {
                println!(
                    "  {:?}: limit={:?} ({:?})",
                    limit.currency, limit.limit, limit.status
                );
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: account_capabilities:read permission not available");
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("not enabled") || err_str.contains("Scale") {
                println!("SKIPPED: Account capabilities feature not enabled");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

// ============================================================================
// Conversion Amendments
// ============================================================================

#[tokio::test]
async fn test_conversion_amendments_list() {
    let client = get_client();

    // First, get a conversion to list amendments for
    let conv_params = ListConversionsParams::new().page_size(10);
    let conv_result = client.conversions().list(conv_params).await;

    match conv_result {
        Ok(conversions) => {
            if conversions.items.is_empty() {
                println!("SKIPPED: No conversions available to test amendments");
                return;
            }

            if let Some(conversion_id) = &conversions.items[0].conversion_id {
                let params = ListAmendmentsParams::new(conversion_id);
                let result = client.conversion_amendments().list(&params).await;

                match result {
                    Ok(amendments) => {
                        println!(
                            "SUCCESS: Got {} conversion amendments for conversion {}",
                            amendments.items.len(),
                            conversion_id
                        );
                        for amendment in &amendments.items {
                            println!(
                                "  {:?}: {:?}",
                                amendment.amendment_id, amendment.amendment_type
                            );
                        }
                    }
                    Err(ref e) if is_permission_error(e) => {
                        println!("SKIPPED: conversion_amendments:read permission not available");
                    }
                    Err(e) => {
                        panic!("Unexpected error: {:?}", e);
                    }
                }
            } else {
                println!("SKIPPED: Conversion has no ID");
            }
        }
        Err(ref e) if is_permission_error(e) => {
            println!("SKIPPED: conversions:read permission not available");
        }
        Err(e) => {
            panic!("Unexpected error getting conversions: {:?}", e);
        }
    }
}
