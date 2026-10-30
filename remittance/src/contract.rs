use serde::{Deserialize, Serialize};

use crate::error::PaymentError;

/// Anything that can process payments through an external provider.
///
/// Implementors handle the HTTP calls to a payment gateway. The engine
/// orchestrates persistence around these calls.
pub trait PaymentProvider: Send + Sync {
    /// Initialize a payment session and return an authorization URL
    /// the payer should be redirected to.
    async fn initialize_transaction(
        &self,
        request: &InitializeRequest,
    ) -> Result<InitializeResponse, PaymentError>;

    /// Verify a completed transaction by its unique reference.
    async fn verify_transaction(
        &self,
        reference: &str,
    ) -> Result<VerifyResponse, PaymentError>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub amount_minor: i64,
    pub currency: String,
    pub email: String,
    pub reference: String,
    pub callback_url: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub status: PaymentStatus,
    pub amount_minor: i64,
    pub currency: String,
    pub channel: Option<String>,
    pub paid_at: Option<String>,
    pub provider_reference: String,
    pub reference: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Success,
    Failed,
    Abandoned,
}
