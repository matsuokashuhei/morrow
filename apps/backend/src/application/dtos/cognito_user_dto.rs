use crate::{
    domain::entities::cognito_user::CognitoUser,
    presentation::graphql::types::user_type::CreateUserInputType,
};

pub struct CognitoUserDto {
    user_confirmed: bool,
    user_sub: String,
}

impl From<CognitoUser> for CognitoUserDto {
    fn from(user: CognitoUser) -> Self {
        Self {
            user_confirmed: user.user_confirmed,
            user_sub: user.user_sub,
        }
    }
}

pub struct CreateCognitoUserDto {
    pub email: String,
    pub password: String,
}

impl From<CreateUserInputType> for CreateCognitoUserDto {
    fn from(input: CreateUserInputType) -> Self {
        Self {
            email: input.email,
            password: input.password,
        }
    }
}
