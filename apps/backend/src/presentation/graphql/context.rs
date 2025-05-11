use crate::application::dtos::auth_dto::AuthContextDto;

pub struct GraphQLContext {
    pub auth_context: AuthContextDto,
}

impl GraphQLContext {
    pub fn new(auth_context: AuthContextDto) -> Self {
        Self { auth_context }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_context.is_authenticated
    }

    pub fn user_id(&self) -> Option<i32> {
        self.auth_context.user_id
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.auth_context.roles.contains(&role.to_string())
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

// GraphQLコンテキストのClone実装
// Arc内のAuthContextを共有するための実装
impl Clone for GraphQLContext {
    fn clone(&self) -> Self {
        Self {
            auth_context: self.auth_context.clone(),
        }
    }
}

// デフォルト実装（未認証コンテキスト）
impl Default for GraphQLContext {
    fn default() -> Self {
        Self {
            auth_context: AuthContextDto {
                user_id: None,
                sub: None,
                email: None,
                roles: vec!["guest".to_string()],
                is_authenticated: false,
            },
        }
    }
}
