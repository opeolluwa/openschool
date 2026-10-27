use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_180522_create_admission_table"
    }
}

pub const ADMISSION_TABLE: &str = "admission";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ADMISSION_TABLE)
                    .if_not_exists()
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(big_integer("application_fee"))
                    .col(date("deadline"))
                    .col(array("eligibility", ColumnType::String(StringLen::None)))
                    .col(string("academic_year"))
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ADMISSION_TABLE).to_owned())
            .await
    }
}
