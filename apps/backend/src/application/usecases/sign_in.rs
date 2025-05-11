use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::authentication_dto::{SignInInputDTO, SignInOutputDTO},
    domain::{
        repositories::identity_link_repository::IdentityLinkRepository,
        services::authentication_service::AuthenticationService,
    },
};

pub struct SignIn {
    authentication_service: Arc<dyn AuthenticationService>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

impl SignIn {
    pub fn new(
        authentication_service: Arc<dyn AuthenticationService>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
    ) -> Self {
        Self {
            authentication_service,
            identity_link_repository,
        }
    }

    pub async fn execute(&self, input: SignInInputDTO) -> Result<SignInOutputDTO> {
        let output = self
            .authentication_service
            .sign_in(&input.email, &input.password)
            .await?;
        Ok(SignInOutputDTO {
            id_token: output.id_token,
            access_token: output.access_token,
            refresh_token: output.refresh_token,
            expires_in: output.expires_in,
        })
    }
}
