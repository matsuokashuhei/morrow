use crate::application::services::Repositories;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

// リポジトリモジュールのインポート
pub mod cognito_user_repository_impl;
pub mod user_repository_impl;

// エクスポート
pub use cognito_user_repository_impl::CognitoUserRepositoryImpl;
pub use user_repository_impl::UserRepositoryImpl;

// リポジトリを初期化する関数
pub fn init_repositories(
    connection: DatabaseConnection,
    aws_config: &aws_config::SdkConfig,
) -> Repositories {
    Repositories {
        // リポジトリの初期化
        user_repository: Arc::new(UserRepositoryImpl::new(Arc::new(connection.clone()))),
        cognito_user_repository: Arc::new(CognitoUserRepositoryImpl::new(aws_config)),
    }
}
