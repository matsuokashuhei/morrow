use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::services::UserService;
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
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let user = self.service.get_user(id).await?;

        Ok(user.map(User::from))
    }

    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = self.service.get_all_users().await?;

        Ok(users.into_iter().map(User::from).collect())
    }
}
