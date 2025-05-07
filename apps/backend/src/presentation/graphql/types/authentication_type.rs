use async_graphql::{InputObject, SimpleObject};

use crate::application::dtos::authentication_dto::TokenSetDto;

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
    pub token_type: String,
}

impl From<TokenSetDto> for TokenSet {
    fn from(input: TokenSetDto) -> Self {
        Self {
            id_token: input.id_token,
            access_token: input.access_token,
            refresh_token: input.refresh_token,
            expires_in: input.expires_in,
            token_type: input.token_type,
        }
    }
}
