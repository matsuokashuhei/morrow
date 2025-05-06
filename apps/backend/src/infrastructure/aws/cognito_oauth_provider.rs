use anyhow::format_err;
use async_trait::async_trait;

use crate::domain::repositories::oauth_provider::OAuthProvider;

pub struct CognitoOAuthProvider {
    client: aws_sdk_cognitoidentityprovider::Client,
}

impl CognitoOAuthProvider {
    pub fn new(aws_config: &aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(aws_config);
        Self { client }
    }
}

#[async_trait]
impl OAuthProvider for CognitoOAuthProvider {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<String> {
        self.client
            .sign_up()
            .client_id("15vl9j7l2q5gctk6f5o8sseelb")
            .username(email)
            .password(password)
            .send()
            .await
            .map(|output| Ok(output.user_sub))
            .map_err(|e| format_err!(e.into_service_error()))?
    }
}
