use anyhow::Result;
use std::sync::Arc;

use crate::application::dtos::auth_dto::AuthContextDto;
use crate::domain::entities::auth::UserRole;
use crate::domain::usecases::authorization_usecase::AuthorizationUseCase;

pub struct AuthorizationService {
    authorization_usecase: Arc<dyn AuthorizationUseCase>,
}

impl AuthorizationService {
    pub fn new(authorization_usecase: Arc<dyn AuthorizationUseCase>) -> Self {
        Self {
            authorization_usecase,
        }
    }

    /// トークンからユーザー認証コンテキストを取得する
    pub async fn get_auth_context(&self, token: Option<&str>) -> Result<AuthContextDto> {
        let auth_context = self
            .authorization_usecase
            .get_auth_context_from_token(token)
            .await?;
        Ok(AuthContextDto::from(auth_context))
    }

    /// 認証済みかどうかをチェックする
    pub fn validate_authenticated(&self, auth_context: &AuthContextDto) -> Result<()> {
        let domain_context = auth_context.clone().into();
        self.authorization_usecase
            .validate_authenticated(&domain_context)
    }

    /// 指定されたロールを持っているかチェックする
    pub fn validate_has_role(&self, auth_context: &AuthContextDto, role: &str) -> Result<()> {
        let domain_context = auth_context.clone().into();
        let domain_role = UserRole::from(role);
        self.authorization_usecase
            .validate_has_any_role(&domain_context, &[domain_role])
    }

    /// 指定されたロールのいずれかを持っているかチェックする
    pub fn validate_has_any_role(
        &self,
        auth_context: &AuthContextDto,
        roles: &[&str],
    ) -> Result<()> {
        let domain_context = auth_context.clone().into();
        let domain_roles = roles.iter().map(|r| UserRole::from(*r)).collect::<Vec<_>>();
        self.authorization_usecase
            .validate_has_any_role(&domain_context, &domain_roles)
    }
}
