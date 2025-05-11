use crate::application::services::Repositories;
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
    Repositories {
        user_repository: Arc::new(UserRepositoryImpl::new(Arc::new(connection.clone()))),
        identity_link_repository: Arc::new(IdentityLinkRepositoryImpl::new(Arc::new(
            connection.clone(),
        ))),
    }
}
