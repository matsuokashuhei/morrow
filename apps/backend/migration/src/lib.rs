pub use sea_orm_migration::prelude::*;

mod columns;
mod m20250505_051849_create_users;
mod m20250506_100520_create_identity_links;
mod m20250601_000000_add_user_role;
// mod m20250524_000000_create_updated_at_trigger;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250505_051849_create_users::Migration),
            Box::new(m20250506_100520_create_identity_links::Migration),
            Box::new(m20250601_000000_add_user_role::Migration),
            // Box::new(m20250524_000000_create_updated_at_trigger::Migration),
        ]
    }
}
