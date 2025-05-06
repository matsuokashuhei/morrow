// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::auth_dto::SignUpDto;
use crate::application::services::auth_service::AuthService;
use crate::presentation::graphql::types::auth_type::SignUpInput;
use crate::presentation::graphql::types::user_type::UserType;

pub struct AuthMutation {
    oauth_service: Arc<AuthService>,
}

impl AuthMutation {
    pub fn new(oauth_service: Arc<AuthService>) -> Self {
        Self { oauth_service }
    }
}

#[Object]
impl AuthMutation {
    async fn sign_up(&self, _ctx: &Context<'_>, input: SignUpInput) -> Result<UserType> {
        let dto = SignUpDto::from(input);
        let user = self.oauth_service.sign_up(dto).await?;

        Ok(UserType::from(user))
    }
}
