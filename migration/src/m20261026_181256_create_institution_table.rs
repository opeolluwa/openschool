use sea_orm_migration::{prelude::*, schema::*};

use crate::m20261026_133221_create_admission_applicant_contact_address_table::CONTACT_ADDRESS_TABLE;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_181256_create_institution_table"
    }
}

pub const INSTITUTION_TABLE: &str = "institution";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(INSTITUTION_TABLE)
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("name"))
                    .col(string("tagline"))
                    .col(string("logo_url"))
                    .col(
                        ColumnDef::new("address_identifier")
                            .binary_len(16)
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk-institution_address")
                            .from(INSTITUTION_TABLE, "address_identifier")
                            .to(CONTACT_ADDRESS_TABLE, "identifier")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(INSTITUTION_TABLE).to_owned())
            .await
    }
}
