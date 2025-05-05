use crate::domain::entities::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub name: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
