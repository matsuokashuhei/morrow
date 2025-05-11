use crate::domain::entities::auth::{AuthContext, UserRole};
use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::domain::repositories::token_verifier::{TokenClaims, TokenVerifier};
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AuthorizationUseCase: Send + Sync + 'static {
    /// アクセストークンからユーザーの認証コンテキストを生成する
    async fn get_auth_context_from_token(&self, token: Option<&str>) -> anyhow::Result<AuthContext>;
    
    /// 指定されたロールのいずれかを持っているかチェックする
    fn validate_has_any_role(&self, context: &AuthContext, roles: &[UserRole]) -> anyhow::Result<()>;
    
    /// 認証済みかどうかをチェックする
    fn validate_authenticated(&self, context: &AuthContext) -> anyhow::Result<()>;
}

pub struct AuthorizationUseCaseImpl {
    token_verifier: Arc<dyn TokenVerifier>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthorizationUseCaseImpl {
    pub fn new(
        token_verifier: Arc<dyn TokenVerifier>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            token_verifier,
            identity_link_repository,
            user_repository,
        }
    }
    
    async fn get_user_roles_from_claims(&self, claims: &TokenClaims) -> Vec<UserRole> {
        // Cognitoグループからロールを取得
        let mut roles = claims.groups
            .iter()
            .map(|group| UserRole::from(group.as_str()))
            .collect::<Vec<_>>();
            
        // デフォルトで少なくともUserロールを持つようにする
        if !roles.contains(&UserRole::Admin) && !roles.contains(&UserRole::User) {
            roles.push(UserRole::User);
        }
        
        roles
    }
}

#[async_trait]
impl AuthorizationUseCase for AuthorizationUseCaseImpl {
    async fn get_auth_context_from_token(&self, token: Option<&str>) -> anyhow::Result<AuthContext> {
        match token {
            Some(token) => {
                // トークンを検証して内容を取得
                let claims = self.token_verifier.verify_access_token(token).await?;
                
                // subからユーザーIDを検索
                let identity_link = self.identity_link_repository.find_by_sub(&claims.sub).await?;
                
                // ユーザー情報を取得
                let user = self.user_repository.find_by_id(identity_link.user_id).await?
                    .ok_or_else(|| anyhow::format_err!("User not found for id: {}", identity_link.user_id))?;
                
                // ロールを取得
                let roles = self.get_user_roles_from_claims(&claims).await;
                
                Ok(AuthContext::new_authenticated(
                    user.id,
                    claims.sub,
                    claims.email,
                    roles,
                ))
            }
            None => Ok(AuthContext::default()),
        }
    }

    fn validate_has_any_role(&self, context: &AuthContext, roles: &[UserRole]) -> anyhow::Result<()> {
        if !context.is_authenticated {
            return Err(anyhow::format_err!("User is not authenticated"));
        }

        let has_role = roles.iter().any(|role| context.has_role(role));
        
        if has_role {
            Ok(())
        } else {
            Err(anyhow::format_err!("User does not have required role"))
        }
    }

    fn validate_authenticated(&self, context: &AuthContext) -> anyhow::Result<()> {
        if context.is_authenticated {
            Ok(())
        } else {
            Err(anyhow::format_err!("User is not authenticated"))
        }
    }
}
