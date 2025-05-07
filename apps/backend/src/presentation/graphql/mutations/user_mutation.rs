// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::user_dto::UpdateUserDto;
use crate::application::services::UserService;
use crate::presentation::graphql::types::user_type::{UpdateUserInput, User};

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
    // async fn create_user(
    //     &self,
    //     _ctx: &Context<'_>,
    //     input: CreateUserInputType,
    // ) -> Result<UserType> {
    //     let create_user_dto = CreateUserDto::from(input.clone());
    //     let user = self.user_service.create_user(create_user_dto).await?;
    //     let create_cognito_user_dto = CreateCognitoUserDto::from(input.clone());
    //     let cognito_user = self
    //         .cognito_user_service
    //         .sign_up(create_cognito_user_dto)
    //         .await?;

    //     Ok(UserType::from(user))
    // }

    async fn update_user(
        &self,
        _ctx: &Context<'_>,
        id: i32,
        input: UpdateUserInput,
    ) -> Result<Option<User>> {
        let dto = UpdateUserDto::from(input);
        let user = self.user_service.update_user(id, dto).await?;

        Ok(user.map(User::from))
    }

    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> Result<bool> {
        Ok(self.user_service.delete_user(id).await?)
    }
}
