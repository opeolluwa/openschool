use serde::Serialize;

use crate::{
    adapters::claims::{Claims, EntityKind, TokenType},
    errors::{
        authentication_error::AuthenticationError, database_error::DatabaseError,
        service_error::ServiceError,
    },
    extractors::admin::{CreateAdminUser, LoginAdminUser},
    extractors::contact_address::ContactAddressPayload,
    extractors::institution::RegisterInstitution,
    extractors::onboarding::RegisterAcademics,
    models::{admin_users, contact_address, institution},
    models::sea_orm_active_enums::{
        AcademicCalendarType, GradingSystemType, InstitutionType,
    },
    repositories::academic_calendar::{AcademicCalendarRepository, AcademicCalendarRepositoryExt},
    repositories::admin_user::{AdminUserRepository, AdminUserRepositoryExt},
    repositories::contact_address::{ContactAddressRepository, ContactAddressRepositoryExt},
    repositories::grading_system::{GradingSystemRepository, GradingSystemRepositoryExt},
    repositories::institution::{InstitutionRepository, InstitutionRepositoryExt},
    services::token_service::{TokenService, TokenServiceTrait},
};

pub struct OnboardingService {
    admin_user_repo: AdminUserRepository,
    token_service: TokenService,
    institution_repo: InstitutionRepository,
    contact_address_repo: ContactAddressRepository,
    academic_calendar_repo: AcademicCalendarRepository,
    grading_system_repo: GradingSystemRepository,
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
    async fn create_admin_account(
        &self,
        req: &CreateAdminUser,
    ) -> Result<admin_users::Model, ServiceError>;

    async fn login(&self, req: &LoginAdminUser) -> Result<LoginResponse, ServiceError>;

    async fn register_institution(
        &self,
        req: &RegisterInstitution,
    ) -> Result<institution::Model, ServiceError>;

    async fn register_institution_address(
        &self,
        req: &ContactAddressPayload,
    ) -> Result<contact_address::Model, ServiceError>;

    async fn register_institution_academics(
        &self,
        req: &RegisterAcademics,
    ) -> Result<institution::Model, ServiceError>;
}

impl OnboardingService {
    pub fn new(
        admin_user_repo: AdminUserRepository,
        token_service: TokenService,
        institution_repo: InstitutionRepository,
        contact_address_repo: ContactAddressRepository,
        academic_calendar_repo: AcademicCalendarRepository,
        grading_system_repo: GradingSystemRepository,
    ) -> Self {
        Self {
            admin_user_repo,
            token_service,
            institution_repo,
            contact_address_repo,
            academic_calendar_repo,
            grading_system_repo,
        }
    }
}

fn parse_institution_type(s: &str) -> Result<InstitutionType, ServiceError> {
    match s {
        "higher_education" => Ok(InstitutionType::HigherEducation),
        "primary_and_secondary_education" => Ok(InstitutionType::PrimaryAndSecondaryEducation),
        "vocational_education" => Ok(InstitutionType::VocationalEducation),
        _ => Err(ServiceError::OperationFailed(format!(
            "invalid institution type: {s}"
        ))),
    }
}

fn parse_academic_calendar_type(s: &str) -> Result<AcademicCalendarType, ServiceError> {
    match s {
        "semester" => Ok(AcademicCalendarType::Semester),
        "terms" => Ok(AcademicCalendarType::Terms),
        "quarters" => Ok(AcademicCalendarType::Quaters),
        "self_paced" => Ok(AcademicCalendarType::SelfPaced),
        _ => Err(ServiceError::OperationFailed(format!(
            "invalid academic calendar type: {s}"
        ))),
    }
}

fn parse_grading_system_type(s: &str) -> Result<GradingSystemType, ServiceError> {
    match s {
        "gpa" => Ok(GradingSystemType::Gpa),
        "grades" => Ok(GradingSystemType::Grades),
        "competency" => Ok(GradingSystemType::Competency),
        "custom" => Ok(GradingSystemType::Custom),
        _ => Err(ServiceError::OperationFailed(format!(
            "invalid grading system type: {s}"
        ))),
    }
}

impl OnboardingServiceTrait for OnboardingService {
    async fn create_admin_account(
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

        let identifier_bytes: [u8; 16] =
            user.identifier.as_slice().try_into().map_err(|_| {
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

    async fn register_institution(
        &self,
        req: &RegisterInstitution,
    ) -> Result<institution::Model, ServiceError> {
        let institution_type = parse_institution_type(&req.institution_type)?;

        if let Some(existing) = self
            .institution_repo
            .find_singleton()
            .await
            .map_err(ServiceError::DatabaseError)?
        {
            return self
                .institution_repo
                .update(existing, req, institution_type)
                .await
                .map_err(ServiceError::DatabaseError);
        }

        self.institution_repo
            .create(req, institution_type)
            .await
            .map_err(ServiceError::DatabaseError)
    }

    async fn register_institution_address(
        &self,
        req: &ContactAddressPayload,
    ) -> Result<contact_address::Model, ServiceError> {
        let address = self
            .contact_address_repo
            .create_contact_address(req)
            .await
            .map_err(ServiceError::DatabaseError)?;

        let institution = self
            .institution_repo
            .find_singleton()
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| {
                ServiceError::OperationFailed(
                    "institution must be registered before adding an address".to_string(),
                )
            })?;

        self.institution_repo
            .update_address(&institution.identifier, &address.identifier)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(address)
    }

    async fn register_institution_academics(
        &self,
        req: &RegisterAcademics,
    ) -> Result<institution::Model, ServiceError> {
        let calendar_type = parse_academic_calendar_type(&req.academic_calendar_type)?;
        let grading_type = parse_grading_system_type(&req.grading_system_type)?;

        let calendar = self
            .academic_calendar_repo
            .create(&req.calendar_name, calendar_type)
            .await
            .map_err(ServiceError::DatabaseError)?;

        let grading = self
            .grading_system_repo
            .create(&req.grading_system_name, grading_type)
            .await
            .map_err(ServiceError::DatabaseError)?;

        let institution = self
            .institution_repo
            .find_singleton()
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| {
                ServiceError::OperationFailed(
                    "institution must be registered before adding academics".to_string(),
                )
            })?;

        self.institution_repo
            .update_academics(
                &institution.identifier,
                &grading.identifier,
                &calendar.identifier,
            )
            .await
            .map_err(ServiceError::DatabaseError)
    }
}
