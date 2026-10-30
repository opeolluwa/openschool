use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use openschool_models::grading_system;
use openschool_models::sea_orm_active_enums::GradingSystemType;
use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait GradingSystemRepositoryExt {
    async fn create(
        &self,
        name: &str,
        system_type: &GradingSystemType,
    ) -> Result<grading_system::Model, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct GradingSystemRepository {
    db: DatabaseConnection,
}

impl Repository for GradingSystemRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl GradingSystemRepositoryExt for GradingSystemRepository {
    async fn create(
        &self,
        name: &str,
        system_type: &GradingSystemType,
    ) -> Result<grading_system::Model, DatabaseError> {
        let model = grading_system::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            name: Set(name.to_owned()),
            grading_system_type: Set(system_type.to_owned()),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(DatabaseError::from)
    }
}
