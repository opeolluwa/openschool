use sea_orm_migration::{
    prelude::*,
    schema::{date_time_default_now, date_time_null, enumeration, string, string_null},
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_134201_create_admission_guardian_table"
    }
}

pub const ADMISSION_APPLICANT_GUARDIAN_TABLE: &str = "admission_applicant_guardian_data";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Relationship::Type)
                    .values(Relationship::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ADMISSION_APPLICANT_GUARDIAN_TABLE)
                    .if_not_exists()
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(
                        ColumnDef::new("applicant_identifier")
                            .binary_len(16)
                            .not_null(),
                    )
                    .col(string("first_name"))
                    .col(string("last_name"))
                    .col(string_null("middle_name"))
                    .col(enumeration(
                        Relationship::Type,
                        Relationship::Type,
                        Relationship::iter(),
                    ))
                    .col(string("email").unique_key())
                    .col(string("phone").unique_key())
                    .col(string_null("occupation"))
                    .col(string("photograph"))
                    .col(string_null("nationality"))
                    .col(
                        ColumnDef::new("address_identifier")
                            .binary_len(16)
                            .not_null()
                            .unique_key(),
                    )
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
                    .table(ADMISSION_APPLICANT_GUARDIAN_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Relationship::Type).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum Relationship {
    #[iden = "relationship"]
    Type,
    Father,
    Mother,
    Other,
}
