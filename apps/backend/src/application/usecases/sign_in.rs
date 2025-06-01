use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::authentication_dto::{SignInInputDTO, SignInOutputDTO},
    domain::{
        repositories::{identity_link_repository::IdentityLinkRepository, user_repository::UserRepository},
        services::authentication_service::AuthenticationService,
    },
};

pub struct SignIn {
    authentication_service: Arc<dyn AuthenticationService>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl SignIn {
    pub fn new(
        authentication_service: Arc<dyn AuthenticationService>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            authentication_service,
            identity_link_repository,
            user_repository,
        }
    }

    pub async fn execute(&self, input: SignInInputDTO) -> Result<SignInOutputDTO> {
        // First, authenticate with the authentication service
        let auth_output = self
            .authentication_service
            .sign_in(&input.email, &input.password)
            .await?;

        // Verify the token to get user claims
        let claims = self
            .authentication_service
            .verify_token(&auth_output.access_token)
            .await?;

        // Look up the identity link to ensure the user exists in our database
        let identity_link = self
            .identity_link_repository
            .find_by_sub(&claims.sub)
            .await?;

        // Verify the user exists in our database
        let _user = self
            .user_repository
            .find_by_id(identity_link.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(SignInOutputDTO {
            id_token: auth_output.id_token,
            access_token: auth_output.access_token,
            refresh_token: auth_output.refresh_token,
            expires_in: auth_output.expires_in,
        })
    }
}
