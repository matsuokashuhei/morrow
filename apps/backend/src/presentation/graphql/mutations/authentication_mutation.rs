// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::authentication_dto::{SignInDto, SignUpDto};
use crate::application::services::authentication_service::AuthenticationService;
use crate::presentation::graphql::types::authentication_type::{
    SignInInput, SignUpInput, TokenSet,
};
use crate::presentation::graphql::types::identity_link_type::IentityLink;

pub struct AuthenticationMutation {
    authentication_service: Arc<AuthenticationService>,
}

impl AuthenticationMutation {
    pub fn new(authentication_service: Arc<AuthenticationService>) -> Self {
        Self {
            authentication_service,
        }
    }
}

#[Object]
impl AuthenticationMutation {
    async fn sign_up(&self, _ctx: &Context<'_>, input: SignUpInput) -> Result<IentityLink> {
        let sign_up_dto = SignUpDto::from(input);
        let identity_link_dto = self.authentication_service.sign_up(sign_up_dto).await?;

        Ok(IentityLink::from(identity_link_dto))
    }

    async fn sign_in(&self, _ctx: &Context<'_>, input: SignInInput) -> Result<TokenSet> {
        let sign_in_dto = SignInDto::from(input);
        let token_set_dto = self.authentication_service.sign_in(sign_in_dto).await?;

        Ok(TokenSet::from(token_set_dto))
    }
}
