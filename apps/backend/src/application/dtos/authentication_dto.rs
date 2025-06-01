use crate::presentation::graphql::types::authentication_type::{SignInInput, SignUpInput};

#[derive(Debug, Clone)]
pub struct SignUpInputDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<SignUpInput> for SignUpInputDTO {
    fn from(input: SignUpInput) -> Self {
        Self {
            name: input.name,
            email: input.email,
            password: input.password,
        }
    }
}

pub struct SignInInputDTO {
    pub email: String,
    pub password: String,
}

impl From<SignInInput> for SignInInputDTO {
    fn from(input: SignInInput) -> Self {
        Self {
            email: input.email,
            password: input.password,
        }
    }
}

pub struct SignInOutputDTO {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}

pub struct SignOutInputDTO {
    pub username: String,
}

// impl From<SignOutInput> for SignOutInputDTO {
//     fn from(input: SignOutInput) -> Self {
//         Self {
//             username: input.username,
//         }
//     }
// }
