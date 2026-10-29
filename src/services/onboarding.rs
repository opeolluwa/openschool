use serde::Serialize;

use crate::{
    adapters::claims::{Claims, EntityKind, TokenType},
    errors::{
        authentication_error::AuthenticationError, database_error::DatabaseError,
        service_error::ServiceError,
    },
    extractors::admin::{CreateAdminUser, LoginAdminUser},
    models::admin_users,
    repositories::admin_user::{AdminUserRepository, AdminUserRepositoryExt},
    services::token_service::{TokenService, TokenServiceTrait},
};

pub struct OnboardingService {
    admin_user_repo: AdminUserRepository,
    token_service: TokenService,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait OnboardingServiceTrait {
    async fn create_user(
        &self,
        req: &CreateAdminUser,
    ) -> Result<admin_users::Model, ServiceError>;

    async fn login(&self, req: &LoginAdminUser) -> Result<LoginResponse, ServiceError>;
}

impl OnboardingService {
    pub fn new(admin_user_repo: AdminUserRepository, token_service: TokenService) -> Self {
        Self {
            admin_user_repo,
            token_service,
        }
    }
}

impl OnboardingServiceTrait for OnboardingService {
    async fn create_user(
        &self,
        req: &CreateAdminUser,
    ) -> Result<admin_users::Model, ServiceError> {
        if self.admin_user_repo.find_by_email(&req.email).await.is_ok() {
            return Err(ServiceError::DatabaseError(
                DatabaseError::DuplicateEmailForUser,
            ));
        }

        let hashed_password = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;
        let user = self
            .admin_user_repo
            .create(req, &hashed_password)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(user)
    }

    async fn login(&self, req: &LoginAdminUser) -> Result<LoginResponse, ServiceError> {
        let user = self
            .admin_user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|_| AuthenticationError::WrongCredentials)?;

        if user.is_blocked {
            return Err(AuthenticationError::AccountBlocked.into());
        }

        if !user.is_active {
            return Err(AuthenticationError::AccountBlocked.into());
        }

        let password_valid = bcrypt::verify(&req.password, &user.password)?;
        if !password_valid {
            return Err(AuthenticationError::WrongCredentials.into());
        }

        let identifier_bytes: [u8; 16] = user.identifier.as_slice().try_into().map_err(|_| {
            ServiceError::OperationFailed("invalid identifier length".to_string())
        })?;
        let identifier_str = ulid::Ulid::from_bytes(identifier_bytes).to_string();

        let claims = Claims {
            email: user.email.clone(),
            identifier: identifier_str,
            token_type: TokenType::Access,
            target_entity: EntityKind::User,
        };

        let access_token = self
            .token_service
            .generate_access_token(&claims)
            .map_err(ServiceError::AuthenticationError)?;

        let refresh_claims = Claims {
            token_type: TokenType::Refresh,
            ..claims
        };
        let refresh_token = self
            .token_service
            .generate_refresh_token(&refresh_claims)
            .map_err(ServiceError::AuthenticationError)?;

        let _ = self
            .admin_user_repo
            .record_last_login(&user.identifier)
            .await;

        Ok(LoginResponse {
            access_token: access_token.token,
            refresh_token: refresh_token.token,
            access_token_expiry: access_token.expires_at,
            refresh_token_expiry: refresh_token.expires_at,
        })
    }
}
