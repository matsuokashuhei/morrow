use std::sync::Arc;

// サービスモジュールのインポート
pub mod authentication_service;
pub mod user_service;

use crate::domain::repositories::oidc_provider::OIDCProvider;
// エクスポート
use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::domain::repositories::user_repository::UserRepository;

pub use authentication_service::AuthenticationService;
pub use user_service::UserService;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    pub oauth_provider: Arc<dyn OIDCProvider>,
    pub user_repository: Arc<dyn UserRepository>,
    pub identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthenticationService>,
}

// リポジトリからサービスを初期化する関数
pub fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        // サービスの初期化
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
        auth_service: Arc::new(AuthenticationService::new(
            repositories.oauth_provider.clone(),
            repositories.user_repository.clone(),
            repositories.identity_link_repository.clone(),
        )),
    }
}
