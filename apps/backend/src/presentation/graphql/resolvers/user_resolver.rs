use async_graphql::{Context, Object, Result, SimpleObject};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::services::UserService;
use crate::presentation::graphql::context::UserContext;
use crate::presentation::graphql::guards::{AuthenticationGuard, RoleGuard};
use crate::presentation::graphql::types::user_type::User;

#[derive(SimpleObject)]
pub struct UserStatistics {
    pub total_users: i32,
    pub active_users: i32,
    pub new_users_today: i32,
    pub last_updated: DateTime<Utc>,
}

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
    #[graphql(guard = "AuthenticationGuard")]
    async fn user(&self, _ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let user = self.service.get_user(id).await?; // Access the inner Uuid via id.0
        Ok(user.map(User::from))
    }

    // 全ユーザー取得 - 管理者ロール必須
    #[graphql(guard = "RoleGuard::admin()")]
    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = self.service.get_all_users().await?;
        Ok(users.into_iter().map(User::from).collect())
    }

    // 現在認証されているユーザーの情報を取得
    #[graphql(guard = "AuthenticationGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user_context = ctx.data::<UserContext>()?;
        if let Some(user) = user_context.user.clone() {
            let user = self.service.get_user(user.id).await?;
            Ok(user.map(User::from))
        } else {
            return Err("User not found".into());
        }
    }

    // User statistics for admin dashboard
    #[graphql(guard = "RoleGuard::admin()")]
    async fn user_statistics(&self, _ctx: &Context<'_>) -> Result<UserStatistics> {
        // In a real application, you'd calculate these from the database
        let all_users = self.service.get_all_users().await?;

        Ok(UserStatistics {
            total_users: all_users.len() as i32,
            active_users: all_users.len() as i32, // In real app, filter by activity
            new_users_today: 0,                   // In real app, filter by creation date
            last_updated: chrono::Utc::now(),
        })
    }
}
