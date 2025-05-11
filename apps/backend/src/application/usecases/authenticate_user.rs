use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::user_dto::UserDTO,
    domain::{
        repositories::{
            identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
        },
        services::authentication_service::AuthenticationService,
    },
};

pub struct AuthenticateUser {
    authentication_service: Arc<dyn AuthenticationService>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthenticateUser {
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

    pub async fn execute(&self, access_token: &str) -> Result<UserDTO> {
        let claims = self
            .authentication_service
            .verify_token(access_token)
            .await?;
        let identity_link = self
            .identity_link_repository
            .find_by_sub(&claims.sub)
            .await?;
        let user = self
            .user_repository
            .find_by_id(identity_link.user_id)
            .await?;
        match user {
            Some(user) => Ok(UserDTO::from(user)),
            None => {
                return Err(anyhow::anyhow!("User not found"));
            }
        }
    }
}
