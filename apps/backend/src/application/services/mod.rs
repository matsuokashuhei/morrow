use std::sync::Arc;

// サービスモジュールのインポート
// pub mod authentication_service;
pub mod authorization_service;
pub mod user_service;

use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::domain::repositories::oidc_provider::OIDCProvider;
use crate::domain::repositories::token_verifier::TokenVerifier;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::authentication_service::AuthenticationService;
use crate::domain::usecases::authorization_usecase::{
    AuthorizationUseCase, AuthorizationUseCaseImpl,
};
use crate::infrastructure::authentication::cognito_service::CognitoService;

// エクスポート
// pub use authentication_service::AuthenticationService;
pub use authorization_service::AuthorizationService;
pub use user_service::UserService;

// リポジトリを格納する構造体
#[derive(Clone)]
pub struct Repositories {
    // 各リポジトリをここに追加
    // pub oauth_provider: Arc<dyn OIDCProvider>,
    pub token_verifier: Arc<dyn TokenVerifier>,
    pub user_repository: Arc<dyn UserRepository>,
    pub identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

// サービスを格納する構造体
#[derive(Clone)]
pub struct Services {
    // 各サービスをここに追加
    pub user_service: Arc<UserService>,
    pub authorization_service: Arc<AuthorizationService>,
    // pub auhthentication_service: Arc<dyn AuthenticationService>,
}

// リポジトリからサービスを初期化する関数
pub async fn init_services(repositories: Arc<Repositories>) -> Services {
    // 認可ユースケースの初期化
    let authorization_usecase: Arc<dyn AuthorizationUseCase> =
        Arc::new(AuthorizationUseCaseImpl::new(
            repositories.token_verifier.clone(),
            repositories.identity_link_repository.clone(),
            repositories.user_repository.clone(),
        ));

    Services {
        // サービスの初期化
        user_service: Arc::new(UserService::new(repositories.user_repository.clone())),
        authorization_service: Arc::new(AuthorizationService::new(authorization_usecase)),
        // authentication_service: Arc::new(CognitoService::new(
        //     repositories.oauth_provider.clone(),
        //     repositories.user_repository.clone(),
        //     repositories.identity_link_repository.clone(),
        // )),
    }
}
