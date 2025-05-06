use anyhow::{Result, format_err};
use std::sync::Arc;

use crate::{
    application::dtos::cognito_user_dto::{CognitoUserDto, CreateCognitoUserDto},
    domain::repositories::cognito_user_repository::CognitoUserRepository,
};

pub struct CognitoUserService {
    cognito_user_repository: Arc<dyn CognitoUserRepository>,
}

impl CognitoUserService {
    pub fn new(cognito_user_repository: Arc<dyn CognitoUserRepository>) -> Self {
        CognitoUserService {
            cognito_user_repository: cognito_user_repository,
        }
    }

    pub async fn sign_up(&self, input: CreateCognitoUserDto) -> Result<CognitoUserDto> {
        self.cognito_user_repository
            .sign_up(&input.email, &input.password)
            .await
            .map(|cognito_user| CognitoUserDto::from(cognito_user))
    }
}
