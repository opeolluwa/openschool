use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use crate::extractors::institution::RegisterInstitution;
use openschool_models::institution;
use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait InstitutionRepositoryExt {
    async fn create(
        &self,
        payload: &RegisterInstitution,
    ) -> Result<institution::Model, DatabaseError>;

    async fn find_singleton(&self) -> Result<Option<institution::Model>, DatabaseError>;

    async fn update(
        &self,
        existing: institution::Model,
        payload: &RegisterInstitution,
    ) -> Result<institution::Model, DatabaseError>;

    async fn update_address(
        &self,
        identifier: &[u8],
        address_id: &[u8],
    ) -> Result<institution::Model, DatabaseError>;

    async fn update_academics(
        &self,
        identifier: &[u8],
        grading_id: &[u8],
        calendar_id: &[u8],
    ) -> Result<institution::Model, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct InstitutionRepository {
    db: DatabaseConnection,
}

impl Repository for InstitutionRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl InstitutionRepositoryExt for InstitutionRepository {
    async fn create(
        &self,
        payload: &RegisterInstitution,
    ) -> Result<institution::Model, DatabaseError> {
        let model = institution::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            name: Set(payload.name.clone()),
            tagline: Set(payload.tagline.clone()),
            logo_url: Set(payload.logo_url.clone().unwrap_or_default()),
            institution_type: Set(payload.institution_type.to_owned()),
            address_identifier: Set(None),
            grading_system_identifier: Set(None),
            academic_calendar_identifier: Set(None),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_singleton(&self) -> Result<Option<institution::Model>, DatabaseError> {
        institution::Entity::find()
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn update(
        &self,
        existing: institution::Model,
        payload: &RegisterInstitution,
    ) -> Result<institution::Model, DatabaseError> {
        let mut active: institution::ActiveModel = existing.into();
        active.name = Set(payload.name.clone());
        active.tagline = Set(payload.tagline.clone());
        active.logo_url = Set(payload.logo_url.clone().unwrap_or_default()); //TODO: this should b be optional
        active.institution_type = Set(payload.institution_type.to_owned());
        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn update_address(
        &self,
        identifier: &[u8],
        address_id: &[u8],
    ) -> Result<institution::Model, DatabaseError> {
        let existing = institution::Entity::find_by_id(identifier.to_vec())
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("institution not found".to_string()))?;

        let mut active: institution::ActiveModel = existing.into();
        active.address_identifier = Set(Some(address_id.to_vec()));
        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn update_academics(
        &self,
        identifier: &[u8],
        grading_id: &[u8],
        calendar_id: &[u8],
    ) -> Result<institution::Model, DatabaseError> {
        let existing = institution::Entity::find_by_id(identifier.to_vec())
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("institution not found".to_string()))?;

        let mut active: institution::ActiveModel = existing.into();
        active.grading_system_identifier = Set(Some(grading_id.to_vec()));
        active.academic_calendar_identifier = Set(Some(calendar_id.to_vec()));
        active.update(&self.db).await.map_err(DatabaseError::from)
    }
}
