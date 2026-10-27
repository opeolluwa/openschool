use sea_orm_migration::{
    prelude::*,
    schema::{date_time_default_now, date_time_null, enumeration, string, string_null},
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261027_145128_create_grading_system_table"
    }
}

pub const GRADING_SYSTEM_TABLE: &str = "grading_system";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(GradingSystem::Type)
                    .values(GradingSystem::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(GRADING_SYSTEM_TABLE)
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("name"))
                    .col(enumeration(
                        GradingSystem::Type,
                        GradingSystem::Type,
                        GradingSystem::iter(),
                    ))
                    .col(string_null("passing_score"))
                    .col(string_null("max_score"))
                    .col(string_null("grading_rules"))
                    .col(
                        ColumnDef::new("supports_report_cards")
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new("supports_competency_tracking")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GRADING_SYSTEM_TABLE).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(GradingSystem::Type)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum GradingSystem {
    #[iden = "grading_system_type"]
    Type,
    GPA,
    Grades,
    Competency,
    Custom,
}
