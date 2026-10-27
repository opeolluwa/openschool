use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261027_162317_create_otp_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("one_time_passwords")
                    .if_not_exists()
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("token"))
                    .col(string("owner_identifier"))
                    .col(date_time("expiration").not_null())
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("one_time_passwords").to_owned())
            .await
    }
}
