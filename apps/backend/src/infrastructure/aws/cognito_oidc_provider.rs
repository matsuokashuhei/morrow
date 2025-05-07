use anyhow::format_err;
use async_trait::async_trait;
use aws_sdk_cognitoidentityprovider::types::AuthFlowType;

use crate::domain::{entities::token_set::TokenSet, repositories::oidc_provider::OIDCProvider};

pub struct CognitoOIDCProvider {
    client: aws_sdk_cognitoidentityprovider::Client,
}

impl CognitoOIDCProvider {
    pub fn new(aws_config: &aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(aws_config);
        Self { client }
    }
}

#[async_trait]
impl OIDCProvider for CognitoOIDCProvider {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<String> {
        self.client
            .sign_up()
            .client_id(std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .username(email)
            .password(password)
            .send()
            .await
            .map(|output| Ok(output.user_sub))
            .map_err(|e| format_err!(e.into_service_error()))?
    }

    async fn sign_in(&self, email: &str, password: &str) -> anyhow::Result<TokenSet> {
        self.client
            .initiate_auth()
            .client_id(std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .auth_flow(AuthFlowType::UserPasswordAuth)
            .auth_parameters("USERNAME", email)
            .auth_parameters("PASSWORD", password)
            .send()
            .await
            .map_err(|e| format_err!(e.into_service_error()))
            .map(|output| {
                if let Some(authentication_result) = output.authentication_result {
                    Ok(TokenSet {
                        access_token: authentication_result.access_token.unwrap_or_default(),
                        refresh_token: authentication_result.refresh_token.unwrap_or_default(),
                        id_token: authentication_result.id_token.unwrap_or_default(),
                        expires_in: authentication_result.expires_in,
                        token_type: authentication_result.token_type.unwrap_or_default(),
                    })
                } else {
                    Err(format_err!("Authentication failed"))
                }
            })?
    }
}
