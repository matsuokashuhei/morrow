use sea_orm_migration::{prelude::*, schema::*};

use crate::columns::{define_created_at, define_id, define_updated_at};
use crate::columns::{OAuthUser, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OAuthUser::Table)
                    .if_not_exists()
                    .col(define_id())
                    .col(string_null(OAuthUser::Provider))
                    .col(string_null(OAuthUser::Sub))
                    .col(integer_null(OAuthUser::UserId))
                    .col(define_created_at())
                    .col(define_updated_at())
                    .index(
                        Index::create()
                            .name("idx-oauth_users-provider-sub")
                            .table(OAuthUser::Table)
                            .col(OAuthUser::Provider)
                            .col(OAuthUser::Sub)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-oauth_users-user_id")
                            .from(OAuthUser::Table, OAuthUser::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OAuthUser::Table).to_owned())
            .await
    }
}
