use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::services::UserService;
use crate::presentation::graphql::context::GraphQLContext;
use crate::presentation::graphql::types::user_type::User;

pub struct UserResolver {
    service: Arc<UserService>,
}

impl UserResolver {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }
}

#[Object]
impl UserResolver {
    // 個別ユーザー取得 - 認証必須
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        // コンテキストからGraphQLContextを取得
        let graphql_ctx = ctx.data::<GraphQLContext>()?;

        // 認証チェック
        if !graphql_ctx.is_authenticated() {
            return Err("Authentication required".into());
        }

        let user = self.service.get_user(id).await?;
        Ok(user.map(User::from))
    }

    // 全ユーザー取得 - 管理者ロール必須
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        // コンテキストからGraphQLContextを取得
        let graphql_ctx = ctx.data::<GraphQLContext>()?;

        // 管理者権限チェック
        if !graphql_ctx.is_admin() {
            return Err("Admin role required".into());
        }

        let users = self.service.get_all_users().await?;
        Ok(users.into_iter().map(User::from).collect())
    }

    // 現在認証されているユーザーの情報を取得
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        // コンテキストからGraphQLContextを取得
        let graphql_ctx = ctx.data::<GraphQLContext>()?;

        // 認証チェック
        if !graphql_ctx.is_authenticated() {
            return Err("Authentication required".into());
        }

        // 認証済みユーザーのIDを取得
        let user_id = graphql_ctx.user_id().ok_or("User ID not found")?;

        let user = self.service.get_user(user_id).await?;
        Ok(user.map(User::from))
    }
}
