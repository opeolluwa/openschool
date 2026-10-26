use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Repositories {}

impl Repositories {
    pub fn new(_db_pool: &DatabaseConnection) -> Self {
        Self {}
    }
}
