// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::authentication_dto::{SignInInputDTO, SignUpInputDTO};
use crate::application::usecases::sign_in::SignIn;
use crate::application::usecases::sign_out::SignOut;
use crate::application::usecases::sign_up::SignUp;
use crate::presentation::graphql::types::authentication_type::{
    SignInInput, SignUpInput, TokenSet,
};
use crate::presentation::graphql::types::identity_link_type::IentityLink;

pub struct AuthenticationMutation {
    sign_up: Arc<SignUp>,
    sign_in: Arc<SignIn>,
    sign_out: Arc<SignOut>,
}

impl AuthenticationMutation {
    pub fn new(sign_up: Arc<SignUp>, sign_in: Arc<SignIn>, sign_out: Arc<SignOut>) -> Self {
        Self {
            sign_up,
            sign_in,
            sign_out,
        }
    }
}

#[Object]
impl AuthenticationMutation {
    async fn sign_up(&self, _ctx: &Context<'_>, input: SignUpInput) -> Result<IentityLink> {
        let input_dto = SignUpInputDTO::from(input);
        let identity_link_dto = self.sign_up.execute(input_dto).await?;

        Ok(IentityLink::from(identity_link_dto))
    }

    async fn sign_in(&self, _ctx: &Context<'_>, input: SignInInput) -> Result<TokenSet> {
        let input_dto = SignInInputDTO::from(input);
        let output_dto = self.sign_in.execute(input_dto).await?;

        Ok(TokenSet::from(output_dto))
    }
}
