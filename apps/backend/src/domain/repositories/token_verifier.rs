use async_trait::async_trait;

#[async_trait]
pub trait TokenVerifier: Send + Sync + 'static {
    /// アクセストークンを検証し、トークンからユーザー情報を抽出します
    async fn verify_access_token(&self, token: &str) -> anyhow::Result<TokenClaims>;
}

/// アクセストークンから抽出されたクレーム情報
pub struct TokenClaims {
    pub sub: String,
    pub email: Option<String>,
    pub groups: Vec<String>,
    pub scope: Option<String>,
    pub exp: u64,
}
