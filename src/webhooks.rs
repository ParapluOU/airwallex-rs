//! Webhook signature verification utilities.
//!
//! Airwallex signs webhook events to allow you to verify that the events were sent by Airwallex.
//! This module provides utilities for verifying these signatures.
//!
//! # Standard Webhooks
//!
//! Standard webhook events use the `x-timestamp` and `x-signature` headers:
//!
//! ```no_run
//! use airwallex_rs::webhooks;
//!
//! let secret = "whsec_your_webhook_secret";
//! let timestamp = "1357872222592";  // x-timestamp header
//! let signature = "abc123...";       // x-signature header
//! let payload = r#"{"event": "..."}"#;
//!
//! if webhooks::verify_signature(secret, timestamp, payload, signature).is_ok() {
//!     // Signature is valid, process the webhook
//! }
//! ```
//!
//! # Remote Authorization Webhooks (Issuing)
//!
//! Remote authorization requests for card issuing use a different signature scheme
//! with `x-nonce` and `x-signature` headers:
//!
//! ```no_run
//! use airwallex_rs::webhooks;
//!
//! let shared_secret = "your_shared_secret";
//! let nonce = "1650458086181.oIS519+CsXhPOM8X";  // x-nonce header
//! let signature = "3qBE5ZmGis7ZlTEwiGRJxgH8jgAsHoSoqM6VSp6paCM=";  // x-signature header
//!
//! if webhooks::verify_remote_auth_signature(shared_secret, nonce, signature).is_ok() {
//!     // Signature is valid
//! }
//! ```

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Errors that can occur during webhook signature verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookError {
    /// The signature does not match the expected value.
    InvalidSignature,
    /// The timestamp is too old (potential replay attack).
    TimestampTooOld {
        /// The age of the webhook in seconds.
        age_seconds: u64,
        /// The maximum allowed age in seconds.
        tolerance_seconds: u64,
    },
    /// The timestamp is in the future.
    TimestampInFuture,
    /// Failed to parse the timestamp.
    InvalidTimestamp,
    /// Failed to parse the nonce format.
    InvalidNonce,
    /// HMAC computation failed.
    HmacError,
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::InvalidSignature => write!(f, "Invalid webhook signature"),
            WebhookError::TimestampTooOld {
                age_seconds,
                tolerance_seconds,
            } => write!(
                f,
                "Webhook timestamp too old: {} seconds (tolerance: {} seconds)",
                age_seconds, tolerance_seconds
            ),
            WebhookError::TimestampInFuture => write!(f, "Webhook timestamp is in the future"),
            WebhookError::InvalidTimestamp => write!(f, "Invalid timestamp format"),
            WebhookError::InvalidNonce => write!(f, "Invalid nonce format"),
            WebhookError::HmacError => write!(f, "HMAC computation failed"),
        }
    }
}

impl std::error::Error for WebhookError {}

/// Default timestamp tolerance (5 minutes).
pub const DEFAULT_TOLERANCE: Duration = Duration::from_secs(300);

/// Verify a standard webhook signature.
///
/// This verifies the signature of a webhook event using the `x-timestamp` and `x-signature` headers.
///
/// # Arguments
///
/// * `secret` - The webhook secret key for your notification URL
/// * `timestamp` - The `x-timestamp` header value (Unix timestamp in milliseconds)
/// * `payload` - The raw JSON payload body (must be the original, unmodified request body)
/// * `signature` - The `x-signature` header value
///
/// # Returns
///
/// `Ok(())` if the signature is valid, `Err(WebhookError)` otherwise.
///
/// # Example
///
/// ```no_run
/// use airwallex_rs::webhooks;
///
/// let secret = "whsec_your_webhook_secret";
/// let timestamp = "1357872222592";
/// let signature = "abc123def456...";
/// let payload = r#"{"name":"payment_intent.succeeded",...}"#;
///
/// match webhooks::verify_signature(secret, timestamp, payload, signature) {
///     Ok(()) => println!("Webhook verified!"),
///     Err(e) => println!("Verification failed: {}", e),
/// }
/// ```
pub fn verify_signature(
    secret: &str,
    timestamp: &str,
    payload: &str,
    signature: &str,
) -> Result<(), WebhookError> {
    verify_signature_with_tolerance(secret, timestamp, payload, signature, DEFAULT_TOLERANCE)
}

