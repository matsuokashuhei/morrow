pub use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{timestamp_with_time_zone, uuid_null};

#[derive(Iden)]
enum Column {
    Id,
    CreatedAt,
    UpdatedAt,
}

pub fn define_id() -> ColumnDef {
    uuid_null(Column::Id)
        .unique_key()
        .extra("DEFAULT gen_random_uuid()")
        .take()
}

pub fn define_created_at() -> ColumnDef {
    timestamp_with_time_zone(Column::CreatedAt)
        .default(Expr::current_timestamp())
        .take()
}

pub fn define_updated_at() -> ColumnDef {
    timestamp_with_time_zone(Column::UpdatedAt)
        .default(Expr::current_timestamp())
        .take()
}

#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum IdentityLink {
    #[sea_orm(iden = "identity_links")]
    Table,
    Id,
    Provider,
    Sub,
    UserId,
}
