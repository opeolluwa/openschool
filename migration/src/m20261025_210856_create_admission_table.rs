use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_210856_create_admission_table"
    }
}

pub const ADMISSION_APPLICANT_DATA_TABLE: &str = "admission_applicant";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Gender::Type)
                    .values(Gender::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ADMISSION_APPLICANT_DATA_TABLE)
                    .if_not_exists()
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("first_name"))
                    .col(string("last_name"))
                    .col(string_null("middle_name"))
                    .col(enumeration(Gender::Type, Gender::Type, Gender::iter()))
                    .col(date("date_of_birth"))
                    .col(date("email").unique_key())
                    .col(date("phone").unique_key())
                    .col(string("photograph"))
                    .col(string_null("nationality"))
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
                    .table(ADMISSION_APPLICANT_DATA_TABLE)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum Gender {
    #[iden = "gender"]
    Type,
    Male,
    Female,
    Others,
}
