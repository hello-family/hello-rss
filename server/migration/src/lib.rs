pub use sea_schema::migration::prelude::*;

mod m20220428_000001_create_user_table;
mod m20220428_000002_create_client_table;
mod m20220428_000003_create_access_token_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220428_000001_create_user_table::Migration),
            Box::new(m20220428_000002_create_client_table::Migration),
            Box::new(m20220428_000003_create_access_token_table::Migration),
        ]
    }
}
