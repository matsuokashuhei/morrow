use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::domain::entities::user::{NewUser, User};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::database::models::user::{
    ActiveModel as UserActiveModel, Entity as UserEntity,
};

pub struct UserRepositoryImpl {
    connection: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, new_user: NewUser) -> Result<User> {
        let active_model = UserActiveModel::from(new_user);
        let model = active_model.insert(self.connection.as_ref()).await?;

        Ok(User::from(model))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let model = UserEntity::find_by_id(id)
            .one(self.connection.as_ref())
            .await?;

        Ok(model.map(User::from))
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let models = UserEntity::find().all(self.connection.as_ref()).await?;

        Ok(models.into_iter().map(User::from).collect())
    }

    async fn update(&self, user: User) -> Result<User> {
        let active_model = UserActiveModel::from(user);
        let model = active_model.update(self.connection.as_ref()).await?;

        Ok(User::from(model))
    }

    async fn delete(&self, id: i32) -> Result<()> {
        UserEntity::delete_by_id(id)
            .exec(self.connection.as_ref())
            .await?;
        Ok(())
    }
}
