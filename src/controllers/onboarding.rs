use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

use crate::services::onboarding::OnboardingServiceTrait;
use crate::{
    errors::service_error::ServiceError, extractors::admin::CreateAdminUser,
    extractors::contact_address::ContactAddressPayload,
    extractors::institution::RegisterInstitution, extractors::onboarding::RegisterAcademics,
    middlewares::validator::ValidatedRequest, states::AppState,
};

pub async fn register_admin(
    State(app): State<Arc<AppState>>,
    ValidatedRequest(req): ValidatedRequest<CreateAdminUser>,
) -> Result<Json<Value>, ServiceError> {
    app.services.onboarding.create_admin_account(&req).await?;
    Ok(Json(json! ({"message":"account created successfully"})))
}

pub async fn register_institution(
    State(app): State<Arc<AppState>>,
    ValidatedRequest(req): ValidatedRequest<RegisterInstitution>,
) -> Result<Json<Value>, ServiceError> {
    app.services.onboarding.register_institution(&req).await?;
    Ok(Json(
        json! ({"message":"institution registered successfully"}),
    ))
}

pub async fn register_institution_address(
    State(app): State<Arc<AppState>>,
    ValidatedRequest(req): ValidatedRequest<ContactAddressPayload>,
) -> Result<Json<Value>, ServiceError> {
    app.services
        .onboarding
        .register_institution_address(&req)
        .await?;
    Ok(Json(json! ({"message":"address registered successfully"})))
}

pub async fn register_institution_academics(
    State(app): State<Arc<AppState>>,
    ValidatedRequest(req): ValidatedRequest<RegisterAcademics>,
) -> Result<Json<Value>, ServiceError> {
    app.services
        .onboarding
        .register_institution_academics(&req)
        .await?;
    Ok(Json(
        json! ({"message":"academics registered successfully"}),
    ))
}
