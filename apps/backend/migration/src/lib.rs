pub use sea_orm_migration::prelude::*;

mod columns;
mod m20250505_051849_create_users;
mod m20250506_100520_create_oauth_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250505_051849_create_users::Migration),
            Box::new(m20250506_100520_create_oauth_users::Migration),
        ]
    }
}
