use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    config::env::AppConfig,
    states::{repositories::Repositories, services::ServiceState},
};

pub struct AppState {
    pub services: Arc<ServiceState>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(db_pool: &DatabaseConnection, app_config: &AppConfig) -> Self {
        let repositories = Repositories::new(db_pool);
        let services = Arc::new(ServiceState::new(repositories));
        let config = Arc::new(app_config.clone());

        Self { services, config }
    }
}
