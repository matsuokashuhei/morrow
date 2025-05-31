// use anyhow::Result;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::application::dtos::authentication_dto::{
    SignInInputDTO, SignOutInputDTO, SignUpInputDTO,
};
use crate::application::usecases::sign_in::SignIn;
use crate::application::usecases::sign_out::SignOut;
use crate::application::usecases::sign_up::SignUp;
use crate::presentation::graphql::context::UserContext;
use crate::presentation::graphql::types::authentication_type::{
    SignInInput, SignOutResponse, SignUpInput, TokenSet,
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

    async fn sign_out(&self, ctx: &Context<'_>) -> Result<SignOutResponse> {
        let user_context = ctx.data::<UserContext>().unwrap();
        match &user_context.user {
            Some(user) => {
                let input_dto = SignOutInputDTO {
                    username: user.identity_links.first().unwrap().clone().sub,
                };
                match self.sign_out.execute(input_dto).await {
                    Ok(_) => Ok(SignOutResponse {
                        success: true,
                        message: "Successfully signed out".to_string(),
                    }),
                    Err(e) => Ok(SignOutResponse {
                        success: false,
                        message: format!("Sign out failed: {}", e),
                    }),
                }
            }
            None => Ok(SignOutResponse {
                success: false,
                message: "No user context available".to_string(),
            }),
        }
        // match ctx.data::<UserContext>() {
        //     Some(user_context) => {
        //         if let Some(username) = &user_context.user {
        //             self.perform_sign_out(SignOutInput {
        //                 username: username.clone(),
        //             })
        //             .await
        //         } else {
        //             Ok(SignOutResponse {
        //                 success: false,
        //                 message: "User not authenticated".to_string(),
        //             })
        //         }
        //     }
        //     None => Ok(SignOutResponse {
        //         success: false,
        //         message: "No user context available".to_string(),
        //     }),
        // }
        // let input_dto = SignOutDTO::from(input);

        // match self.sign_out.execute(input_dto).await {
        //     Ok(_) => Ok(SignOutResponse {
        //         success: true,
        //         message: "Successfully signed out".to_string(),
        //     }),
        //     Err(e) => Ok(SignOutResponse {
        //         success: false,
        //         message: format!("Sign out failed: {}", e),
        //     }),
        // }
    }
}
