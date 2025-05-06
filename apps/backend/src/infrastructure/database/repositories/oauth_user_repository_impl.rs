use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::domain::entities::oauth_user::{NewOAuthUser, OAuthUser};
use crate::domain::repositories::oauth_user_repository::OAuthUserRepository;
use crate::infrastructure::database::models::oauth_user;

pub struct OAuthUserRepositoryImpl {
    connection: Arc<DatabaseConnection>,
}

impl OAuthUserRepositoryImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl OAuthUserRepository for OAuthUserRepositoryImpl {
    async fn create(&self, new_oauth_user: NewOAuthUser) -> Result<OAuthUser> {
        let active_model = oauth_user::ActiveModel::from(new_oauth_user);
        let model = active_model.insert(self.connection.as_ref()).await?;

        Ok(OAuthUser::from(model))
    }
}
