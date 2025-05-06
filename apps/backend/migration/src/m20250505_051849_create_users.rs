use sea_orm_migration::{prelude::*, schema::*};

use crate::columns::User;
use crate::columns::{define_created_at, define_id, define_updated_at};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(define_id())
                    .col(string_null(User::Name))
                    .col(define_created_at())
                    .col(define_updated_at())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
