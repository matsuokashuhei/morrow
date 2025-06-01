use crate::{
    domain::entities::user::User,
    domain::enums::user_role::UserRole,
    presentation::graphql::types::user_type::{CreateUserInput, UpdateUserInput},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::identity_link_dto::IdentityLinkDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub identity_links: Vec<IdentityLinkDto>,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            identity_links: user.identity_links.into_iter().map(IdentityLinkDto::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}

impl From<CreateUserInput> for CreateUserDto {
    fn from(input: CreateUserInput) -> Self {
        Self { name: input.name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
}

impl From<UpdateUserInput> for UpdateUserDto {
    fn from(input: UpdateUserInput) -> Self {
        Self { name: input.name }
    }
}
