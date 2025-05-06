use crate::{
    application::services::Repositories,
    infrastructure::aws::cognito_oauth_provider::CognitoOAuthProvider,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

// リポジトリモジュールのインポート
pub mod oauth_user_repository_impl;
pub mod user_repository_impl;

// エクスポート
pub use oauth_user_repository_impl::OAuthUserRepositoryImpl;
pub use user_repository_impl::UserRepositoryImpl;

// リポジトリを初期化する関数
pub fn init_repositories(
    connection: DatabaseConnection,
    aws_config: &aws_config::SdkConfig,
) -> Repositories {
    Repositories {
        // リポジトリの初期化
        oauth_provider: Arc::new(CognitoOAuthProvider::new(aws_config)),
        user_repository: Arc::new(UserRepositoryImpl::new(Arc::new(connection.clone()))),
        oauth_user_repository: Arc::new(OAuthUserRepositoryImpl::new(Arc::new(connection.clone()))),
    }
}
