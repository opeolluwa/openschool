use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use crate::extractors::admin::CreateAdminUser;
use openschool_models::admin_users;
use crate::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct AdminUserRepository {
    db: DatabaseConnection,
}

impl Repository for AdminUserRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait AdminUserRepositoryExt {
    async fn create(
        &self,
        req: &CreateAdminUser,
        hashed_password: &str,
    ) -> Result<admin_users::Model, DatabaseError>;

    async fn find_by_email(&self, email: &str) -> Result<admin_users::Model, DatabaseError>;

    async fn find_by_identifier(
        &self,
        identifier: &[u8],
    ) -> Result<admin_users::Model, DatabaseError>;

    async fn record_last_login(&self, identifier: &[u8]) -> Result<(), DatabaseError>;

    async fn update_blocked_status(
        &self,
        identifier: &[u8],
        is_blocked: bool,
    ) -> Result<(), DatabaseError>;
}

impl AdminUserRepositoryExt for AdminUserRepository {
    async fn create(
        &self,
        req: &CreateAdminUser,
        hashed_password: &str,
    ) -> Result<admin_users::Model, DatabaseError> {
        let model = admin_users::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            first_name: Set(req.first_name.clone()),
            last_name: Set(req.last_name.clone()),
            email: Set(req.email.clone()),
            password: Set(hashed_password.to_owned()),
            last_logged_in: Set(String::new()),
            is_blocked: Set(false),
            is_active: Set(true),
            is_suspended: Set(false),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(|err| {
            if let sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(ref sqlx_err)) = err {
                if let Some(db_err) = sqlx_err.as_database_error() {
                    if db_err.is_unique_violation() {
                        return DatabaseError::DuplicateEmailForUser;
                    }
                }
            }
            DatabaseError::from(err)
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<admin_users::Model, DatabaseError> {
        admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("admin user not found".to_string()))
    }

    async fn find_by_identifier(
        &self,
        identifier: &[u8],
    ) -> Result<admin_users::Model, DatabaseError> {
        admin_users::Entity::find()
            .filter(admin_users::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("admin user not found".to_string()))
    }

    async fn record_last_login(&self, identifier: &[u8]) -> Result<(), DatabaseError> {
        let user = self.find_by_identifier(identifier).await?;
        let mut active: admin_users::ActiveModel = user.into();
        active.last_logged_in = Set(chrono::Utc::now().to_rfc3339());
        active.update(&self.db).await.map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn update_blocked_status(
        &self,
        identifier: &[u8],
        is_blocked: bool,
    ) -> Result<(), DatabaseError> {
        let user = self.find_by_identifier(identifier).await?;
        let mut active: admin_users::ActiveModel = user.into();
        active.is_blocked = Set(is_blocked);
        active.update(&self.db).await.map_err(DatabaseError::from)?;
        Ok(())
    }
}
