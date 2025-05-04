use sea_orm::DatabaseConnection;
use crate::application::services::Repositories;

// リポジトリモジュールのインポート
// mod user_repository_impl;

// エクスポート
// pub use user_repository_impl::UserRepositoryImpl;

// リポジトリを初期化する関数
pub fn init_repositories(db: DatabaseConnection) -> Repositories {
    Repositories {
        // リポジトリの初期化
        // user_repository: Arc::new(UserRepositoryImpl::new(db.clone())),
    }
}
