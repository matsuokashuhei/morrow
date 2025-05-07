use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

use crate::domain::entities::identity_link::IdentityLink;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "identity_links")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub provider: String,
    pub sub: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for IdentityLink {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            provider: model.provider,
            sub: model.sub,
            user_id: model.user_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
