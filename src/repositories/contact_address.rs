use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use crate::extractors::contact_address::ContactAddressPayload;
use crate::models::contact_address;
use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait ContactAddressRepositoryExt {
    async fn create_contact_address(
        &self,
        payload: &ContactAddressPayload,
    ) -> Result<contact_address::Model, DatabaseError>;

    async fn retrieve_contact_address(
        &self,
        identifier: &[u8],
    ) -> Result<contact_address::Model, DatabaseError>;

    async fn update_contact_address(
        &self,
        identifier: &[u8],
        payload: &ContactAddressPayload,
    ) -> Result<contact_address::Model, DatabaseError>;

    async fn delete_contact_address(&self, identifier: &[u8]) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct ContactAddressRepository {
    db: DatabaseConnection,
}

impl Repository for ContactAddressRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl ContactAddressRepositoryExt for ContactAddressRepository {
    async fn create_contact_address(
        &self,
        payload: &ContactAddressPayload,
    ) -> Result<contact_address::Model, DatabaseError> {
        let model = contact_address::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            address_line_one: Set(payload.address_line_one.clone()),
            address_line_two: Set(payload.address_line_two.clone()),
            city: Set(payload.city.clone()),
            region: Set(payload.region.clone()),
            country: Set(payload.country.clone()),
            zip_code: Set(payload.zip_code.clone()),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn retrieve_contact_address(
        &self,
        identifier: &[u8],
    ) -> Result<contact_address::Model, DatabaseError> {
        contact_address::Entity::find()
            .filter(contact_address::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("contact address not found".to_string()))
    }

    async fn update_contact_address(
        &self,
        identifier: &[u8],
        payload: &ContactAddressPayload,
    ) -> Result<contact_address::Model, DatabaseError> {
        let existing = self.retrieve_contact_address(identifier).await?;

        let model = contact_address::ActiveModel {
            identifier: Set(existing.identifier),
            address_line_one: Set(payload.address_line_one.clone()),
            address_line_two: Set(payload.address_line_two.clone()),
            city: Set(payload.city.clone()),
            region: Set(payload.region.clone()),
            country: Set(payload.country.clone()),
            zip_code: Set(payload.zip_code.clone()),
            ..Default::default()
        };

        model.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_contact_address(&self, identifier: &[u8]) -> Result<(), DatabaseError> {
        let result = contact_address::Entity::delete_many()
            .filter(contact_address::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;

        if result.rows_affected == 0 {
            return Err(DatabaseError::NotFound(
                "contact address not found".to_string(),
            ));
        }

        Ok(())
    }
}
