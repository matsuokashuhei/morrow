use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::services::UserService;
use crate::presentation::graphql::context::UserContext;
// use crate::presentation::graphql::context::GraphQLContext;
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
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let user = self.service.get_user(id).await?;
        Ok(user.map(User::from))
    }

    // 全ユーザー取得 - 管理者ロール必須
    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = self.service.get_all_users().await?;
        Ok(users.into_iter().map(User::from).collect())
    }

    // 現在認証されているユーザーの情報を取得
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user_context = ctx.data::<UserContext>()?;
        if let Some(user) = user_context.user.clone() {
            let user = self.service.get_user(user.id).await?;
            Ok(user.map(User::from))
        } else {
            return Err("User not found".into());
        }
    }
}
