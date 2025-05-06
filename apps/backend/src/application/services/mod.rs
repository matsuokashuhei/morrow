use std::sync::Arc;

// サービスモジュールのインポート
pub mod auth_service;
pub mod user_service;

use crate::domain::repositories::oauth_provider::OAuthProvider;
// エクスポート
use crate::domain::repositories::oauth_user_repository::OAuthUserRepository;
use crate::domain::repositories::user_repository::UserRepository;

pub use auth_service::AuthService;
pub use user_service::UserService;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    pub oauth_provider: Arc<dyn OAuthProvider>,
    pub user_repository: Arc<dyn UserRepository>,
    pub oauth_user_repository: Arc<dyn OAuthUserRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}

// リポジトリからサービスを初期化する関数
pub fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        // サービスの初期化
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
        auth_service: Arc::new(AuthService::new(
            repositories.oauth_provider.clone(),
            repositories.user_repository.clone(),
            repositories.oauth_user_repository.clone(),
        )),
    }
}
