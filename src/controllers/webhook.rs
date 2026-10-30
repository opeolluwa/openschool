use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use sha2::Sha512;
use serde_json::json;

use crate::states::AppState;

type HmacSha512 = Hmac<Sha512>;

pub async fn paystack_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let signature = match headers.get("x-paystack-signature") {
        Some(sig) => match sig.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "invalid signature header"})),
                );
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "missing signature header"})),
            );
        }
    };

    let secret = state.config.paystack_api_secret.expose_secret();
    let mut mac = match HmacSha512::new_from_slice(secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => {
            tracing::error!("failed to create HMAC instance");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "internal error"})),
            );
        }
    };

    mac.update(&body);
    let expected = hex::encode(mac.finalize().into_bytes());

    if expected != signature {
        tracing::warn!("paystack webhook signature mismatch");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"message": "invalid signature"})),
        );
    }

    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "invalid JSON body"})),
            );
        }
    };

    let reference = match payload.get("data").and_then(|d| d.get("reference")).and_then(|r| r.as_str()) {
        Some(r) => r.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "missing data.reference"})),
            );
        }
    };

    match state.remittance.handle_webhook(&reference).await {
        Ok(transaction) => {
            tracing::info!(reference = %reference, status = ?transaction.status, "webhook processed");
            (StatusCode::OK, Json(json!({"message": "ok"})))
        }
        Err(err) => {
            tracing::error!(reference = %reference, error = %err, "webhook processing failed");
            (StatusCode::OK, Json(json!({"message": "ok"})))
        }
    }
}
