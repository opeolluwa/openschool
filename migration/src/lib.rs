pub use sea_orm_migration::prelude::*;

mod m20261025_210856_create_admission_table;
mod m20261026_133221_create_admission_applicant_contact_address_table;
mod m20261026_134201_create_admission_guardian_table;
mod m20261026_134221_create_admission_guardian_contact_address_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261025_210856_create_admission_table::Migration),
            Box::new(m20261026_133221_create_admission_applicant_contact_address_table::Migration),
            Box::new(m20261026_134201_create_admission_guardian_table::Migration),
            Box::new(m20261026_134221_create_admission_guardian_contact_address_table::Migration),
        ]
    }
}
