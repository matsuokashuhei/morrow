use crate::domain::entities::auth::{AuthContext, UserRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContextDto {
    pub user_id: Option<i32>,
    pub sub: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub is_authenticated: bool,
}

impl From<AuthContext> for AuthContextDto {
    fn from(context: AuthContext) -> Self {
        Self {
            user_id: context.user_id,
            sub: context.sub,
            email: context.email,
            roles: context
                .roles
                .iter()
                .map(|r| r.as_str().to_string())
                .collect(),
            is_authenticated: context.is_authenticated,
        }
    }
}

impl From<AuthContextDto> for AuthContext {
    fn from(dto: AuthContextDto) -> Self {
        AuthContext {
            user_id: dto.user_id,
            sub: dto.sub,
            email: dto.email,
            roles: dto
                .roles
                .iter()
                .map(|r| UserRole::from(r.as_str()))
                .collect(),
            is_authenticated: dto.is_authenticated,
        }
    }
}

// pub struct SignUpInput {
//     pub email: String,
//     pub password: String,
// }

// pub struct SignUpOutput {
//     pub user_sub: String,
//     pub user_confirmed: bool,
//     pub session: Option<String>,
// }

// pub struct SignInOutput {
//     pub id_token: String,
//     pub access_token: String,
//     pub refresh_token: String,
//     pub expires_in: i32,
// }
