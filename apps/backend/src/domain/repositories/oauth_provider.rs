use async_trait::async_trait;

#[async_trait]
pub trait OAuthProvider: Send + Sync + 'static {
    async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<String>;
}
