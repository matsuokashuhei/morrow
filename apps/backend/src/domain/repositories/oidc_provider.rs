use async_trait::async_trait;

use crate::domain::entities::token_set::TokenSet;

#[async_trait]
pub trait OIDCProvider: Send + Sync + 'static {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<String>;
    async fn sign_in(&self, email: &str, password: &str) -> anyhow::Result<TokenSet>;
}
