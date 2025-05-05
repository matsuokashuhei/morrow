use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::application::dtos::user_dto::{
    CreateUserInput as CreateUserDto, UpdateUserInput as UpdateUserDto,
};
use crate::application::services::UserService;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub name: String,
}

pub struct UserQuery {
    service: Arc<UserService>,
}

impl UserQuery {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }
}

#[Object]
impl UserQuery {
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let user = self.service.get_user(id).await?;

        Ok(user.map(|u| User {
            id: u.id,
            name: u.name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }))
    }

    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = self.service.get_all_users().await?;

        Ok(users
            .into_iter()
            .map(|u| User {
                id: u.id,
                name: u.name,
                created_at: u.created_at,
                updated_at: u.updated_at,
            })
            .collect())
    }
}

pub struct UserMutation {
    service: Arc<UserService>,
}

impl UserMutation {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }
}

#[Object]
impl UserMutation {
    async fn create_user(&self, _ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let dto = CreateUserDto { name: input.name };

        let user = self.service.create_user(dto).await?;

        Ok(User {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    async fn update_user(
        &self,
        _ctx: &Context<'_>,
        id: i32,
        input: UpdateUserInput,
    ) -> Result<Option<User>> {
        let dto = UpdateUserDto { name: input.name };

        let user = self.service.update_user(id, dto).await?;

        Ok(user.map(|u| User {
            id: u.id,
            name: u.name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }))
    }

    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> Result<bool> {
        Ok(self.service.delete_user(id).await?)
    }
}
