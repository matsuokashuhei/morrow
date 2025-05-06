use anyhow::{Result, format_err};
use async_trait::async_trait;

use crate::domain::{
    entities::cognito_user::CognitoUser,
    repositories::cognito_user_repository::CognitoUserRepository,
};

pub struct CognitoUserRepositoryImpl {
    client: aws_sdk_cognitoidentityprovider::Client,
}

impl CognitoUserRepositoryImpl {
    // pub fn new(client: aws_sdk_cognitoidentityprovider::Client) -> Self {
    //     Self { client }
    // }
    pub fn new(aws_config: &aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(aws_config);
        Self { client }
    }
}

#[async_trait]
impl CognitoUserRepository for CognitoUserRepositoryImpl {
    async fn sign_up(&self, email: &str, password: &str) -> Result<CognitoUser> {
        self.client
            .sign_up()
            .client_id("15vl9j7l2q5gctk6f5o8sseelb")
            .username(email)
            .password(password)
            .send()
            .await
            .map(|output| Ok(CognitoUser::from(output)))
            .map_err(|e| format_err!(e.into_service_error()))?
    }
}
