pub use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_auto, timestamp};

#[derive(Iden)]
enum Column {
    Id,
    CreatedAt,
    UpdatedAt,
}

pub fn define_id() -> ColumnDef {
    pk_auto(Column::Id)
}

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

#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum CognitoUser {
    #[sea_orm(iden = "cognito_users")]
    Table,
    Id,
    Sub,
    UserId,
}
