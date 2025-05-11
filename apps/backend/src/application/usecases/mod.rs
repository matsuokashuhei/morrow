use std::sync::Arc;

use authenticate_user::AuthenticateUser;
use sign_in::SignIn;
use sign_up::SignUp;

use crate::domain::services::authentication_service::AuthenticationService;

use super::services::Repositories;

pub mod authenticate_user;
pub mod sign_in;
pub mod sign_out;
pub mod sign_up;

pub struct UseCases {
    pub sign_up: Arc<SignUp>,
    pub sign_in: Arc<SignIn>,
    pub sign_out: Arc<sign_out::SignOut>,
    pub authenticate_user: Arc<AuthenticateUser>,
}

pub fn init_use_cases(
    repositories: Arc<Repositories>,
    authentication_service: Arc<dyn AuthenticationService>,
) -> UseCases {
    let sign_up = SignUp::new(
        authentication_service.clone(),
        repositories.user_repository.clone(),
        repositories.identity_link_repository.clone(),
    );

    let sign_in = SignIn::new(
        authentication_service.clone(),
        repositories.identity_link_repository.clone(),
    );

    let sign_out = sign_out::SignOut::new(authentication_service.clone());
    let authenticate_user = AuthenticateUser::new(
        authentication_service.clone(),
        repositories.identity_link_repository.clone(),
        repositories.user_repository.clone(),
    );

    UseCases {
        sign_up: Arc::new(sign_up),
        sign_in: Arc::new(sign_in),
        sign_out: Arc::new(sign_out),
        authenticate_user: Arc::new(authenticate_user),
    }
}
