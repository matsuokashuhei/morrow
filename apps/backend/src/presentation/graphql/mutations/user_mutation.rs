use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::user_dto::{CreateUserDto, UpdateUserDto};
use crate::application::services::UserService;
use crate::presentation::graphql::types::user_type::{
    CreateUserInputType, UpdateUserInputType, UserType,
};

pub struct UserMutation {
    service: Arc<UserService>,
}

impl UserMutation {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }
}

#[Object]
impl UserMutation {
    async fn create_user(
        &self,
        _ctx: &Context<'_>,
        input: CreateUserInputType,
    ) -> Result<UserType> {
        let dto = CreateUserDto::from(input);
        let user = self.service.create_user(dto).await?;

        Ok(UserType::from(user))
    }

    async fn update_user(
        &self,
        _ctx: &Context<'_>,
        id: i32,
        input: UpdateUserInputType,
    ) -> Result<Option<UserType>> {
        let dto = UpdateUserDto::from(input);
        let user = self.service.update_user(id, dto).await?;

        Ok(user.map(UserType::from))
    }

    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> Result<bool> {
        Ok(self.service.delete_user(id).await?)
    }
}
