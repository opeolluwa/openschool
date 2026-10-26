use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_133221_create_admission_applicant_contact_address_table"
    }
}

pub const ADMISSION_APPLICANT_CONTACT_ADDRESS_TABLE: &str = "admission_applicant_contact_address";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ADMISSION_APPLICANT_CONTACT_ADDRESS_TABLE)
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("address_line_one"))
                    .col(string_null("address_line_two"))
                    .col(string("city"))
                    .col(string_null("region"))
                    .col(string("country"))
                    .col(string("zip_code"))
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ADMISSION_APPLICANT_CONTACT_ADDRESS_TABLE)
                    .to_owned(),
            )
            .await
    }
}
