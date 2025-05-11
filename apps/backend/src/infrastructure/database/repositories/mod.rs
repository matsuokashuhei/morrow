use crate::{
    application::services::Repositories,
    infrastructure::aws::cognito_token_verifier::CognitoTokenVerifier,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

// リポジトリモジュールのインポート
pub mod identity_link_repository_impl;
pub mod user_repository_impl;

// エクスポート
pub use identity_link_repository_impl::IdentityLinkRepositoryImpl;
pub use user_repository_impl::UserRepositoryImpl;

// リポジトリを初期化する関数
pub fn init_repositories(connection: DatabaseConnection) -> Repositories {
    // AWS Cognitoの設定を環境変数から取得
    let cognito_region = std::env::var("AWS_REGION").unwrap();
    let cognito_user_pool_id = std::env::var("AWS_COGNITO_USER_POOL_ID").unwrap();
    let cognito_client_id = std::env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap();

    Repositories {
        // リポジトリの初期化
        // oauth_provider: Arc::new(CognitoOIDCProvider::new(aws_config)),
        token_verifier: Arc::new(CognitoTokenVerifier::new(
            cognito_region,
            cognito_user_pool_id,
            cognito_client_id,
        )),
        user_repository: Arc::new(UserRepositoryImpl::new(Arc::new(connection.clone()))),
        identity_link_repository: Arc::new(IdentityLinkRepositoryImpl::new(Arc::new(
            connection.clone(),
        ))),
    }
}
