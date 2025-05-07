use anyhow::Result;
use std::sync::Arc;

use crate::application::dtos::authentication_dto::{SignInDto, SignUpDto, TokenSetDto};
use crate::application::dtos::identity_link_dto::IdentityLinkDto;
use crate::application::dtos::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use crate::domain::entities::identity_link::NewIdentityLink;
use crate::domain::entities::user::NewUser;
use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::domain::repositories::oidc_provider::OIDCProvider;
use crate::domain::repositories::user_repository::UserRepository;

pub struct AuthenticationService {
    oidc_provider: Arc<dyn OIDCProvider>,
    user_repository: Arc<dyn UserRepository>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

impl AuthenticationService {
    pub fn new(
        oidc_provider: Arc<dyn OIDCProvider>,
        user_repository: Arc<dyn UserRepository>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
    ) -> Self {
        Self {
            oidc_provider,
            user_repository,
            identity_link_repository,
        }
    }

    pub async fn sign_up(&self, input: SignUpDto) -> Result<IdentityLinkDto> {
        let sub = self
            .oidc_provider
            .sign_up(&input.email, &input.password)
            .await?;
        let user = self.user_repository.create(NewUser::from(input)).await?;
        let identity_link = self
            .identity_link_repository
            .create(NewIdentityLink {
                provider: "cognito".to_string(),
                sub,
                user_id: user.id,
            })
            .await?;
        Ok(IdentityLinkDto::from(identity_link))
    }

    pub async fn sign_in(&self, input: SignInDto) -> Result<TokenSetDto> {
        let dto = self
            .oidc_provider
            .sign_in(&input.email, &input.password)
            .await
            .map(TokenSetDto::from)?;
        Ok(dto)
    }
}
