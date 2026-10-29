use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    controllers::onboarding::{
        register_admin, register_institution, register_institution_academics,
        register_institution_address,
    },
    states::AppState,
};

pub(super) fn register_onboarding_routes(app_state: &Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/admin", post(register_admin))
        .route("/institution", post(register_institution))
        .route("/institution/address", post(register_institution_address))
        .route(
            "/institution/academics",
            post(register_institution_academics),
        )
        .with_state(app_state.clone());

    Router::new().nest("/onboarding", routes)
}
