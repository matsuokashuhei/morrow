pub use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_auto, timestamp};

mod m20250505_051849_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250505_051849_create_users::Migration)]
    }
}

#[derive(Iden)]
enum Column {
    Id,
    CreatedAt,
    UpdatedAt,
}

pub fn define_id() -> ColumnDef {
    pk_auto(Column::Id)
}

// pub fn define_timpestamps(t: TableCreateStatement) -> TableCreateStatement {
//     let mut t = t;
//     let created_at = timestamp(Column::CreatedAt)
//         .default(Expr::current_timestamp())
//         .take();
//     let updated_at = timestamp(Column::UpdatedAt)
//         .default(Expr::current_timestamp())
//         .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned())
//         .take();
//     t.col(created_at).col(updated_at).take()
// }

pub fn define_created_at() -> ColumnDef {
    timestamp(Column::CreatedAt)
        .default(Expr::current_timestamp())
        .take()
}

pub fn define_updated_at() -> ColumnDef {
    timestamp(Column::UpdatedAt)
        .default(Expr::current_timestamp())
        .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned())
        .take()
}