/// Verify a standard webhook signature with a custom timestamp tolerance.
///
/// # Arguments
///
/// * `secret` - The webhook secret key for your notification URL
/// * `timestamp` - The `x-timestamp` header value (Unix timestamp in milliseconds)
/// * `payload` - The raw JSON payload body
/// * `signature` - The `x-signature` header value
/// * `tolerance` - Maximum age allowed for the webhook timestamp
///
/// # Returns
///
/// `Ok(())` if the signature is valid, `Err(WebhookError)` otherwise.
pub fn verify_signature_with_tolerance(
    secret: &str,
    timestamp: &str,
    payload: &str,
    signature: &str,
    tolerance: Duration,
) -> Result<(), WebhookError> {
    // Verify timestamp is within tolerance
    verify_timestamp(timestamp, tolerance)?;

    // Compute expected signature
    let expected = compute_signature(secret, timestamp, payload)?;

    // Compare signatures using constant-time comparison
    if constant_time_compare(&expected, signature) {
        Ok(())
    } else {
        Err(WebhookError::InvalidSignature)
    }
}

/// Compute the expected webhook signature.
///
/// This can be useful for debugging or generating test signatures.
///
/// # Arguments
///
/// * `secret` - The webhook secret key
/// * `timestamp` - The timestamp value
/// * `payload` - The raw JSON payload body
///
/// # Returns
///
/// The hex-encoded HMAC-SHA256 signature.
pub fn compute_signature(
    secret: &str,
    timestamp: &str,
    payload: &str,
) -> Result<String, WebhookError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::HmacError)?;

    // value_to_digest = timestamp + payload
    mac.update(timestamp.as_bytes());
    mac.update(payload.as_bytes());

    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}

/// Verify a remote authorization webhook signature (for Issuing).
///
/// Remote authorization uses a different signature scheme with the `x-nonce` and `x-signature` headers.
/// The signature is a base64-encoded HMAC-SHA256 of the nonce.
///
/// # Arguments
///
/// * `shared_secret` - Your configured shared secret for remote authorization
/// * `nonce` - The `x-nonce` header value (format: `{timestamp}.{random_string}`)
/// * `signature` - The `x-signature` header value (base64-encoded)
///
/// # Returns
///
/// `Ok(())` if the signature is valid, `Err(WebhookError)` otherwise.
///
/// # Example
///
/// ```no_run
/// use airwallex_rs::webhooks;
///
/// let shared_secret = "your_shared_secret";
/// let nonce = "1650458086181.oIS519+CsXhPOM8X";
/// let signature = "3qBE5ZmGis7ZlTEwiGRJxgH8jgAsHoSoqM6VSp6paCM=";
///
/// match webhooks::verify_remote_auth_signature(shared_secret, nonce, signature) {
///     Ok(()) => println!("Remote auth signature verified!"),
///     Err(e) => println!("Verification failed: {}", e),
/// }
/// ```
pub fn verify_remote_auth_signature(
    shared_secret: &str,
    nonce: &str,
    signature: &str,
) -> Result<(), WebhookError> {
    verify_remote_auth_signature_with_tolerance(shared_secret, nonce, signature, DEFAULT_TOLERANCE)
}

/// Verify a remote authorization webhook signature with a custom timestamp tolerance.
///
/// # Arguments
///
/// * `shared_secret` - Your configured shared secret for remote authorization
/// * `nonce` - The `x-nonce` header value (format: `{timestamp}.{random_string}`)
/// * `signature` - The `x-signature` header value (base64-encoded)
/// * `tolerance` - Maximum age allowed for the timestamp in the nonce
///
/// # Returns
///
/// `Ok(())` if the signature is valid, `Err(WebhookError)` otherwise.
pub fn verify_remote_auth_signature_with_tolerance(
    shared_secret: &str,
    nonce: &str,
    signature: &str,
    tolerance: Duration,
) -> Result<(), WebhookError> {
    // Extract timestamp from nonce (format: {timestamp}.{random_string})
    let timestamp_str = nonce.split('.').next().ok_or(WebhookError::InvalidNonce)?;

    // Verify timestamp is within tolerance
    verify_timestamp(timestamp_str, tolerance)?;

    // Compute expected signature
    let expected = compute_remote_auth_signature(shared_secret, nonce)?;

    // Compare signatures using constant-time comparison
    if constant_time_compare(&expected, signature) {
        Ok(())
    } else {
        Err(WebhookError::InvalidSignature)
    }
}

