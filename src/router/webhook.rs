use std::sync::Arc;

use axum::{Router, routing::post};

use crate::controllers::webhook::paystack_webhook;
use crate::states::AppState;

pub fn register_webhook_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/webhooks/paystack", post(paystack_webhook))
        .with_state(Arc::clone(app_state))
}
