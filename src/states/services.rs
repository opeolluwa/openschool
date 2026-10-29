use crate::config::env::AppConfig;
use crate::services::onboarding::OnboardingService;
use crate::services::token_service::TokenService;
use crate::states::repositories::Repositories;

pub struct ServiceState {
    pub repositories: Repositories,
    pub token_service: TokenService,
    pub onboarding: OnboardingService,
}

impl ServiceState {
    pub fn new(repositories: Repositories, config: &AppConfig) -> Self {
        let token_service = TokenService::new(
            config.access_token_ttl_secs.as_secs(),
            config.refresh_token_ttl_secs.as_secs(),
            config.jwt_signing_key.clone(),
        );

        let onboarding = OnboardingService::new(
            repositories.admin_user.clone(),
            token_service.clone(),
            repositories.institution.clone(),
            repositories.contact_address.clone(),
            repositories.academic_calendar.clone(),
            repositories.grading_system.clone(),
        );

        Self {
            repositories,
            token_service,
            onboarding,
        }
    }
}
