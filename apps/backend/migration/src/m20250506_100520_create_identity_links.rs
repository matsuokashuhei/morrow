use sea_orm_migration::{prelude::*, schema::*};

use crate::columns::{define_created_at, define_id, define_updated_at};
use crate::columns::{IdentityLink, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IdentityLink::Table)
                    .if_not_exists()
                    .col(define_id())
                    .col(string_null(IdentityLink::Provider))
                    .col(string_null(IdentityLink::Sub))
                    .col(integer_null(IdentityLink::UserId))
                    .col(define_created_at())
                    .col(define_updated_at())
                    .index(
                        Index::create()
                            .name("idx-identity_links-provider-sub")
                            .table(IdentityLink::Table)
                            .col(IdentityLink::Provider)
                            .col(IdentityLink::Sub)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-identity_links-user_id")
                            .from(IdentityLink::Table, IdentityLink::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IdentityLink::Table).to_owned())
            .await
    }
}
