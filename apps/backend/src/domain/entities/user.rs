use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use uuid::Uuid;

use crate::{
    application::dtos::{authentication_dto::SignUpInputDTO, user_dto::CreateUserDto},
    domain::enums::user_role::UserRole,
    infrastructure::database::models::user,
};

use super::identity_link::IdentityLink;

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub role: UserRole,
}

impl From<SignUpInputDTO> for NewUser {
    fn from(input: SignUpInputDTO) -> Self {
        Self {
            name: input.name,
            role: UserRole::default(), // Default to User role for new signups
        }
    }
}

impl From<NewUser> for user::ActiveModel {
    fn from(user: NewUser) -> Self {
        user::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(user.name),
            role: ActiveValue::Set(user.role.to_string()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub identity_links: Vec<IdentityLink>,
}

impl From<CreateUserDto> for NewUser {
    fn from(input: CreateUserDto) -> Self {
        Self {
            name: input.name,
            role: UserRole::default(), // Default to User role for new users
        }
    }
}

impl From<User> for user::ActiveModel {
    fn from(user: User) -> user::ActiveModel {
        user::ActiveModel {
            id: ActiveValue::Set(user.id),
            name: ActiveValue::Set(user.name.clone()),
            role: ActiveValue::Set(user.role.to_string()),
            ..Default::default()
        }
    }
}
