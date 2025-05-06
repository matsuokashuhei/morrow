use anyhow::Result;
use std::sync::Arc;

use crate::application::dtos::auth_dto::SignUpDto;
use crate::application::dtos::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use crate::domain::entities::oauth_user::NewOAuthUser;
use crate::domain::entities::user::NewUser;
use crate::domain::repositories::oauth_provider::OAuthProvider;
use crate::domain::repositories::oauth_user_repository::OAuthUserRepository;
use crate::domain::repositories::user_repository::UserRepository;

pub struct AuthService {
    oauth_provider: Arc<dyn OAuthProvider>,
    user_repository: Arc<dyn UserRepository>,
    oauth_user_repository: Arc<dyn OAuthUserRepository>,
}

impl AuthService {
    pub fn new(
        oauth_provider: Arc<dyn OAuthProvider>,
        user_repository: Arc<dyn UserRepository>,
        oauth_user_repository: Arc<dyn OAuthUserRepository>,
    ) -> Self {
        Self {
            oauth_provider,
            user_repository,
            oauth_user_repository,
        }
    }

    pub async fn sign_up(&self, input: SignUpDto) -> Result<UserDto> {
        let sub = self
            .oauth_provider
            .sign_up(&input.email, &input.password)
            .await?;
        let user = self.user_repository.create(NewUser::from(input)).await?;
        self.oauth_user_repository
            .create(NewOAuthUser {
                provider: "cognito".to_string(),
                sub,
                user_id: user.id,
            })
            .await?;
        Ok(UserDto::from(user))
    }
}
