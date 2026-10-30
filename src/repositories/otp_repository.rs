use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
    sea_query::{Expr, ExprTrait},
};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use openschool_models::one_time_passwords;
use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait OtpRepositoryExt {
    /// Replaces any live OTP for the owner with a new one, inside a
    /// transaction so an owner never ends up with two codes in flight.
    async fn create_otp(
        &self,
        owner_identifier: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<one_time_passwords::Model, DatabaseError>;

    async fn retrieve_otp(
        &self,
        owner_identifier: &str,
    ) -> Result<one_time_passwords::Model, DatabaseError>;

    async fn revoke_otp(&self, owner_identifier: &str) -> Result<(), DatabaseError>;

    async fn increment_failed_attempts(&self, owner_identifier: &str)
    -> Result<i32, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct OtpRepository {
    db: DatabaseConnection,
}

impl Repository for OtpRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl OtpRepositoryExt for OtpRepository {
    async fn create_otp(
        &self,
        owner_identifier: &str,
        token: &str,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<one_time_passwords::Model, DatabaseError> {
        let txn = self.db.begin().await.map_err(DatabaseError::from)?;

        // Invalidate any existing OTP for this owner before creating a new one.
        one_time_passwords::Entity::delete_many()
            .filter(one_time_passwords::Column::OwnerIdentifier.eq(owner_identifier))
            .exec(&txn)
            .await
            .map_err(DatabaseError::from)?;

        let model = one_time_passwords::ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            owner_identifier: Set(owner_identifier.to_owned()),
            token: Set(token.to_owned()),
            expiration: Set(expiration.naive_utc()),
            ..Default::default()
        };

        let inserted = model.insert(&txn).await.map_err(DatabaseError::from)?;
        txn.commit().await.map_err(DatabaseError::from)?;
        Ok(inserted)
    }

    async fn retrieve_otp(
        &self,
        owner_identifier: &str,
    ) -> Result<one_time_passwords::Model, DatabaseError> {
        one_time_passwords::Entity::find()
            .filter(one_time_passwords::Column::OwnerIdentifier.eq(owner_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("otp not found".to_string()))
    }

    async fn revoke_otp(&self, owner_identifier: &str) -> Result<(), DatabaseError> {
        let result = one_time_passwords::Entity::delete_many()
            .filter(one_time_passwords::Column::OwnerIdentifier.eq(owner_identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;

        if result.rows_affected == 0 {
            return Err(DatabaseError::NotFound("otp not found".to_string()));
        }

        Ok(())
    }

    async fn increment_failed_attempts(
        &self,
        owner_identifier: &str,
    ) -> Result<i32, DatabaseError> {
        let result = one_time_passwords::Entity::update_many()
            .col_expr(
                one_time_passwords::Column::FailedAttempts,
                Expr::col(one_time_passwords::Column::FailedAttempts).add(1),
            )
            .filter(one_time_passwords::Column::OwnerIdentifier.eq(owner_identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;

        if result.rows_affected == 0 {
            return Err(DatabaseError::NotFound("otp not found".to_string()));
        }

        let otp = self.retrieve_otp(owner_identifier).await?;
        Ok(otp.failed_attempts)
    }
}
