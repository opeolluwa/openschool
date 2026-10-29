use sea_orm_migration::prelude::*;

use crate::m20261026_181256_create_institution_table::INSTITUTION_TABLE;
use crate::m20261027_145128_create_grading_system_table::GRADING_SYSTEM_TABLE;
use crate::m20261027_145153_create_academic_calendar_table::ACADEMIC_CALENDAR_TABLE;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261029_100000_make_institution_fks_nullable"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-institution_address")
                    .table(INSTITUTION_TABLE)
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
                    .modify_column(ColumnDef::new("address_identifier").binary_len(16).null())
                    .modify_column(
                        ColumnDef::new("grading_system_identifier")
                            .binary_len(16)
                            .null(),
                    )
                    .modify_column(
                        ColumnDef::new("academic_calendar_identifier")
                            .binary_len(16)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_address")
                    .from(INSTITUTION_TABLE, "address_identifier")
                    .to("contact_address", "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_grading_system")
                    .from(INSTITUTION_TABLE, "grading_system_identifier")
                    .to(GRADING_SYSTEM_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_academic_calendar")
                    .from(INSTITUTION_TABLE, "academic_calendar_identifier")
                    .to(ACADEMIC_CALENDAR_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-institution_address")
                    .table(INSTITUTION_TABLE)
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
                    .modify_column(
                        ColumnDef::new("address_identifier")
                            .binary_len(16)
                            .not_null(),
                    )
                    .modify_column(
                        ColumnDef::new("grading_system_identifier")
                            .binary_len(16)
                            .not_null(),
                    )
                    .modify_column(
                        ColumnDef::new("academic_calendar_identifier")
                            .binary_len(16)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_address")
                    .from(INSTITUTION_TABLE, "address_identifier")
                    .to("contact_address", "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_grading_system")
                    .from(INSTITUTION_TABLE, "grading_system_identifier")
                    .to(GRADING_SYSTEM_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-institution_academic_calendar")
                    .from(INSTITUTION_TABLE, "academic_calendar_identifier")
                    .to(ACADEMIC_CALENDAR_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }
}
