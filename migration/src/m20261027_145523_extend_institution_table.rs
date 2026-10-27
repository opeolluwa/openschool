use sea_orm_migration::prelude::*;

use crate::m20261026_180522_create_admission_table::ADMISSION_TABLE;
use crate::m20261026_181256_create_institution_table::INSTITUTION_TABLE;
use crate::m20261027_145128_create_grading_system_table::GRADING_SYSTEM_TABLE;
use crate::m20261027_145153_create_academic_calendar_table::ACADEMIC_CALENDAR_TABLE;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261027_145523_extend_institution_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(INSTITUTION_TABLE)
                    .add_column(
                        ColumnDef::new("grading_system_identifier")
                            .binary_len(16)
                            .not_null()
                            .unique_key(),
                    )
                    .add_column(
                        ColumnDef::new("academic_calendar_identifier")
                            .binary_len(16)
                            .not_null()
                            .unique_key(),
                    )
                    .add_foreign_key(
                        &mut TableForeignKey::new()
                            .name("fk-institution_grading_system")
                            .from_tbl(INSTITUTION_TABLE)
                            .from_col("grading_system_identifier")
                            .to_tbl(GRADING_SYSTEM_TABLE)
                            .to_col("identifier")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_foreign_key(
                        &mut TableForeignKey::new()
                            .name("fk-institution_academic_calendar")
                            .from_tbl(INSTITUTION_TABLE)
                            .from_col("academic_calendar_identifier")
                            .to_tbl(ACADEMIC_CALENDAR_TABLE)
                            .to_col("identifier")
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-admission_academic_calendar")
                    .from(ADMISSION_TABLE, "academic_calendar_identifier")
                    .to(ACADEMIC_CALENDAR_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-admission_academic_calendar")
                    .table(ADMISSION_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-institution_grading_system")
                    .table(INSTITUTION_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-institution_academic_calendar")
                    .table(INSTITUTION_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(INSTITUTION_TABLE)
                    .drop_column(Alias::new("grading_system_identifier"))
                    .drop_column(Alias::new("academic_calendar_identifier"))
                    .to_owned(),
            )
            .await
    }
}
