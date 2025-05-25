// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::user_dto::UpdateUserDto;
use crate::application::services::UserService;
use crate::presentation::graphql::types::user_type::{UpdateUserInput, User}; // This path should now be correct

pub struct UserMutation {
    user_service: Arc<UserService>,
}

impl UserMutation {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}

#[Object]
impl UserMutation {
    async fn update_user(
        &self,
        _ctx: &Context<'_>,
        id: Uuid,
        input: UpdateUserInput,
    ) -> Result<Option<User>> {
        let dto = UpdateUserDto::from(input);
        let user = self.user_service.update_user(id, dto).await?;

        Ok(user.map(User::from))
    }

    async fn delete_user(&self, _ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        Ok(self.user_service.delete_user(id).await?)
    }
}
