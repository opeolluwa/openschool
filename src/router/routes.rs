use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::any;
use axum::{Json, Router};
use serde_json::json;

use crate::states::AppState;

pub fn load_routes(_app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/health", any(|| async { "Service healthy" }))
        .fallback(global_404_fallback)
}

async fn global_404_fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(
            json!({"message":&"the resource you're looking for does not exist or it has been permanently removed".to_string()}),
        ),
    )
}
