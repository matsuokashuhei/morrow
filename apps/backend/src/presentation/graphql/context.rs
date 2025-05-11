use crate::application::dtos::user_dto::UserDTO;

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user: Option<UserDTO>,
}

impl Default for UserContext {
    fn default() -> Self {
        Self { user: None }
    }
}
