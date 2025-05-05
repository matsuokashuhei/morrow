use std::sync::Arc;

// サービスモジュールのインポート
pub mod user_service;
// mod auth_service;

// エクスポート
pub use user_service::UserService;
// pub use auth_service::AuthService;

use crate::domain::repositories::user_repository::UserRepository;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    pub user_repository: Arc<dyn UserRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    pub user_service: Arc<UserService>,
    // pub auth_service: AuthService,
}

// リポジトリからサービスを初期化する関数
pub fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        // サービスの初期化
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
        // auth_service: AuthService::new(Arc::clone(&repositories.user_repository)),
    }
}
