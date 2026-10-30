use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use ulid::Ulid;

use openschool_models::invoice::{self, ActiveModel, Column, Entity, Model};
use openschool_models::sea_orm_active_enums::ProductCurrency;

use crate::error::RemittanceError;

#[derive(Clone)]
pub struct InvoiceRepository {
    db: DatabaseConnection,
}

impl InvoiceRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create(
        &self,
        invoice_number: &str,
        transaction_identifier: Vec<u8>,
        recipient_name: &str,
        recipient_email: &str,
        recipient_phone: Option<String>,
        description: Option<String>,
        amount: i64,
        currency: ProductCurrency,
        due_date: Option<String>,
        institution_identifier: Vec<u8>,
    ) -> Result<Model, RemittanceError> {
        let model = ActiveModel {
            identifier: Set(Ulid::new().to_bytes().to_vec()),
            invoice_number: Set(invoice_number.to_string()),
            transaction_identifier: Set(transaction_identifier),
            recipient_name: Set(recipient_name.to_string()),
            recipient_email: Set(recipient_email.to_string()),
            recipient_phone: Set(recipient_phone),
            description: Set(description),
            amount: Set(amount),
            currency: Set(currency),
            due_date: Set(due_date),
            institution_identifier: Set(institution_identifier),
            ..Default::default()
        };

        Ok(model.insert(&self.db).await?)
    }

    pub async fn find_by_transaction(
        &self,
        transaction_id: &[u8],
    ) -> Result<Option<Model>, RemittanceError> {
        Ok(Entity::find()
            .filter(Column::TransactionIdentifier.eq(transaction_id))
            .one(&self.db)
            .await?)
    }

    pub async fn find_by_number(&self, invoice_number: &str) -> Result<Option<Model>, RemittanceError> {
        Ok(Entity::find()
            .filter(Column::InvoiceNumber.eq(invoice_number))
            .one(&self.db)
            .await?)
    }
}
