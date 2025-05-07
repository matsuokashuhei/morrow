use crate::{
    domain::entities::token_set::TokenSet,
    presentation::graphql::types::authentication_type::{SignInInput, SignUpInput},
};

#[derive(Debug, Clone)]
pub struct SignUpDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<SignUpInput> for SignUpDto {
    fn from(input: SignUpInput) -> Self {
        Self {
            name: input.name,
            email: input.email,
            password: input.password,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}

impl From<SignInInput> for SignInDto {
    fn from(input: SignInInput) -> Self {
        Self {
            email: input.email,
            password: input.password,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenSetDto {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

impl From<TokenSet> for TokenSetDto {
    fn from(input: TokenSet) -> Self {
        Self {
            id_token: input.id_token,
            access_token: input.access_token,
            refresh_token: input.refresh_token,
            expires_in: input.expires_in,
            token_type: input.token_type,
        }
    }
}
