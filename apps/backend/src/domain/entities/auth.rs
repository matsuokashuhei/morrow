/// アプリケーションにおけるユーザーロールを定義
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for UserRole {
    fn from(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            _ => UserRole::Guest,
        }
    }
}

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        Self::from(role.as_str())
    }
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }
}

/// ユーザーの認証コンテキスト
/// GraphQLリゾルバやアプリケーションサービスで使用
pub struct AuthContext {
    pub user_id: Option<i32>,
    pub sub: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<UserRole>,
    pub is_authenticated: bool,
}

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            user_id: None,
            sub: None,
            email: None,
            roles: vec![UserRole::Guest],
            is_authenticated: false,
        }
    }
}

impl AuthContext {
    pub fn new_authenticated(
        user_id: i32,
        sub: String,
        email: Option<String>,
        roles: Vec<UserRole>,
    ) -> Self {
        Self {
            user_id: Some(user_id),
            sub: Some(sub),
            email,
            roles,
            is_authenticated: true,
        }
    }

    pub fn has_role(&self, role: &UserRole) -> bool {
        self.roles.contains(role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&UserRole::Admin)
    }
}
