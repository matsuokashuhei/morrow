use sea_orm_migration::{prelude::*, schema::*};

use crate::columns::{define_created_at, define_id, define_updated_at};
use crate::columns::{CognitoUser, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CognitoUser::Table)
                    .if_not_exists()
                    .col(define_id())
                    .col(string_null(CognitoUser::Sub))
                    .col(integer_null(CognitoUser::UserId))
                    .col(define_created_at())
                    .col(define_updated_at())
                    .index(
                        Index::create()
                            .name("idx-cognito_users-sub-user_id")
                            .table(CognitoUser::Table)
                            .col(CognitoUser::Sub)
                            .col(CognitoUser::UserId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cognito_users-user_id")
                            .from(CognitoUser::Table, CognitoUser::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CognitoUser::Table).to_owned())
            .await
    }
}
