use std::sync::Arc;

// サービスモジュールのインポート
// mod user_service;
// mod auth_service;

// エクスポート
// pub use user_service::UserService;
// pub use auth_service::AuthService;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    // pub user_repository: Arc<dyn UserRepository + Send + Sync>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    // pub user_service: UserService,
    // pub auth_service: AuthService,
}

// リポジトリからサービスを初期化する関数
pub fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        // サービスの初期化
        // user_service: UserService::new(Arc::clone(&repositories.user_repository)),
        // auth_service: AuthService::new(Arc::clone(&repositories.user_repository)),
    }
}