/// Compute the expected remote authorization signature.
///
/// # Arguments
///
/// * `shared_secret` - The shared secret
/// * `nonce` - The nonce value
///
/// # Returns
///
/// The base64-encoded HMAC-SHA256 signature.
pub fn compute_remote_auth_signature(
    shared_secret: &str,
    nonce: &str,
) -> Result<String, WebhookError> {
    use base64::{engine::general_purpose::STANDARD, Engine};

    let mut mac = HmacSha256::new_from_slice(shared_secret.as_bytes())
        .map_err(|_| WebhookError::HmacError)?;

    mac.update(nonce.as_bytes());

    let result = mac.finalize();
    Ok(STANDARD.encode(result.into_bytes()))
}

/// Verify that a timestamp is within the allowed tolerance.
fn verify_timestamp(timestamp_ms_str: &str, tolerance: Duration) -> Result<(), WebhookError> {
    let timestamp_ms: u64 = timestamp_ms_str
        .parse()
        .map_err(|_| WebhookError::InvalidTimestamp)?;

    let webhook_time = UNIX_EPOCH + Duration::from_millis(timestamp_ms);
    let now = SystemTime::now();

    if webhook_time > now {
        // Allow a small grace period for clock skew (30 seconds)
        let grace_period = Duration::from_secs(30);
        if webhook_time > now + grace_period {
            return Err(WebhookError::TimestampInFuture);
        }
    }

    if let Ok(age) = now.duration_since(webhook_time) {
        if age > tolerance {
            return Err(WebhookError::TimestampTooOld {
                age_seconds: age.as_secs(),
                tolerance_seconds: tolerance.as_secs(),
            });
        }
    }

    Ok(())
}

/// Constant-time string comparison to prevent timing attacks.
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

/// Parsed webhook event with common fields.
#[derive(Debug, Clone)]
pub struct WebhookEvent<T> {
    /// The event name (e.g., "payment_intent.succeeded").
    pub name: String,
    /// The account ID associated with the event.
    pub account_id: Option<String>,
    /// The timestamp when the event occurred.
    pub created_at: Option<String>,
    /// The event data payload.
    pub data: T,
}

/// A raw webhook event where the data is an untyped JSON value.
pub type RawWebhookEvent = WebhookEvent<serde_json::Value>;

