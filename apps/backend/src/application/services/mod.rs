use std::sync::Arc;

// サービスモジュールのインポート
pub mod cognito_user_service;
pub mod user_service;
// mod auth_service;

// エクスポート
pub use cognito_user_service::CognitoUserService;
pub use user_service::UserService;

use crate::domain::repositories::{
    cognito_user_repository::CognitoUserRepository, user_repository::UserRepository,
};

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    pub user_repository: Arc<dyn UserRepository>,
    pub cognito_user_repository: Arc<dyn CognitoUserRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    pub user_service: Arc<UserService>,
    pub cognito_user_service: Arc<CognitoUserService>,
    // pub auth_service: AuthService,
}

// リポジトリからサービスを初期化する関数
pub fn init_services(repositories: Arc<Repositories>) -> Services {
    Services {
        // サービスの初期化
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
        cognito_user_service: Arc::new(CognitoUserService::new(
            repositories.cognito_user_repository.clone(),
        )),
    }
}
