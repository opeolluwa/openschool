use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use ulid::Ulid;

use openschool_models::payment_transaction::{self, ActiveModel, Column, Entity, Model};
use openschool_models::sea_orm_active_enums::{OrderStatus, PaymentChannel, ProductCurrency};

use crate::error::RemittanceError;

#[derive(Clone)]
pub struct PaymentTransactionRepository {
    db: DatabaseConnection,
}

impl PaymentTransactionRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create(
        &self,
        reference: &str,
        amount: i64,
        currency: ProductCurrency,
        payer_email: &str,
        provider: &str,
        institution_identifier: Vec<u8>,
        metadata: serde_json::Value,
    ) -> Result<Model, RemittanceError> {
        let model = ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            reference: Set(reference.to_string()),
            amount: Set(amount),
            currency: Set(currency),
            status: Set(OrderStatus::Pending),
            channel: Set(None),
            provider: Set(provider.to_string()),
            provider_reference: Set(None),
            provider_authorization_url: Set(None),
            payer_email: Set(payer_email.to_string()),
            metadata: Set(metadata),
            institution_identifier: Set(institution_identifier),
            paid_at: Set(None),
            ..Default::default()
        };

        Ok(model.insert(&self.db).await?)
    }

    pub async fn find_by_reference(&self, reference: &str) -> Result<Option<Model>, RemittanceError> {
        Ok(Entity::find()
            .filter(Column::Reference.eq(reference))
            .one(&self.db)
            .await?)
    }

    pub async fn find_by_identifier(&self, id: &[u8]) -> Result<Option<Model>, RemittanceError> {
        Ok(Entity::find()
            .filter(Column::Identifier.eq(id))
            .one(&self.db)
            .await?)
    }

    pub async fn update_authorization_url(
        &self,
        id: &[u8],
        url: &str,
    ) -> Result<Model, RemittanceError> {
        let record = self.find_by_identifier(id).await?.ok_or(
            RemittanceError::Provider(crate::error::PaymentError::NotFound),
        )?;

        let mut active: ActiveModel = record.into();
        active.provider_authorization_url = Set(Some(url.to_string()));
        Ok(active.update(&self.db).await?)
    }

    pub async fn update_status(
        &self,
        id: &[u8],
        status: OrderStatus,
        channel: Option<PaymentChannel>,
        provider_reference: Option<String>,
        paid_at: Option<String>,
    ) -> Result<Model, RemittanceError> {
        let record = self.find_by_identifier(id).await?.ok_or(
            RemittanceError::Provider(crate::error::PaymentError::NotFound),
        )?;

        let mut active: ActiveModel = record.into();
        active.status = Set(status);
        active.channel = Set(channel);
        active.provider_reference = Set(provider_reference);
        active.paid_at = Set(paid_at);
        Ok(active.update(&self.db).await?)
    }

    pub async fn list_by_institution(
        &self,
        institution_id: &[u8],
        status_filter: Option<OrderStatus>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64), RemittanceError> {
        let mut query = Entity::find()
            .filter(Column::InstitutionIdentifier.eq(institution_id));

        if let Some(status) = status_filter {
            query = query.filter(Column::Status.eq(status));
        }

        let paginator = query
            .order_by_desc(Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await?;
        let records = paginator.fetch_page(page).await?;

        Ok((records, total))
    }
}