impl RawWebhookEvent {
    /// Parse a raw webhook payload into a RawWebhookEvent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use airwallex_rs::webhooks::RawWebhookEvent;
    ///
    /// let payload = r#"{"name":"payment_intent.succeeded","account_id":"acct_123","data":{...}}"#;
    /// let event = RawWebhookEvent::from_payload(payload).unwrap();
    /// println!("Event type: {}", event.name);
    /// ```
    pub fn from_payload(payload: &str) -> Result<Self, serde_json::Error> {
        #[derive(serde::Deserialize)]
        struct RawEvent {
            name: String,
            account_id: Option<String>,
            created_at: Option<String>,
            data: serde_json::Value,
        }

        let raw: RawEvent = serde_json::from_str(payload)?;
        Ok(Self {
            name: raw.name,
            account_id: raw.account_id,
            created_at: raw.created_at,
            data: raw.data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_signature() {
        let secret = "whsec_test_secret";
        let timestamp = "1357872222592";
        let payload = r#"{"name":"test.event","data":{}}"#;

        let signature = compute_signature(secret, timestamp, payload).unwrap();

        // Signature should be a hex string
        assert!(!signature.is_empty());
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));

        // Same inputs should produce same signature
        let signature2 = compute_signature(secret, timestamp, payload).unwrap();
        assert_eq!(signature, signature2);

        // Different secret should produce different signature
        let signature3 = compute_signature("different_secret", timestamp, payload).unwrap();
        assert_ne!(signature, signature3);
    }

    #[test]
    fn test_compute_remote_auth_signature() {
        let shared_secret = "test_shared_secret";
        let nonce = "1650458086181.oIS519+CsXhPOM8X";

        let signature = compute_remote_auth_signature(shared_secret, nonce).unwrap();

        // Signature should be a base64 string
        assert!(!signature.is_empty());
        assert!(signature.ends_with('=') || signature.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/'));

        // Same inputs should produce same signature
        let signature2 = compute_remote_auth_signature(shared_secret, nonce).unwrap();
        assert_eq!(signature, signature2);
    }

    #[test]
    fn test_verify_signature_with_fresh_timestamp() {
        let secret = "whsec_test_secret";
        let payload = r#"{"name":"test.event","data":{}}"#;

        // Use current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let signature = compute_signature(secret, &timestamp, payload).unwrap();

        // Should verify successfully
        assert!(verify_signature(secret, &timestamp, payload, &signature).is_ok());
    }

    #[test]
    fn test_verify_signature_invalid() {
        let secret = "whsec_test_secret";
        let payload = r#"{"name":"test.event","data":{}}"#;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        // Invalid signature should fail
        let result = verify_signature(secret, &timestamp, payload, "invalid_signature");
        assert!(matches!(result, Err(WebhookError::InvalidSignature)));
    }

    #[test]
    fn test_verify_signature_wrong_secret() {
        let secret = "whsec_test_secret";
        let wrong_secret = "whsec_wrong_secret";
        let payload = r#"{"name":"test.event","data":{}}"#;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let signature = compute_signature(secret, &timestamp, payload).unwrap();

        // Verification with wrong secret should fail
        let result = verify_signature(wrong_secret, &timestamp, payload, &signature);
        assert!(matches!(result, Err(WebhookError::InvalidSignature)));
    }

    #[test]
    fn test_verify_signature_old_timestamp() {
        let secret = "whsec_test_secret";
        let payload = r#"{"name":"test.event","data":{}}"#;

        // Timestamp from 10 minutes ago
        let old_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .saturating_sub(600_000)
            .to_string();

        let signature = compute_signature(secret, &old_timestamp, payload).unwrap();

        // Should fail due to old timestamp (default tolerance is 5 minutes)
        let result = verify_signature(secret, &old_timestamp, payload, &signature);
        assert!(matches!(result, Err(WebhookError::TimestampTooOld { .. })));
    }

    #[test]
    fn test_verify_remote_auth_signature_fresh() {
        let shared_secret = "test_shared_secret";

        // Use current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let nonce = format!("{}.randomstring123", timestamp);

        let signature = compute_remote_auth_signature(shared_secret, &nonce).unwrap();

        // Should verify successfully
        assert!(verify_remote_auth_signature(shared_secret, &nonce, &signature).is_ok());
    }

    #[test]
    fn test_verify_remote_auth_signature_invalid() {
        let shared_secret = "test_shared_secret";

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let nonce = format!("{}.randomstring123", timestamp);

        // Invalid signature should fail
        let result = verify_remote_auth_signature(shared_secret, &nonce, "invalid_signature");
        assert!(matches!(result, Err(WebhookError::InvalidSignature)));
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("abc", "abc"));
        assert!(!constant_time_compare("abc", "abd"));
        assert!(!constant_time_compare("abc", "ab"));
        assert!(!constant_time_compare("ab", "abc"));
        assert!(constant_time_compare("", ""));
    }

    #[test]
    fn test_raw_webhook_event_parsing() {
        let payload = r#"{"name":"payment_intent.succeeded","account_id":"acct_123","created_at":"2024-01-01T00:00:00Z","data":{"id":"pi_456"}}"#;

        let event = RawWebhookEvent::from_payload(payload).unwrap();
        assert_eq!(event.name, "payment_intent.succeeded");
        assert_eq!(event.account_id, Some("acct_123".to_string()));
        assert_eq!(event.data["id"], "pi_456");
    }
}
