use crate::{
    domain::entities::user::User,
    presentation::graphql::types::user_type::{CreateUserInput, UpdateUserInput},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Add this line

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
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
