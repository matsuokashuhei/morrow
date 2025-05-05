use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::database::models::user::{
    ActiveModel as UserActiveModel, Entity as UserEntity,
};

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let active_model = UserActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(user.name.clone()),
            ..Default::default()
        };

        let model = active_model.insert(self.db.as_ref()).await?;

        Ok(User::with_id(
            model.id,
            model.name,
            model.created_at,
            model.updated_at,
        ))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let model = UserEntity::find_by_id(id).one(self.db.as_ref()).await?;

        Ok(model.map(|m| User::with_id(m.id, m.name, m.created_at, m.updated_at)))
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let models = UserEntity::find().all(self.db.as_ref()).await?;

        Ok(models
            .into_iter()
            .map(|m| User::with_id(m.id, m.name, m.created_at, m.updated_at))
            .collect())
    }

    async fn update(&self, user: User) -> Result<User> {
        let id = user
            .id
            .ok_or_else(|| anyhow::anyhow!("User ID is required for update"))?;

        // Use the concrete initialization instead of new()
        let active_model = UserActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(user.name.clone()),
            ..Default::default()
        };

        let model = active_model.update(self.db.as_ref()).await?;

        Ok(User::with_id(
            model.id,
            model.name,
            model.created_at,
            model.updated_at,
        ))
    }

    async fn delete(&self, id: i32) -> Result<()> {
        UserEntity::delete_by_id(id).exec(self.db.as_ref()).await?;
        Ok(())
    }
}
