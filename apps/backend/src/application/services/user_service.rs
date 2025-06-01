use anyhow::Result;
use std::sync::Arc;

use crate::application::dtos::user_dto::{CreateUserDto, UpdateUserDto, UserDTO};
use crate::domain::{repositories::user_repository::UserRepository, entities::user::NewUser};
use uuid::Uuid;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user_repository: user_repository,
        }
    }

    pub async fn create_user(&self, input: CreateUserDto) -> Result<UserDTO> {
        let user = NewUser::from(input);
        let created_user = self.user_repository.create(user).await?;
        Ok(UserDTO::from(created_user))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<UserDTO>> {
        let user = self.user_repository.find_by_id(id).await?; // id is Uuid, find_by_id now expects Uuid
        Ok(user.map(UserDTO::from))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserDTO>> {
        let users = self.user_repository.find_all().await?;
        Ok(users.into_iter().map(UserDTO::from).collect())
    }

    pub async fn update_user(&self, id: Uuid, input: UpdateUserDto) -> Result<Option<UserDTO>> {
        if let Some(mut user) = self.user_repository.find_by_id(id).await? { // id is Uuid, find_by_id now expects Uuid
            user.name = input.name;
            let updated_user = self.user_repository.update(user).await?;
            Ok(Some(UserDTO::from(updated_user)))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool> { // id is Uuid
        if let Some(_) = self.user_repository.find_by_id(id).await? { // id is Uuid, find_by_id now expects Uuid
            self.user_repository.delete(id).await?; // id is Uuid, delete now expects Uuid
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
