use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::application::dtos::user_dto::UserDTO;
use crate::presentation::graphql::types::identity_link_type::IdentityLink;

#[derive(SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub role: String, // Convert UserRole to String for GraphQL
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub identity_links: Vec<IdentityLink>,
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: user.id,
            name: user.name,
            role: user.role.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            identity_links: user
                .identity_links
                .into_iter()
                .map(IdentityLink::from)
                .collect(),
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
