use std::sync::Arc;

// サービスモジュールのインポート
// pub mod authentication_service;
pub mod user_service;

use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::domain::repositories::user_repository::UserRepository;

// エクスポート
pub use user_service::UserService;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    pub user_repository: Arc<dyn UserRepository>,
    pub identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<UserService>,
}

// リポジトリからサービスを初期化する関数
pub async fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
    }
}
