mod m20261025_210856_create_admission_table;
mod m20261026_133221_create_admission_applicant_contact_address_table;
mod m20261026_134201_create_admission_guardian_table;
mod m20261026_144622_create_admission_foreign_keys;
mod m20261026_180522_create_admission_table;
mod m20261026_181256_create_institution_table;
mod m20261027_145128_create_grading_system_table;
mod m20261027_145153_create_academic_calendar_table;
mod m20261027_145523_extend_institution_table;
mod m20261027_162317_create_otp_table;
mod m20261027_163136_add_failed_attempts_to_otp;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261025_210856_create_admission_table::Migration),
            Box::new(m20261026_133221_create_admission_applicant_contact_address_table::Migration),
            Box::new(m20261026_134201_create_admission_guardian_table::Migration),
            Box::new(m20261026_144622_create_admission_foreign_keys::Migration),
            Box::new(m20261026_180522_create_admission_table::Migration),
            Box::new(m20261026_181256_create_institution_table::Migration),
            Box::new(m20261027_145128_create_grading_system_table::Migration),
            Box::new(m20261027_145153_create_academic_calendar_table::Migration),
            Box::new(m20261027_145523_extend_institution_table::Migration),
            Box::new(m20261027_162317_create_otp_table::Migration),
            Box::new(m20261027_163136_add_failed_attempts_to_otp::Migration),
        ]
    }
}
