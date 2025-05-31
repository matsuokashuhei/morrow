use anyhow::{Result, format_err};
use async_trait::async_trait;
use jsonwebtokens_cognito::KeySet;

use crate::domain::{
    services::authentication_service::AuthenticationService,
    value_objects::authentication::{Claims, SignInOutput, SignUpOutput},
};

pub struct CognitoService {
    client: aws_sdk_cognitoidentityprovider::Client,
}

impl CognitoService {
    pub fn new(aws_config: &aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(aws_config);
        Self { client }
    }
}

#[async_trait]
impl AuthenticationService for CognitoService {
    fn provider_name(&self) -> String {
        "cognito".to_string()
    }

    async fn sign_up(&self, email: &str, password: &str) -> Result<SignUpOutput> {
        self.client
            .sign_up()
            .client_id(std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .username(email)
            .password(password)
            .send()
            .await
            .map(|output| {
                Ok(SignUpOutput {
                    user_sub: output.user_sub,
                    user_confirmed: output.user_confirmed,
                    session: output.session,
                })
            })
            .map_err(|e| format_err!(e.into_service_error()))?
    }

    async fn sign_in(&self, email: &str, password: &str) -> Result<SignInOutput> {
        self.client
            .initiate_auth()
            .client_id(std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap())
            .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
            .auth_parameters("USERNAME", email)
            .auth_parameters("PASSWORD", password)
            .send()
            .await
            .map(|output| {
                if let Some(authentication_result) = output.authentication_result {
                    if let (Some(access_token), Some(refresh_token), Some(id_token)) = (
                        authentication_result.access_token,
                        authentication_result.refresh_token,
                        authentication_result.id_token,
                    ) {
                        Ok(SignInOutput {
                            id_token,
                            access_token,
                            refresh_token,
                            expires_in: authentication_result.expires_in,
                        })
                    } else {
                        Err(format_err!("Authentication failed"))
                    }
                } else {
                    Err(format_err!("Authentication failed"))
                }
            })
            .map_err(|e| format_err!(e.into_service_error()))?
    }

    async fn sign_out(&self, username: &str) -> Result<()> {
        self.client
            .admin_user_global_sign_out()
            .user_pool_id(std::env::var("AWS_COGNITO_USER_POOL_ID").unwrap())
            .username(username)
            .send()
            .await
            .map(|_| Ok(()))
            .map_err(|e| format_err!(e.into_service_error()))?
    }

    async fn verify_token(&self, access_token: &str) -> Result<Claims> {
        let key_set = KeySet::new(
            &std::env::var("AWS_REGION").unwrap(),
            &std::env::var("AWS_COGNITO_USER_POOL_ID").unwrap(),
        )
        .unwrap();
        let verifier = key_set
            .new_access_token_verifier(
                &[&std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap()],
            )
            .build()?;
        let claims_json = key_set
            .verify(&access_token, &verifier)
            .await
            .map_err(|e| format_err!(e.to_string()))?;
        let claims: Claims = serde_json::from_value(claims_json)?;
        Ok(claims)
    }
}
