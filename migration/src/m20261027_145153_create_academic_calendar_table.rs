use sea_orm_migration::{
    prelude::*,
    schema::{date_null, date_time_default_now, date_time_null, enumeration, integer, string},
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261027_145153_create_academic_calendar_table"
    }
}

pub const ACADEMIC_CALENDAR_TABLE: &str = "academic_calendar";
pub const ACADEMIC_PERIOD_TABLE: &str = "academic_period";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AcademicCalendar::Type)
                    .values(AcademicCalendar::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(ACADEMIC_CALENDAR_TABLE)
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(string("name"))
                    .col(enumeration(
                        AcademicCalendar::Type,
                        AcademicCalendar::Type,
                        AcademicCalendar::iter(),
                    ))
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(ACADEMIC_PERIOD_TABLE)
                    .col(ColumnDef::new("identifier").binary_len(16).primary_key())
                    .col(
                        ColumnDef::new("calendar_identifier")
                            .binary_len(16)
                            .not_null(),
                    )
                    .col(string("name"))
                    .col(integer("sequence"))
                    .col(date_null("start_date"))
                    .col(date_null("end_date"))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk-academic_period_calendar")
                            .from(ACADEMIC_PERIOD_TABLE, "calendar_identifier")
                            .to(ACADEMIC_CALENDAR_TABLE, "identifier")
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
            .drop_table(Table::drop().table(ACADEMIC_PERIOD_TABLE).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ACADEMIC_CALENDAR_TABLE).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(AcademicCalendar::Type)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum AcademicCalendar {
    #[iden = "academic_calendar_type"]
    Type,
    Semester,
    Terms,
    Quarters,
    SelfPaced,
}
