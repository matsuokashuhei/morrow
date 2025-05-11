use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};

use crate::application::dtos::user_dto::UserDTO;

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(InputObject, Clone)]
pub struct CreateUserInput {
    #[graphql(validator(min_length = 1))]
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    #[graphql(validator(min_length = 1))]
    pub name: String,
}
