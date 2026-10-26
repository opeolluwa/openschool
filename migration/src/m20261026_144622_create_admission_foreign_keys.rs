use sea_orm_migration::prelude::*;

use super::m20261025_210856_create_admission_table::ADMISSION_APPLICANT_DATA_TABLE;
use super::m20261026_133221_create_admission_applicant_contact_address_table::CONTACT_ADDRESS_TABLE;
use super::m20261026_134201_create_admission_guardian_table::ADMISSION_APPLICANT_GUARDIAN_TABLE;
use super::m20261026_134221_create_admission_guardian_contact_address_table::ADMISSION_GUARDIAN_CONTACT_ADDRESS_TABLE;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_144622_create_Admission_foreign_keys"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-admission_applicant_contact_address-applicant_id")
                    .from(ADMISSION_APPLICANT_DATA_TABLE, "address_identifier")
                    .to(CONTACT_ADDRESS_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-admission_applicant_guardian_data-applicant_id")
                    .from(ADMISSION_APPLICANT_GUARDIAN_TABLE, "applicant_identifier")
                    .to(ADMISSION_APPLICANT_DATA_TABLE, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-admission_guardian_contact_address-guardian_identifier")
                    .from(
                        ADMISSION_GUARDIAN_CONTACT_ADDRESS_TABLE,
                        "guardian_identifier",
                    )
                    .to(ADMISSION_APPLICANT_GUARDIAN_TABLE, "identifier")
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
                    .name("fk-admission_guardian_contact_address-guardian_id")
                    .table(ADMISSION_GUARDIAN_CONTACT_ADDRESS_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-admission_applicant_guardian_data-applicant_id")
                    .table(ADMISSION_APPLICANT_GUARDIAN_TABLE)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-admission_applicant_contact_address-applicant_id")
                    .table(CONTACT_ADDRESS_TABLE)
                    .to_owned(),
            )
            .await
    }
}
