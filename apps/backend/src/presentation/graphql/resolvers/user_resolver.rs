use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::services::UserService;
use crate::presentation::graphql::types::user_type::UserType;

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
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<UserType>> {
        let user = self.service.get_user(id).await?;

        Ok(user.map(UserType::from))
    }

    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<UserType>> {
        let users = self.service.get_all_users().await?;

        Ok(users.into_iter().map(UserType::from).collect())
    }
}
