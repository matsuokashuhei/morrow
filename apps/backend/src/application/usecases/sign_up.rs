use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::{
        authentication_dto::{SignInInputDTO, SignUpInputDTO},
        identity_link_dto::IdentityLinkDto,
    },
    domain::{
        entities::{identity_link::NewIdentityLink, user::NewUser},
        repositories::{
            identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
        },
        services::authentication_service::AuthenticationService,
    },
};

pub struct SignUp {
    authentication_service: Arc<dyn AuthenticationService>,
    user_repository: Arc<dyn UserRepository>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

impl SignUp {
    pub fn new(
        authentication_service: Arc<dyn AuthenticationService>,
        user_repository: Arc<dyn UserRepository>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
    ) -> Self {
        Self {
            authentication_service,
            user_repository,
            identity_link_repository,
        }
    }

    pub async fn execute(&self, input: SignUpInputDTO) -> Result<IdentityLinkDto> {
        let output = self
            .authentication_service
            .sign_up(&input.email, &input.password)
            .await?;
        let user = self.user_repository.create(NewUser::from(input)).await?;
        let identity_link = self
            .identity_link_repository
            .create(NewIdentityLink {
                provider: self.authentication_service.provider_name(),
                sub: output.user_sub,
                user_id: user.id,
            })
            .await?;
        Ok(IdentityLinkDto::from(identity_link))
    }
}
