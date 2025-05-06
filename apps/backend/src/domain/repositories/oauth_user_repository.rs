use crate::domain::entities::oauth_user::{NewOAuthUser, OAuthUser};
use async_trait::async_trait;

#[async_trait]
pub trait OAuthUserRepository: Send + Sync + 'static {
    async fn create(&self, user: NewOAuthUser) -> anyhow::Result<OAuthUser>;
}
