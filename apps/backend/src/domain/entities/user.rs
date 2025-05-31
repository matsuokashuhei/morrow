use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use tokio::task::Id;
use uuid::Uuid;

use crate::{
    application::dtos::authentication_dto::SignUpInputDTO, infrastructure::database::models::user,
};

use super::identity_link::IdentityLink;

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
}

impl From<SignUpInputDTO> for NewUser {
    fn from(input: SignUpInputDTO) -> Self {
        Self { name: input.name }
    }
}

impl From<NewUser> for user::ActiveModel {
    fn from(user: NewUser) -> Self {
        user::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(user.name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub identity_links: Vec<IdentityLink>,
}

// impl From<CreateUserDto> for NewUser {
//     fn from(input: CreateUserDto) -> Self {
//         Self { name: input.name }
//     }
// }

impl From<User> for user::ActiveModel {
    fn from(user: User) -> user::ActiveModel {
        user::ActiveModel {
            id: ActiveValue::Set(user.id),
            name: ActiveValue::Set(user.name.clone()),
            ..Default::default()
        }
    }
}
