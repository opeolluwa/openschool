use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use openschool_models::academic_calendar;
use openschool_models::sea_orm_active_enums::AcademicCalendarType;
use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait AcademicCalendarRepositoryExt {
    async fn create(
        &self,
        name: &str,
        calendar_type: &AcademicCalendarType,
    ) -> Result<academic_calendar::Model, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct AcademicCalendarRepository {
    db: DatabaseConnection,
}

impl Repository for AcademicCalendarRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl AcademicCalendarRepositoryExt for AcademicCalendarRepository {
    async fn create(
        &self,
        name: &str,
        calendar_type: &AcademicCalendarType,
    ) -> Result<academic_calendar::Model, DatabaseError> {
        let model = academic_calendar::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            name: Set(name.to_owned()),
            academic_calendar_type: Set(calendar_type.to_owned()),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(DatabaseError::from)
    }
}
