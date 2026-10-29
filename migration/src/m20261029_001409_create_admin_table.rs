use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261029_001409_create_admin_table"
    }
}

pub const ADMIN_TABLE: &str = "admin_users";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ADMIN_TABLE)
                    .if_not_exists()
                    .col(binary_len("identifier", 16).primary_key())
                    .col(string("first_name"))
                    .col(string("last_name"))
                    .col(string("email").unique_key())
                    .col(string("password"))
                    .col(string("last_logged_in"))
                    .col(boolean("is_blocked").default(false))
                    .col(boolean("is_active").default(false))
                    .col(boolean("is_suspended").default(false))
                    .col(date_time_default_now("created_at"))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ADMIN_TABLE).to_owned())
            .await
    }
}
