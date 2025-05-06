use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::cognito_user::CognitoUser;

#[async_trait]
pub trait CognitoUserRepository: Send + Sync {
    async fn sign_up(&self, email: &str, password: &str) -> Result<CognitoUser>;
    // async fn sign_in(&self, email: &str, password: &str) -> Result<String, String>;
    // async fn sign_out(&self, token: &str) -> Result<(), String>;
    // async fn refresh_token(&self, token: &str) -> Result<String, String>;
}
