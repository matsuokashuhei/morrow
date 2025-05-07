use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::sync::Arc;

use crate::domain::entities::identity_link::{IdentityLink, NewIdentityLink};
use crate::domain::repositories::identity_link_repository::IdentityLinkRepository;
use crate::infrastructure::database::models::identity_link;

pub struct IdentityLinkRepositoryImpl {
    connection: Arc<DatabaseConnection>,
}

impl IdentityLinkRepositoryImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl IdentityLinkRepository for IdentityLinkRepositoryImpl {
    async fn create(&self, new_oauth_user: NewIdentityLink) -> Result<IdentityLink> {
        let active_model = identity_link::ActiveModel::from(new_oauth_user);
        let model = active_model.insert(self.connection.as_ref()).await?;

        Ok(IdentityLink::from(model))
    }
}
