use anyhow::Result;
use async_trait::async_trait;

use crate::domain::value_objects::authentication::{Claims, SignInOutput, SignUpOutput};

#[async_trait]
pub trait AuthenticationService: Send + Sync + 'static {
    fn provider_name(&self) -> String;
    async fn sign_up(&self, email: &str, password: &str) -> Result<SignUpOutput>;
    async fn sign_in(&self, email: &str, password: &str) -> Result<SignInOutput>;
    async fn sign_out(&self, access_token: &str) -> Result<()>;
    async fn verify_token(&self, access_token: &str) -> Result<Claims>;
}
