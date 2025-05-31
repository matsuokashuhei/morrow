use async_graphql::{InputObject, SimpleObject};

use crate::application::dtos::authentication_dto::SignInOutputDTO;

#[derive(InputObject, Clone)]
pub struct SignUpInput {
    #[graphql(validator(min_length = 1))]
    pub name: String,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(min_length = 8))]
    pub password: String,
}

#[derive(InputObject, Clone)]
pub struct SignInInput {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(min_length = 8))]
    pub password: String,
}

#[derive(SimpleObject)]
pub struct TokenSet {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}

#[derive(SimpleObject)]
pub struct SignOutResponse {
    pub success: bool,
    pub message: String,
}

impl From<SignInOutputDTO> for TokenSet {
    fn from(input: SignInOutputDTO) -> Self {
        Self {
            id_token: input.id_token,
            access_token: input.access_token,
            refresh_token: input.refresh_token,
            expires_in: input.expires_in,
        }
    }
}
