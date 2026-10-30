use std::sync::Arc;

use openschool_remittance::RemittanceEngine;
use sea_orm::DatabaseConnection;

use crate::{
    config::env::AppConfig,
    states::{repositories::Repositories, services::ServiceState},
};

pub struct AppState {
    pub services: Arc<ServiceState>,
    pub config: Arc<AppConfig>,
    pub remittance: Arc<RemittanceEngine>,
}

impl AppState {
    pub fn new(
        db_pool: &DatabaseConnection,
        app_config: &AppConfig,
        remittance: RemittanceEngine,
    ) -> Self {
        let repositories = Repositories::new(db_pool);
        let services = Arc::new(ServiceState::new(repositories, app_config));
        let config = Arc::new(app_config.clone());

        Self {
            services,
            config,
            remittance: Arc::new(remittance),
        }
    }
}
