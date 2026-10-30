use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::{
    contract::{
        InitializeRequest, InitializeResponse, PaymentProvider, PaymentStatus, VerifyResponse,
    },
    error::PaymentError,
};

#[derive(Clone)]
pub struct PaystackProvider {
    client: Client,
    api_key: SecretString,
    base_url: String,
    verify_base_url: String,
}

impl PaystackProvider {
    pub fn new(
        api_key: SecretString,
        base_url: String,
        verify_base_url: String,
    ) -> Result<Self, PaymentError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|err| PaymentError::Unexpected(err.to_string()))?;

        Ok(Self {
            client,
            api_key,
            base_url,
            verify_base_url,
        })
    }
}

impl PaymentProvider for PaystackProvider {
    async fn initialize_transaction(
        &self,
        request: &InitializeRequest,
    ) -> Result<InitializeResponse, PaymentError> {
        let body = serde_json::json!({
            "amount": request.amount_minor,
            "email": request.email,
            "reference": request.reference,
            "currency": request.currency,
            "callback_url": request.callback_url,
            "metadata": request.metadata,
        });

        let response = self
            .client
            .post(&self.base_url)
            .bearer_auth(self.api_key.expose_secret())
            .json(&body)
            .send()
            .await
            .map_err(classify_reqwest)?;

        let status = response.status();
        if !status.is_success() {
            return Err(classify_http_status(status, &response.text().await.unwrap_or_default()));
        }

        let envelope: PaystackEnvelope<PaystackInitData> =
            response.json().await.map_err(|err| {
                PaymentError::Unexpected(format!("failed to parse response: {err}"))
            })?;

        Ok(InitializeResponse {
            authorization_url: envelope.data.authorization_url,
            access_code: envelope.data.access_code,
            reference: envelope.data.reference,
        })
    }

    async fn verify_transaction(
        &self,
        reference: &str,
    ) -> Result<VerifyResponse, PaymentError> {
        let url = format!("{}/{}", self.verify_base_url, reference);

        let response = self
            .client
            .get(&url)
            .bearer_auth(self.api_key.expose_secret())
            .send()
            .await
            .map_err(classify_reqwest)?;

        let status = response.status();
        if !status.is_success() {
            return Err(classify_http_status(status, &response.text().await.unwrap_or_default()));
        }

        let envelope: PaystackEnvelope<PaystackVerifyData> =
            response.json().await.map_err(|err| {
                PaymentError::Unexpected(format!("failed to parse response: {err}"))
            })?;

        let payment_status = match envelope.data.status.as_str() {
            "success" => PaymentStatus::Success,
            "abandoned" => PaymentStatus::Abandoned,
            _ => PaymentStatus::Failed,
        };

        Ok(VerifyResponse {
            status: payment_status,
            amount_minor: envelope.data.amount,
            currency: envelope.data.currency,
            channel: envelope.data.channel,
            paid_at: envelope.data.paid_at,
            provider_reference: envelope.data.id.to_string(),
            reference: envelope.data.reference,
        })
    }
}

fn classify_reqwest(err: reqwest::Error) -> PaymentError {
    if err.is_timeout() || err.is_connect() {
        PaymentError::ProviderUnavailable(err.to_string())
    } else {
        PaymentError::Unexpected(err.to_string())
    }
}

fn classify_http_status(status: reqwest::StatusCode, body: &str) -> PaymentError {
    match status.as_u16() {
        401 => PaymentError::Unauthorized,
        400 => PaymentError::InvalidRequest(body.to_string()),
        404 => PaymentError::NotFound,
        500..=599 => PaymentError::ProviderUnavailable(format!("server error {status}: {body}")),
        _ => PaymentError::Unexpected(format!("unexpected status {status}: {body}")),
    }
}

#[derive(Debug, Deserialize)]
struct PaystackEnvelope<T> {
    #[allow(dead_code)]
    status: bool,
    #[allow(dead_code)]
    message: String,
    data: T,
}

#[derive(Debug, Deserialize)]
struct PaystackInitData {
    authorization_url: String,
    access_code: String,
    reference: String,
}

#[derive(Debug, Deserialize)]
struct PaystackVerifyData {
    id: i64,
    status: String,
    reference: String,
    amount: i64,
    currency: String,
    channel: Option<String>,
    paid_at: Option<String>,
}
